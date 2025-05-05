use anyhow::{Context, Result};
use console::style;
use dialoguer::{Input, Select, theme::ColorfulTheme};

use crate::config::Prefix;

/// Trait defining the interaction methods for commit message creation
pub trait Prompter {
    fn prompt_prefix(&self, prefixes: &[Prefix]) -> Result<String>;
    fn prompt_scope(&self, scopes: &[String]) -> Result<String>;
    fn prompt_subject(&self, max_length: usize) -> Result<String>;
    fn prompt_body(&self) -> Result<String>;
    fn prompt_footer(&self) -> Result<String>;
}

/// Terminal-based implementation of the Prompter trait using dialoguer
pub struct TerminalPrompter {
    theme: ColorfulTheme,
}

impl TerminalPrompter {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }
}

impl Default for TerminalPrompter {
    fn default() -> Self {
        Self::new()
    }
}

impl Prompter for TerminalPrompter {
    fn prompt_prefix(&self, prefixes: &[Prefix]) -> Result<String> {
        let prefix_titles: Vec<String> = prefixes
            .iter()
            .map(|p| format!("{}: {}", p.title, p.description))
            .collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select commit type")
            .items(&prefix_titles)
            .default(0)
            .interact()
            .context("Failed to get prefix selection")?;

        Ok(prefixes[selection].title.clone())
    }

    fn prompt_scope(&self, scopes: &[String]) -> Result<String> {
        if scopes.is_empty() {
            return Ok(String::new());
        }

        // Add "None" as the first option
        let mut display_scopes = vec!["None".to_string()];
        display_scopes.extend(scopes.iter().cloned());

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select scope (optional)")
            .items(&display_scopes)
            .default(0)
            .interact()
            .context("Failed to get scope selection")?;

        // Return empty string if "None" was selected
        if selection == 0 {
            Ok(String::new())
        } else {
            Ok(scopes[selection - 1].clone())
        }
    }

    fn prompt_subject(&self, max_length: usize) -> Result<String> {
        let prompt = format!("Enter commit subject (max {} characters)", max_length);
        
        let subject = Input::with_theme(&self.theme)
            .with_prompt(&prompt)
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err("Subject cannot be empty")
                } else if input.len() > max_length {
                    Err("Subject is too long")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .context("Failed to get commit subject")?;

        Ok(subject)
    }

    fn prompt_body(&self) -> Result<String> {
        println!("{}", style("Enter commit body (leave empty to skip):").bold());
        println!("{}", style("Press Ctrl+D (Unix) or Ctrl+Z (Windows) followed by Enter when done.").dim());
        
        let mut body = String::new();
        let stdin = std::io::stdin();
        
        while let Ok(n) = stdin.read_line(&mut body) {
            if n == 0 || body.trim().is_empty() {
                break;
            }
        }
        
        Ok(body.trim().to_string())
    }

    fn prompt_footer(&self) -> Result<String> {
        let footer = Input::<String>::with_theme(&self.theme)
            .with_prompt("Enter commit footer (optional)")
            .allow_empty(true)
            .interact_text()
            .context("Failed to get commit footer")?;

        Ok(footer)
    }
}