use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub project_type: ProjectType,
    pub status: ProjectStatus,
    pub priority: Priority,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub technologies: Vec<String>,
    pub status_history: Vec<StatusEntry>,
    #[serde(skip)]
    pub path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ProjectType {
    WebApp,
    Tool,
    Content,
    Api,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ProjectStatus {
    Idea,
    Planning,
    Development,
    Testing,
    Completed,
    Archived,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusEntry {
    pub status: ProjectStatus,
    pub timestamp: DateTime<Utc>,
    pub note: Option<String>,
}

impl Project {
    pub fn new(
        name: String,
        project_type: crate::cli::ProjectType,
        priority: crate::cli::Priority,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        
        let project_type = match project_type {
            crate::cli::ProjectType::WebApp => ProjectType::WebApp,
            crate::cli::ProjectType::Tool => ProjectType::Tool,
            crate::cli::ProjectType::Content => ProjectType::Content,
            crate::cli::ProjectType::Api => ProjectType::Api,
        };

        let priority = match priority {
            crate::cli::Priority::Low => Priority::Low,
            crate::cli::Priority::Medium => Priority::Medium,
            crate::cli::Priority::High => Priority::High,
        };

        let status = ProjectStatus::Idea;
        let status_history = vec![StatusEntry {
            status: status.clone(),
            timestamp: now,
            note: Some("Project created".to_string()),
        }];

        let mut tags = vec![
            "project".to_string(),
            format!("{:?}", project_type).to_lowercase(),
        ];

        // Add priority tag
        tags.push(format!("priority-{:?}", priority).to_lowercase());

        Self {
            id,
            name,
            project_type,
            status,
            priority,
            created: now,
            updated: now,
            description,
            tags,
            technologies: Vec::new(),
            status_history,
            path: None,
        }
    }

    pub fn update_status(&mut self, new_status: ProjectStatus, note: Option<String>) {
        self.status = new_status.clone();
        self.updated = Utc::now();
        
        self.status_history.push(StatusEntry {
            status: new_status,
            timestamp: self.updated,
            note,
        });

        // Update status tag
        self.tags.retain(|tag| !tag.starts_with("status-"));
        self.tags.push(format!("status-{:?}", self.status).to_lowercase());
    }

    pub fn add_technology(&mut self, tech: String) {
        if !self.technologies.contains(&tech) {
            self.technologies.push(tech.clone());
            // Add as tag too (in lowercase)
            let tag = tech.to_lowercase().replace(" ", "-");
            if !self.tags.contains(&tag) {
                self.tags.push(tag);
            }
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn get_progress_percentage(&self) -> f32 {
        match self.status {
            ProjectStatus::Idea => 10.0,
            ProjectStatus::Planning => 25.0,
            ProjectStatus::Development => 60.0,
            ProjectStatus::Testing => 85.0,
            ProjectStatus::Completed => 100.0,
            ProjectStatus::Archived => 100.0,
            ProjectStatus::Cancelled => 0.0,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            ProjectStatus::Planning | ProjectStatus::Development | ProjectStatus::Testing
        )
    }

    pub fn days_since_creation(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.created).num_days()
    }

    pub fn days_since_update(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.updated).num_days()
    }
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::WebApp => write!(f, "Web-App"),
            ProjectType::Tool => write!(f, "Tool"),
            ProjectType::Content => write!(f, "Content"),
            ProjectType::Api => write!(f, "API"),
            ProjectType::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Idea => write!(f, "Idea"),
            ProjectStatus::Planning => write!(f, "Planning"),
            ProjectStatus::Development => write!(f, "Development"),
            ProjectStatus::Testing => write!(f, "Testing"),
            ProjectStatus::Completed => write!(f, "Completed"),
            ProjectStatus::Archived => write!(f, "Archived"),
            ProjectStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

// Conversion from CLI enums to core enums
impl From<crate::cli::ProjectType> for ProjectType {
    fn from(cli_type: crate::cli::ProjectType) -> Self {
        match cli_type {
            crate::cli::ProjectType::WebApp => ProjectType::WebApp,
            crate::cli::ProjectType::Tool => ProjectType::Tool,
            crate::cli::ProjectType::Content => ProjectType::Content,
            crate::cli::ProjectType::Api => ProjectType::Api,
        }
    }
}

impl From<crate::cli::ProjectStatus> for ProjectStatus {
    fn from(cli_status: crate::cli::ProjectStatus) -> Self {
        match cli_status {
            crate::cli::ProjectStatus::Idea => ProjectStatus::Idea,
            crate::cli::ProjectStatus::Planning => ProjectStatus::Planning,
            crate::cli::ProjectStatus::Development => ProjectStatus::Development,
            crate::cli::ProjectStatus::Testing => ProjectStatus::Testing,
            crate::cli::ProjectStatus::Completed => ProjectStatus::Completed,
            crate::cli::ProjectStatus::Archived => ProjectStatus::Archived,
            crate::cli::ProjectStatus::Cancelled => ProjectStatus::Cancelled,
        }
    }
}

impl From<crate::cli::Priority> for Priority {
    fn from(cli_priority: crate::cli::Priority) -> Self {
        match cli_priority {
            crate::cli::Priority::Low => Priority::Low,
            crate::cli::Priority::Medium => Priority::Medium,
            crate::cli::Priority::High => Priority::High,
        }
    }
}
