use idos_computa::prelude::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() -> anyhow::Result<()> {
        let input = serde_json::json!({
            "owner": "0xc490803bc98DAec6775132F54503331D8C79967e",
            "x1": {
                "params": {
                    "file_id": "kjzl6hvfrbw6c9v8rx8rhwmm5i3fh35twf5fh7h2er6083api4t5g0uhdj7xabf",
                    "commit_ids": ["f06f4b5a50d5bd1976a0247940d17b2690c953732d0c8d7cac64841a448b0087"],
                    "attrs": ["abc", "abb.ab"]
                },
                "payload": [{"abc": 1, "abb": {"ab": 2}}]
            },
            "x2": {
                "params": {
                    "file_id": "kjzl6hvfrbw6c5oxgp89ovqka7v8k82r77ke9vf4tjyut6edhh9aa2nzfpdi9pj",
                    "commit_ids": ["14193aa9d9c7c73c16cce90c4e7f733e0cd91b1e00beb36cf2ff5690e91276d6"],
                    "attrs": ["number"]
                },
                "payload": [{"number": 2}]
            },
            "global": {
                "network": "mainnet",
                "block_number": 100,
                "dlink_node": "https://dlink.com"
            }
        });
        let input: Input = serde_json::from_value(input)?;
        assert_eq!(sum(input), 3);
        Ok(())
    }
}
