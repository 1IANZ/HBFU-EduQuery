use crate::jwxt::aescbc::aes_cbc_encrypt;
use crate::jwxt::conwork::encode_inp;
use crate::jwxt::error::SessionError;
use regex::Regex;
use reqwest::Client;
use reqwest::header::HeaderMap;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex, RawCookie};
use std::collections::HashMap;
use std::sync::Arc;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36";

pub struct HttpSession {
    pub client: Client,
    pub _cookie_store: Arc<CookieStoreMutex>,
}

impl HttpSession {
    pub fn new() -> Self {
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::default()));
        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .user_agent(USER_AGENT)
            .danger_accept_invalid_certs(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("构建 HTTP 客户端失败");

        Self {
            client,
            _cookie_store: cookie_store,
        }
    }

    /// 使用现有 cookies 初始化 HttpSession
    pub fn with_cookies(cookies: &HashMap<String, String>) -> Self {
        let cookie_store = CookieStore::default();
        let cookie_store = Arc::new(CookieStoreMutex::new(cookie_store));

        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .user_agent(USER_AGENT)
            .danger_accept_invalid_certs(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("构建 HTTP 客户端失败");

        // 手动添加 cookies 到 store
        {
            let mut store = cookie_store.lock().unwrap();
            for (name, value) in cookies {
                // 跳过重复的 JSESSIONID，只保留最后一个
                if name.to_uppercase() == "JSESSIONID" {
                    // 先尝试删除旧的
                    let url = "https://jw.v.hbfu.edu.cn".parse().unwrap();
                    if let Ok(cookie) =
                        RawCookie::parse(format!("{}={}; Path=/; Domain=.hbfu.edu.cn", name, value))
                    {
                        let _ = store.insert_raw(&cookie, &url);
                    }
                } else {
                    let url = "https://jw.v.hbfu.edu.cn".parse().unwrap();
                    if let Ok(cookie) =
                        RawCookie::parse(format!("{}={}; Path=/; Domain=.hbfu.edu.cn", name, value))
                    {
                        let _ = store.insert_raw(&cookie, &url);
                    }
                }
            }
        }

        Self {
            client,
            _cookie_store: cookie_store,
        }
    }

    pub async fn get_captcha(&self) -> Result<Vec<u8>, SessionError> {
        let r = rand::random::<f64>();
        let url = format!(
            "https://oa-443.v.hbfu.edu.cn/backstage/cas/captcha.jpg?r={}",
            r
        );
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "text/html;charset=utf-8".parse().unwrap());
        headers.insert("Vary", "Accept-Encoding".parse().unwrap());

        let resp = self.client.get(&url).headers(headers).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(SessionError::Custom(format!(
                "获取验证码失败: HTTP {}",
                status
            )));
        }
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn get_flow_execution_key(&self) -> Result<String, SessionError> {
        let response = self
            .client
            .get("https://oa-443.v.hbfu.edu.cn/backstage/cas/login")
            .send()
            .await?;
        let text = response.text().await?;
        let pattern = Regex::new(r#"flowExecutionKey: "(.*?)""#)?;

        if let Some(captures) = pattern.captures(&text) {
            if let Some(matched) = captures.get(1) {
                Ok(matched.as_str().to_string())
            } else {
                Err(SessionError::PatternNotFound)
            }
        } else {
            Err(SessionError::PatternNotFound)
        }
    }

    async fn login_vpn(
        &self,
        username: &str,
        password: &str,
        flow_execution_key: &str,
        captcha: &str,
    ) -> Result<(bool, String), SessionError> {
        let encrypted_password = aes_cbc_encrypt(password)
            .map_err(|_| SessionError::Custom("密码加密失败".to_string()))?;
        let mut form_data = HashMap::new();
        form_data.insert("username", username);
        form_data.insert("password", &encrypted_password);
        form_data.insert("execution", flow_execution_key);
        form_data.insert("_eventId", "submit");
        form_data.insert("captcha", captcha);
        form_data.insert("rememberMe", "false");
        form_data.insert("domain", "oa-443.v.hbfu.edu.cn");
        form_data.insert("geolocation", "");
        form_data.insert("tenantId", "");

        let res = self
            .client
            .post("https://oa-443.v.hbfu.edu.cn/backstage/cas/login")
            .form(&form_data)
            .send()
            .await?;
        let text = res.text().await?;

        // 登录成功检查
        if text.contains("修改密码") {
            return Ok((true, "Success".to_string()));
        }

        // 提取 CAS bridgeData.errors 错误信息
        let bridge_error_re = Regex::new(r#"errors:\s*\["(.*?)"\]"#)?;
        if let Some(caps) = bridge_error_re.captures(&text) {
            if let Some(m) = caps.get(1) {
                return Ok((false, m.as_str().to_string()));
            }
        }

        // 提取 div.errors 中的错误信息
        let error_div_re = Regex::new(r#"<div[^>]*class="[^"]*errors[^"]*"[^>]*>(.*?)</div>"#)?;
        if let Some(caps) = error_div_re.captures(&text) {
            if let Some(m) = caps.get(1) {
                // 移除 HTML 标签
                let tag_re = Regex::new(r"<[^>]+>")?;
                let error_msg = tag_re.replace_all(m.as_str(), "").trim().to_string();
                if !error_msg.is_empty() {
                    return Ok((false, error_msg));
                }
            }
        }

        Ok((false, "未知登录错误".to_string()))
    }

    /// 使用二维码登录 VPN
    async fn login_vpn_with_qrcode(
        &self,
        username: &str,
        qr_uuid: &str,
        flow_execution_key: &str,
    ) -> Result<(bool, String), SessionError> {
        // QR 码登录密码：直接使用 qrcode{UUID}，不加密
        let qr_password = format!("qrcode{}", qr_uuid);

        let mut form_data = HashMap::new();
        form_data.insert("username", username);
        form_data.insert("password", qr_password.as_str());
        form_data.insert("execution", flow_execution_key);
        form_data.insert("_eventId", "submit");
        form_data.insert("captcha", "");
        form_data.insert("rememberMe", "false");
        form_data.insert("domain", "oa-443.v.hbfu.edu.cn");
        form_data.insert("geolocation", "");
        form_data.insert("tenantId", "");

        let res = self
            .client
            .post("https://oa-443.v.hbfu.edu.cn/backstage/cas/login")
            .form(&form_data)
            .send()
            .await?;
        let text = res.text().await?;

        if text.contains("修改密码") {
            return Ok((true, "Success".to_string()));
        }

        // 提取错误信息
        let bridge_error_re = Regex::new(r#"errors:\s*\["(.*?)"\]"#)?;
        if let Some(caps) = bridge_error_re.captures(&text) {
            if let Some(m) = caps.get(1) {
                return Ok((false, m.as_str().to_string()));
            }
        }

        let error_div_re = Regex::new(r#"<div[^>]*class="[^"]*errors[^"]*"[^>]*>(.*?)</div>"#)?;
        if let Some(caps) = error_div_re.captures(&text) {
            if let Some(m) = caps.get(1) {
                let tag_re = Regex::new(r"<[^>]+>")?;
                let error_msg = tag_re.replace_all(m.as_str(), "").trim().to_string();
                if !error_msg.is_empty() {
                    return Ok((false, error_msg));
                }
            }
        }

        Ok((false, "未知登录错误".to_string()))
    }

    pub async fn access_jwxt(&self) -> Result<(), SessionError> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "text/html;charset=utf-8".parse().unwrap());
        headers.insert("Vary", "Accept-Encoding".parse().unwrap());

        self.client
            .get("https://jw.v.hbfu.edu.cn/")
            .headers(headers)
            .send()
            .await?;
        Ok(())
    }

    async fn login_jwxt(&self, username: &str, password: &str) -> Result<bool, SessionError> {
        let encoded_username = encode_inp(username);
        let encoded_password = encode_inp(password);
        let encoded_data = format!("{}%%%{}", encoded_username, encoded_password);

        let response = self
            .client
            .post("https://jw.v.hbfu.edu.cn/jsxsd/xk/LoginToXk")
            .form(&[("encoded", &encoded_data)])
            .send()
            .await?;
        let res = response.text().await?.contains("学生个人中心");
        Ok(res)
    }

    /// 完成手动登录流程 (VPN + JWXT)
    pub async fn complete_login(
        &self,
        username: &str,
        vpn_password: &str,
        oa_password: &str,
        captcha: &str,
        flow_execution_key: &str,
    ) -> Result<String, SessionError> {
        let (vpn_success, vpn_msg) = self
            .login_vpn(username, vpn_password, flow_execution_key, captcha)
            .await?;
        if !vpn_success {
            return Ok(format!("VPN登录失败: {}", vpn_msg));
        }

        self.access_jwxt().await?;

        let jwxt_login_result = self.login_jwxt(username, oa_password).await?;
        if !jwxt_login_result {
            return Ok("教务系统登录失败".to_string());
        }

        Ok("登录成功".to_string())
    }

    // ============ QR Code Login Methods ============

    /// 获取二维码 UUID 和二维码图片
    pub async fn get_qrcode_uuid(&self) -> Result<(String, Vec<u8>), SessionError> {
        // 1. 获取 UUID
        let url = "https://oa-443.v.hbfu.edu.cn/backstage/cas/qr/getQRCodeUUID";
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            return Err(SessionError::Custom(format!(
                "获取二维码UUID失败: HTTP {}",
                resp.status()
            )));
        }

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|_| SessionError::Custom("解析二维码UUID响应失败".to_string()))?;

        if !data
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let msg = data
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("未知错误");
            return Err(SessionError::Custom(format!("获取二维码UUID失败: {}", msg)));
        }

        let qr_uuid = data
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SessionError::Custom("二维码UUID数据缺失".to_string()))?
            .to_string();

        // 2. 获取二维码图片
        let qr_image_url = format!(
            "https://oa-443.v.hbfu.edu.cn/backstage/app-manager/qrcode/generate?key=login&ext={}",
            qr_uuid
        );
        let qr_resp = self.client.get(&qr_image_url).send().await?;

        if !qr_resp.status().is_success() {
            return Err(SessionError::Custom(format!(
                "获取二维码图片失败: HTTP {}",
                qr_resp.status()
            )));
        }

        let qr_image_bytes = qr_resp.bytes().await?.to_vec();

        Ok((qr_uuid, qr_image_bytes))
    }

    /// 轮询二维码扫码状态
    pub async fn poll_qrcode_status(
        &self,
        qr_uuid: &str,
    ) -> Result<(String, Option<String>), SessionError> {
        let url = format!(
            "https://oa-443.v.hbfu.edu.cn/backstage/cas/qr/findBindUser?qrCodeUUID={}",
            qr_uuid
        );
        let resp = self.client.post(&url).send().await?;

        if !resp.status().is_success() {
            return Err(SessionError::Custom(format!(
                "轮询二维码状态失败: HTTP {}",
                resp.status()
            )));
        }

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|_| SessionError::Custom("解析二维码状态响应失败".to_string()))?;

        let success = data
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let message = data.get("message").and_then(|v| v.as_str()).unwrap_or("");

        if success && message == "found" {
            let student_id = data
                .get("data")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            Ok(("found".to_string(), student_id))
        } else if message == "scanning" {
            Ok(("scanning".to_string(), None))
        } else {
            Ok(("not_found".to_string(), None))
        }
    }

    /// 完成二维码登录流程
    pub async fn complete_qrcode_login(
        &self,
        student_id: &str,
        qr_uuid: &str,
        oa_password: &str,
        flow_execution_key: &str,
        vpn_logged_in: bool,
    ) -> Result<String, SessionError> {
        // 如果 VPN 尚未登录，先登录 VPN
        if !vpn_logged_in {
            let (vpn_success, vpn_msg) = self
                .login_vpn_with_qrcode(student_id, qr_uuid, flow_execution_key)
                .await?;

            if !vpn_success {
                return Ok(format!("VPN登录失败: {}", vpn_msg));
            }
        }

        // Access JWXT
        self.access_jwxt().await?;

        // Login JWXT
        let jwxt_success = self.login_jwxt(student_id, oa_password).await?;
        if !jwxt_success {
            return Ok("教务系统登录失败".to_string());
        }

        Ok("登录成功".to_string())
    }

    /// 仅 VPN 手动登录
    pub async fn complete_vpn_manual_login(
        &self,
        username: &str,
        vpn_password: &str,
        captcha: &str,
        flow_execution_key: &str,
    ) -> Result<String, SessionError> {
        let (vpn_success, vpn_msg) = self
            .login_vpn(username, vpn_password, flow_execution_key, captcha)
            .await?;

        if !vpn_success {
            return Ok(format!("VPN登录失败: {}", vpn_msg));
        }

        // 访问 JWXT 首页以建立 VPN 会话并收集 cookies
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "text/html;charset=utf-8".parse().unwrap());
        headers.insert("Vary", "Accept-Encoding".parse().unwrap());

        self.client
            .get("https://jw.v.hbfu.edu.cn/")
            .headers(headers)
            .send()
            .await?;

        Ok("登录成功".to_string())
    }

    /// 仅 VPN 扫码登录
    pub async fn complete_vpn_qrcode_login(
        &self,
        username: &str,
        qr_uuid: &str,
        flow_execution_key: &str,
    ) -> Result<String, SessionError> {
        let (vpn_success, vpn_msg) = self
            .login_vpn_with_qrcode(username, qr_uuid, flow_execution_key)
            .await?;

        if !vpn_success {
            return Ok(format!("VPN登录失败: {}", vpn_msg));
        }

        Ok("登录成功".to_string())
    }

    /// 使用现有 VPN cookies 仅登录教务系统
    pub async fn complete_jwxt_login_only(
        &self,
        username: &str,
        oa_password: &str,
    ) -> Result<String, SessionError> {
        // 1. 访问 JWXT 检查 VPN session 是否有效
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "text/html;charset=utf-8".parse().unwrap());
        headers.insert("Vary", "Accept-Encoding".parse().unwrap());

        let resp = self
            .client
            .get("https://jw.v.hbfu.edu.cn/")
            .headers(headers)
            .send()
            .await?;

        // 检查是否被重定向到 VPN 登录页面
        let final_url = resp.url().to_string();
        if final_url.contains("cas/login") || resp.status() == 302 {
            return Ok("VPN会话已失效，请重新登录VPN".to_string());
        }

        // 2. 登录 JWXT
        let jwxt_success = self.login_jwxt(username, oa_password).await?;
        if !jwxt_success {
            return Ok("教务系统登录失败".to_string());
        }

        Ok("登录成功".to_string())
    }

    /// 从 cookie store 提取 cookies 为 HashMap
    pub fn extract_cookies(&self) -> HashMap<String, String> {
        let store = self._cookie_store.lock().unwrap();
        let mut cookies = HashMap::new();
        for cookie in store.iter_any() {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());
        }
        cookies
    }
}
