use ethers::contract::abigen;

abigen!(
  DataverseRelayer, "contracts/abi/IDataverseRelayer.abi",
  derives(serde::Deserialize, serde::Serialize);

  DataverseVerifier, "contracts/abi/IDataverseVerifier.abi",
  derives(serde::Deserialize, serde::Serialize);
);
