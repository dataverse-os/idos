use std::sync::OnceLock;

pub use crate::global::*;
pub use crate::{get_input, handle_with_entry};
pub use idos_types::*;
pub use primitive_types::{H160, U256};
pub use risc0_zkvm;
pub use serde::{Deserialize, Serialize};

pub type Address = H160;

pub static GLOBAL: OnceLock<Global> = OnceLock::new();

impl Global {
    pub fn env() -> Option<&'static Self> {
        GLOBAL.get()
    }
}
