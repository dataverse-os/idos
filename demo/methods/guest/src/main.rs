#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std] // std support is experimental

// use risc0_zkvm::guest::env;

// risc0_zkvm::guest::entry!(main);

// fn main() {
//     // TODO: Implement your guest code here

//     // read the input
//     let input: u32 = env::read();

//     // TODO: do something with the input

//     // write public output to the journal
//     env::commit(&input);
// }

use idos_computa::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumPayload {
    pub number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub owner: Address,
    pub x1: File<NumPayload>,
    pub x2: File<NumPayload>,
}

handle_with_entry!(sum, Input);
pub fn sum(input: Input) -> u32 {
    if input.owner == Address::zero() {
        panic!("invalid owner");
    }
    if Global::env().is_none() {
        panic!("dlink_node is not set");
    }

    return input.x1.latest().unwrap().number + input.x2.commit(0).number;
}
