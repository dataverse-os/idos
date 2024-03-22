#[macro_use]
extern crate serde;

mod config;
mod default;

pub use config::Config;
use ethers::prelude::*;

abigen!(
  DataverseRelayer, "abi/IDataverseRelayer.abi",
  derives(Deserialize, Serialize);

  DataverseVerifier, "abi/IDataverseVerifier.abi",
  derives(Deserialize, Serialize);
);
