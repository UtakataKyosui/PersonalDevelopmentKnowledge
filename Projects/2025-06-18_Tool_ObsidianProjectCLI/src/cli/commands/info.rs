use anyhow::Result;
use colored::*;
use crate::config::Settings;
use crate::fs::scanner::ProjectScanner;

pub async fn execute(project_name: String) -> Result<()> {
    let settings = Settings::load().await?;
    let scanner = ProjectScanner::new(settings.projects_path());
    
    // Find the project
    let project = scanner.find_project_by_name(&project_name).await?
        .ok_or_else(|| anyhow::anyhow!("Project '{}' not found", project_name))?;

    // Display detailed project information
    print_project_header(&project)?;
    print_project_details(&project)?;
    print_project_files(&project).await?;
    print_project_statistics(&project)?;

    Ok(())
}

fn print_project_header(project: &crate::core::Project) -> Result<()> {
    let type_emoji = match &project.project_type {
        crate::core::ProjectType::WebApp => "🌐",
        crate::core::ProjectType::Tool => "🛠️",
        crate::core::ProjectType::Content => "📝",
        crate::core::ProjectType::Api => "🔌",
        crate::core::ProjectType::Custom(_) => "📦",
    };

    println!("{} {}", 
        format!("{} Project Information", type_emoji).bright_cyan().bold(),
        project.name.bright_white().bold()
    );
    println!();

    Ok(())
}

fn print_project_details(project: &crate::core::Project) -> Result<()> {
    // Basic Information
    println!("{}", "📋 Basic Information".bright_blue().bold());
    println!("  Name: {}", project.name.bright_white());
    println!("  Type: {}", format_project_type(&project.project_type));
    println!("  Status: {}", format_status(&project.status));
    println!("  Priority: {}", format_priority(&project.priority));
    println!("  Progress: {}%", 
        format!("{:.0}", project.get_progress_percentage()).bright_green()
    );
    println!();

    // Description
    if let Some(description) = &project.description {
        println!("{}", "📄 Description".bright_blue().bold());
        println!("  {}", description.bright_cyan());
        println!();
    }

    // Technologies
    if !project.technologies.is_empty() {
        println!("{}", "🔧 Technologies".bright_blue().bold());
        for tech in &project.technologies {
            println!("  • {}", tech.bright_yellow());
        }
        println!();
    }

    // Tags
    if !project.tags.is_empty() {
        println!("{}", "🏷️  Tags".bright_blue().bold());
        let tag_string = project.tags.iter()
            .map(|tag| format!("#{}", tag))
            .collect::<Vec<_>>()
            .join(" ");
        println!("  {}", tag_string.bright_magenta());
        println!();
    }

    // Timeline
    println!("{}", "⏰ Timeline".bright_blue().bold());
    println!("  Created: {}", 
        project.created.format("%Y-%m-%d %H:%M UTC").to_string().bright_cyan()
    );
    println!("  Last Updated: {}", 
        project.updated.format("%Y-%m-%d %H:%M UTC").to_string().bright_cyan()
    );
    println!("  Days Active: {}", 
        project.days_since_creation().to_string().bright_green()
    );
    println!("  Days Since Update: {}", 
        project.days_since_update().to_string().bright_yellow()
    );
    println!();

    Ok(())
}

async fn print_project_files(project: &crate::core::Project) -> Result<()> {
    if let Some(project_path) = &project.path {
        println!("{}", "📁 Project Files".bright_blue().bold());
        
        // List key files and their status
        let key_files = [
            ("00_PROJECT-OVERVIEW.md", "Project Overview"),
            ("01_Requirements.md", "Requirements"),
            ("02_Architecture.md", "Architecture"),
            ("03_Development-Log.md", "Development Log"),
            ("04_Testing.md", "Testing"),
            ("05_Deployment.md", "Deployment"),
        ];

        for (filename, description) in &key_files {
            let file_path = project_path.join(filename);
            let status = if file_path.exists() {
                if let Ok(metadata) = tokio::fs::metadata(&file_path).await {
                    let size = metadata.len();
                    if size > 100 {
                        "✅ Complete".bright_green()
                    } else {
                        "📝 Started".bright_yellow()
                    }
                } else {
                    "❌ Missing".bright_red()
                }
            } else {
                "❌ Missing".bright_red()
            };

            println!("  {} - {}", description.bright_white(), status);
        }

        // Show assets if they exist
        let assets_path = project_path.join("assets");
        if assets_path.exists() {
            if let Ok(mut entries) = tokio::fs::read_dir(&assets_path).await {
                let mut asset_count = 0;
                while let Ok(Some(_)) = entries.next_entry().await {
                    asset_count += 1;
                }
                if asset_count > 0 {
                    println!("  Assets - {} {} found", 
                        asset_count.to_string().bright_cyan(),
                        if asset_count == 1 { "file" } else { "files" }
                    );
                }
            }
        }

        println!();
        println!("  📍 Location: {}", 
            project_path.display().to_string().bright_yellow()
        );
        println!();
    }

    Ok(())
}

fn print_project_statistics(project: &crate::core::Project) -> Result<()> {
    println!("{}", "📊 Statistics".bright_blue().bold());
    
    // Status history count
    println!("  Status Changes: {}", 
        project.status_history.len().to_string().bright_cyan()
    );
    
    // Activity status
    let activity_status = if project.is_active() {
        "Active".bright_green()
    } else {
        match project.status {
            crate::core::ProjectStatus::Completed => "Completed".bright_blue(),
            crate::core::ProjectStatus::Archived => "Archived".bright_black(),
            crate::core::ProjectStatus::Cancelled => "Cancelled".bright_red(),
            _ => "Inactive".bright_yellow(),
        }
    };
    
    println!("  Activity: {}", activity_status);
    
    // Time in current status
    if let Some(current_entry) = project.status_history.last() {
        let time_in_status = chrono::Utc::now()
            .signed_duration_since(current_entry.timestamp)
            .num_days();
        
        println!("  Time in Current Status: {} days", 
            time_in_status.to_string().bright_cyan()
        );
    }

    println!();

    // Quick actions
    println!("{}", "🚀 Quick Actions".bright_blue().bold());
    println!("  Update status: {}", 
        format!("opj status \"{}\" --set <status>", project.name).bright_cyan()
    );
    println!("  View history: {}", 
        format!("opj status \"{}\" --log", project.name).bright_cyan()
    );
    
    if let Some(project_path) = &project.path {
        println!("  Open in editor: {}", 
            format!("cd \"{}\"", project_path.display()).bright_cyan()
        );
    }

    Ok(())
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

fn format_status(status: &crate::core::ProjectStatus) -> ColoredString {
    match status {
        crate::core::ProjectStatus::Idea => "💡 Idea".bright_blue(),
        crate::core::ProjectStatus::Planning => "📋 Planning".bright_cyan(),
        crate::core::ProjectStatus::Development => "🚧 Development".bright_green(),
        crate::core::ProjectStatus::Testing => "🧪 Testing".bright_yellow(),
        crate::core::ProjectStatus::Completed => "✅ Completed".bright_green().bold(),
        crate::core::ProjectStatus::Archived => "📦 Archived".bright_black(),
        crate::core::ProjectStatus::Cancelled => "❌ Cancelled".bright_red(),
    }
}

fn format_priority(priority: &crate::core::Priority) -> ColoredString {
    match priority {
        crate::core::Priority::High => "🔴 High".bright_red(),
        crate::core::Priority::Medium => "🟡 Medium".bright_yellow(),
        crate::core::Priority::Low => "🔵 Low".bright_blue(),
    }
}
