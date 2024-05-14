use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    sign_off_commits: bool,
    prefixes: Vec<Prefix>,
    scopes: Vec<String>,
    max_subject_len: usize,
}

#[derive(Deserialize)]
struct Prefix {
    title: String,
    description: String,
}

fn main() {
    let config = load_config();

    let prefix = prompt_prefix(&config.prefixes);
    let scope = prompt_scope(&config.scopes);
    let subject = prompt_subject(config.max_subject_len);
    let body = prompt_body();
    let footer = prompt_footer();

    let commit_message = build_commit_message(&prefix, &scope, &subject, &body, &footer);
    run_git_commit(&commit_message, config.sign_off_commits);
}

fn load_config() -> Config {
    let config_path = find_config_file();

    let config_str = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(_) => String::from(
            r#"
            sign_off_commits = false
            prefixes = [
                { title = "feat", description = "a new feature" },
                { title = "fix", description = "a bug fix" },
                { title = "docs", description = "documentation changes" }
            ]
            scopes = []
            max_subject_len = 50
            "#,
        ),
    };

    toml::from_str(&config_str).expect("Failed to parse config")
}

fn find_config_file() -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let home_dir = env::var("HOME").expect("Failed to get home directory");

    let config_paths = [
        current_dir.join(".commitkit.toml"),
        PathBuf::from(home_dir).join(".commitkit.toml"),
    ];

    config_paths
        .iter()
        .find(|path| path.exists())
        .unwrap_or(&config_paths[0])
        .to_path_buf()
}

fn prompt_prefix(prefixes: &[Prefix]) -> String {
    println!("Select a prefix:");
    for (i, prefix) in prefixes.iter().enumerate() {
        println!("{}. {} ({})", i + 1, prefix.title, prefix.description);
    }

    print!("Enter the number of your choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let selected_index = input.trim().parse::<usize>().unwrap_or(0);

    if selected_index < 1 || selected_index > prefixes.len() {
        println!("Invalid selection. Using default prefix.");
        prefixes[0].title.clone()
    } else {
        prefixes[selected_index - 1].title.clone()
    }
}

fn prompt_scope(scopes: &[String]) -> String {
    if scopes.is_empty() {
        return String::new();
    }

    println!("Select a scope:");
    for (i, scope) in scopes.iter().enumerate() {
        println!("{}. {}", i + 1, scope);
    }

    print!("Enter the number of your choice (or 0 to skip): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let selected_index = input.trim().parse::<usize>().unwrap_or(0);

    if selected_index == 0 {
        String::new()
    } else if selected_index < 1 || selected_index > scopes.len() {
        println!("Invalid selection. Skipping scope.");
        String::new()
    } else {
        scopes[selected_index - 1].clone()
    }
}

fn prompt_subject(max_length: usize) -> String {
    loop {
        print!("Enter the commit subject (max {} characters): ", max_length);
        io::stdout().flush().unwrap();

        let mut subject = String::new();
        io::stdin().read_line(&mut subject).unwrap();
        let subject = subject.trim();

        if subject.len() <= max_length {
            return subject.to_string();
        }
        println!(
            "Subject exceeds maximum length of {} characters. Please try again.",
            max_length
        );
    }
}

fn prompt_body() -> String {
    println!("Enter the commit body (press Enter twice to finish):");

    let mut body = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }
        body.push_str(&line);
    }
    body.trim().to_string()
}

fn prompt_footer() -> String {
    print!("Enter the commit footer (optional): ");
    io::stdout().flush().unwrap();

    let mut footer = String::new();
    io::stdin().read_line(&mut footer).unwrap();
    footer.trim().to_string()
}

fn build_commit_message(prefix: &str, scope: &str, subject: &str, body: &str, footer: &str) -> String {
    let mut message = String::new();
    message.push_str(prefix);
    if !scope.is_empty() {
        message.push('(');
        message.push_str(scope);
        message.push(')');
    }
    message.push_str(": ");
    message.push_str(subject);
    message.push_str("\n\n");
    if !body.is_empty() {
        message.push_str(body);
        message.push('\n');
    }
    if !footer.is_empty() {
        message.push_str(footer);
        message.push('\n');
    }
    message
}

fn run_git_commit(message: &str, sign_off: bool) {
    let mut command = Command::new("git");
    command.arg("commit").arg("-m").arg(message);
    if sign_off {
        command.arg("-s");
    }

    let output = command.output().expect("Failed to execute git commit");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
