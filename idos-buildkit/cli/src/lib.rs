mod commands;

use async_trait::async_trait;
use clap::Args;
pub use commands::*;

#[async_trait]
pub trait CommandLineHandler {
    /// Abstraction for command line operations arguments.
    ///
    /// NOTE that this parameter is used to generate the command line arguments.
    /// Currently we are directly integrating with `clap` crate. In the future we can use our own
    /// implementation to abstract away external crates. But this should be good for now.
    type Arguments: std::fmt::Debug + Args;

    /// Handles the request with the provided arguments. Dev should handle the content to print and how
    async fn handle(global: &GlobalArguments, arguments: &Self::Arguments) -> anyhow::Result<()>;
}

/// The global arguments that will be shared by all cli commands.
#[derive(Debug, Args, Clone, Default)]
pub struct GlobalArguments {
    #[arg(
        long,
        help = "The toml config file path for IDOS Agent, default to ${HOME}/.idos/config.toml",
        env = "IDOS_CLI_CONFIG_PATH"
    )]
    config_path: Option<String>,
}

impl GlobalArguments {
    pub fn config_path(&self) -> String {
        self.config_path
            .clone()
            .unwrap_or_else(idos_runtime::default_config_path)
    }
}
