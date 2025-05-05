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
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Check if we're in a git repository
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
    
    if args.verbose {
        println!("Starting interactive commit process...");
    }
    
    let prompter = TerminalPrompter::new();
    
    let prefix = prompter.prompt_prefix(&config.prefixes)?;
    let scope = prompter.prompt_scope(&config.scopes)?;
    let subject = prompter.prompt_subject(config.max_subject_len)?;
    let body = prompter.prompt_body()?;
    let footer = prompter.prompt_footer()?;

    let commit_message = commitkit::build_commit_message(&prefix, &scope, &subject, &body, &footer);
    
    if args.verbose {
        println!("Commit message generated successfully.");
    }
    
    if args.dry_run {
        println!("--- Commit Message ---");
        println!("{}", commit_message);
        println!("---------------------");
        if args.verbose {
            println!("Dry run mode: No commit was made.");
        }
    } else {
        if args.verbose {
            println!("Executing git commit...");
        }
        git::run_git_commit(&commit_message, config.sign_off_commits)
            .context("Failed to commit changes")?;
        println!("Successfully committed changes!");
    }
    
    Ok(())
}