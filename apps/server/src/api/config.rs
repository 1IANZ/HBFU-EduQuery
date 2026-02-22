use axum::response::Json;
use axum::{Router, routing};

pub fn create_router() -> Router {
    Router::new().route("/", routing::get(get_config))
}

async fn get_config() -> Json<serde_json::Value> {
    let config_path = std::path::Path::new("data.json");

    if config_path.exists() {
        if let Ok(content) = std::fs::read_to_string(config_path) {
            if let Ok(json) = serde_json::from_str(&content) {
                return Json(json);
            }
        }
    }

    // 默认配置
    Json(serde_json::json!({
        "code": 0,
        "message": "ok",
        "data": {
            "course": {
                "semester": "2025-2026-2",
                "startDate": "2026-03-02"
            },
            "notice": {
                "messages": []
            }
        }
    }))
}
