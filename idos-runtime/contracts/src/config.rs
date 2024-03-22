use ethers::prelude::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default::relayer")]
    pub relayer: Address,
    #[serde(default = "default::verifier")]
    pub verifier: Address,
    pub rpc_url: String,
    pub key: String,
}
