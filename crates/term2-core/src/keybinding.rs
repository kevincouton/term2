//! Keyboard shortcuts and keybinding configuration.
//!
//! Warp exposes a large set of keyboard shortcuts and lets users remap them.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A parsed keyboard shortcut such as "CMD-SHIFT-K" or "CTRL-R".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Shortcut {
    pub modifiers: Vec<String>,
    pub key: String,
}

impl Shortcut {
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<_> = s.split('-').map(|p| p.trim().to_uppercase()).collect();
        if parts.is_empty() {
            return Err("empty shortcut".to_string());
        }
        let key = parts.last().unwrap().clone();
        let modifiers = parts[..parts.len() - 1].to_vec();
        Ok(Self { modifiers, key })
    }

    pub fn has_modifier(&self, modifier: &str) -> bool {
        self.modifiers
            .iter()
            .any(|m| m.eq_ignore_ascii_case(modifier))
    }

    pub fn is(&self, key: &str) -> bool {
        self.key.eq_ignore_ascii_case(key)
    }
}

/// A mapping from shortcut to action command.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeybindingSet {
    bindings: HashMap<Shortcut, String>,
}

impl KeybindingSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind(&mut self, shortcut: Shortcut, action: impl Into<String>) {
        self.bindings.insert(shortcut, action.into());
    }

    pub fn action_for(&self, shortcut: &Shortcut) -> Option<&str> {
        self.bindings.get(shortcut).map(|s| s.as_str())
    }

    pub fn conflicts(&self) -> Vec<(&Shortcut, &String)> {
        // In the current HashMap model, duplicate keys overwrite each other,
        // so conflicts can only be detected at insert time or by scanning for
        // equivalent normalized shortcuts. We keep this API for future validation.
        Vec::new()
    }

    pub fn all(&self) -> &HashMap<Shortcut, String> {
        &self.bindings
    }

    pub fn reset(&mut self) {
        self.bindings.clear();
    }
}

/// Default Warp shortcuts mapped to action commands.
pub fn default_keybindings() -> KeybindingSet {
    let mut kb = KeybindingSet::new();
    kb.bind(Shortcut::parse("CMD-D").unwrap(), "pane_group:add_right");
    kb.bind(
        Shortcut::parse("CTRL-CMD-L").unwrap(),
        "workspace:toggle_launch_config_palette",
    );
    kb.bind(
        Shortcut::parse("CTRL-CMD-T").unwrap(),
        "workspace:show_theme_chooser",
    );
    kb.bind(
        Shortcut::parse("CTRL-R").unwrap(),
        "workspace:show_command_search",
    );
    kb.bind(
        Shortcut::parse("CTRL-SHIFT-R").unwrap(),
        "input:toggle_workflows",
    );
    kb.bind(Shortcut::parse("CMD-L").unwrap(), "terminal:focus_input");
    kb.bind(
        Shortcut::parse("CMD-\\").unwrap(),
        "terminal:toggle_warp_drive",
    );
    kb.bind(
        Shortcut::parse("CMD-P").unwrap(),
        "workspace:toggle_command_palette",
    );
    kb.bind(Shortcut::parse("CMD-K").unwrap(), "terminal:clear_blocks");
    kb.bind(
        Shortcut::parse("CMD-B").unwrap(),
        "terminal:bookmark_selected_block",
    );
    kb.bind(
        Shortcut::parse("CMD-UP").unwrap(),
        "terminal:select_previous_block",
    );
    kb.bind(
        Shortcut::parse("CMD-DOWN").unwrap(),
        "terminal:select_next_block",
    );
    kb.bind(Shortcut::parse("CMD-T").unwrap(), "workspace:open_new_tab");
    kb.bind(
        Shortcut::parse("CMD-1").unwrap(),
        "workspace:activate_first_tab",
    );
    kb.bind(
        Shortcut::parse("CMD-2").unwrap(),
        "workspace:activate_second_tab",
    );
    kb.bind(
        Shortcut::parse("CMD-3").unwrap(),
        "workspace:activate_third_tab",
    );
    kb.bind(
        Shortcut::parse("CMD-4").unwrap(),
        "workspace:activate_fourth_tab",
    );
    kb.bind(
        Shortcut::parse("CMD-5").unwrap(),
        "workspace:activate_fifth_tab",
    );
    kb.bind(
        Shortcut::parse("CMD-6").unwrap(),
        "workspace:activate_sixth_tab",
    );
    kb.bind(
        Shortcut::parse("CMD-7").unwrap(),
        "workspace:activate_seventh_tab",
    );
    kb.bind(
        Shortcut::parse("CMD-8").unwrap(),
        "workspace:activate_eighth_tab",
    );
    kb.bind(
        Shortcut::parse("CMD-0").unwrap(),
        "workspace:reset_font_size",
    );
    kb.bind(
        Shortcut::parse("CMD-=").unwrap(),
        "workspace:increase_font_size",
    );
    kb.bind(
        Shortcut::parse("CMD--").unwrap(),
        "workspace:decrease_font_size",
    );
    kb.bind(Shortcut::parse("CMD-D").unwrap(), "pane:split_right");
    kb.bind(Shortcut::parse("CMD-SHIFT-D").unwrap(), "pane:split_down");
    kb.bind(Shortcut::parse("CMD-W").unwrap(), "pane:close");
    kb.bind(Shortcut::parse("CMD-[").unwrap(), "pane:focus_prev");
    kb.bind(Shortcut::parse("CMD-]").unwrap(), "pane:focus_next");
    kb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_shortcut() {
        let s = Shortcut::parse("CMD-SHIFT-K").unwrap();
        assert_eq!(s.key, "K");
        assert!(s.has_modifier("CMD"));
        assert!(s.has_modifier("SHIFT"));
        assert!(!s.has_modifier("ALT"));
    }

    #[test]
    fn parse_shortcut_with_ctrl() {
        let s = Shortcut::parse("CTRL-R").unwrap();
        assert_eq!(s.key, "R");
        assert!(s.has_modifier("CTRL"));
    }

    #[test]
    fn keybinding_lookup() {
        let mut kb = KeybindingSet::new();
        let shortcut = Shortcut::parse("CMD-P").unwrap();
        kb.bind(shortcut.clone(), "workspace:toggle_command_palette");
        assert_eq!(
            kb.action_for(&shortcut),
            Some("workspace:toggle_command_palette")
        );
    }

    #[test]
    fn default_keybindings_contain_essentials() {
        let kb = default_keybindings();
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-D").unwrap()),
            Some("pane:split_right")
        );
        assert_eq!(
            kb.action_for(&Shortcut::parse("CTRL-R").unwrap()),
            Some("workspace:show_command_search")
        );
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-P").unwrap()),
            Some("workspace:toggle_command_palette")
        );
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-K").unwrap()),
            Some("terminal:clear_blocks")
        );
    }

    #[test]
    fn tab_shortcuts_1_through_8() {
        let kb = default_keybindings();
        for i in 1..=8 {
            let shortcut = Shortcut::parse(&format!("CMD-{i}")).unwrap();
            let expected = format!("workspace:activate_{}tab", ordinal(i));
            assert_eq!(kb.action_for(&shortcut), Some(expected.as_str()));
        }
    }

    fn ordinal(n: usize) -> &'static str {
        match n {
            1 => "first_",
            2 => "second_",
            3 => "third_",
            4 => "fourth_",
            5 => "fifth_",
            6 => "sixth_",
            7 => "seventh_",
            8 => "eighth_",
            _ => "",
        }
    }

    #[test]
    fn keybinding_overwrite() {
        let mut kb = KeybindingSet::new();
        let shortcut = Shortcut::parse("CMD-T").unwrap();
        kb.bind(shortcut.clone(), "old");
        kb.bind(shortcut.clone(), "new");
        assert_eq!(kb.action_for(&shortcut), Some("new"));
    }

    // Warp-derived scenarios:

    #[test]
    fn split_pane_shortcut_maps_to_action() {
        // CMD-D splits pane right.
        let kb = default_keybindings();
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-D").unwrap()),
            Some("pane:split_right")
        );
    }

    #[test]
    fn pane_split_actions_are_bound() {
        let kb = default_keybindings();
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-D").unwrap()),
            Some("pane:split_right")
        );
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-SHIFT-D").unwrap()),
            Some("pane:split_down")
        );
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-W").unwrap()),
            Some("pane:close")
        );
    }

    #[test]
    fn command_palette_shortcut_maps_to_action() {
        // CMD-P / CTRL-SHIFT-P toggles command palette.
        let kb = default_keybindings();
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-P").unwrap()),
            Some("workspace:toggle_command_palette")
        );
    }

    #[test]
    fn block_navigation_shortcuts() {
        // CMD-UP/DOWN select previous/next block.
        let kb = default_keybindings();
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-UP").unwrap()),
            Some("terminal:select_previous_block")
        );
        assert_eq!(
            kb.action_for(&Shortcut::parse("CMD-DOWN").unwrap()),
            Some("terminal:select_next_block")
        );
    }

    #[test]
    #[ignore = "custom keybinding editor not yet implemented"]
    fn custom_keybindings_can_override_defaults() {
        // Users can remap shortcuts in Settings > Keyboard shortcuts.
    }
}
