#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod bonsai;
mod config;
mod error;
mod extension;
mod handlers;
mod response;
mod state;

pub use crate::config::*;
pub use crate::state::*;
use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Context;
use handlers::*;
use tokio::task::JoinHandle;

pub fn bonsai_query(cfg: &Config) -> JoinHandle<anyhow::Result<()>> {
    let cfg = cfg.clone();
    tokio::spawn(async move {
        let bonsai_client = bonsai::Client::new(&cfg).await?;
        let extension_client = extension::Client::new(&cfg.contract).await?;
        extension_client
            .subscribe_on_query(&bonsai_client)
            .await
            .context("bonsai subscribe error")
    })
}

pub fn web_server(state: AppState) -> anyhow::Result<JoinHandle<anyhow::Result<()>>> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(run_zkvm)
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    let web = tokio::spawn(async { server.await.context("server error") });
    return Ok(web);
}
