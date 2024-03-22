use std::{
    env,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;

use idos_computa::global;
pub use idos_computa::global::GLOBAL_KEY;

pub static GLOBAL: Lazy<Arc<Mutex<dyn GlobalGetter + Send + Sync>>> =
    Lazy::new(|| Arc::new(Mutex::new(EnvGlobalGetter {})));

pub trait GlobalGetter {
    fn get_global(&self) -> anyhow::Result<global::Global>;
}

struct EnvGlobalGetter;

impl GlobalGetter for EnvGlobalGetter {
    fn get_global(&self) -> anyhow::Result<global::Global> {
        Ok(global::Global {
            network: env::var("IDOS_NETWORK").ok(),
            block_number: env::var("IDOS_BLOCK_NUMBER")
                .ok()
                .map(|x| x.parse().unwrap()),
            dlink_node: env::var("IDOS_DLINK_NODE").ok(),
        })
    }
}
