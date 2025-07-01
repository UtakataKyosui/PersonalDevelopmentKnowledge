use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub obsidian_path: PathBuf,
    pub projects_dir: String,
    pub templates_dir: String,
    pub default_type: crate::cli::ProjectType,
    pub default_priority: crate::cli::Priority,
    pub auto_git_init: bool,
    pub editor: Option<String>,
    pub date_format: String,
    pub status_values: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            obsidian_path: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Documents")
                .join("Obsidian-Vault"),
            projects_dir: "Projects".to_string(),
            templates_dir: "Projects/_TEMPLATES".to_string(),
            default_type: crate::cli::ProjectType::WebApp,
            default_priority: crate::cli::Priority::Medium,
            auto_git_init: true,
            editor: Some("code".to_string()),
            date_format: "%Y-%m-%d".to_string(),
            status_values: vec![
                "idea".to_string(),
                "planning".to_string(),
                "development".to_string(),
                "testing".to_string(),
                "completed".to_string(),
                "archived".to_string(),
                "cancelled".to_string(),
            ],
        }
    }
}

impl Settings {
    pub async fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;
        
        if config_path.exists() {
            let content = tokio::fs::read_to_string(&config_path).await
                .context("Failed to read config file")?;
            
            let settings: Settings = serde_yaml::from_str(&content)
                .context("Failed to parse config file")?;
                
            Ok(settings)
        } else {
            // Create default config
            let settings = Self::default();
            settings.save().await?;
            Ok(settings)
        }
    }

    pub async fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .context("Failed to create config directory")?;
        }

        let content = serde_yaml::to_string(self)
            .context("Failed to serialize config")?;
            
        tokio::fs::write(&config_path, content).await
            .context("Failed to write config file")?;
            
        Ok(())
    }

    pub fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?;
            
        Ok(config_dir.join("opj").join("config.yaml"))
    }

    pub fn projects_path(&self) -> PathBuf {
        self.obsidian_path.join(&self.projects_dir)
    }

    pub fn templates_path(&self) -> PathBuf {
        self.obsidian_path.join(&self.templates_dir)
    }

    pub fn set_obsidian_path<P: AsRef<Path>>(&mut self, path: P) {
        self.obsidian_path = path.as_ref().to_path_buf();
    }

    pub fn set_default_type(&mut self, project_type: crate::cli::ProjectType) {
        self.default_type = project_type;
    }

    pub fn set_default_priority(&mut self, priority: crate::cli::Priority) {
        self.default_priority = priority;
    }

    pub fn set_auto_git_init(&mut self, enabled: bool) {
        self.auto_git_init = enabled;
    }

    pub fn set_editor<S: Into<String>>(&mut self, editor: Option<S>) {
        self.editor = editor.map(|e| e.into());
    }

    pub fn validate(&self) -> Result<()> {
        // Check if obsidian path exists
        if !self.obsidian_path.exists() {
            return Err(anyhow::anyhow!(
                "Obsidian path does not exist: {}",
                self.obsidian_path.display()
            ));
        }

        // Check if projects directory exists or can be created
        let projects_path = self.projects_path();
        if !projects_path.exists() {
            if let Err(e) = std::fs::create_dir_all(&projects_path) {
                return Err(anyhow::anyhow!(
                    "Cannot create projects directory {}: {}",
                    projects_path.display(),
                    e
                ));
            }
        }

        // Check if templates directory exists
        let templates_path = self.templates_path();
        if !templates_path.exists() {
            eprintln!(
                "Warning: Templates directory does not exist: {}",
                templates_path.display()
            );
        }

        Ok(())
    }

    pub fn get_template_path(&self, template_name: &str) -> PathBuf {
        self.templates_path().join(template_name)
    }

    pub async fn list_available_templates(&self) -> Result<Vec<String>> {
        let templates_path = self.templates_path();
        
        if !templates_path.exists() {
            return Ok(Vec::new());
        }

        let mut templates = Vec::new();
        let mut entries = tokio::fs::read_dir(&templates_path).await
            .context("Failed to read templates directory")?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if !name.starts_with('.') && !name.starts_with('_') {
                        templates.push(name.to_string());
                    }
                }
            }
        }

        templates.sort();
        Ok(templates)
    }

    pub fn auto_detect_obsidian_path() -> Option<PathBuf> {
        let home = dirs::home_dir()?;
        
        // Common Obsidian vault locations
        let candidates = vec![
            home.join("Documents").join("Obsidian"),
            home.join("Documents").join("Obsidian-Vault"),
            home.join("Documents").join("ObsidianVault"),
            home.join("Obsidian"),
            home.join("ObsidianVault"),
            home.join("vault"),
            home.join("notes"),
        ];

        for candidate in candidates {
            if candidate.exists() && candidate.join("Projects").exists() {
                return Some(candidate);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.projects_dir, "Projects");
        assert_eq!(settings.templates_dir, "Projects/_TEMPLATES");
        assert!(settings.auto_git_init);
    }

    #[tokio::test]
    async fn test_settings_save_load() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.yaml");
        
        let mut settings = Settings::default();
        settings.auto_git_init = false;
        settings.default_priority = crate::cli::Priority::High;
        
        // Save settings
        let content = serde_yaml::to_string(&settings).unwrap();
        tokio::fs::write(&config_path, content).await.unwrap();
        
        // Load settings
        let content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let loaded_settings: Settings = serde_yaml::from_str(&content).unwrap();
        
        assert_eq!(loaded_settings.auto_git_init, false);
        assert!(matches!(loaded_settings.default_priority, crate::cli::Priority::High));
    }
}
