//! Launch configurations: save and restore windows, tabs, and panes.
//!
//! Launch Configurations let users save a layout of windows, tabs, and panes
//! and reopen it later.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaunchConfig {
    pub name: String,
    #[serde(default)]
    pub active_window_index: Option<usize>,
    #[serde(default)]
    pub windows: Vec<WindowConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowConfig {
    #[serde(default)]
    pub active_tab_index: Option<usize>,
    #[serde(default)]
    pub tabs: Vec<TabConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TabConfig {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub color: Option<TabColor>,
    #[serde(default)]
    pub layout: LayoutConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TabColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SplitDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayoutConfig {
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    #[serde(default)]
    pub split_direction: SplitDirection,
    #[serde(default)]
    pub panes: Vec<PaneConfig>,
    #[serde(default)]
    pub commands: Vec<CommandConfig>,
    #[serde(default)]
    pub is_focused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaneConfig {
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    #[serde(default)]
    pub commands: Vec<CommandConfig>,
    #[serde(default)]
    pub is_focused: bool,
    #[serde(default)]
    pub layout: Option<Box<LayoutConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandConfig {
    pub exec: String,
}

impl LaunchConfig {
    pub fn from_yaml(yaml: &str) -> Result<Self, String> {
        serde_yaml::from_str(yaml).map_err(|e| e.to_string())
    }

    pub fn to_yaml(&self) -> Result<String, String> {
        serde_yaml::to_string(self).map_err(|e| e.to_string())
    }

    pub fn total_tabs(&self) -> usize {
        self.windows.iter().map(|w| w.tabs.len()).sum()
    }

    pub fn total_panes(&self) -> usize {
        self.windows
            .iter()
            .map(|w| w.tabs.iter().map(|t| t.layout.pane_count()).sum::<usize>())
            .sum()
    }

    pub fn focused_pane(&self) -> Option<&PaneConfig> {
        let window = self
            .active_window_index
            .and_then(|i| self.windows.get(i))
            .or(self.windows.first())?;
        let tab = window
            .active_tab_index
            .and_then(|i| window.tabs.get(i))
            .or(window.tabs.first())?;
        tab.layout.focused_pane()
    }

    pub fn all_cwds(&self) -> Vec<&PathBuf> {
        let mut cwds = Vec::new();
        for window in &self.windows {
            for tab in &window.tabs {
                tab.layout.collect_cwds(&mut cwds);
            }
        }
        cwds
    }
}

impl LayoutConfig {
    pub fn pane_count(&self) -> usize {
        if self.panes.is_empty() {
            1
        } else {
            self.panes.iter().map(|p| p.pane_count()).sum()
        }
    }

    pub fn focused_pane(&self) -> Option<&PaneConfig> {
        if self.panes.is_empty() {
            return None;
        }
        for pane in &self.panes {
            if let Some(found) = pane.focused_pane() {
                return Some(found);
            }
        }
        self.panes.first()
    }

    pub fn collect_cwds<'a>(&'a self, out: &mut Vec<&'a PathBuf>) {
        if let Some(cwd) = &self.cwd {
            out.push(cwd);
        }
        for pane in &self.panes {
            pane.collect_cwds(out);
        }
    }
}

impl PaneConfig {
    pub fn pane_count(&self) -> usize {
        self.layout.as_ref().map(|l| l.pane_count()).unwrap_or(1)
    }

    pub fn focused_pane(&self) -> Option<&PaneConfig> {
        if self.is_focused {
            Some(self)
        } else if let Some(layout) = &self.layout {
            layout.focused_pane()
        } else {
            None
        }
    }

    pub fn collect_cwds<'a>(&'a self, out: &mut Vec<&'a PathBuf>) {
        if let Some(cwd) = &self.cwd {
            out.push(cwd);
        }
        if let Some(layout) = &self.layout {
            layout.collect_cwds(out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_launch_config() {
        let yaml = r#"
---
name: Example Windows
windows:
  - tabs:
      - title: Documents
        layout:
          cwd: /Users/warp-user/Documents
        color: blue
  - tabs:
      - title: Warp User
        layout:
          cwd: /Users/warp-user
        color: green
"#;
        let config = LaunchConfig::from_yaml(yaml).unwrap();
        assert_eq!(config.name, "Example Windows");
        assert_eq!(config.total_tabs(), 2);
        assert_eq!(config.total_panes(), 2);
    }

    #[test]
    fn parse_split_panes_config() {
        let yaml = r#"
---
name: Example Panes
windows:
  - tabs:
      - title: Downloads and Warp User
        layout:
          split_direction: vertical
          panes:
            - cwd: /Users/warp-user/Downloads
            - cwd: /Users/warp-user
        color: blue
"#;
        let config = LaunchConfig::from_yaml(yaml).unwrap();
        assert_eq!(config.total_panes(), 2);
        let cwds: Vec<_> = config
            .all_cwds()
            .iter()
            .map(|p| p.to_str().unwrap())
            .collect();
        assert!(cwds.contains(&"/Users/warp-user/Downloads"));
        assert!(cwds.contains(&"/Users/warp-user"));
    }

    #[test]
    fn parse_nested_splits() {
        let yaml = r#"
---
name: Nested
windows:
  - tabs:
      - title: Nested
        layout:
          split_direction: horizontal
          panes:
            - cwd: /Users/warp-user/Desktop
            - layout:
                split_direction: vertical
                panes:
                  - cwd: /Users/warp-user/Documents
                  - cwd: /Users/warp-user
"#;
        let config = LaunchConfig::from_yaml(yaml).unwrap();
        assert_eq!(config.total_panes(), 3);
    }

    #[test]
    fn focused_pane_is_respected() {
        let yaml = r#"
---
name: Example Active and Focus
active_window_index: 0
windows:
  - active_tab_index: 1
    tabs:
      - title: Tab 1
        layout:
          split_direction: vertical
          panes:
            - cwd: /Users/warp-user/Documents
              is_focused: true
            - cwd: /Users/warp-user/Documents/Projects
      - title: Tab 2
        layout:
          split_direction: horizontal
          panes:
            - cwd: /Users/warp-user/Downloads
            - cwd: /Users/warp-user
              is_focused: true
"#;
        let config = LaunchConfig::from_yaml(yaml).unwrap();
        let focused = config.focused_pane().unwrap();
        assert_eq!(
            focused.cwd.as_ref().unwrap().to_str(),
            Some("/Users/warp-user")
        );
        assert!(focused.is_focused);
    }

    #[test]
    fn parse_commands() {
        let yaml = r#"
---
name: Example Commands
windows:
  - tabs:
      - title: Documents
        layout:
          cwd: /Users/warp-user/Documents
          commands:
            - exec: ls
            - exec: code .
        color: blue
"#;
        let config = LaunchConfig::from_yaml(yaml).unwrap();
        let pane = &config.windows[0].tabs[0].layout;
        assert_eq!(pane.commands.len(), 2);
        assert_eq!(pane.commands[0].exec, "ls");
    }

    #[test]
    fn serialize_round_trip() {
        let config = LaunchConfig {
            name: "Dev".into(),
            active_window_index: Some(0),
            windows: vec![WindowConfig {
                active_tab_index: Some(0),
                tabs: vec![TabConfig {
                    title: Some("main".into()),
                    color: Some(TabColor::Blue),
                    layout: LayoutConfig {
                        cwd: Some(PathBuf::from("/tmp")),
                        split_direction: SplitDirection::Vertical,
                        panes: vec![],
                        commands: vec![],
                        is_focused: true,
                    },
                }],
            }],
        };
        let yaml = config.to_yaml().unwrap();
        let parsed = LaunchConfig::from_yaml(&yaml).unwrap();
        assert_eq!(config, parsed);
    }

    // Warp-derived scenarios:

    #[test]
    fn launch_config_restores_windows_tabs_panes() {
        // Launch configurations save and restore windows, tabs, and panes.
        let yaml = r#"
name: Dev
windows:
  - tabs:
      - title: Server
        layout:
          cwd: /app
      - title: Tests
        layout:
          cwd: /app
"#;
        let config = LaunchConfig::from_yaml(yaml).unwrap();
        assert_eq!(config.total_tabs(), 2);
    }

    #[test]
    fn launch_config_supports_split_directions() {
        // Panes can be split vertically or horizontally.
        let yaml = r#"
name: Split
windows:
  - tabs:
      - title: Split
        layout:
          split_direction: vertical
          panes:
            - cwd: /a
            - cwd: /b
"#;
        let config = LaunchConfig::from_yaml(yaml).unwrap();
        assert_eq!(config.total_panes(), 2);
    }

    #[test]
    fn launch_config_runs_startup_commands() {
        // Commands can run when a launch configuration is applied.
        let yaml = r#"
name: Boot
windows:
  - tabs:
      - title: Boot
        layout:
          cwd: /app
          commands:
            - exec: npm install
"#;
        let config = LaunchConfig::from_yaml(yaml).unwrap();
        assert_eq!(
            config.windows[0].tabs[0].layout.commands[0].exec,
            "npm install"
        );
    }

    #[test]
    #[ignore = "launcher not yet implemented"]
    fn launch_config_opens_in_current_window() {
        // Single-window launch configs can be launched into the active window.
    }
}
