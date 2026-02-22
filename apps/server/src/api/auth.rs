use crate::app::AppState;
use crate::app::error::{ApiError, ApiResult};
use crate::app::response::ApiResponse;
use crate::jwxt::session::HttpSession;
use axum::extract::{Json, Query, State};
use axum::{Router, routing};
use base64::Engine;
use base64::engine::general_purpose;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/captcha", routing::get(get_captcha))
        .route("/login", routing::post(login))
        // QR code login endpoints
        .route("/qrcode/uuid", routing::get(get_qrcode_uuid))
        .route("/qrcode/status", routing::get(get_qrcode_status))
        .route("/qrcode/login", routing::post(qrcode_login))
        // Separate login endpoints
        .route("/login/vpn/manual", routing::post(login_vpn_manual))
        .route("/login/vpn/qrcode", routing::post(login_vpn_qrcode))
        .route("/login/jwxt", routing::post(login_jwxt))
}

// ============ Common Response Types ============

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptchaResponse {
    pub captcha_base64: String,
    pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub cookies: HashMap<String, String>,
}

// ============ Captcha Login (Manual Mode) ============

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub student_id: String,
    pub vpn_password: String,
    pub oa_password: String,
    pub captcha: String,
    pub session_id: String,
}

pub async fn get_captcha(State(state): State<AppState>) -> ApiResult<ApiResponse<CaptchaResponse>> {
    let session = HttpSession::new();

    let flow_execution_key = session.get_flow_execution_key().await?;
    let bytes = session.get_captcha().await?;

    // 生成 session_id (使用 xid)
    let session_id = xid::new().to_string();

    // 将 session 存储到临时会话中，使用 session_id 作为 key
    let mut temp_sessions = state.temporary_sessions.lock().await;
    temp_sessions.insert(
        session_id.clone(),
        crate::app::TempSessionEntry {
            session,
            flow_execution_key,
            created_at: std::time::Instant::now(),
            qr_uuid: None,
            student_id: None,
            vpn_logged_in: false,
        },
    );

    let captcha_base64_img = general_purpose::STANDARD.encode(&bytes);

    Ok(ApiResponse::ok(
        "ok",
        Some(CaptchaResponse {
            captcha_base64: format!("data:image/jpeg;base64,{}", captcha_base64_img),
            session_id,
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(data): Json<LoginRequest>,
) -> ApiResult<ApiResponse<LoginResponse>> {
    state.cleanup_expired_sessions().await;

    // 从临时会话中获取之前创建的 session
    let mut temp_sessions = state.temporary_sessions.lock().await;
    let temp_entry = temp_sessions
        .remove(&data.session_id)
        .ok_or_else(|| ApiError::BadRequest("会话已过期,请重新登录".to_string()))?;
    drop(temp_sessions);

    let session = temp_entry.session;
    let flow_execution_key = temp_entry.flow_execution_key;

    let result = session
        .complete_login(
            &data.student_id,
            &data.vpn_password,
            &data.oa_password,
            &data.captcha,
            &flow_execution_key,
        )
        .await?;

    if result == "登录成功" {
        let cookies = session.extract_cookies();
        Ok(ApiResponse::ok("登录成功", Some(LoginResponse { cookies })))
    } else {
        Err(ApiError::BadRequest(result).into())
    }
}

// ============ QR Code Login Endpoints ============

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QRCodeUUIDResponse {
    pub qr_uuid: String,
    pub session_id: String,
    pub qr_code_image: String,
}

/// 获取二维码 UUID 接口
pub async fn get_qrcode_uuid(
    State(state): State<AppState>,
) -> ApiResult<ApiResponse<QRCodeUUIDResponse>> {
    let session = HttpSession::new();

    // 获取 QR UUID 和图片
    let (qr_uuid, qr_image_bytes) = session.get_qrcode_uuid().await?;

    // 生成临时会话ID
    let session_id = xid::new().to_string();

    // 获取 flow execution key 用于后续登录
    let flow_execution_key = session.get_flow_execution_key().await?;

    // 保存会话和 UUID
    let mut temp_sessions = state.temporary_sessions.lock().await;
    temp_sessions.insert(
        session_id.clone(),
        crate::app::TempSessionEntry {
            session,
            flow_execution_key,
            created_at: std::time::Instant::now(),
            qr_uuid: Some(qr_uuid.clone()),
            student_id: None,
            vpn_logged_in: false,
        },
    );

    // 返回结果（包含二维码图片的 base64）
    let qr_image_b64 = general_purpose::STANDARD.encode(&qr_image_bytes);

    Ok(ApiResponse::ok(
        "ok",
        Some(QRCodeUUIDResponse {
            qr_uuid,
            session_id,
            qr_code_image: format!("data:image/png;base64,{}", qr_image_b64),
        }),
    ))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QRCodeStatusQuery {
    pub session_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QRCodeStatusResponse {
    pub status: String,
    pub student_id: Option<String>,
}

/// 轮询二维码扫码状态
pub async fn get_qrcode_status(
    State(state): State<AppState>,
    Query(query): Query<QRCodeStatusQuery>,
) -> ApiResult<ApiResponse<QRCodeStatusResponse>> {
    // 获取保存的会话 (不移除,因为还需要用于登录)
    let mut temp_sessions = state.temporary_sessions.lock().await;
    let temp_entry = temp_sessions
        .get_mut(&query.session_id)
        .ok_or_else(|| ApiError::BadRequest("会话已过期,请重新获取二维码".to_string()))?;

    let qr_uuid = temp_entry
        .qr_uuid
        .clone()
        .ok_or_else(|| ApiError::BadRequest("会话数据异常".to_string()))?;

    // 轮询状态
    let (status, student_id) = temp_entry.session.poll_qrcode_status(&qr_uuid).await?;

    // 如果扫码成功，保存学号到 metadata
    if status == "found" && student_id.is_some() {
        temp_entry.student_id = student_id.clone();
    }

    Ok(ApiResponse::ok(
        "ok",
        Some(QRCodeStatusResponse { status, student_id }),
    ))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QRCodeLoginRequest {
    pub session_id: String,
    pub oa_password: String,
}

/// 完成二维码登录
pub async fn qrcode_login(
    State(state): State<AppState>,
    Json(data): Json<QRCodeLoginRequest>,
) -> ApiResult<ApiResponse<LoginResponse>> {
    let mut temp_sessions = state.temporary_sessions.lock().await;
    let temp_entry = temp_sessions
        .get_mut(&data.session_id)
        .ok_or_else(|| ApiError::BadRequest("会话已过期,请重新登录".to_string()))?;

    let qr_uuid = temp_entry
        .qr_uuid
        .clone()
        .ok_or_else(|| ApiError::BadRequest("会话数据异常,请重新扫码".to_string()))?;
    let student_id = temp_entry
        .student_id
        .clone()
        .ok_or_else(|| ApiError::BadRequest("会话数据异常,请重新扫码".to_string()))?;

    let flow_execution_key = temp_entry.flow_execution_key.clone();
    let vpn_logged_in = temp_entry.vpn_logged_in;

    // 使用 QR 码登录
    let result = temp_entry
        .session
        .complete_qrcode_login(
            &student_id,
            &qr_uuid,
            &data.oa_password,
            &flow_execution_key,
            vpn_logged_in,
        )
        .await?;

    if result == "登录成功" {
        let cookies = temp_entry.session.extract_cookies();
        // 只在登录成功时移除会话
        temp_sessions.remove(&data.session_id);
        Ok(ApiResponse::ok("登录成功", Some(LoginResponse { cookies })))
    } else {
        // 登录失败时，标记 VPN 已登录（如果 VPN 登录成功的话）
        if result.contains("教务系统登录失败") {
            temp_entry.vpn_logged_in = true;
        }
        Err(ApiError::BadRequest(result).into())
    }
}

// ============ Separate Login Endpoints ============

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpnLoginRequest {
    pub student_id: String,
    pub vpn_password: String,
    pub captcha: String,
    pub session_id: String,
}

/// VPN 登录接口 - 手动模式 (仅登录 VPN)
pub async fn login_vpn_manual(
    State(state): State<AppState>,
    Json(data): Json<VpnLoginRequest>,
) -> ApiResult<ApiResponse<LoginResponse>> {
    let mut temp_sessions = state.temporary_sessions.lock().await;
    let temp_entry = temp_sessions
        .remove(&data.session_id)
        .ok_or_else(|| ApiError::BadRequest("会话已过期,请重新登录".to_string()))?;
    drop(temp_sessions);

    let session = temp_entry.session;
    let flow_execution_key = temp_entry.flow_execution_key;

    let result = session
        .complete_vpn_manual_login(
            &data.student_id,
            &data.vpn_password,
            &data.captcha,
            &flow_execution_key,
        )
        .await?;

    if result == "登录成功" {
        let cookies = session.extract_cookies();
        Ok(ApiResponse::ok("登录成功", Some(LoginResponse { cookies })))
    } else {
        Err(ApiError::BadRequest(result).into())
    }
}

/// VPN 登录接口 - 扫码模式 (仅登录 VPN)
pub async fn login_vpn_qrcode(
    State(state): State<AppState>,
    Json(data): Json<QRCodeLoginRequest>,
) -> ApiResult<ApiResponse<LoginResponse>> {
    let mut temp_sessions = state.temporary_sessions.lock().await;
    let temp_entry = temp_sessions
        .get_mut(&data.session_id)
        .ok_or_else(|| ApiError::BadRequest("会话已过期,请重新登录".to_string()))?;

    let qr_uuid = temp_entry
        .qr_uuid
        .clone()
        .ok_or_else(|| ApiError::BadRequest("会话数据异常,请重新扫码".to_string()))?;
    let student_id = temp_entry
        .student_id
        .clone()
        .ok_or_else(|| ApiError::BadRequest("会话数据异常,请重新扫码".to_string()))?;
    let flow_execution_key = temp_entry.flow_execution_key.clone();

    let result = temp_entry
        .session
        .complete_vpn_qrcode_login(&student_id, &qr_uuid, &flow_execution_key)
        .await?;

    if result == "登录成功" {
        let cookies = temp_entry.session.extract_cookies();
        // 登录成功，关闭服务端 session，返回 cookies 给客户端保管
        temp_sessions.remove(&data.session_id);
        Ok(ApiResponse::ok("登录成功", Some(LoginResponse { cookies })))
    } else {
        // 登录失败，保留会话以便用户重试
        Err(ApiError::BadRequest(result).into())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwxtLoginRequest {
    pub student_id: String,
    pub oa_password: String,
    pub cookies: HashMap<String, String>,
}

/// 教务系统登录接口 (使用现有 VPN Cookies)
pub async fn login_jwxt(
    Json(data): Json<JwxtLoginRequest>,
) -> ApiResult<ApiResponse<LoginResponse>> {
    // 使用传入的 cookies 初始化 HttpSession
    let session = HttpSession::with_cookies(&data.cookies);

    let result = session
        .complete_jwxt_login_only(&data.student_id, &data.oa_password)
        .await?;

    if result == "登录成功" {
        let cookies = session.extract_cookies();
        Ok(ApiResponse::ok("登录成功", Some(LoginResponse { cookies })))
    } else {
        Err(ApiError::BadRequest(result).into())
    }
}
