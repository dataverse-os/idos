use futures::future;
use idos_runtime::*;
use tokio::task::JoinHandle;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::GlobalArguments;

use super::Cli;

#[derive(Debug, Args)]
#[command(name = "daemon", about = "daemon related commands")]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct DaemonCommandsArgs {
    #[command(subcommand)]
    command: Commands,
}

impl DaemonCommandsArgs {
    pub async fn handle(global: &GlobalArguments, args: &DaemonCommandsArgs) -> anyhow::Result<()> {
        // match &args.command {
        //     Commands::Start => {
        //         let daemon = Daemon::new()?;
        //         daemon.run().await?;
        //     }
        // }
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Start,
}

pub struct Daemon {
    cfg: Config,
}

impl Daemon {
    pub fn new() -> anyhow::Result<Self> {
        let cfg = Config::load()?;
        Ok(Self { cfg })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();

        let mut futures: Vec<JoinHandle<anyhow::Result<()>>> = Vec::new();

        // init bonsai query subscription
        futures.push(bonsai_query(&self.cfg));

        // init web server
        let state = AppState::new(&self.cfg).await?;
        futures.push(web_server(state)?);

        if let (Err(err), idx, remaining) = future::select_all(futures).await {
            error!("error in {}: {}", idx, err);
            for future in remaining {
                future.abort();
            }
            anyhow::bail!("error: {}", err);
        }
        Ok(())
    }
}
