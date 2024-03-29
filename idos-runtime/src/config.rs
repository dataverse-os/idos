mod datasource;

use crate::bonsai::BonsaiConfig;
use anyhow::Context;
use datasource::*;
use directories::ProjectDirs;
use std::path::PathBuf;

static APP_NAME: &str = "dataverse-node";

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    pub dsn: String,

    pub datasources: Vec<DataSource>,

    pub bonsai: BonsaiConfig,
    pub contract: idos_contracts::Config,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IrohConfig {
    pub key: String,
    pub author: String,
    pub model: String,
    pub streams: String,
}

impl Into<dataverse_iroh_store::KeySet> for IrohConfig {
    fn into(self) -> dataverse_iroh_store::KeySet {
        dataverse_iroh_store::KeySet {
            author: self.author,
            model: self.model,
            streams: self.streams,
        }
    }
}

const DEFAULT_REPO_PATH: &str = ".idos";
const DEFAULT_CONFIG_NAME: &str = "config.toml";

pub fn default_repo_path() -> String {
    let home = match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => panic!("cannot get home"),
    };
    format!("{home:}/{:}", DEFAULT_REPO_PATH)
}

pub fn default_config_path() -> String {
    format!("{}/{:}", default_repo_path(), DEFAULT_CONFIG_NAME)
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let file = confy::get_configuration_file_path(APP_NAME, None)?;
        info!("use config path: {:#?}", file);
        let cfg: Self = confy::load(APP_NAME, None)?;
        info!("use data path: {:#?}", cfg.data_path()?);
        Ok(cfg)
    }

    pub fn data_path(&self) -> anyhow::Result<PathBuf> {
        let project =
            ProjectDirs::from("rs", "", APP_NAME).context("Failed to get project dirs")?;
        Ok(project.data_dir().to_path_buf())
    }
}
