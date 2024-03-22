use global::*;
use idos_types::Proof;
use stream::Client;

use crate::PayloadHandler;

mod file;
mod global;
mod stream;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub stream_url: Option<String>,
}

#[derive(Default)]
pub struct Handler {
    cfg: Config,
}

#[async_trait]
impl PayloadHandler for Handler {
    async fn new_with_config(cfg: serde_json::Value) -> anyhow::Result<Self> {
        let cfg = serde_json::from_value(cfg)?;
        Ok(Handler { cfg })
    }

    async fn handle_payload(&self, value: &mut serde_json::Value) -> anyhow::Result<Proof> {
        let obj = match value.as_object() {
            Some(value) => value.clone(),
            None => anyhow::bail!("invalid input"),
        };
        let cli = Client::new(self.cfg.stream_url.clone());
        let mut proof = Proof::default();

        for (key, val) in obj {
            if let Ok(val) = serde_json::from_value::<idos_types::Value>(val) {
                match val {
                    idos_types::Value::Stream(mut stream) => {
                        let stream_id = &stream.params.stream_id;
                        let content = cli.get_content_with_proof(&stream_id, &mut proof).await?;
                        if !stream.params.extractors.is_empty() {
                            let mut payload = serde_json::Value::Null;
                            for ele in &stream.params.extractors {
                                ele.patch_from_content(&content, &mut payload)?;
                            }
                            stream.payload = Some(payload);
                        } else {
                            stream.payload = Some(content);
                        }
                        value[key] = serde_json::to_value(&stream)?;
                    }
                    idos_types::Value::File(mut file) => {
                        let file_id = &file.params.file_id;
                        todo!("get file content with proof")
                    }
                    _ => {}
                }
            }
        }

        value[GLOBAL_KEY] =
            serde_json::to_value(GLOBAL.lock().unwrap().get_global().unwrap()).unwrap();

        Ok(proof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_handle_payload() {
        let mut value = json!({
            "key": {
                "type": "stream",
                "params": {
                    "stream_id": "kjzl6kcym7w8y7bq5vaz6mcx2dzskob3zl0v0eszkgs91ntj0uwq12bztdbd73g",
                    "extractors": [
                        ["/network", "$.network"]
                    ]
                }
            }
        });

        let handler = Handler::default();

        let proof = handler.handle_payload(&mut value).await;
        assert!(proof.is_ok());

        assert_eq!(value["key"]["payload"], json!({ "network": "1" }));
    }
}
