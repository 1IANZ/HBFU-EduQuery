use crate::config;
use crate::jwxt::session::HttpSession;
use axum::Router;
use server::Server;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::interval;

pub mod error;
mod laytency;
mod logger;
pub mod response;
mod server;

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("Starting server");
    let state = AppState::new();
    tokio::spawn(session_cleanup_task(state.clone()));
    let server = Server::new(config::get().server());
    server.start(state, router).await
}

#[derive(Clone)]
pub struct AppState {
    pub temporary_sessions: Arc<Mutex<HashMap<String, TempSessionEntry>>>,
}
pub struct TempSessionEntry {
    pub session: HttpSession,
    pub flow_execution_key: String,
    pub created_at: Instant,
    /// QR code UUID for QR login flow
    pub qr_uuid: Option<String>,
    /// Student ID obtained from QR scan
    pub student_id: Option<String>,
    /// Flag indicating if VPN login has been completed
    pub vpn_logged_in: bool,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            temporary_sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub async fn cleanup_expired_sessions(&self) {
        let mut temp_sessions = self.temporary_sessions.lock().await;
        let now = Instant::now();
        temp_sessions
            .retain(|_, entry| now.duration_since(entry.created_at) < Duration::from_secs(300));
    }
}
async fn session_cleanup_task(state: AppState) {
    let mut interval = interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        state.cleanup_expired_sessions().await;
    }
}
