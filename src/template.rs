use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, serde::Serialize)]
pub struct CommitTemplate {
    pub name: String,
    pub description: String,
    pub subject_template: String,
    pub body_template: Option<String>,
    pub footer_template: Option<String>,
}

#[derive(Debug)]
pub struct TemplateManager {
    templates: HashMap<String, CommitTemplate>,
    template_dir: PathBuf,
}

impl TemplateManager {
    /// Create a new template manager
    pub fn new<P: AsRef<Path>>(template_dir: P) -> Result<Self> {
        let dir_path = PathBuf::from(template_dir.as_ref());
        
        // Create template directory if it doesn't exist
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)
                .context("Failed to create template directory")?;
        }
        
        let mut manager = Self {
            templates: HashMap::new(),
            template_dir: dir_path,
        };
        
        manager.load_templates()?;
        
        Ok(manager)
    }
    
    /// Load templates from the template directory
    pub fn load_templates(&mut self) -> Result<()> {
        self.templates.clear();
        
        // Skip if the directory doesn't exist
        if !self.template_dir.exists() {
            return Ok(());
        }
        
        // Load all .toml files from the template directory
        for entry in fs::read_dir(&self.template_dir)
            .context("Failed to read template directory")? {
                
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            // Only process .toml files
            if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
                self.load_template_file(&path)?;
            }
        }
        
        // Add default templates if none exist
        if self.templates.is_empty() {
            self.create_default_templates()?;
        }
        
        Ok(())
    }
    
    /// Load a single template file
    fn load_template_file(&mut self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read template file: {:?}", path))?;
            
        let template: CommitTemplate = toml::from_str(&content)
            .context(format!("Failed to parse template file: {:?}", path))?;
            
        self.templates.insert(template.name.clone(), template);
        
        Ok(())
    }
    
    /// Create default templates
    fn create_default_templates(&mut self) -> Result<()> {
        let templates = [
            CommitTemplate {
                name: "feature".to_string(),
                description: "Template for new features".to_string(),
                subject_template: "add {feature_name}".to_string(),
                body_template: Some("This change adds the ability to {description}\n\nThe following functionality is now available:\n- {point_1}\n- {point_2}".to_string()),
                footer_template: Some("Closes #{issue_number}".to_string()),
            },
            CommitTemplate {
                name: "bugfix".to_string(),
                description: "Template for bug fixes".to_string(),
                subject_template: "fix {issue_description}".to_string(),
                body_template: Some("This fixes an issue where {problem_description}\n\nRoot cause: {root_cause}".to_string()),
                footer_template: Some("Fixes #{issue_number}".to_string()),
            },
            CommitTemplate {
                name: "refactor".to_string(),
                description: "Template for code refactoring".to_string(),
                subject_template: "refactor {component_name}".to_string(),
                body_template: Some("This refactors {component_name} to improve {goal}\n\nChanges:\n- {change_1}\n- {change_2}".to_string()),
                footer_template: None,
            },
        ];
        
        for template in templates.iter() {
            let file_path = self.template_dir.join(format!("{}.toml", template.name));
            let content = toml::to_string_pretty(template)
                .context("Failed to serialize template")?;
                
            fs::write(&file_path, content)
                .context(format!("Failed to write template file: {:?}", file_path))?;
                
            self.templates.insert(template.name.clone(), template.clone());
        }
        
        Ok(())
    }
    
    /// Get a template by name
    pub fn get_template(&self, name: &str) -> Option<&CommitTemplate> {
        self.templates.get(name)
    }
    
    /// Get all available templates
    pub fn get_templates(&self) -> Vec<&CommitTemplate> {
        self.templates.values().collect()
    }
    
    /// Create a new template
    pub fn add_template(&mut self, template: CommitTemplate) -> Result<()> {
        let file_path = self.template_dir.join(format!("{}.toml", template.name));
        let content = toml::to_string_pretty(&template)
            .context("Failed to serialize template")?;
            
        fs::write(&file_path, content)
            .context(format!("Failed to write template file: {:?}", file_path))?;
            
        self.templates.insert(template.name.clone(), template);
        
        Ok(())
    }
    
    /// Delete a template
    pub fn delete_template(&mut self, name: &str) -> Result<()> {
        if let Some(_) = self.templates.remove(name) {
            let file_path = self.template_dir.join(format!("{}.toml", name));
            
            if file_path.exists() {
                fs::remove_file(&file_path)
                    .context(format!("Failed to delete template file: {:?}", file_path))?;
            }
        }
        
        Ok(())
    }
}

/// Fill a template with provided values
pub fn fill_template(template: &str, values: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    
    for (key, value) in values {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_fill_template() {
        let mut values = HashMap::new();
        values.insert("name".to_string(), "John".to_string());
        values.insert("age".to_string(), "30".to_string());
        
        let template = "Hello, {name}! You are {age} years old.";
        let filled = fill_template(template, &values);
        
        assert_eq!(filled, "Hello, John! You are 30 years old.");
    }
    
    #[test]
    fn test_template_manager() {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager::new(temp_dir.path()).unwrap();
        
        // Default templates should be created
        assert!(manager.get_template("feature").is_some());
        assert!(manager.get_template("bugfix").is_some());
        assert!(manager.get_template("refactor").is_some());
    }
}