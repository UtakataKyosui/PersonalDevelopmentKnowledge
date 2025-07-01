// Placeholder for metadata management
// This will be expanded in future versions

use crate::core::Project;
use anyhow::Result;
use std::path::Path;

pub async fn save_project_metadata(project: &Project, project_path: &Path) -> Result<()> {
    let metadata_path = project_path.join(".project-meta.yaml");
    let metadata_content = serde_yaml::to_string(project)?;
    tokio::fs::write(metadata_path, metadata_content).await?;
    Ok(())
}

pub async fn load_project_metadata(project_path: &Path) -> Result<Project> {
    let metadata_path = project_path.join(".project-meta.yaml");
    let content = tokio::fs::read_to_string(metadata_path).await?;
    let project: Project = serde_yaml::from_str(&content)?;
    Ok(project)
}
