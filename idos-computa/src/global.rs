pub static GLOBAL_KEY: &str = "global";

// Global struct to store global variables
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Global {
    // idos network id
    pub network: Option<String>,
    pub block_number: Option<u32>,
    pub dlink_node: Option<String>,
}
