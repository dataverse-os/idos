use crate::config::Config;
use bonsai::SessionId;
use bonsai_sdk::alpha as bonsai;
use bonsai_sdk::alpha_async as bonsai_async;
use risc0_zkvm::{serde::to_vec, Receipt};
use std::time::Duration;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BonsaiConfig {
    pub relay_api_url: String,
    pub api_key: String,

    pub session_waiting_duration: u64,
}

pub struct Client {
    pub config: BonsaiConfig,
    pub contract_config: idos_contracts::Config,
    client: bonsai::Client,
}

impl Client {
    pub async fn new(cfg: &Config) -> anyhow::Result<Self> {
        let client = bonsai_async::get_client_from_parts(
            cfg.bonsai.relay_api_url.clone(),
            cfg.bonsai.api_key.clone(),
            risc0_zkvm::VERSION,
        )
        .await?;
        Ok(Self {
            config: cfg.bonsai.clone(),
            contract_config: cfg.contract.clone(),
            client,
        })
    }

    pub async fn run_bonsai(
        &self,
        img_id: String,
        input: Vec<u8>,
    ) -> anyhow::Result<(SessionId, Vec<u8>)> {
        // Prepare input data and upload it.
        let input_data = to_vec(&input)?;
        let input_data = bytemuck::cast_slice(&input_data).to_vec();
        let input_id = bonsai_async::upload_input(self.client.clone(), input_data).await?;

        // Start a session running the prover
        let session: SessionId =
            bonsai_async::create_session(self.client.clone(), img_id.clone(), input_id, vec![])
                .await?;
        loop {
            let res = bonsai_async::session_status(self.client.clone(), session.clone()).await?;
            if res.status == "RUNNING" {
                info!(session = session.uuid, ?res.status, ?res.state, "session running");
                std::thread::sleep(Duration::from_secs(self.config.session_waiting_duration));
                continue;
            }
            if res.status == "SUCCEEDED" {
                // Download the receipt, containing the output
                let receipt_url = res
                    .receipt_url
                    .expect("API error, missing receipt on completed session");

                let receipt_buf = bonsai_async::download(self.client.clone(), receipt_url).await?;
                let receipt: Receipt = bincode::deserialize(&receipt_buf)?;
                let decoded = hex::decode(img_id)?;
                let mut img_id: [u8; 32] = [0; 32];
                img_id.copy_from_slice(&decoded);
                receipt.verify(img_id).expect("Receipt verification failed");
                let response_data = receipt.journal.bytes;

                return Ok((session, response_data));
            } else {
                error!(session = session.uuid, ?res.status, error = ?res.error_msg, "workflow exited");
            }
        }
    }
}
