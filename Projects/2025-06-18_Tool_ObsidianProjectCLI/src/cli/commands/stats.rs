use anyhow::Result;
use colored::*;
use crate::config::Settings;
use crate::fs::scanner::ProjectScanner;

pub async fn execute() -> Result<()> {
    let settings = Settings::load().await?;
    let scanner = ProjectScanner::new(settings.projects_path());
    let projects = scanner.scan_projects().await?;

    println!("{}", "📊 Project Statistics".bright_cyan().bold());
    println!();

    // Overall statistics
    print_overall_stats(&projects)?;
    print_status_breakdown(&projects)?;
    print_type_breakdown(&projects)?;
    print_priority_breakdown(&projects)?;
    print_activity_stats(&projects)?;

    Ok(())
}

fn print_overall_stats(projects: &[crate::core::Project]) -> Result<()> {
    let total_projects = projects.len();
    let active_projects = projects.iter()
        .filter(|p| p.is_active())
        .count();
    let completed_projects = projects.iter()
        .filter(|p| matches!(p.status, crate::core::ProjectStatus::Completed))
        .count();

    println!("{}", "📈 Overall".bright_blue().bold());
    println!("  Total Projects: {}", total_projects.to_string().bright_white().bold());
    println!("  Active Projects: {}", active_projects.to_string().bright_green());
    println!("  Completed Projects: {}", completed_projects.to_string().bright_cyan());
    println!("  Completion Rate: {}%", 
        if total_projects > 0 {
            format!("{:.1}", (completed_projects as f32 / total_projects as f32) * 100.0).bright_yellow()
        } else {
            "0".bright_yellow()
        }
    );
    println!();

    Ok(())
}

fn print_status_breakdown(projects: &[crate::core::Project]) -> Result<()> {
    println!("{}", "📋 Status Breakdown".bright_blue().bold());

    let statuses = [
        ("Idea", crate::core::ProjectStatus::Idea),
        ("Planning", crate::core::ProjectStatus::Planning),
        ("Development", crate::core::ProjectStatus::Development),
        ("Testing", crate::core::ProjectStatus::Testing),
        ("Completed", crate::core::ProjectStatus::Completed),
        ("Archived", crate::core::ProjectStatus::Archived),
        ("Cancelled", crate::core::ProjectStatus::Cancelled),
    ];

    for (name, status) in &statuses {
        let count = projects.iter()
            .filter(|p| std::mem::discriminant(&p.status) == std::mem::discriminant(status))
            .count();
        
        if count > 0 {
            let emoji = match status {
                crate::core::ProjectStatus::Idea => "💡",
                crate::core::ProjectStatus::Planning => "📋",
                crate::core::ProjectStatus::Development => "🚧",
                crate::core::ProjectStatus::Testing => "🧪",
                crate::core::ProjectStatus::Completed => "✅",
                crate::core::ProjectStatus::Archived => "📦",
                crate::core::ProjectStatus::Cancelled => "❌",
            };
            
            println!("  {} {}: {}", emoji, name, count.to_string().bright_white());
        }
    }
    println!();

    Ok(())
}

fn print_type_breakdown(projects: &[crate::core::Project]) -> Result<()> {
    println!("{}", "🏷️ Type Breakdown".bright_blue().bold());

    let mut type_counts = std::collections::HashMap::new();
    for project in projects {
        let type_name = match &project.project_type {
            crate::core::ProjectType::WebApp => "Web-App",
            crate::core::ProjectType::Tool => "Tool",
            crate::core::ProjectType::Content => "Content",
            crate::core::ProjectType::Api => "API",
            crate::core::ProjectType::Custom(name) => name.as_str(),
        };
        *type_counts.entry(type_name).or_insert(0) += 1;
    }

    let mut sorted_types: Vec<_> = type_counts.iter().collect();
    sorted_types.sort_by_key(|(_, count)| std::cmp::Reverse(**count));

    for (type_name, count) in sorted_types {
        let emoji = match *type_name {
            "Web-App" => "🌐",
            "Tool" => "🛠️",
            "Content" => "📝",
            "API" => "🔌",
            _ => "📦",
        };
        println!("  {} {}: {}", emoji, type_name, count.to_string().bright_white());
    }
    println!();

    Ok(())
}

fn print_priority_breakdown(projects: &[crate::core::Project]) -> Result<()> {
    println!("{}", "🎯 Priority Breakdown".bright_blue().bold());

    let priorities = [
        ("High", crate::core::Priority::High, "🔴"),
        ("Medium", crate::core::Priority::Medium, "🟡"),
        ("Low", crate::core::Priority::Low, "🔵"),
    ];

    for (name, priority, emoji) in &priorities {
        let count = projects.iter()
            .filter(|p| std::mem::discriminant(&p.priority) == std::mem::discriminant(priority))
            .count();
        
        if count > 0 {
            println!("  {} {}: {}", emoji, name, count.to_string().bright_white());
        }
    }
    println!();

    Ok(())
}

fn print_activity_stats(projects: &[crate::core::Project]) -> Result<()> {
    if projects.is_empty() {
        return Ok(());
    }

    println!("{}", "⏰ Activity Stats".bright_blue().bold());

    // Find most recent activity
    let most_recent = projects.iter()
        .max_by_key(|p| p.updated);
    
    if let Some(recent_project) = most_recent {
        let days_ago = recent_project.days_since_update();
        println!("  Most Recent Activity: {} ({} days ago)", 
            recent_project.name.bright_white(),
            days_ago.to_string().bright_cyan()
        );
    }

    // Find oldest project
    let oldest = projects.iter()
        .min_by_key(|p| p.created);
    
    if let Some(old_project) = oldest {
        let days_old = old_project.days_since_creation();
        println!("  Oldest Project: {} ({} days old)", 
            old_project.name.bright_white(),
            days_old.to_string().bright_cyan()
        );
    }

    // Average project age
    let total_age: i64 = projects.iter()
        .map(|p| p.days_since_creation())
        .sum();
    let avg_age = total_age as f32 / projects.len() as f32;
    
    println!("  Average Project Age: {:.1} days", 
        avg_age.to_string().bright_cyan()
    );

    // Active projects needing attention (not updated in a week)
    let stale_projects = projects.iter()
        .filter(|p| p.is_active() && p.days_since_update() > 7)
        .count();
    
    if stale_projects > 0 {
        println!("  {} Projects Need Attention (>7 days inactive)", 
            stale_projects.to_string().bright_red()
        );
    }

    println!();

    Ok(())
}
