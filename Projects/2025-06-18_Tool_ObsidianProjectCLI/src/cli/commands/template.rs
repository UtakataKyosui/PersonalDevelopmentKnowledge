use anyhow::Result;
use crate::cli::TemplateCommand;

pub async fn execute(subcommand: TemplateCommand) -> Result<()> {
    match subcommand {
        TemplateCommand::List => list_templates().await,
        TemplateCommand::Add { name, path } => add_template(name, path).await,
        TemplateCommand::Edit { name } => edit_template(name).await,
        TemplateCommand::Remove { name } => remove_template(name).await,
    }
}

async fn list_templates() -> Result<()> {
    println!("📋 Available Templates");
    println!();
    println!("• Web-App-Project - Web application development");
    println!("• Tool-Project - CLI tools and libraries");
    println!("• Content-Project - Blog posts and tutorials");
    println!("• API-Project - Backend APIs and services");
    Ok(())
}

async fn add_template(_name: String, _path: String) -> Result<()> {
    println!("🚧 Template management features coming soon!");
    Ok(())
}

async fn edit_template(_name: String) -> Result<()> {
    println!("🚧 Template editing features coming soon!");
    Ok(())
}

async fn remove_template(_name: String) -> Result<()> {
    println!("🚧 Template removal features coming soon!");
    Ok(())
}
