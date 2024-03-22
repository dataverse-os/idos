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
