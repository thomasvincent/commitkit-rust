use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Represents a changelog manager for generating and updating CHANGELOG.md files
pub struct ChangelogManager {
    file_path: PathBuf,
    project_name: String,
    version: Option<String>,
}

impl ChangelogManager {
    /// Creates a new changelog manager
    pub fn new<P: AsRef<Path>>(path: P, project_name: &str) -> Self {
        Self {
            file_path: PathBuf::from(path.as_ref()),
            project_name: project_name.to_string(),
            version: None,
        }
    }

    /// Sets the current version for changelog entries
    pub fn with_version(mut self, version: &str) -> Self {
        self.version = Some(version.to_string());
        self
    }

    /// Adds a new entry to the changelog file
    pub fn add_entry(
        &self,
        commit_type: &str,
        scope: Option<&str>,
        subject: &str,
        body: Option<&str>,
    ) -> Result<()> {
        // Create file if it doesn't exist
        if !self.file_path.exists() {
            self.create_new_changelog()?;
        }

        // Read existing content
        let content =
            fs::read_to_string(&self.file_path).context("Failed to read changelog file")?;

        // Find the insertion point (after the first section header)
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut insert_index = 0;

        // Find the first version header or create one
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("## ") {
                insert_index = i;
                break;
            }
        }

        // Prepare the entry
        let entry = self.format_entry(commit_type, scope, subject, body);

        // Insert the entry
        if insert_index > 0 {
            lines.insert(insert_index + 1, String::new());
            lines.insert(insert_index + 2, entry);
        } else {
            // No version headers found, add one with the current version
            let version = self.version.as_deref().unwrap_or("Unreleased");
            lines.push(format!(
                "## {} ({})",
                version,
                chrono::Local::now().format("%Y-%m-%d")
            ));
            lines.push(String::new());
            lines.push(entry);
        }

        // Write back to file
        fs::write(&self.file_path, lines.join("\n")).context("Failed to write changelog file")?;

        Ok(())
    }

    /// Creates a new changelog file with standard header
    fn create_new_changelog(&self) -> Result<()> {
        let header = format!("# Changelog\n\nAll notable changes to {} will be documented in this file.\n\n\
        The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\n\
        and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n",
            self.project_name
        );

        let mut file = File::create(&self.file_path).context("Failed to create changelog file")?;

        file.write_all(header.as_bytes())
            .context("Failed to write changelog header")?;

        Ok(())
    }

    /// Formats a changelog entry based on conventional commit format
    fn format_entry(
        &self,
        commit_type: &str,
        scope: Option<&str>,
        subject: &str,
        body: Option<&str>,
    ) -> String {
        let display_type = match commit_type {
            "feat" => "Added",
            "fix" => "Fixed",
            "perf" => "Performance",
            "refactor" => "Changed",
            "docs" => "Documentation",
            "test" => "Tests",
            "build" => "Build",
            "ci" => "CI",
            "chore" => "Maintenance",
            "style" => "Style",
            "revert" => "Reverted",
            _ => commit_type,
        };

        let mut entry = format!("- **{}**", display_type);

        if let Some(scope_value) = scope {
            if !scope_value.is_empty() {
                entry.push_str(&format!(" ({})", scope_value));
            }
        }

        entry.push_str(&format!(": {}", subject));

        if let Some(body_text) = body {
            if !body_text.is_empty() {
                // Add the body as sub-points, indented
                for line in body_text.lines() {
                    if !line.trim().is_empty() {
                        entry.push_str(&format!("\n  - {}", line.trim()));
                    }
                }
            }
        }

        entry
    }

    /// Update version header in changelog
    pub fn update_version(&self, new_version: &str) -> Result<()> {
        if !self.file_path.exists() {
            return Err(anyhow::anyhow!("Changelog file does not exist"));
        }

        let file = File::open(&self.file_path).context("Failed to open changelog file")?;

        let reader = BufReader::new(file);
        let mut lines: Vec<String> = reader
            .lines()
            .collect::<std::io::Result<_>>()
            .context("Failed to read changelog lines")?;

        // Look for "Unreleased" section and update it
        for i in 0..lines.len() {
            if lines[i].starts_with("## Unreleased") {
                lines[i] = format!(
                    "## {} ({})",
                    new_version,
                    chrono::Local::now().format("%Y-%m-%d")
                );

                // Add a new Unreleased section above
                lines.insert(i, "## Unreleased".to_string());
                lines.insert(i + 1, String::new());
                break;
            }
        }

        // Write back to file
        fs::write(&self.file_path, lines.join("\n"))
            .context("Failed to write updated changelog")?;

        Ok(())
    }
}
