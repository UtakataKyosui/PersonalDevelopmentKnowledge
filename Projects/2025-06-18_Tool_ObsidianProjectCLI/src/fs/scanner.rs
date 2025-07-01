use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use crate::core::Project;

pub struct ProjectScanner {
    projects_path: PathBuf,
}

impl ProjectScanner {
    pub fn new<P: AsRef<Path>>(projects_path: P) -> Self {
        Self {
            projects_path: projects_path.as_ref().to_path_buf(),
        }
    }

    pub async fn scan_projects(&self) -> Result<Vec<Project>> {
        if !self.projects_path.exists() {
            return Ok(Vec::new());
        }

        let mut projects = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.projects_path).await
            .context("Failed to read projects directory")?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                let dir_name = entry.file_name();
                let dir_name_str = dir_name.to_string_lossy();
                
                // Skip template and hidden directories
                if dir_name_str.starts_with('_') || dir_name_str.starts_with('.') {
                    continue;
                }

                // Look for project metadata
                let project_path = entry.path();
                if let Ok(project) = self.load_project_from_directory(&project_path).await {
                    projects.push(project);
                }
            }
        }

        // Sort by update time (most recent first)
        projects.sort_by(|a, b| b.updated.cmp(&a.updated));
        
        Ok(projects)
    }

    pub async fn find_project_by_name(&self, name: &str) -> Result<Option<Project>> {
        let projects = self.scan_projects().await?;
        
        // Try exact match first
        for project in &projects {
            if project.name == name {
                return Ok(Some(project.clone()));
            }
        }

        // Try case-insensitive match
        for project in &projects {
            if project.name.to_lowercase() == name.to_lowercase() {
                return Ok(Some(project.clone()));
            }
        }

        // Try partial match
        for project in &projects {
            if project.name.to_lowercase().contains(&name.to_lowercase()) {
                return Ok(Some(project.clone()));
            }
        }

        Ok(None)
    }

    pub async fn find_project_by_id(&self, id: &str) -> Result<Option<Project>> {
        let projects = self.scan_projects().await?;
        
        for project in projects {
            if project.id == id {
                return Ok(Some(project));
            }
        }

        Ok(None)
    }

    async fn load_project_from_directory(&self, project_path: &Path) -> Result<Project> {
        // Look for metadata file
        let metadata_path = project_path.join(".project-meta.yaml");
        
        if metadata_path.exists() {
            // Load from metadata file
            let content = tokio::fs::read_to_string(&metadata_path).await
                .context("Failed to read project metadata")?;
            
            let mut project: Project = serde_yaml::from_str(&content)
                .context("Failed to parse project metadata")?;
            
            project.path = Some(project_path.to_path_buf());
            return Ok(project);
        }

        // Try to extract info from directory name and overview file
        self.parse_project_from_structure(project_path).await
    }

    async fn parse_project_from_structure(&self, project_path: &Path) -> Result<Project> {
        let dir_name = project_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid directory name"))?;

        // Parse directory name format: YYYY-MM-DD_Type_ProjectName
        let parts: Vec<&str> = dir_name.splitn(3, '_').collect();
        
        let (project_name, project_type) = if parts.len() >= 3 {
            let name = parts[2].replace('-', " ");
            let type_str = parts[1].to_lowercase();
            
            let project_type = match type_str.as_str() {
                "web-app" | "webapp" => crate::core::ProjectType::WebApp,
                "tool" | "cli" => crate::core::ProjectType::Tool,
                "content" | "article" => crate::core::ProjectType::Content,
                "api" | "backend" => crate::core::ProjectType::Api,
                _ => crate::core::ProjectType::Custom(type_str),
            };
            
            (name, project_type)
        } else {
            // Fallback: use directory name as project name
            (dir_name.to_string(), crate::core::ProjectType::Custom("Unknown".to_string()))
        };

        // Try to read additional info from overview file
        let overview_path = project_path.join("00_PROJECT-OVERVIEW.md");
        let (description, status, priority, tags) = if overview_path.exists() {
            self.parse_overview_file(&overview_path).await.unwrap_or_default()
        } else {
            (None, crate::core::ProjectStatus::Idea, crate::core::Priority::Medium, Vec::new())
        };

        // Create project with extracted information
        let now = chrono::Utc::now();
        let created = self.get_directory_creation_time(project_path).await.unwrap_or(now);
        let updated = self.get_directory_modification_time(project_path).await.unwrap_or(now);

        let project = Project {
            id: uuid::Uuid::new_v4().to_string(),
            name: project_name,
            project_type,
            status,
            priority,
            created,
            updated,
            description,
            tags,
            technologies: Vec::new(),
            status_history: vec![crate::core::StatusEntry {
                status: status.clone(),
                timestamp: created,
                note: Some("Project discovered".to_string()),
            }],
            path: Some(project_path.to_path_buf()),
        };

        Ok(project)
    }

    async fn parse_overview_file(&self, overview_path: &Path) -> Result<(Option<String>, crate::core::ProjectStatus, crate::core::Priority, Vec<String>)> {
        let content = tokio::fs::read_to_string(overview_path).await?;
        
        let mut description = None;
        let mut status = crate::core::ProjectStatus::Idea;
        let mut priority = crate::core::Priority::Medium;
        let mut tags = Vec::new();

        // Parse frontmatter if present
        if content.starts_with("---") {
            let end_marker = content[3..].find("---").map(|i| i + 6);
            if let Some(end) = end_marker {
                let frontmatter = &content[3..end-3];
                
                // Parse YAML frontmatter
                if let Ok(metadata) = serde_yaml::from_str::<serde_yaml::Value>(frontmatter) {
                    if let Some(map) = metadata.as_mapping() {
                        // Extract description
                        if let Some(desc) = map.get("description").and_then(|v| v.as_str()) {
                            description = Some(desc.to_string());
                        }

                        // Extract status
                        if let Some(status_str) = map.get("status").and_then(|v| v.as_str()) {
                            status = parse_status(status_str);
                        }

                        // Extract priority
                        if let Some(priority_str) = map.get("priority").and_then(|v| v.as_str()) {
                            priority = parse_priority(priority_str);
                        }

                        // Extract tags
                        if let Some(tags_array) = map.get("tags").and_then(|v| v.as_sequence()) {
                            for tag in tags_array {
                                if let Some(tag_str) = tag.as_str() {
                                    tags.push(tag_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        // If no description in frontmatter, try to extract from content
        if description.is_none() {
            description = extract_description_from_content(&content);
        }

        Ok((description, status, priority, tags))
    }

    async fn get_directory_creation_time(&self, path: &Path) -> Result<chrono::DateTime<chrono::Utc>> {
        let metadata = tokio::fs::metadata(path).await?;
        let created = metadata.created()
            .or_else(|_| metadata.modified())
            .unwrap_or_else(|_| std::time::SystemTime::now());
        
        Ok(chrono::DateTime::from(created))
    }

    async fn get_directory_modification_time(&self, path: &Path) -> Result<chrono::DateTime<chrono::Utc>> {
        let metadata = tokio::fs::metadata(path).await?;
        let modified = metadata.modified()
            .unwrap_or_else(|_| std::time::SystemTime::now());
        
        Ok(chrono::DateTime::from(modified))
    }
}

fn parse_status(status_str: &str) -> crate::core::ProjectStatus {
    match status_str.to_lowercase().as_str() {
        "idea" => crate::core::ProjectStatus::Idea,
        "planning" => crate::core::ProjectStatus::Planning,
        "development" => crate::core::ProjectStatus::Development,
        "testing" => crate::core::ProjectStatus::Testing,
        "completed" => crate::core::ProjectStatus::Completed,
        "archived" => crate::core::ProjectStatus::Archived,
        "cancelled" => crate::core::ProjectStatus::Cancelled,
        _ => crate::core::ProjectStatus::Idea,
    }
}

fn parse_priority(priority_str: &str) -> crate::core::Priority {
    match priority_str.to_lowercase().as_str() {
        "high" => crate::core::Priority::High,
        "medium" => crate::core::Priority::Medium,
        "low" => crate::core::Priority::Low,
        _ => crate::core::Priority::Medium,
    }
}

fn extract_description_from_content(content: &str) -> Option<String> {
    // Look for description after first heading
    let lines: Vec<&str> = content.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("## ") && line.to_lowercase().contains("概要") {
            // Found overview section, get next non-empty line
            for j in (i + 1)..lines.len() {
                let desc_line = lines[j].trim();
                if !desc_line.is_empty() && !desc_line.starts_with('#') {
                    return Some(desc_line.to_string());
                }
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_parse_project_from_directory_name() {
        let temp_dir = tempdir().unwrap();
        let project_path = temp_dir.path().join("2025-06-18_Web-App_Test-Project");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let scanner = ProjectScanner::new(temp_dir.path());
        let project = scanner.parse_project_from_structure(&project_path).await.unwrap();

        assert_eq!(project.name, "Test Project");
        assert!(matches!(project.project_type, crate::core::ProjectType::WebApp));
    }

    #[tokio::test]
    async fn test_scan_projects() {
        let temp_dir = tempdir().unwrap();
        
        // Create test project directories
        tokio::fs::create_dir_all(temp_dir.path().join("2025-06-18_Tool_CLI-Tool")).await.unwrap();
        tokio::fs::create_dir_all(temp_dir.path().join("2025-06-19_Content_Blog-Post")).await.unwrap();
        tokio::fs::create_dir_all(temp_dir.path().join("_TEMPLATES")).await.unwrap(); // Should be skipped

        let scanner = ProjectScanner::new(temp_dir.path());
        let projects = scanner.scan_projects().await.unwrap();

        assert_eq!(projects.len(), 2);
        assert!(projects.iter().any(|p| p.name == "CLI Tool"));
        assert!(projects.iter().any(|p| p.name == "Blog Post"));
    }
}
