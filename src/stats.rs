use anyhow::{Context, Result};
use std::collections::HashMap;
use std::process::Command;

/// Statistics for commit types
#[derive(Debug, Default)]
pub struct CommitStats {
    pub total_commits: usize,
    pub type_counts: HashMap<String, usize>,
    pub scope_counts: HashMap<String, usize>,
    pub contributors: HashMap<String, usize>,
    pub commits_by_date: HashMap<String, usize>,
}

pub struct CommitAnalyzer {
    repo_path: String,
}

impl CommitAnalyzer {
    /// Create a new commit analyzer for the given repository
    pub fn new(repo_path: &str) -> Self {
        Self {
            repo_path: repo_path.to_string(),
        }
    }

    /// Analyze commit history and generate statistics
    pub fn analyze_commits(&self, days: Option<u32>) -> Result<CommitStats> {
        let mut stats = CommitStats::default();

        // Build git log command with appropriate format
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.repo_path)
            .arg("log")
            .arg("--pretty=format:%h|%an|%ae|%ad|%s")
            .arg("--date=short");

        // Add date filter if specified
        if let Some(days) = days {
            cmd.arg(format!("--since={} days ago", days));
        }

        let output = cmd.output().context("Failed to run git log")?;
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Process each commit
        for line in output_str.lines() {
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 5 {
                continue;
            }

            let _hash = parts[0];
            let author = parts[1];
            let date = parts[3];
            let subject = parts[4];

            // Extract commit type and scope using regex
            let re = regex::Regex::new(r"^(\w+)(?:\(([\w-]+)\))?: .+$").unwrap();
            if let Some(captures) = re.captures(subject) {
                let commit_type = captures.get(1).map_or("", |m| m.as_str()).to_string();

                // Increment type count
                *stats.type_counts.entry(commit_type).or_insert(0) += 1;

                // Extract and count scope if present
                if let Some(scope_match) = captures.get(2) {
                    let scope = scope_match.as_str().to_string();
                    *stats.scope_counts.entry(scope).or_insert(0) += 1;
                }
            }

            // Count by author
            *stats.contributors.entry(author.to_string()).or_insert(0) += 1;

            // Count by date
            *stats.commits_by_date.entry(date.to_string()).or_insert(0) += 1;

            stats.total_commits += 1;
        }

        Ok(stats)
    }

    /// Get commit count by type as a formatted string
    pub fn get_type_summary(&self, days: Option<u32>) -> Result<String> {
        let stats = self.analyze_commits(days)?;

        let mut result = format!(
            "Commit statistics for the past {} days:\n\n",
            days.map_or("all".to_string(), |d| d.to_string())
        );

        result.push_str(&format!("Total commits: {}\n\n", stats.total_commits));

        // Type breakdown
        result.push_str("Commit types:\n");
        let mut type_counts: Vec<(String, usize)> = stats.type_counts.into_iter().collect();
        type_counts.sort_by(|a, b| b.1.cmp(&a.1));

        for (commit_type, count) in type_counts {
            let percentage = (count as f64 / stats.total_commits as f64) * 100.0;
            result.push_str(&format!(
                "  {}: {} ({:.1}%)\n",
                commit_type, count, percentage
            ));
        }

        // Top scopes
        if !stats.scope_counts.is_empty() {
            result.push_str("\nTop scopes:\n");
            let mut scope_counts: Vec<(String, usize)> = stats.scope_counts.into_iter().collect();
            scope_counts.sort_by(|a, b| b.1.cmp(&a.1));

            for (scope, count) in scope_counts.iter().take(5) {
                result.push_str(&format!("  {}: {}\n", scope, count));
            }
        }

        // Top contributors
        if !stats.contributors.is_empty() {
            result.push_str("\nTop contributors:\n");
            let mut contributor_counts: Vec<(String, usize)> =
                stats.contributors.into_iter().collect();
            contributor_counts.sort_by(|a, b| b.1.cmp(&a.1));

            for (contributor, count) in contributor_counts.iter().take(5) {
                let percentage = (*count as f64 / stats.total_commits as f64) * 100.0;
                result.push_str(&format!(
                    "  {}: {} ({:.1}%)\n",
                    contributor, count, percentage
                ));
            }
        }

        Ok(result)
    }
}
