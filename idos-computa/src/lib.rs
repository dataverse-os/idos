#[macro_use]
extern crate serde;

pub mod global;
pub mod prelude;

use anyhow::Context;
use ethabi::ParamType;
use prelude::GLOBAL;
use risc0_zkvm::guest::env;
use serde::Deserialize;
use serde_json::Value;
use std::io::Read;

#[macro_export]
macro_rules! handle_with_entry {
    ($name:ident, $input:ident) => {
        risc0_zkvm::guest::entry!(main);
        fn main() {
            let input = get_input::<$input>().expect("failed to get input");
            let data = input.query;
            let result = $name(data);
            // risc0_zkvm::guest::env::commit_slice(&ethabi::encode(&result.tokens()));
        }
    };
}

#[derive(Debug)]
pub struct Input<T> {
    pub query_id: Vec<u8>,
    pub query_data: Vec<u8>,
    pub query: T,
    pub payload: Vec<u8>,
}

pub fn get_input<T>() -> anyhow::Result<Input<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut input_bytes = Vec::<u8>::new();
    env::stdin()
        .read_to_end(&mut input_bytes)
        .context("failed to read input bytes")?;

    let (query_id, query_data, payload) = decode_query_input(input_bytes)?;

    let query_data_value: Value = serde_json::from_slice(&query_data)?;
    let global_data = serde_json::from_value(query_data_value["global"].clone())?;
    GLOBAL.get_or_init(|| global_data);
    let query = serde_json::from_slice(&query_data)?;

    Ok(Input {
        query_id,
        query_data,
        query,
        payload,
    })
}

pub fn decode_query_input(input_bytes: Vec<u8>) -> anyhow::Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let types = [
        ParamType::FixedBytes(32),
        ParamType::Bytes,
        ParamType::Bytes,
    ];
    let tokens = ethabi::decode(&types, &input_bytes).map_err(|err| {
        anyhow::anyhow!(
            "failed to decode input bytes: {:?}, error: {:?}",
            input_bytes,
            err
        )
    })?;

    if let (Some(query_id), Some(query_data), Some(validation_data)) = (
        tokens[0].clone().into_fixed_bytes(),
        tokens[1].clone().into_bytes(),
        tokens[2].clone().into_bytes(),
    ) {
        return Ok((query_id, query_data, validation_data));
    };

    anyhow::bail!("failed to decode input bytes: {:?}", input_bytes)
}
