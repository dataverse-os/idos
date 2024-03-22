use idos_datasource::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(rename = "type")]
    pub type_: DataSourceType,
    pub config: DataSourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataSourceType {
    Http,
    Kubo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataSourceConfig {
    Http(http::Config),
    Kubo(kubo::Config),
}
