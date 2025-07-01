use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;
use regex::Regex;
use walkdir::WalkDir;

use crate::core::Project;

pub struct TemplateProcessor {
    variables: HashMap<String, String>,
}

impl TemplateProcessor {
    pub fn new(project: &Project) -> Self {
        let mut variables = HashMap::new();
        
        // Basic project variables
        variables.insert("project_name".to_string(), project.name.clone());
        variables.insert("プロジェクト名".to_string(), project.name.clone());
        variables.insert("project_type".to_string(), project.project_type.to_string());
        variables.insert("priority".to_string(), project.priority.to_string());
        variables.insert("優先度".to_string(), project.priority.to_string());
        variables.insert("status".to_string(), project.status.to_string());
        variables.insert("project_id".to_string(), project.id.clone());
        
        // Date variables
        let now = chrono::Utc::now();
        variables.insert("date".to_string(), now.format("%Y-%m-%d").to_string());
        variables.insert("datetime".to_string(), now.format("%Y-%m-%d %H:%M:%S UTC").to_string());
        variables.insert("year".to_string(), now.format("%Y").to_string());
        variables.insert("month".to_string(), now.format("%m").to_string());
        variables.insert("day".to_string(), now.format("%d").to_string());
        
        // Description and other optional fields
        if let Some(desc) = &project.description {
            variables.insert("description".to_string(), desc.clone());
            variables.insert("説明".to_string(), desc.clone());
        }

        // Technology stack placeholders
        variables.insert("技術スタック".to_string(), "{{技術スタック}}".to_string());
        variables.insert("期間".to_string(), "{{期間}}".to_string());
        variables.insert("目標文字数".to_string(), "{{目標文字数}}".to_string());
        variables.insert("公開予定日".to_string(), "{{公開予定日}}".to_string());

        Self { variables }
    }

    pub fn add_variable<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.variables.insert(key.into(), value.into());
    }

    pub async fn process_directory<P: AsRef<Path>>(&self, dir_path: P) -> Result<()> {
        let dir_path = dir_path.as_ref();
        
        for entry in WalkDir::new(dir_path) {
            let entry = entry?;
            let path = entry.path();
            
            if entry.file_type().is_file() {
                // Skip binary files and hidden files
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.starts_with('.') {
                        continue;
                    }
                }

                // Process text files
                if self.is_text_file(path) {
                    self.process_file(path).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn process_file<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        let file_path = file_path.as_ref();
        
        let content = tokio::fs::read_to_string(file_path).await?;
        let processed_content = self.process_string(&content);
        
        // Only write if content changed
        if content != processed_content {
            tokio::fs::write(file_path, processed_content).await?;
        }

        Ok(())
    }

    pub fn process_string(&self, content: &str) -> String {
        let mut result = content.to_string();
        
        // Replace template variables using {{variable}} syntax
        let re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        
        result = re.replace_all(&result, |caps: &regex::Captures| {
            let variable_name = caps.get(1).unwrap().as_str().trim();
            
            // Handle special date formatting
            if variable_name.starts_with("date:") {
                let format = &variable_name[5..];
                return chrono::Utc::now().format(format).to_string();
            }
            
            // Look up variable in our map
            self.variables.get(variable_name)
                .cloned()
                .unwrap_or_else(|| format!("{{{{{}}}}}", variable_name)) // Keep original if not found
        }).to_string();

        result
    }

    fn is_text_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            matches!(
                extension.to_lowercase().as_str(),
                "md" | "txt" | "yaml" | "yml" | "json" | "toml" | "rs" | "js" | "ts" | "py" | "go" | "java" | "c" | "cpp" | "h" | "hpp" | "css" | "html" | "xml" | "svg"
            )
        } else {
            // Check if file has no extension but might be text
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                matches!(
                    file_name.to_lowercase().as_str(),
                    "readme" | "license" | "dockerfile" | "makefile" | "gitignore"
                )
            } else {
                false
            }
        }
    }

    pub fn create_filename_variables(project_name: &str, project_type: &crate::core::ProjectType) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let type_str = match project_type {
            crate::core::ProjectType::WebApp => "Web-App",
            crate::core::ProjectType::Tool => "Tool",
            crate::core::ProjectType::Content => "Content",
            crate::core::ProjectType::Api => "API",
            crate::core::ProjectType::Custom(name) => name,
        };
        
        let safe_project_name = project_name.replace(" ", "-");
        let directory_name = format!("{}_{}_{}",today, type_str, safe_project_name);
        
        variables.insert("directory_name".to_string(), directory_name);
        variables.insert("safe_project_name".to_string(), safe_project_name);
        variables.insert("project_type_short".to_string(), type_str.to_string());
        
        variables
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_process_string() {
        let project = crate::core::Project::new(
            "Test Project".to_string(),
            crate::cli::ProjectType::WebApp,
            crate::cli::Priority::High,
            Some("A test project".to_string()),
        );
        
        let processor = TemplateProcessor::new(&project);
        
        let input = "Project: {{project_name}}\nType: {{project_type}}\nPriority: {{priority}}";
        let output = processor.process_string(input);
        
        assert!(output.contains("Project: Test Project"));
        assert!(output.contains("Type: Web-App"));
        assert!(output.contains("Priority: High"));
    }

    #[test]
    fn test_date_formatting() {
        let project = crate::core::Project::new(
            "Test".to_string(),
            crate::cli::ProjectType::Tool,
            crate::cli::Priority::Medium,
            None,
        );
        
        let processor = TemplateProcessor::new(&project);
        
        let input = "Date: {{date:YYYY-MM-DD}}";
        let output = processor.process_string(input);
        
        // Should contain current date in YYYY-MM-DD format
        let current_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        assert!(output.contains(&current_date));
    }

    #[tokio::test]
    async fn test_process_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.md");
        
        let content = "# {{project_name}}\n\nDescription: {{description}}";
        tokio::fs::write(&file_path, content).await.unwrap();
        
        let project = crate::core::Project::new(
            "My Project".to_string(),
            crate::cli::ProjectType::Content,
            crate::cli::Priority::Low,
            Some("Test description".to_string()),
        );
        
        let processor = TemplateProcessor::new(&project);
        processor.process_file(&file_path).await.unwrap();
        
        let result = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert!(result.contains("# My Project"));
        assert!(result.contains("Description: Test description"));
    }
}
