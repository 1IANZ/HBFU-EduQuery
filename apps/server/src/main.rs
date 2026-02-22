mod api;
mod app;
mod config;
mod jwxt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await?;
    Ok(())
}
