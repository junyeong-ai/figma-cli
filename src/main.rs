//! Figma CLI binary

use anyhow::Result;
use clap::Parser;
use figma_cli::cli::{Cli, Commands};
use figma_cli::cli::{handle_auth, handle_config, handle_extract, handle_images, handle_inspect};

fn init_logging(verbose: bool) {
    use tracing_subscriber::{EnvFilter, fmt};

    let filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::from_default_env().add_directive("warn".parse().unwrap())
    };

    fmt().with_env_filter(filter).with_target(false).init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(cli.verbose);

    match cli.command {
        Commands::Extract(args) => handle_extract(args).await,
        Commands::Inspect(args) => handle_inspect(args).await,
        Commands::Images(args) => handle_images(args).await,
        Commands::Auth(args) => handle_auth(args.command).await,
        Commands::Config(args) => handle_config(args.command).await,
    }
}
