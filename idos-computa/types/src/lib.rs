#[macro_use]
extern crate serde;

mod file;
mod path;
pub mod proof;
mod stream;

pub use file::*;
pub use proof::*;
pub use stream::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueType {
    Stream,
    File,
    Folder,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    #[serde(rename_all = "camelCase")]
    Stream(Stream<serde_json::Value>),
    File(File<serde_json::Value>),
    Folder,
}
