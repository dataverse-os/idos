#[macro_use]
extern crate serde;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate tracing;

use idos_types::Proof;
use serde_json::Value;

pub mod http;
pub mod kubo;

#[async_trait]
pub trait PayloadHandler {
    async fn new_with_config(cfg: serde_json::Value) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn handle_payload(&self, value: &mut Value) -> anyhow::Result<Proof>;
}

pub struct None;

#[async_trait]
impl PayloadHandler for None {
    async fn new_with_config(_cfg: serde_json::Value) -> anyhow::Result<Self> {
        Ok(None)
    }

    async fn handle_payload(&self, _value: &mut Value) -> anyhow::Result<Proof> {
        anyhow::bail!("datasource none not support")
    }
}
