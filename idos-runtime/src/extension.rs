mod contract;
mod default;

use crate::{bonsai, load_query_payload};
use bonsai_sdk::alpha::SessionId;
pub use contract::*;
use ethabi::Address;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers_contract::stream::EventStream;
use ethers_contract::FunctionCall;
use risc0_zkvm::sha::Digest;
use std::sync::Arc;

pub struct Client {
    cfg: ContractConfig,
    client: Arc<SignerMiddleware<Provider<Ws>, Wallet<k256::ecdsa::SigningKey>>>,
}

impl Client {
    pub async fn new(cfg: &ContractConfig) -> anyhow::Result<Self> {
        let provider = Provider::<Ws>::connect(&cfg.rpc_url).await?;
        let chain_id = provider.get_chainid().await?.as_u64();
        let signer = cfg.key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let provider = provider.with_signer(signer);

        Ok(Self {
            cfg: cfg.clone(),
            client: provider.into(),
        })
    }

    pub async fn subscribe_on_query(&self, bonsai_client: &bonsai::Client) -> anyhow::Result<()> {
        let dataverse_relayer: DataverseRelayer<_> =
            DataverseRelayer::new(self.cfg.relayer, self.client.clone());

        let request_event: Event<Arc<_>, _, RequestReceivedFilter> =
            dataverse_relayer.event::<RequestReceivedFilter>();
        let mut stream: EventStream<_, _, _> = request_event.subscribe_with_meta().await?;

        while let Some(Ok((event, meta))) = stream.next().await {
            let tx_hash = meta.transaction_hash.to_string();
            info!(tx_hash, ?event, "RequestReceived event");
            if let Err(err) = self.handle_request(event, &bonsai_client).await {
                error!(?meta, ?err, "Failed to handle query event");
            };
        }
        Ok(())
    }

    async fn handle_request(
        &self,
        event: RequestReceivedFilter,
        bonsai_client: &bonsai::Client,
    ) -> anyhow::Result<SessionId> {
        let event_clone = event.clone();
        let _request_id: [u8; 32] = event.request_id;
        let _sender: Address = event.sender;
        let _payment: U256 = event.payment;
        let _expiration: U256 = event.expiration;
        let request_params = event.request_params;
        let request_params_clone = request_params.clone();

        let image_id: [u8; 32] = request_params.image_id;
        let payload: String = request_params.payload;
        let _address: Address = request_params.callback_addr;
        let _callback_func: [u8; 4] = request_params.callback_func;
        let _nonce: U256 = request_params.nonce;

        let img_id: Digest = image_id.into();
        let img_id: String = img_id.to_string();
        let input_data: Vec<u8> = payload.into_bytes();
        let input_query = serde_json::from_slice(&input_data)?;
        let input_payload = load_query_payload(input_query).await?;
        let input = serde_json::to_vec(&input_payload)?;
        let (session_id, response_data) = bonsai_client.run_bonsai(img_id, input).await?;

        self.submit(event_clone, request_params_clone, response_data)
            .await?;
        Ok(session_id)
    }

    pub async fn submit(
        &self,
        event: RequestReceivedFilter,
        request_params: RequestParams,
        response_data: Vec<u8>,
    ) -> anyhow::Result<()> {
        let dataverse_verifier: DataverseVerifier<_> =
            DataverseVerifier::new(self.cfg.verifier, self.client.clone());

        let call: FunctionCall<_, _, bool> = dataverse_verifier.submit(
            event.request_id,
            request_params,
            event.payment,
            event.expiration,
            response_data.into(),
        );
        let tx: Option<TransactionReceipt> = call.send().await?.await?;
        match tx {
            Some(tx) => {
                info!(?tx, "submit request tx");
            }
            None => {
                error!("submit request tx failed");
            }
        }
        Ok(())
    }
}
