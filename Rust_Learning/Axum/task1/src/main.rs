use task1::start_server;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let file_appender = rolling::daily("logs", "app.log");

    tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_env_filter(EnvFilter::new("tower_http=trace")) // Logs only info+ level
        .init();

    info!("Logging initialized - now saving to logs/app.log");
    start_server().await;
}
