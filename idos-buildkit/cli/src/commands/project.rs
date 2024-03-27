use crate::{CommandLineHandler, GlobalArguments};

use super::Cli;

#[derive(Debug, Args)]
#[command(name = "project", about = "build related commands")]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct ProjectCommandsArgs {
    #[command(subcommand)]
    command: Commands,
}

impl ProjectCommandsArgs {
    pub async fn handle(
        global: &GlobalArguments,
        args: &ProjectCommandsArgs,
    ) -> anyhow::Result<()> {
        // match &args.command {
        //     Commands::New(args) => NewProject::handle(global, args).await,
        //     Commands::Build(args) => BuildProject::handle(global, args).await,
        // }
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    New(NewProjectArgs),
    Build(BuildProjectArgs),
}

#[derive(Debug, Args)]
#[command(about = "Arguments to create a new project")]
pub(crate) struct NewProjectArgs {
    #[arg(short, long, value_name = "FILE")]
    pub name: String,
}

struct NewProject;

impl CommandLineHandler for NewProject {
    type Arguments = NewProjectArgs;

    async fn handle(global: &GlobalArguments, _arguments: &Self::Arguments) -> anyhow::Result<()> {
        let path = global.config_path();
        log::debug!("initializing empty config file in {}", path);

        let file_path = std::path::Path::new(&path);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::File::create(&path).map_err(|e| {
            log::error!("couldn't create config file");
            e
        })?;
        // file.write_all(DEFAULT_CONFIG_TEMPLATE.as_bytes())
        //     .map_err(|e| {
        //         log::error!("error populating empty config template");
        //         e
        //     })?;

        log::info!("Empty config populated successful in {}", &path);

        Ok(())
    }
}

#[derive(Debug, Args)]
#[command(about = "Arguments to build a project")]
pub(crate) struct BuildProjectArgs {
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,
}
