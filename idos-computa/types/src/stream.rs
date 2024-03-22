use super::{path::Extractor, ValueType};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stream<T>
where
    T: Clone,
{
    #[serde(rename = "type")]
    pub r#type: ValueType,
    pub params: StreamParams,
    pub payload: Option<T>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamParams {
    pub stream_id: String,
    #[serde(default = "Vec::new")]
    pub commit_ids: Vec<String>,
    #[serde(default = "Vec::new")]
    pub attrs: Vec<String>,
    #[serde(default = "Vec::new")]
    pub extractors: Vec<Extractor>,
}
