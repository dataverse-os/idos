use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{commands::config::*, GlobalArguments};

mod config;
mod daemon;

#[derive(Subcommand)]
enum Commands {
    Config(InitConfigArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, default_value = "false")]
    debug: bool,

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

    Ok(())
}
