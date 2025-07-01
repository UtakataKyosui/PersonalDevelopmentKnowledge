use anyhow::Result;
use colored::*;
use crate::cli::ProjectStatus;
use crate::config::Settings;
use crate::fs::scanner::ProjectScanner;

pub async fn execute(
    project_name: String,
    set_status: Option<ProjectStatus>,
    note: Option<String>,
    log: bool,
) -> Result<()> {
    let settings = Settings::load().await?;
    let scanner = ProjectScanner::new(settings.projects_path());
    
    // Find the project
    let mut project = scanner.find_project_by_name(&project_name).await?
        .ok_or_else(|| anyhow::anyhow!("Project '{}' not found", project_name))?;

    if log {
        // Show status history
        print_status_history(&project)?;
        return Ok(());
    }

    if let Some(new_status) = set_status {
        // Update status
        let old_status = project.status.clone();
        let new_core_status = new_status.into();
        
        project.update_status(new_core_status, note.clone());
        
        // Save updated metadata
        if let Some(project_path) = &project.path {
            let metadata_path = project_path.join(".project-meta.yaml");
            let metadata_content = serde_yaml::to_string(&project)?;
            tokio::fs::write(&metadata_path, metadata_content).await?;
        }

        // Show update confirmation
        println!("{} Updating project status...", "🔄".bright_blue());
        println!();
        println!("Project: {}", project.name.bright_white().bold());
        println!("Status: {} → {}", 
            format_status_colored(&old_status),
            format_status_colored(&project.status)
        );
        
        if let Some(note_text) = &note {
            println!("Note: {}", note_text.bright_cyan());
        }
        
        println!();
        println!("{} Status updated successfully!", "✅".bright_green());
        
        // Show progress
        print_progress_timeline(&project)?;
        
        // Suggest next action
        suggest_next_action(&project.status)?;
        
    } else {
        // Show current status
        print_current_status(&project)?;
    }

    Ok(())
}

fn print_status_history(project: &crate::core::Project) -> Result<()> {
    println!("{} Status History for \"{}\"", 
        "📈".bright_blue(), 
        project.name.bright_white().bold()
    );
    println!();

    for (i, entry) in project.status_history.iter().enumerate() {
        let is_current = i == project.status_history.len() - 1;
        let marker = if is_current { "🚧" } else { "✅" };
        
        println!("{} {} ({})", 
            marker,
            format_status_colored(&entry.status),
            entry.timestamp.format("%Y-%m-%d %H:%M UTC").to_string().bright_black()
        );
        
        if let Some(note) = &entry.note {
            println!("   {}", note.bright_cyan());
        }
        
        if !is_current {
            println!("   │");
        }
    }

    Ok(())
}

fn print_current_status(project: &crate::core::Project) -> Result<()> {
    println!("{} Project Status", "📊".bright_blue());
    println!();
    
    println!("Name: {}", project.name.bright_white().bold());
    println!("Type: {}", format_project_type(&project.project_type));
    println!("Status: {}", format_status_colored(&project.status));
    println!("Priority: {}", format_priority_colored(&project.priority));
    println!("Progress: {}%", 
        format!("{:.0}", project.get_progress_percentage()).bright_green()
    );
    
    println!();
    println!("Created: {}", 
        project.created.format("%Y-%m-%d").to_string().bright_black()
    );
    println!("Updated: {}", 
        project.updated.format("%Y-%m-%d").to_string().bright_black()
    );
    
    if let Some(desc) = &project.description {
        println!();
        println!("Description:");
        println!("  {}", desc.bright_cyan());
    }

    if !project.technologies.is_empty() {
        println!();
        println!("Technologies: {}", 
            project.technologies.join(", ").bright_yellow()
        );
    }

    Ok(())
}

fn print_progress_timeline(project: &crate::core::Project) -> Result<()> {
    println!();
    println!("{} Progress:", "📈".bright_blue());
    
    let statuses = [
        ("Idea", crate::core::ProjectStatus::Idea),
        ("Planning", crate::core::ProjectStatus::Planning),
        ("Development", crate::core::ProjectStatus::Development),
        ("Testing", crate::core::ProjectStatus::Testing),
        ("Completed", crate::core::ProjectStatus::Completed),
    ];

    for (name, status) in &statuses {
        let has_reached = project.status_history.iter()
            .any(|entry| std::mem::discriminant(&entry.status) == std::mem::discriminant(status));
        
        let is_current = std::mem::discriminant(&project.status) == std::mem::discriminant(status);
        
        let (marker, color_fn): (&str, fn(&str) -> ColoredString) = if is_current {
            ("🚧", |s| s.bright_yellow())
        } else if has_reached {
            ("✅", |s| s.bright_green())
        } else {
            ("⏳", |s| s.bright_black())
        };
        
        let date_info = if let Some(entry) = project.status_history.iter()
            .find(|entry| std::mem::discriminant(&entry.status) == std::mem::discriminant(status)) {
            format!(" ({})", entry.timestamp.format("%Y-%m-%d"))
        } else {
            String::new()
        };
        
        println!("  {} {}{}", 
            marker, 
            color_fn(name),
            date_info.bright_black()
        );
    }

    Ok(())
}

fn suggest_next_action(status: &crate::core::ProjectStatus) -> Result<()> {
    println!();
    let suggestion = match status {
        crate::core::ProjectStatus::Idea => {
            "💡 Next: Start planning with 'opj status \"project\" --set planning'"
        },
        crate::core::ProjectStatus::Planning => {
            "📋 Next: Begin development with 'opj status \"project\" --set development'"
        },
        crate::core::ProjectStatus::Development => {
            "🚧 Next: Start testing with 'opj status \"project\" --set testing'"
        },
        crate::core::ProjectStatus::Testing => {
            "🧪 Next: Mark as completed with 'opj status \"project\" --set completed'"
        },
        crate::core::ProjectStatus::Completed => {
            "🎉 Congratulations! Consider archiving or starting a new project."
        },
        crate::core::ProjectStatus::Archived => {
            "📦 Project is archived. Use 'opj list' to see active projects."
        },
        crate::core::ProjectStatus::Cancelled => {
            "❌ Project is cancelled. Use 'opj list' to see active projects."
        },
    };
    
    println!("{}", suggestion.bright_cyan());
    
    Ok(())
}

fn format_status_colored(status: &crate::core::ProjectStatus) -> ColoredString {
    match status {
        crate::core::ProjectStatus::Idea => "Idea".bright_blue(),
        crate::core::ProjectStatus::Planning => "Planning".bright_cyan(),
        crate::core::ProjectStatus::Development => "Development".bright_green(),
        crate::core::ProjectStatus::Testing => "Testing".bright_yellow(),
        crate::core::ProjectStatus::Completed => "Completed".bright_green().bold(),
        crate::core::ProjectStatus::Archived => "Archived".bright_black(),
        crate::core::ProjectStatus::Cancelled => "Cancelled".bright_red(),
    }
}

fn format_priority_colored(priority: &crate::core::Priority) -> ColoredString {
    match priority {
        crate::core::Priority::High => "High".bright_red(),
        crate::core::Priority::Medium => "Medium".bright_yellow(),
        crate::core::Priority::Low => "Low".bright_blue(),
    }
}

fn format_project_type(project_type: &crate::core::ProjectType) -> ColoredString {
    let (name, emoji) = match project_type {
        crate::core::ProjectType::WebApp => ("Web-App", "🌐"),
        crate::core::ProjectType::Tool => ("Tool", "🛠️"),
        crate::core::ProjectType::Content => ("Content", "📝"),
        crate::core::ProjectType::Api => ("API", "🔌"),
        crate::core::ProjectType::Custom(name) => (name.as_str(), "📦"),
    };
    
    format!("{} {}", emoji, name).bright_cyan()
}
