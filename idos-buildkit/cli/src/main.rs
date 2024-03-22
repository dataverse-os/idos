#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if let Err(e) = idos_cli::cli().await {
        log::error!("main process failed: {e:#}");
        std::process::exit(1);
    }
}
