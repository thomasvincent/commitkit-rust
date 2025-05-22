pub mod changelog;
pub mod config;
pub mod emoji;
pub mod error;
pub mod git;
pub mod hooks;
pub mod prompt;
pub mod stats;
pub mod template;
pub mod version;

#[cfg(test)]
pub mod tests;

/// Builds a commit message from its components
pub fn build_commit_message(
    prefix: &str,
    scope: &str,
    subject: &str,
    body: &str,
    footer: &str,
) -> String {
    let mut message = String::new();

    // Header (type(scope): subject)
    message.push_str(prefix);
    if !scope.is_empty() {
        message.push('(');
        message.push_str(scope);
        message.push(')');
    }
    message.push_str(": ");
    message.push_str(subject);

    // Body with proper separation
    if !body.is_empty() {
        message.push_str("\n\n");
        message.push_str(body);
    }

    // Footer with proper separation
    if !footer.is_empty() {
        if !body.is_empty() && !body.ends_with('\n') {
            message.push('\n');
        }
        message.push_str("\n");
        message.push_str(footer);
    }

    message
}

/// Formats a commit message with optional emoji
pub fn build_commit_message_with_emoji(
    prefix: &str,
    scope: &str,
    subject: &str,
    body: &str,
    footer: &str,
    use_emoji: bool,
) -> String {
    let message = build_commit_message(prefix, scope, subject, body, footer);
    if use_emoji {
        emoji::apply_emoji(prefix, &message, true)
    } else {
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_commit_message_simple() {
        let message = build_commit_message("feat", "", "add new feature", "", "");
        assert_eq!(message, "feat: add new feature");
    }

    #[test]
    fn test_build_commit_message_with_scope() {
        let message = build_commit_message("fix", "core", "resolve crash on startup", "", "");
        assert_eq!(message, "fix(core): resolve crash on startup");
    }

    #[test]
    fn test_build_commit_message_with_body() {
        let message = build_commit_message(
            "feat",
            "ui",
            "add dark mode",
            "This adds support for dark mode\nAnd improves accessibility",
            "",
        );
        assert_eq!(
            message,
            "feat(ui): add dark mode\n\nThis adds support for dark mode\nAnd improves accessibility"
        );
    }

    #[test]
    fn test_build_commit_message_full() {
        let message = build_commit_message(
            "fix",
            "api",
            "correct error handling",
            "Previously errors were being swallowed",
            "Fixes #123",
        );
        assert_eq!(
            message,
            "fix(api): correct error handling\n\nPreviously errors were being swallowed\n\nFixes #123"
        );
    }
}
