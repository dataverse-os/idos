use std::collections::HashMap;

use ethabi::Address;
use lazy_static::lazy_static;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "contracts/dataverse/"]
#[include = "addresses.json"]
struct AddressAsset;

lazy_static! {
    pub static ref ADDRESSES: HashMap<Chain, Contracts> = {
        let data = AddressAsset::get("addresses.json").unwrap();
        serde_json::from_slice(&data.data).unwrap()
    };
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Chain {
    PolygonMumbai,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Contracts {
    dapp_table_registry: Address,
    extension: HashMap<Contract, Address>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
enum Contract {
    DataverseToken,
    DataverseRelayer,
    DataverseVerifier,
    DataverseClientMock,
}

pub fn relayer() -> Address {
    ADDRESSES[&Chain::PolygonMumbai].extension[&Contract::DataverseRelayer]
}

pub fn verifier() -> Address {
    ADDRESSES[&Chain::PolygonMumbai].extension[&Contract::DataverseVerifier]
}
