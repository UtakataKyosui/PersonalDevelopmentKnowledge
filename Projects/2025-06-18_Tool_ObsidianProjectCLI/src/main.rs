use anyhow::Result;
use clap::Parser;

mod cli;
mod core;
mod fs;
mod config;
mod utils;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { 
            name, 
            project_type, 
            priority, 
            description,
            template,
            no_git,
            interactive 
        } => {
            cli::commands::new::execute(
                name, 
                project_type, 
                priority, 
                description, 
                template, 
                no_git,
                interactive
            ).await?;
        }
        Commands::List { 
            status, 
            project_type, 
            priority, 
            sort, 
            format 
        } => {
            cli::commands::list::execute(
                status, 
                project_type, 
                priority, 
                sort, 
                format
            ).await?;
        }
        Commands::Status { 
            project, 
            set_status, 
            note, 
            log 
        } => {
            cli::commands::status::execute(
                project, 
                set_status, 
                note, 
                log
            ).await?;
        }
        Commands::Info { project } => {
            cli::commands::info::execute(project).await?;
        }
        Commands::Template { subcommand } => {
            cli::commands::template::execute(subcommand).await?;
        }
        Commands::Migrate { source } => {
            cli::commands::migrate::execute(source).await?;
        }
        Commands::Config { key, value } => {
            cli::commands::config::execute(key, value).await?;
        }
        Commands::Stats => {
            cli::commands::stats::execute().await?;
        }
    }

    Ok(())
}
