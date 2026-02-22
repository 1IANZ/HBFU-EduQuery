use crate::api::course::{CourseSchedule, parse_course_schedule};
use crate::api::dekt::{DEKT, DEKTDetail, parse_dekt, parse_dekt_detail};
use crate::api::elective::{ElectiveResponse, parse_elective};
use crate::api::exam::{ExamSchedule, parse_exam};
use crate::api::info::{StudentInfo, parse_student_info};
use crate::api::plan::{ExecutionPlanResponse, parse_plan};
use crate::api::score::{Score, parse_score_all};
use crate::api::semester::{SemesterInfo, parse_semester};
use crate::app::error::{ApiError, ApiResult};
use crate::app::response::ApiResponse;
use axum::extract::{Json, Path, Query};
use axum::{Router, routing};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

pub fn create_router() -> Router {
    Router::new()
        .route("/info/{id}", routing::post(info))
        .route("/semester/{id}", routing::post(semester))
        .route("/score/{id}", routing::post(score))
        .route("/exam/{id}", routing::post(exam))
        .route("/elective/{id}", routing::post(elective))
        .route("/plan/{id}", routing::post(plan))
        .route("/course/{id}", routing::post(course))
        .nest(
            "/dekt",
            Router::new()
                .route("/{id}", routing::post(dekt))
                .route("/detail/{id}", routing::post(dekt_detail)),
        )
}

#[derive(Deserialize)]
pub struct BaseRequest {
    pub cookies: HashMap<String, String>,
}

// Query params for semester endpoint
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemesterQuery {
    #[serde(default)]
    pub is_all: bool,
}

// Query params for score/exam/elective/course endpoints
#[derive(Deserialize)]
pub struct SemesterQueryParam {
    pub semester: Option<String>,
}

// Query params for dekt detail endpoint
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DektDetailQuery {
    pub operation_id: Option<String>,
}

/// 从 cookies 创建临时客户端
fn get_client(cookies: &HashMap<String, String>) -> Client {
    let jar = reqwest::cookie::Jar::default();
    for (name, value) in cookies {
        let cookie_str = format!("{}={}; Domain=.hbfu.edu.cn; Path=/", name, value);
        if let Ok(url) = "https://jw.v.hbfu.edu.cn".parse() {
            jar.add_cookie_str(&cookie_str, &url);
        }
    }

    Client::builder()
        .cookie_provider(Arc::new(jar))
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(std::time::Duration::from_secs(30))
        .danger_accept_invalid_certs(true)
        .build()
        .expect("构建 HTTP 客户端失败")
}

pub async fn info(
    Path(_id): Path<String>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<StudentInfo>> {
    let client = get_client(&req.cookies);
    let res = client
        .get("https://jw.v.hbfu.edu.cn/jsxsd/grxx/xsxx")
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let data = parse_student_info(&res)?;
    Ok(ApiResponse::ok("ok", Some(data)))
}

pub async fn semester(
    Path(_id): Path<String>,
    Query(query): Query<SemesterQuery>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<Vec<SemesterInfo>>> {
    let client = get_client(&req.cookies);
    let res_text = client
        .get("https://jw.v.hbfu.edu.cn/jsxsd/xsks/xsksap_query")
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let semester_list = parse_semester(&res_text, query.is_all)?;
    Ok(ApiResponse::ok("ok", Some(semester_list)))
}

pub async fn dekt_detail(
    Path(_id): Path<String>,
    Query(query): Query<DektDetailQuery>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<DEKTDetail>> {
    let client = get_client(&req.cookies);
    let operation_id = query
        .operation_id
        .ok_or_else(|| ApiError::BadRequest("缺少operationId".to_string()))?;
    let res_text = client
        .get(format!(
            "https://jw.v.hbfu.edu.cn/jsxsd/pyfa/cxxf07View?cxxf07id={}&type=view",
            operation_id
        ))
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let data = parse_dekt_detail(&res_text)?;
    Ok(ApiResponse::ok("ok", Some(data)))
}

pub async fn plan(
    Path(_id): Path<String>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<ExecutionPlanResponse>> {
    let client = get_client(&req.cookies);
    let res_text = client
        .get("https://jw.v.hbfu.edu.cn/jsxsd/pyfa/pyfa_query")
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let data = parse_plan(&res_text)?;
    Ok(ApiResponse::ok("ok", Some(data)))
}

pub async fn dekt(
    Path(_id): Path<String>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<DEKT>> {
    let client = get_client(&req.cookies);
    let res_text = client
        .get("https://jw.v.hbfu.edu.cn/jsxsd/pyfa/cxxf07List")
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let data = parse_dekt(&res_text)?;
    Ok(ApiResponse::ok("ok", Some(data)))
}

pub async fn score(
    Path(_id): Path<String>,
    Query(query): Query<SemesterQueryParam>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<Score>> {
    let client = get_client(&req.cookies);
    let semester = query.semester.unwrap_or_default();
    let mut form_data = HashMap::new();
    form_data.insert("kksj", semester.as_str());
    form_data.insert("xsfs", "all");

    let res = client
        .post("https://jw.v.hbfu.edu.cn/jsxsd/kscj/cjcx_list")
        .form(&form_data)
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let data = parse_score_all(&res)?;
    Ok(ApiResponse::ok("ok", Some(data)))
}

pub async fn exam(
    Path(_id): Path<String>,
    Query(query): Query<SemesterQueryParam>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<Vec<ExamSchedule>>> {
    let client = get_client(&req.cookies);
    let semester = query.semester.unwrap_or_default();
    let mut form_data = HashMap::new();
    form_data.insert("xnxqid", semester.as_str());

    let res = client
        .post("https://jw.v.hbfu.edu.cn/jsxsd/xsks/xsksap_list")
        .form(&form_data)
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let data = parse_exam(&res)?;
    Ok(ApiResponse::ok("ok", Some(data)))
}

pub async fn elective(
    Path(_id): Path<String>,
    Query(query): Query<SemesterQueryParam>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<ElectiveResponse>> {
    let client = get_client(&req.cookies);
    let semester = query.semester.unwrap_or_default();
    let mut form_data = HashMap::new();
    form_data.insert("xnxqid", semester.as_str());

    let res = client
        .post("https://jw.v.hbfu.edu.cn/jsxsd/xkgl/xqxkchList")
        .form(&form_data)
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let data = parse_elective(&res)?;
    Ok(ApiResponse::ok("ok", Some(data)))
}

pub async fn course(
    Path(_id): Path<String>,
    Query(query): Query<SemesterQueryParam>,
    Json(req): Json<BaseRequest>,
) -> ApiResult<ApiResponse<Vec<CourseSchedule>>> {
    let client = get_client(&req.cookies);
    let semester = query.semester.unwrap_or_default();
    let mut form_data = HashMap::new();
    form_data.insert("xnxq01id", semester.as_str());
    form_data.insert("sfFD", "1");

    let res = client
        .post("https://jw.v.hbfu.edu.cn/jsxsd/xskb/xskb_list.do")
        .form(&form_data)
        .send()
        .await
        .map_err(|_| ApiError::RequestTimeout)?
        .text()
        .await
        .map_err(|_| ApiError::RequestTimeout)?;
    let data = parse_course_schedule(&res)?;
    Ok(ApiResponse::ok("ok", Some(data)))
}
