use axum::extract::Path;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::{Router, routing};
use std::path::PathBuf;

pub fn create_router() -> Router {
    Router::new().route("/{filename}", routing::get(get_image))
}

async fn get_image(Path(filename): Path<String>) -> Response {
    // 图片文件夹路径
    let image_dir = PathBuf::from("image");
    let file_path = image_dir.join(&filename);

    // 安全检查：防止目录遍历攻击
    if let Ok(canonical_path) = file_path.canonicalize() {
        if let Ok(canonical_dir) = image_dir.canonicalize() {
            if !canonical_path.starts_with(&canonical_dir) {
                return (StatusCode::FORBIDDEN, "禁止访问").into_response();
            }
        }
    }

    // 检查文件是否存在
    if !file_path.exists() {
        return (StatusCode::NOT_FOUND, "图片不存在").into_response();
    }

    // 检查是否是文件
    if !file_path.is_file() {
        return (StatusCode::BAD_REQUEST, "无效的文件路径").into_response();
    }

    // 读取文件内容
    match std::fs::read(&file_path) {
        Ok(contents) => {
            // 确定 MIME 类型
            let content_type = match file_path.extension().and_then(|e| e.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("webp") => "image/webp",
                Some("svg") => "image/svg+xml",
                _ => "application/octet-stream",
            };

            ([(header::CONTENT_TYPE, content_type)], contents).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "读取文件失败").into_response(),
    }
}
