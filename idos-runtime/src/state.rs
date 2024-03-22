use std::sync::Arc;

use crate::bonsai;
use anyhow::Ok;

#[derive(Clone)]
pub struct AppState {
    pub db_client: Arc<database::Client>,
    pub bonsai_client: Arc<bonsai::Client>,
}

impl AppState {
    pub async fn new(cfg: &crate::config::Config) -> anyhow::Result<Self> {
        let bonsai_client = bonsai::Client::new(&cfg).await?;
        let db_client = database::Client::new(&cfg.dsn)?;
        Ok(Self {
            db_client: db_client.into(),
            bonsai_client: bonsai_client.into(),
        })
    }
}

pub async fn load_query_payload(input: serde_json::Value) -> anyhow::Result<serde_json::Value> {
    //TODO query payload
    Ok(input)
}
