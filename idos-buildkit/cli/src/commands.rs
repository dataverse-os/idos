use clap::{Parser, Subcommand};
use std::path::PathBuf;

use self::{config::*, daemon::*, project::*};
use crate::GlobalArguments;

mod config;
mod daemon;
mod project;

#[derive(Subcommand)]
enum Commands {
    Config(ConfigCommandsArgs),
    Daemon(DaemonCommandsArgs),
    Build(ProjectCommandsArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[clap(flatten)]
    global_params: GlobalArguments,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
#[command(version, disable_help_flag = true)]
struct GlobalOptions {
    #[command(flatten)]
    global_params: GlobalArguments,

    /// Capture all the normal commands, basically to ingore them.
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    pub cmd: Vec<String>,
}

pub async fn cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(cmd) = &cli.command {
        match cmd {
            Commands::Config(args) => ConfigCommandsArgs::handle(&cli.global_params, &args).await?,
            Commands::Daemon(args) => DaemonCommandsArgs::handle(&cli.global_params, &args).await?,
            Commands::Build(args) => ProjectCommandsArgs::handle(&cli.global_params, &args).await?,
        }
    }

    Ok(())
}
