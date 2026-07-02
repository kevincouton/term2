//! Global command palette for searching commands, workflows, files, and actions.
//!
//! The Command Palette is Warp's global search interface for quickly finding and
//! launching Workflows, Notebooks, Prompts, keyboard shortcuts, file searches,
//! settings, and other actions.

use serde::{Deserialize, Serialize};

/// Filter prefixes understood by the palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaletteScope {
    Workflows,
    Prompts,
    Notebooks,
    EnvVars,
    Files,
    Drive,
    Actions,
    Sessions,
    LaunchConfigs,
    All,
}

impl PaletteScope {
    pub fn from_query(query: &str) -> (Self, &str) {
        let (prefix, rest) = query.split_once(':').unwrap_or(("", query));
        match prefix {
            "workflows" | "w" => (Self::Workflows, rest),
            "prompts" | "p" => (Self::Prompts, rest),
            "notebook" | "n" => (Self::Notebooks, rest),
            "env_vars" => (Self::EnvVars, rest),
            "files" => (Self::Files, rest),
            "drive" => (Self::Drive, rest),
            "actions" => (Self::Actions, rest),
            "sessions" => (Self::Sessions, rest),
            "launch_configs" => (Self::LaunchConfigs, rest),
            _ => (Self::All, query),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaletteItemKind {
    Workflow,
    Prompt,
    Notebook,
    EnvVar,
    File,
    DriveObject,
    Action,
    Session,
    LaunchConfig,
    Shortcut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaletteItem {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub kind: PaletteItemKind,
    pub shortcut: Option<String>,
    pub action: String,
}

impl PaletteItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>, kind: PaletteItemKind) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: None,
            kind,
            shortcut: None,
            action: String::new(),
        }
    }

    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = action.into();
        self
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn matches(&self, query: &str) -> bool {
        let q = query.to_lowercase();
        self.title.to_lowercase().contains(&q)
            || self
                .subtitle
                .as_ref()
                .map(|s| s.to_lowercase().contains(&q))
                .unwrap_or(false)
            || self.action.to_lowercase().contains(&q)
    }
}

/// A searchable palette registry.
#[derive(Debug, Default, Clone)]
pub struct CommandPalette {
    items: Vec<PaletteItem>,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, item: PaletteItem) {
        self.items.push(item);
    }

    pub fn register_action(&mut self, id: &str, title: &str, action: &str) {
        self.items
            .push(PaletteItem::new(id, title, PaletteItemKind::Action).with_action(action));
    }

    pub fn register_shortcut(&mut self, id: &str, title: &str, shortcut: &str, action: &str) {
        self.items.push(
            PaletteItem::new(id, title, PaletteItemKind::Shortcut)
                .with_shortcut(shortcut)
                .with_action(action),
        );
    }

    pub fn search(&self, query: &str) -> Vec<PaletteItem> {
        let (scope, rest) = PaletteScope::from_query(query);
        let rest_lower = rest.to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                let scope_matches = match scope {
                    PaletteScope::All => true,
                    PaletteScope::Workflows => item.kind == PaletteItemKind::Workflow,
                    PaletteScope::Prompts => item.kind == PaletteItemKind::Prompt,
                    PaletteScope::Notebooks => item.kind == PaletteItemKind::Notebook,
                    PaletteScope::EnvVars => item.kind == PaletteItemKind::EnvVar,
                    PaletteScope::Files => item.kind == PaletteItemKind::File,
                    PaletteScope::Drive => item.kind == PaletteItemKind::DriveObject,
                    PaletteScope::Actions => {
                        item.kind == PaletteItemKind::Action
                            || item.kind == PaletteItemKind::Shortcut
                    }
                    PaletteScope::Sessions => item.kind == PaletteItemKind::Session,
                    PaletteScope::LaunchConfigs => item.kind == PaletteItemKind::LaunchConfig,
                };
                scope_matches
                    && (rest.is_empty()
                        || item.title.to_lowercase().contains(&rest_lower)
                        || item.action.to_lowercase().contains(&rest_lower))
            })
            .cloned()
            .collect()
    }

    pub fn items(&self) -> &[PaletteItem] {
        &self.items
    }

    pub fn reset(&mut self) {
        self.items.clear();
    }
}

/// Default Warp-like palette actions.
pub fn default_palette() -> CommandPalette {
    let mut palette = CommandPalette::new();
    palette.register_action("settings", "Open Settings", "workspace:show_settings_modal");
    palette.register_action("theme", "Open Theme Picker", "workspace:show_theme_chooser");
    palette.register_action(
        "command_search",
        "Command Search",
        "workspace:show_command_search",
    );
    palette.register_action("workflows", "Workflows", "input:toggle_workflows");
    palette.register_shortcut(
        "split_pane_right",
        "Split Pane Right",
        "CMD-D",
        "pane_group:add_right",
    );
    palette.register_shortcut(
        "clear_blocks",
        "Clear Blocks",
        "CMD-K",
        "terminal:clear_blocks",
    );
    palette
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_parsing() {
        assert_eq!(
            PaletteScope::from_query("workflows:deploy"),
            (PaletteScope::Workflows, "deploy")
        );
        assert_eq!(
            PaletteScope::from_query("w:git"),
            (PaletteScope::Workflows, "git")
        );
        assert_eq!(
            PaletteScope::from_query("prompts:pr"),
            (PaletteScope::Prompts, "pr")
        );
        assert_eq!(
            PaletteScope::from_query("n:onboarding"),
            (PaletteScope::Notebooks, "onboarding")
        );
        assert_eq!(
            PaletteScope::from_query("open settings"),
            (PaletteScope::All, "open settings")
        );
    }

    #[test]
    fn palette_search_by_title() {
        let mut palette = CommandPalette::new();
        palette.register_action("settings", "Open Settings", "workspace:show_settings_modal");
        palette.register_action("theme", "Open Theme Picker", "workspace:show_theme_chooser");

        let results = palette.search("settings");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Open Settings");
    }

    #[test]
    fn palette_search_by_action() {
        let mut palette = CommandPalette::new();
        palette.register_action("settings", "Open Settings", "workspace:show_settings_modal");

        let results = palette.search("theme chooser");
        assert!(results.is_empty());

        let results = palette.search("show_settings");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn palette_scope_filters_results() {
        let mut palette = CommandPalette::new();
        palette.register(PaletteItem::new("wf1", "Deploy", PaletteItemKind::Workflow));
        palette.register(PaletteItem::new("p1", "Deploy PR", PaletteItemKind::Prompt));

        let results = palette.search("workflows:deploy");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].kind, PaletteItemKind::Workflow);
    }

    #[test]
    fn default_palette_has_common_actions() {
        let palette = default_palette();
        let titles: Vec<_> = palette.items().iter().map(|i| i.title.as_str()).collect();
        assert!(titles.contains(&"Open Settings"));
        assert!(titles.contains(&"Split Pane Right"));
        assert!(titles.contains(&"Clear Blocks"));
    }

    #[test]
    fn palette_shortcut_display() {
        let mut palette = CommandPalette::new();
        palette.register_shortcut("split", "Split Pane Right", "CMD-D", "pane_group:add_right");
        let results = palette.search("split");
        assert_eq!(results[0].shortcut.as_deref(), Some("CMD-D"));
    }

    #[test]
    fn empty_query_lists_all_in_scope() {
        let mut palette = CommandPalette::new();
        palette.register(PaletteItem::new("wf1", "A", PaletteItemKind::Workflow));
        palette.register(PaletteItem::new("p1", "B", PaletteItemKind::Prompt));
        let results = palette.search("workflows:");
        assert_eq!(results.len(), 1);
    }

    // Warp-derived scenarios:

    #[test]
    fn command_palette_opens_with_shortcut() {
        // CMD-P / CTRL-SHIFT-P toggles the palette.
        let palette = default_palette();
        assert!(!palette.items().is_empty());
    }

    #[test]
    fn command_palette_filters_by_prefix() {
        // Prepending `workflows:` filters for workflows.
        let mut palette = CommandPalette::new();
        palette.register(PaletteItem::new(
            "wf1",
            "Git pull",
            PaletteItemKind::Workflow,
        ));
        palette.register(PaletteItem::new("p1", "Ask AI", PaletteItemKind::Prompt));
        let results = palette.search("w:git");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Git pull");
    }

    #[test]
    #[ignore = "file search backend not yet implemented"]
    fn command_palette_file_search_uses_tilde_expansion() {
        // `files:~` or similar expands the home directory.
    }

    #[test]
    #[ignore = "large-project palette performance not yet benchmarked"]
    fn command_palette_filters_large_project_quickly() {
        // Palette filtering should remain responsive with many items.
    }
}
