use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Proof {
    pub streams: HashMap<String, String>,
}
