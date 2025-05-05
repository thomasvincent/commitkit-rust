use anyhow::{Context, Result};
use clap::Parser;
use std::process;

use commitkit::config::Config;
use commitkit::git;
use commitkit::prompt::{Prompter, TerminalPrompter};

/// CLI tool for creating conventional commits
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Skip git commit and print the message to stdout
    #[clap(short, long)]
    dry_run: bool,
    
    /// Show verbose output
    #[clap(short, long)]
    verbose: bool,
    
    /// Path to config file (default: .commitkit.toml in current or home directory)
    #[clap(short, long)]
    config: Option<String>,
    
    /// Toggle emoji in commit messages
    #[clap(long)]
    emoji: bool,
    
    /// Use a specific commit template
    #[clap(long)]
    template: Option<String>,
    
    /// Update changelog after committing
    #[clap(long)]
    changelog: bool,
    
    /// Install git hooks
    #[clap(long)]
    install_hooks: bool,
    
    /// Show commit statistics
    #[clap(long)]
    stats: bool,
    
    /// Number of days to analyze for stats (default: all)
    #[clap(long)]
    days: Option<u32>,
    
    /// Validate a commit message file
    #[clap(long)]
    validate: Option<String>,
    
    /// Prepare commit message (used by git hook)
    #[clap(long)]
    prepare_msg: Option<String>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    
    // Load config (custom path if provided)
    let config = if let Some(config_path) = &args.config {
        let path = std::path::PathBuf::from(config_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("Config file not found: {}", config_path));
        }
        
        let config_str = std::fs::read_to_string(&path)
            .context(format!("Failed to read config file: {}", config_path))?;
            
        toml::from_str(&config_str)
            .context(format!("Failed to parse config from {}", config_path))?
    } else {
        Config::load()?
    };
    
    // Handle special commands that don't require a git repo
    if args.install_hooks {
        return install_git_hooks(&args);
    }
    
    if let Some(validate_file) = &args.validate {
        return validate_commit_message(validate_file, &config);
    }
    
    if let Some(prepare_msg) = &args.prepare_msg {
        return prepare_commit_message(prepare_msg, &config);
    }
    
    if args.stats {
        return show_commit_stats(&args);
    }
    
    // The main commit workflow requires a git repo
    if !git::is_git_repo()? {
        return Err(anyhow::anyhow!("Not in a git repository"));
    }

    // Check if there are staged changes to commit
    if !args.dry_run && !git::has_staged_changes()? {
        return Err(anyhow::anyhow!("No staged changes to commit. Stage your changes with 'git add' first."));
    }

    if args.verbose {
        println!("Loading configuration...");
    }
    
    // Determine if emoji should be used (CLI argument overrides config)
    let use_emoji = if args.emoji { true } else { config.use_emoji };
    
    // Determine if changelog should be updated (CLI argument overrides config)
    let update_changelog = if args.changelog { true } else { config.update_changelog };
    
    if args.verbose {
        println!("Starting interactive commit process...");
    }
    
    // Handle template-based commit if requested
    if let Some(template_name) = &args.template {
        return create_template_commit(template_name, &config, use_emoji, update_changelog, &args);
    }
    
    // Standard interactive commit flow
    let prompter = TerminalPrompter::new();
    
    let prefix = prompter.prompt_prefix(&config.prefixes)?;
    let scope = prompter.prompt_scope(&config.scopes)?;
    let subject = prompter.prompt_subject(config.max_subject_len)?;
    let body = prompter.prompt_body()?;
    let footer = prompter.prompt_footer()?;

    // Build commit message with or without emoji
    let commit_message = if use_emoji {
        commitkit::build_commit_message_with_emoji(&prefix, &scope, &subject, &body, &footer, true)
    } else {
        commitkit::build_commit_message(&prefix, &scope, &subject, &body, &footer)
    };
    
    if args.verbose {
        println!("Commit message generated successfully.");
    }
    
    // Handle dry run
    if args.dry_run {
        println!("--- Commit Message ---");
        println!("{}", commit_message);
        println!("---------------------");
        if args.verbose {
            println!("Dry run mode: No commit was made.");
        }
        return Ok(());
    }
    
    // Execute the commit
    if args.verbose {
        println!("Executing git commit...");
    }
    
    git::run_git_commit(&commit_message, config.sign_off_commits)
        .context("Failed to commit changes")?;
    
    println!("Successfully committed changes!");
    
    // Update changelog if enabled
    if update_changelog {
        if args.verbose {
            println!("Updating changelog...");
        }
        
        let current_dir = std::env::current_dir()?;
        let changelog_path = current_dir.join("CHANGELOG.md");
        
        let changelog = commitkit::changelog::ChangelogManager::new(
            &changelog_path, 
            &current_dir.file_name().unwrap_or_default().to_string_lossy()
        );
        
        changelog.add_entry(&prefix, Some(&scope), &subject, Some(&body))
            .context("Failed to update changelog")?;
            
        if args.verbose {
            println!("Changelog updated successfully.");
        }
    }
    
    Ok(())
}

/// Install git hooks
fn install_git_hooks(args: &Args) -> Result<()> {
    // Find the git repo
    let current_dir = std::env::current_dir()?;
    let repo_root = commitkit::hooks::GitHookManager::find_repo_root(&current_dir)
        .ok_or_else(|| anyhow::anyhow!("Not in a git repository"))?;
    
    let hook_manager = commitkit::hooks::GitHookManager::new(&repo_root);
    
    // Install the hooks
    hook_manager.install_prepare_commit_msg_hook()
        .context("Failed to install prepare-commit-msg hook")?;
        
    hook_manager.install_commit_msg_hook()
        .context("Failed to install commit-msg hook")?;
    
    println!("Git hooks installed successfully!");
    
    if args.verbose {
        println!("Installed hooks:");
        println!("  - prepare-commit-msg: For automatically formatting messages");
        println!("  - commit-msg: For validating commit messages");
    }
    
    Ok(())
}

/// Validate a commit message against conventional commits standard
fn validate_commit_message(file_path: &str, config: &Config) -> Result<()> {
    let validator = commitkit::hooks::CommitMessageValidator::new()
        .min_subject_length(config.min_subject_len)
        .max_subject_length(config.max_subject_len)
        .required_types(config.prefixes.iter().map(|p| p.title.clone()).collect());
    
    match validator.validate_file(file_path) {
        Ok(_) => {
            println!("Commit message is valid.");
            Ok(())
        },
        Err(err) => {
            let error_message = match err {
                commitkit::hooks::ValidationError::InvalidFormat => 
                    "Invalid format. Expected: <type>[(scope)]: <subject>",
                commitkit::hooks::ValidationError::InvalidType => 
                    "Invalid commit type. Use one of the conventional commit types.",
                commitkit::hooks::ValidationError::SubjectTooShort => 
                    "Subject is too short. Make it more descriptive.",
                commitkit::hooks::ValidationError::SubjectTooLong => 
                    "Subject is too long. Keep it under the maximum length.",
                commitkit::hooks::ValidationError::InvalidScope => 
                    "Invalid scope format.",
            };
            
            Err(anyhow::anyhow!(error_message))
        }
    }
}

/// Prepare a commit message for the git hook
fn prepare_commit_message(message: &str, config: &Config) -> Result<()> {
    // If the message already follows the conventional format, don't modify it
    let re = regex::Regex::new(r"^(\w+)(\(([\w-]+)\))?: (.+)").unwrap();
    if re.is_match(message) {
        println!("{}", message);
        return Ok(());
    }
    
    // Otherwise, assume it's a regular message and format it as a conventional commit
    let prefixes = &config.prefixes;
    
    // Default to "chore" if available, otherwise first prefix
    let default_prefix = prefixes.iter()
        .find(|p| p.title == "chore")
        .unwrap_or(&prefixes[0]);
        
    let formatted = format!("{}: {}", default_prefix.title, message.trim());
    
    // Apply emoji if configured
    let final_message = if config.use_emoji {
        commitkit::emoji::apply_emoji(&default_prefix.title, &formatted, true)
    } else {
        formatted
    };
    
    println!("{}", final_message);
    Ok(())
}

/// Show commit statistics
fn show_commit_stats(args: &Args) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let repo_root = commitkit::hooks::GitHookManager::find_repo_root(&current_dir)
        .ok_or_else(|| anyhow::anyhow!("Not in a git repository"))?;
    
    let analyzer = commitkit::stats::CommitAnalyzer::new(repo_root.to_str().unwrap_or("."));
    let summary = analyzer.get_type_summary(args.days)?;
    
    println!("{}", summary);
    Ok(())
}

/// Create a commit using a template
fn create_template_commit(
    template_name: &str, 
    config: &Config,
    use_emoji: bool,
    update_changelog: bool,
    args: &Args
) -> Result<()> {
    // Initialize the template manager
    let template_manager = commitkit::template::TemplateManager::new(&config.templates_dir)?;
    
    // Get the requested template
    let template = template_manager.get_template(template_name)
        .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", template_name))?;
    
    println!("Using template: {} - {}", template.name, template.description);
    
    // Collect values for the template
    let mut values = std::collections::HashMap::new();
    let prompter = TerminalPrompter::new();
    
    // Extract placeholders from templates
    let re = regex::Regex::new(r"\{([^}]+)\}").unwrap();
    
    // From subject template
    for cap in re.captures_iter(&template.subject_template) {
        let placeholder = cap[1].to_string();
        if !values.contains_key(&placeholder) {
            let prompt = format!("Enter value for {}: ", placeholder);
            let value = prompter.prompt_custom(&prompt)?;
            values.insert(placeholder, value);
        }
    }
    
    // From body template if present
    if let Some(body_tpl) = &template.body_template {
        for cap in re.captures_iter(body_tpl) {
            let placeholder = cap[1].to_string();
            if !values.contains_key(&placeholder) {
                let prompt = format!("Enter value for {}: ", placeholder);
                let value = prompter.prompt_custom(&prompt)?;
                values.insert(placeholder, value);
            }
        }
    }
    
    // From footer template if present
    if let Some(footer_tpl) = &template.footer_template {
        for cap in re.captures_iter(footer_tpl) {
            let placeholder = cap[1].to_string();
            if !values.contains_key(&placeholder) {
                let prompt = format!("Enter value for {}: ", placeholder);
                let value = prompter.prompt_custom(&prompt)?;
                values.insert(placeholder, value);
            }
        }
    }
    
    // Fill the templates
    let subject = commitkit::template::fill_template(&template.subject_template, &values);
    let body = template.body_template.as_ref()
        .map(|tpl| commitkit::template::fill_template(tpl, &values))
        .unwrap_or_default();
    let footer = template.footer_template.as_ref()
        .map(|tpl| commitkit::template::fill_template(tpl, &values))
        .unwrap_or_default();
    
    // Determine prefix and scope
    let re_type = regex::Regex::new(r"^(\w+)(?:\(([\w-]+)\))?:").unwrap();
    let (prefix, scope) = if let Some(caps) = re_type.captures(&subject) {
        (
            caps.get(1).map(|m| m.as_str()).unwrap_or("chore").to_string(),
            caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string()
        )
    } else {
        // Default to first prefix type if not found
        (config.prefixes[0].title.clone(), String::new())
    };
    
    // Clean subject of any type/scope prefix
    let clean_subject = re_type.replace(&subject, "").trim().to_string();
    
    // Build commit message with or without emoji
    let commit_message = if use_emoji {
        commitkit::build_commit_message_with_emoji(&prefix, &scope, &clean_subject, &body, &footer, true)
    } else {
        commitkit::build_commit_message(&prefix, &scope, &clean_subject, &body, &footer)
    };
    
    // Handle dry run
    if args.dry_run {
        println!("--- Commit Message from Template ---");
        println!("{}", commit_message);
        println!("-----------------------------------");
        if args.verbose {
            println!("Dry run mode: No commit was made.");
        }
        return Ok(());
    }
    
    // Execute the commit
    if args.verbose {
        println!("Executing git commit...");
    }
    
    git::run_git_commit(&commit_message, config.sign_off_commits)
        .context("Failed to commit changes")?;
    
    println!("Successfully committed changes using template!");
    
    // Update changelog if enabled
    if update_changelog {
        if args.verbose {
            println!("Updating changelog...");
        }
        
        let current_dir = std::env::current_dir()?;
        let changelog_path = current_dir.join("CHANGELOG.md");
        
        let changelog = commitkit::changelog::ChangelogManager::new(
            &changelog_path, 
            &current_dir.file_name().unwrap_or_default().to_string_lossy()
        );
        
        changelog.add_entry(&prefix, Some(&scope), &clean_subject, Some(&body))
            .context("Failed to update changelog")?;
            
        if args.verbose {
            println!("Changelog updated successfully.");
        }
    }
    
    Ok(())
}