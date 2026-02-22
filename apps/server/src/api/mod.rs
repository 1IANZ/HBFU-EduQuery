mod auth;
mod config;
mod course;
mod dekt;
mod elective;
mod exam;
mod image;
mod info;
mod plan;
mod score;
mod semester;
mod user;

use crate::app::AppState;
use crate::app::error::{ApiError, ApiResult};
use axum::Router;

pub fn create_router() -> Router<AppState> {
    let user_router: Router<AppState> = user::create_router().with_state(());
    let image_router: Router<AppState> = image::create_router().with_state(());
    let config_router: Router<AppState> = config::create_router().with_state(());

    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/user", user_router)
                .nest("/auth", auth::create_router())
                .nest("/image", image_router)
                .nest("/config", config_router)
                .fallback(async || -> ApiResult<()> {
                    tracing::warn!("Not Found");
                    Err(ApiError::NotFound)
                }),
        )
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        })
}
