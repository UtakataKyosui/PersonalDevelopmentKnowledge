use anyhow::Result;
use colored::*;
use tabled::{Table, Tabled};

use crate::cli::{ProjectType, ProjectStatus, Priority, SortField, OutputFormat};
use crate::core::Project;
use crate::config::Settings;
use crate::fs::scanner::ProjectScanner;

#[derive(Tabled)]
struct ProjectRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    project_type: String,
    #[tabled(rename = "Priority")]
    priority: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Updated")]
    updated: String,
}

pub async fn execute(
    status_filter: Option<ProjectStatus>,
    type_filter: Option<ProjectType>,
    priority_filter: Option<Priority>,
    sort: SortField,
    format: OutputFormat,
) -> Result<()> {
    let settings = Settings::load().await?;
    let scanner = ProjectScanner::new(settings.projects_path());
    
    let mut projects = scanner.scan_projects().await?;

    // Apply filters
    if let Some(status) = status_filter {
        projects.retain(|p| std::mem::discriminant(&p.status) == std::mem::discriminant(&status));
    }

    if let Some(project_type) = type_filter {
        projects.retain(|p| std::mem::discriminant(&p.project_type) == std::mem::discriminant(&project_type));
    }

    if let Some(priority) = priority_filter {
        projects.retain(|p| std::mem::discriminant(&p.priority) == std::mem::discriminant(&priority));
    }

    // Sort projects
    projects.sort_by(|a, b| {
        match sort {
            SortField::Name => a.name.cmp(&b.name),
            SortField::Date => a.created.cmp(&b.created),
            SortField::Updated => b.updated.cmp(&a.updated), // Most recent first
            SortField::Status => a.status.to_string().cmp(&b.status.to_string()),
            SortField::Priority => priority_order(&b.priority).cmp(&priority_order(&a.priority)), // High first
            SortField::Type => a.project_type.to_string().cmp(&b.project_type.to_string()),
        }
    });

    match format {
        OutputFormat::Table => print_table(&projects).await?,
        OutputFormat::Json => print_json(&projects).await?,
        OutputFormat::Yaml => print_yaml(&projects).await?,
    }

    Ok(())
}

async fn print_table(projects: &[Project]) -> Result<()> {
    if projects.is_empty() {
        println!("{}", "No projects found matching the criteria.".bright_yellow());
        return Ok(());
    }

    // Print header with status info
    let status_counts = count_by_status(projects);
    let active_count = projects.iter()
        .filter(|p| matches!(p.status, 
            crate::core::ProjectStatus::Planning | 
            crate::core::ProjectStatus::Development |
            crate::core::ProjectStatus::Testing
        ))
        .count();

    println!("{} ({} projects, {} active)", 
        "📊 Projects".bright_cyan().bold(),
        projects.len().to_string().bright_white(),
        active_count.to_string().bright_green()
    );
    println!();

    // Convert to table rows
    let rows: Vec<ProjectRow> = projects.iter().map(|project| {
        ProjectRow {
            name: format!("{} {}", 
                get_type_emoji(&project.project_type),
                project.name
            ),
            project_type: format_project_type(&project.project_type),
            priority: format_priority(&project.priority),
            status: format_status(&project.status),
            updated: format_relative_time(&project.updated),
        }
    }).collect();

    let table = Table::new(rows)
        .with(tabled::settings::Style::rounded())
        .to_string();

    println!("{}", table);
    println!();

    // Print help hints
    println!("{} Use {} for detailed information", 
        "💡".bright_blue(),
        "'opj info <name>'".bright_cyan()
    );
    println!("{} Use {} to update status", 
        "🔄".bright_blue(),
        "'opj status <name>'".bright_cyan()
    );

    Ok(())
}

async fn print_json(projects: &[Project]) -> Result<()> {
    let json = serde_json::to_string_pretty(projects)?;
    println!("{}", json);
    Ok(())
}

async fn print_yaml(projects: &[Project]) -> Result<()> {
    let yaml = serde_yaml::to_string(projects)?;
    println!("{}", yaml);
    Ok(())
}

fn count_by_status(projects: &[Project]) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for project in projects {
        let status = project.status.to_string();
        *counts.entry(status).or_insert(0) += 1;
    }
    counts
}

fn get_type_emoji(project_type: &crate::core::ProjectType) -> &'static str {
    match project_type {
        crate::core::ProjectType::WebApp => "🌐",
        crate::core::ProjectType::Tool => "🛠️",
        crate::core::ProjectType::Content => "📝",
        crate::core::ProjectType::Api => "🔌",
        crate::core::ProjectType::Custom(_) => "📦",
    }
}

fn format_project_type(project_type: &crate::core::ProjectType) -> String {
    match project_type {
        crate::core::ProjectType::WebApp => "Web-App".to_string(),
        crate::core::ProjectType::Tool => "Tool".to_string(),
        crate::core::ProjectType::Content => "Content".to_string(),
        crate::core::ProjectType::Api => "API".to_string(),
        crate::core::ProjectType::Custom(name) => name.clone(),
    }
}

fn format_priority(priority: &crate::core::Priority) -> String {
    match priority {
        crate::core::Priority::High => "High".bright_red().to_string(),
        crate::core::Priority::Medium => "Medium".bright_yellow().to_string(),
        crate::core::Priority::Low => "Low".bright_blue().to_string(),
    }
}

fn format_status(status: &crate::core::ProjectStatus) -> String {
    match status {
        crate::core::ProjectStatus::Idea => "Idea".bright_blue().to_string(),
        crate::core::ProjectStatus::Planning => "Planning".bright_cyan().to_string(),
        crate::core::ProjectStatus::Development => "Development".bright_green().to_string(),
        crate::core::ProjectStatus::Testing => "Testing".bright_yellow().to_string(),
        crate::core::ProjectStatus::Completed => "Completed".bright_green().bold().to_string(),
        crate::core::ProjectStatus::Archived => "Archived".bright_black().to_string(),
        crate::core::ProjectStatus::Cancelled => "Cancelled".bright_red().to_string(),
    }
}

fn format_relative_time(time: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(*time);

    if duration.num_days() == 0 {
        if duration.num_hours() == 0 {
            if duration.num_minutes() == 0 {
                "Just now".to_string()
            } else {
                format!("{} min ago", duration.num_minutes())
            }
        } else {
            format!("{} hours ago", duration.num_hours())
        }
    } else if duration.num_days() == 1 {
        "1 day ago".to_string()
    } else if duration.num_days() < 7 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_weeks() == 1 {
        "1 week ago".to_string()
    } else if duration.num_weeks() < 4 {
        format!("{} weeks ago", duration.num_weeks())
    } else {
        time.format("%Y-%m-%d").to_string()
    }
}

fn priority_order(priority: &crate::core::Priority) -> u8 {
    match priority {
        crate::core::Priority::High => 3,
        crate::core::Priority::Medium => 2,
        crate::core::Priority::Low => 1,
    }
}
