use anyhow::Context;
use idos_types::Proof;
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;

pub static BASE_URL: &str = "https://ceramic-clay.3boxlabs.com";

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: Option<String>) -> Self {
        Client {
            base_url: base_url.unwrap_or_else(|| BASE_URL.into()),
        }
    }

    pub async fn get_content(&self, stream_id: &str) -> Result<Value, Error> {
        let response = reqwest::get(format!(
            "{}/api/v0/streams/{}/content",
            self.base_url, stream_id
        ))
        .await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    pub async fn get_content_with_proof(
        &self,
        stream_id: &str,
        proof: &mut Proof,
    ) -> anyhow::Result<Value> {
        let response =
            reqwest::get(format!("{}/api/v0/streams/{}", self.base_url, stream_id)).await?;
        let resp: StreamResponse = response.json().await?;

        proof.streams.insert(stream_id.into(), resp.state.tip()?);
        Ok(resp.state.content)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct StreamResponse {
    state: Stream,
}

#[derive(Debug, Clone, Deserialize)]
struct Stream {
    content: Value,
    log: Vec<Log>,
}

impl Stream {
    fn tip(&self) -> anyhow::Result<String> {
        Ok(self.log.last().context("missing log")?.cid.clone())
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Log {
    cid: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_get_content() {
        let ceramic = Client::new(None);
        let stream_id = "kjzl6kcym7w8y7bq5vaz6mcx2dzskob3zl0v0eszkgs91ntj0uwq12bztdbd73g";
        let result = ceramic.get_content(stream_id).await;
        assert!(result.is_ok());

        let mut proof = Proof::default();

        let result2 = ceramic.get_content_with_proof(stream_id, &mut proof).await;
        assert!(result2.is_ok());

        assert_eq!(result.unwrap(), result2.unwrap());

        assert_eq!(
            proof.streams["kjzl6kcym7w8y7bq5vaz6mcx2dzskob3zl0v0eszkgs91ntj0uwq12bztdbd73g"],
            "bafyreiesojhur4wvg5n5oefobrqbiolubz2bnjwsgkrwcctmwxl65imhrq"
        );
    }
}
