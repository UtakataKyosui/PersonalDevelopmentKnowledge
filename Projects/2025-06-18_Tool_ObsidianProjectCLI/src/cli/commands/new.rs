use anyhow::{Result, Context};
use colored::*;
use dialoguer::{Input, Select};
use chrono::Utc;

use crate::cli::{ProjectType, Priority};
use crate::core::{Project, ProjectStatus};
use crate::config::Settings;
use crate::fs::operations;
use crate::utils::template::TemplateProcessor;

pub async fn execute(
    name: Option<String>,
    project_type: Option<ProjectType>,
    priority: Option<Priority>,
    description: Option<String>,
    template: Option<String>,
    no_git: bool,
    interactive: bool,
) -> Result<()> {
    println!("{}", "✨ Create a new project".bright_cyan().bold());
    println!();

    let settings = Settings::load().await?;

    // Interactive mode or collect missing information
    let project_name = if let Some(name) = name {
        name
    } else if interactive || name.is_none() {
        Input::new()
            .with_prompt("Project name")
            .interact_text()?
    } else {
        return Err(anyhow::anyhow!("Project name is required"));
    };

    let project_type = if let Some(pt) = project_type {
        pt
    } else if interactive {
        let types = vec!["Web Application", "CLI Tool", "Content/Article", "API/Backend"];
        let selection = Select::new()
            .with_prompt("Project type")
            .items(&types)
            .default(0)
            .interact()?;
        
        match selection {
            0 => ProjectType::WebApp,
            1 => ProjectType::Tool,
            2 => ProjectType::Content,
            3 => ProjectType::Api,
            _ => ProjectType::WebApp,
        }
    } else {
        settings.default_type
    };

    let priority = if let Some(p) = priority {
        p
    } else if interactive {
        let priorities = vec!["Low", "Medium", "High"];
        let selection = Select::new()
            .with_prompt("Priority")
            .items(&priorities)
            .default(1)
            .interact()?;
        
        match selection {
            0 => Priority::Low,
            1 => Priority::Medium,
            2 => Priority::High,
            _ => Priority::Medium,
        }
    } else {
        settings.default_priority
    };

    let description = if description.is_some() {
        description
    } else if interactive {
        let desc: String = Input::new()
            .with_prompt("Description (optional)")
            .allow_empty(true)
            .interact_text()?;
        if desc.is_empty() { None } else { Some(desc) }
    } else {
        None
    };

    // Create project structure
    let project = Project::new(
        project_name.clone(),
        project_type.clone(),
        priority.clone(),
        description,
    );

    println!();
    println!("{} Creating project \"{}\"...", "🚀".bright_green(), project_name.bright_white().bold());

    // Generate project directory name
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let type_str = match project_type {
        ProjectType::WebApp => "Web-App",
        ProjectType::Tool => "Tool",
        ProjectType::Content => "Content",
        ProjectType::Api => "API",
    };
    let dir_name = format!("{}_{}_{}",, type_str, project_name);
    
    // Create project directory
    let project_path = settings.projects_path().join(&dir_name);
    
    println!("{} Creating directory: {}", 
        "📁".bright_blue(), 
        project_path.display().to_string().bright_yellow()
    );

    if !project_path.exists() {
        std::fs::create_dir_all(&project_path)
            .context("Failed to create project directory")?;
    }

    // Copy template
    let template_name = template.unwrap_or_else(|| {
        match project_type {
            ProjectType::WebApp => "Web-App-Project".to_string(),
            ProjectType::Tool => "Tool-Project".to_string(),
            ProjectType::Content => "Content-Project".to_string(),
            ProjectType::Api => "API-Project".to_string(),
        }
    });

    let template_path = settings.templates_path().join(&template_name);
    
    println!("{} Copying template: {} -> {}", 
        "📄".bright_blue(), 
        template_name.bright_cyan(), 
        project_name.bright_white()
    );

    if template_path.exists() {
        operations::copy_dir_recursive(&template_path, &project_path).await
            .context("Failed to copy template")?;
    } else {
        println!("{} Template not found, creating basic structure", "⚠️".bright_yellow());
        create_basic_structure(&project_path, &project).await?;
    }

    // Process template variables
    println!("{} Updating project metadata...", "✏️".bright_blue());
    
    let processor = TemplateProcessor::new(&project);
    processor.process_directory(&project_path).await
        .context("Failed to process template variables")?;

    // Create metadata file
    let metadata_path = project_path.join(".project-meta.yaml");
    let metadata_content = serde_yaml::to_string(&project)
        .context("Failed to serialize project metadata")?;
    
    tokio::fs::write(&metadata_path, metadata_content).await
        .context("Failed to write project metadata")?;

    // Initialize Git repository if requested
    if !no_git && settings.auto_git_init {
        println!("{} Initializing Git repository...", "📦".bright_blue());
        
        let output = std::process::Command::new("git")
            .args(&["init"])
            .current_dir(&project_path)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                println!("{} Git repository initialized", "✅".bright_green());
            }
            _ => {
                println!("{} Failed to initialize Git repository", "⚠️".bright_yellow());
            }
        }
    }

    // Success message
    println!();
    println!("{} Project \"{}\" created successfully!", 
        "✅".bright_green(), 
        project_name.bright_white().bold()
    );
    println!();
    println!("{} Location: {}", 
        "📍".bright_blue(), 
        project_path.display().to_string().bright_yellow()
    );
    println!("{} Next steps:", "📋".bright_blue());
    println!("  1. Review project overview: {}", 
        format!("opj info \"{}\"", project_name).bright_cyan()
    );
    println!("  2. Edit requirements: {}", 
        "edit 01_Requirements.md".bright_cyan()
    );
    println!("  3. Start development: {}", 
        format!("opj status \"{}\" --set development", project_name).bright_cyan()
    );

    Ok(())
}

async fn create_basic_structure(project_path: &std::path::Path, project: &Project) -> Result<()> {
    // Create basic directories
    tokio::fs::create_dir_all(project_path.join("assets/images")).await?;
    tokio::fs::create_dir_all(project_path.join("assets/diagrams")).await?;
    tokio::fs::create_dir_all(project_path.join("assets/mockups")).await?;

    // Create basic overview file
    let overview_content = format!(
        r#"---
title: "{}"
type: "{:?}"
category: "{:?}"
status: "idea"
priority: "{:?}"
created: "{}"
updated: "{}"
tags:
  - project
  - {:?}
---

# {}

## Project Overview

This project was created using obsidian-project-cli.

### Description
{}

### Goals
- Define your project goals here

### Next Steps
1. Define requirements in 01_Requirements.md
2. Plan architecture in 02_Architecture.md
3. Start development and log progress in 03_Development-Log.md

"#,
        project.name,
        project.project_type,
        project.project_type,
        project.priority,
        project.created.format("%Y-%m-%d"),
        project.updated.format("%Y-%m-%d"),
        project.project_type,
        project.name,
        project.description.as_deref().unwrap_or("Add project description here")
    );

    tokio::fs::write(
        project_path.join("00_PROJECT-OVERVIEW.md"),
        overview_content
    ).await?;

    Ok(())
}
