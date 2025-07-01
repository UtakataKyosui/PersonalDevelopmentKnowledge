// Placeholder for template management
// This will be expanded in future versions

pub struct Template {
    pub name: String,
    pub path: std::path::PathBuf,
    pub description: Option<String>,
}

impl Template {
    pub fn new(name: String, path: std::path::PathBuf) -> Self {
        Self {
            name,
            path,
            description: None,
        }
    }
}
