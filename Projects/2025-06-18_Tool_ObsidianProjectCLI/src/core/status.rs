// Status management utilities

use crate::core::{ProjectStatus, StatusEntry};
use chrono::{DateTime, Utc};

pub fn create_status_entry(status: ProjectStatus, note: Option<String>) -> StatusEntry {
    StatusEntry {
        status,
        timestamp: Utc::now(),
        note,
    }
}

pub fn get_status_progression() -> Vec<ProjectStatus> {
    vec![
        ProjectStatus::Idea,
        ProjectStatus::Planning,
        ProjectStatus::Development,
        ProjectStatus::Testing,
        ProjectStatus::Completed,
    ]
}

pub fn get_next_suggested_status(current: &ProjectStatus) -> Option<ProjectStatus> {
    match current {
        ProjectStatus::Idea => Some(ProjectStatus::Planning),
        ProjectStatus::Planning => Some(ProjectStatus::Development),
        ProjectStatus::Development => Some(ProjectStatus::Testing),
        ProjectStatus::Testing => Some(ProjectStatus::Completed),
        ProjectStatus::Completed => None,
        ProjectStatus::Archived => None,
        ProjectStatus::Cancelled => None,
    }
}

pub fn is_valid_transition(from: &ProjectStatus, to: &ProjectStatus) -> bool {
    // Allow any transition for now - we might add restrictions later
    !matches!((from, to), 
        (ProjectStatus::Completed, ProjectStatus::Idea) |
        (ProjectStatus::Completed, ProjectStatus::Planning) |
        (ProjectStatus::Completed, ProjectStatus::Development) |
        (ProjectStatus::Completed, ProjectStatus::Testing)
    )
}
