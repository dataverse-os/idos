use super::default;
use ethers::contract::abigen;
use ethers::prelude::*;

abigen!(
  DataverseRelayer, "contracts/abi/IDataverseRelayer.abi",
  derives(serde::Deserialize, serde::Serialize);

  DataverseVerifier, "contracts/abi/IDataverseVerifier.abi",
  derives(serde::Deserialize, serde::Serialize);
);

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContractConfig {
    #[serde(default = "default::relayer")]
    pub relayer: Address,
    #[serde(default = "default::verifier")]
    pub verifier: Address,
    pub rpc_url: String,
    pub key: String,
}
