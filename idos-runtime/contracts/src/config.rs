use ethers::prelude::*;

use crate::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_relayer")]
    pub relayer: Address,
    #[serde(default = "default_verifier")]
    pub verifier: Address,
    pub rpc_url: String,
    pub key: String,
}

fn default_relayer() -> Address {
    DATAVERSE_RELAYER.parse().unwrap()
}

fn default_verifier() -> Address {
    DATAVERSE_VERIFIER.parse().unwrap()
}
