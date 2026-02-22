mod server;

pub use server::ServerConfig;
use std::sync::LazyLock;

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::new());

#[derive(Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
}
impl AppConfig {
    pub fn new() -> Self {
        Self {
            server: ServerConfig { port: Some(3000) },
        }
    }
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
}
pub fn get() -> &'static AppConfig {
    &CONFIG
}
