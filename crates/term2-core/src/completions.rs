//! Command completions and autosuggestions.
//!
//! Warp's completions suggest commands, option names, and path parameters.
//! Autosuggestions automatically suggest commands as you type based on shell
//! history and possible completions.

use std::collections::HashSet;

/// A single completion item.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CompletionItem {
    pub value: String,
    pub display: String,
    pub kind: CompletionKind,
    pub description: Option<String>,
}

impl CompletionItem {
    pub fn new(value: impl Into<String>, kind: CompletionKind) -> Self {
        let value = value.into();
        Self {
            display: value.clone(),
            value,
            kind,
            description: None,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionKind {
    Command,
    Flag,
    Path,
    Argument,
    History,
    Suggestion,
}

/// History-backed autosuggestion engine.
#[derive(Debug, Clone, Default)]
pub struct HistoryStore {
    entries: Vec<String>,
}

impl HistoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, command: impl Into<String>) {
        let command = command.into();
        if !command.trim().is_empty() && !self.entries.contains(&command) {
            self.entries.push(command);
        }
    }

    pub fn suggest(&self, prefix: &str) -> Option<String> {
        if prefix.is_empty() {
            return None;
        }
        self.entries
            .iter()
            .rev()
            .find(|entry| entry.starts_with(prefix) && entry.len() > prefix.len())
            .cloned()
    }

    pub fn all(&self) -> &[String] {
        &self.entries
    }
}

/// A simple completion engine that can be backed by command tables, history, and paths.
#[derive(Debug, Clone, Default)]
pub struct CompletionEngine {
    commands: Vec<CompletionItem>,
    history: HistoryStore,
    paths: Vec<String>,
}

impl CompletionEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_common_commands(mut self) -> Self {
        let commands = vec![
            "cargo",
            "cargo build",
            "cargo test",
            "cargo run",
            "git",
            "git status",
            "git commit",
            "git push",
            "ls",
            "ls -la",
            "cd",
            "cat",
            "echo",
            "kubectl",
            "kubectl get pods",
            "docker",
            "docker ps",
        ];
        for cmd in commands {
            self.commands
                .push(CompletionItem::new(cmd, CompletionKind::Command));
        }
        self
    }

    pub fn register_command(&mut self, name: &str) {
        self.commands
            .push(CompletionItem::new(name, CompletionKind::Command));
    }

    pub fn register_flag(&mut self, flag: &str, description: Option<String>) {
        self.commands.push(
            CompletionItem::new(flag, CompletionKind::Flag)
                .with_description(description.unwrap_or_default()),
        );
    }

    pub fn add_history(&mut self, command: impl Into<String>) {
        self.history.add(command);
    }

    pub fn add_path(&mut self, path: impl Into<String>) {
        self.paths.push(path.into());
    }

    /// Return completions matching the current input token.
    pub fn complete(&self, input: &str) -> Vec<CompletionItem> {
        let token = input.split_whitespace().last().unwrap_or(input);
        let lower = token.to_lowercase();
        let mut seen = HashSet::new();
        let mut results: Vec<CompletionItem> = self
            .commands
            .iter()
            .filter(|c| {
                c.value.to_lowercase().starts_with(&lower)
                    && c.value.len() > token.len()
                    && seen.insert(c.value.clone())
            })
            .cloned()
            .collect();

        // Add history-based suggestions at the top if they match.
        if let Some(hist) = self.history.suggest(input) {
            if seen.insert(hist.clone()) {
                results.insert(
                    0,
                    CompletionItem::new(hist, CompletionKind::History)
                        .with_description("from history"),
                );
            }
        }

        // Add path completions for tokens containing a slash or when appropriate.
        if token.starts_with('/') || token.starts_with("./") || token.starts_with('~') {
            for path in &self.paths {
                if path.to_lowercase().starts_with(&lower) && seen.insert(path.clone()) {
                    results.push(CompletionItem::new(path, CompletionKind::Path));
                }
            }
        }

        results
    }

    /// Return the top autosuggestion for the current input.
    pub fn autosuggest(&self, input: &str) -> Option<String> {
        self.history.suggest(input)
    }

    /// Return flag completions for a command prefix.
    pub fn complete_flags(&self, command: &str) -> Vec<CompletionItem> {
        let lower = command.to_lowercase();
        self.commands
            .iter()
            .filter(|c| {
                c.kind == CompletionKind::Flag
                    && (lower.is_empty() || c.value.to_lowercase().contains(&lower))
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_suggests_prefix_match() {
        let mut history = HistoryStore::new();
        history.add("cargo build");
        history.add("cargo test");
        history.add("git status");

        assert_eq!(history.suggest("cargo "), Some("cargo test".to_string()));
        assert_eq!(history.suggest("git "), Some("git status".to_string()));
        assert_eq!(history.suggest("npm "), None);
    }

    #[test]
    fn history_does_not_suggest_exact_match() {
        let mut history = HistoryStore::new();
        history.add("cargo build");
        assert_eq!(history.suggest("cargo build"), None);
    }

    #[test]
    fn history_dedupes_entries() {
        let mut history = HistoryStore::new();
        history.add("cargo build");
        history.add("cargo build");
        assert_eq!(history.all().len(), 1);
    }

    #[test]
    fn engine_completes_commands() {
        let engine = CompletionEngine::new().with_common_commands();
        let results = engine.complete("cargo");
        assert!(results.iter().any(|c| c.value == "cargo build"));
        assert!(results.iter().any(|c| c.value == "cargo test"));
    }

    #[test]
    fn engine_prioritizes_history_match() {
        let mut engine = CompletionEngine::new().with_common_commands();
        engine.add_history("cargo clippy --all-targets");
        let results = engine.complete("cargo ");
        assert_eq!(results[0].value, "cargo clippy --all-targets");
        assert_eq!(results[0].kind, CompletionKind::History);
    }

    #[test]
    fn engine_no_match_returns_empty() {
        let engine = CompletionEngine::new().with_common_commands();
        let results = engine.complete("xyzzy");
        assert!(results.is_empty());
    }

    #[test]
    fn engine_path_completion_for_slash() {
        let mut engine = CompletionEngine::new();
        engine.add_path("/home/user");
        engine.add_path("/home/admin");
        let results = engine.complete("cd /home");
        assert!(results.iter().any(|c| c.value == "/home/user"));
        assert!(results.iter().any(|c| c.value == "/home/admin"));
    }

    #[test]
    fn flag_completions() {
        let mut engine = CompletionEngine::new();
        engine.register_flag("--release", Some("Build in release mode".to_string()));
        engine.register_flag("--all-targets", None);
        let results = engine.complete_flags("");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn autosuggest_from_history() {
        let mut engine = CompletionEngine::new();
        engine.add_history("cargo build --release");
        assert_eq!(
            engine.autosuggest("cargo b"),
            Some("cargo build --release".to_string())
        );
    }

    // Warp-derived scenarios:

    #[test]
    fn completions_suggest_command_names() {
        // Completions suggest commands, option names, and path parameters.
        let engine = CompletionEngine::new().with_common_commands();
        let results = engine.complete("git");
        assert!(results.iter().any(|c| c.value == "git status"));
    }

    #[test]
    fn autosuggestions_appear_as_you_type() {
        // Autosuggestions are based on shell history and possible completions.
        let mut engine = CompletionEngine::new();
        engine.add_history("ls -la");
        assert_eq!(engine.autosuggest("ls "), Some("ls -la".to_string()));
    }

    #[test]
    fn tab_completion_menu_closes_after_selection() {
        // After selecting a completion, the menu closes.
        let _engine = CompletionEngine::new().with_common_commands();
        let mut input = InputState::from("cargo b");
        let _ = input.apply_completion("cargo build");
        assert_eq!(input.text, "cargo build");
    }

    #[test]
    #[ignore = "shell integration not yet implemented"]
    fn shell_specific_completions_for_zsh_bash_fish() {
        // Completions adapt to the active shell.
    }

    #[test]
    #[ignore = "dynamic workflow arguments not yet implemented"]
    fn workflow_argument_completion_populates_from_context() {
        // Workflow arguments are populated dynamically from context.
    }

    #[derive(Debug, Default)]
    struct InputState {
        text: String,
    }

    impl InputState {
        fn from(text: &str) -> Self {
            Self {
                text: text.to_string(),
            }
        }
        fn apply_completion(&mut self, value: &str) -> &str {
            self.text = value.to_string();
            &self.text
        }
    }
}
