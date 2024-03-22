#[macro_use]
extern crate tracing;

use futures::future;
use idos_runtime::*;
use tokio::task::JoinHandle;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    let cfg = Config::load()?;

    let mut futures: Vec<JoinHandle<anyhow::Result<()>>> = Vec::new();

    // init bonsai query subscription
    futures.push(bonsai_query(&cfg));

    // init web server
    let state = AppState::new(&cfg).await?;
    futures.push(web_server(state)?);

    if let (Err(err), idx, remaining) = future::select_all(futures).await {
        error!("error in {}: {}", idx, err);
        for future in remaining {
            future.abort();
        }
        anyhow::bail!("error: {}", err);
    }
    Ok(())
}
