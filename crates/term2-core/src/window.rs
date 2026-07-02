//! A window owns a tiling layout of panes.

use std::collections::HashMap;
use std::path::PathBuf;

use uuid::Uuid;

use crate::layout::{LayoutNode, PaneId, SplitDirection};
use crate::pane::{Pane, PaneInfo};
use crate::profile::{Profile, ProfileRegistry};
use crate::{Result, Session};

pub struct Window {
    pub id: String,
    pub title: String,
    pub layout: LayoutNode,
    pub active_pane_id: PaneId,
    panes: HashMap<PaneId, Pane>,
    scrollback_root: PathBuf,
}

impl Window {
    pub async fn new(
        session_id: impl Into<String>,
        window_id: impl Into<String>,
        title: impl Into<String>,
        profile: &Profile,
        registry: &ProfileRegistry,
        scrollback_root: impl Into<PathBuf>,
    ) -> Result<Self> {
        let session_id = session_id.into();
        let window_id = window_id.into();
        let title = title.into();
        let scrollback_root = scrollback_root.into();
        let pane_id = Uuid::new_v4().to_string();
        let pane_scrollback = scrollback_root.join(&pane_id);
        let pane = Pane::from_profile(
            &pane_id,
            &session_id,
            &title,
            profile,
            registry,
            pane_scrollback,
        )?;
        let mut panes = HashMap::new();
        panes.insert(pane_id.clone(), pane);
        Ok(Self {
            id: window_id,
            title,
            layout: LayoutNode::Pane(pane_id.clone()),
            active_pane_id: pane_id,
            panes,
            scrollback_root,
        })
    }

    pub fn active_pane(&self) -> Option<&Pane> {
        self.panes.get(&self.active_pane_id)
    }

    pub fn active_pane_mut(&mut self) -> Option<&mut Pane> {
        self.panes.get_mut(&self.active_pane_id)
    }

    pub fn pane(&self, pane_id: &PaneId) -> Option<&Pane> {
        self.panes.get(pane_id)
    }

    pub fn split_active_pane(
        &mut self,
        direction: SplitDirection,
        profile: &Profile,
        registry: &ProfileRegistry,
    ) -> Result<PaneInfo> {
        let new_pane_id = Uuid::new_v4().to_string();
        let pane_scrollback = self.scrollback_root.join(&new_pane_id);
        let pane = Pane::from_profile(
            &new_pane_id,
            &self.panes[&self.active_pane_id].session_id,
            &self.title,
            profile,
            registry,
            pane_scrollback,
        )?;
        self.layout
            .split_pane(&self.active_pane_id, direction, new_pane_id.clone())
            .map_err(|e| crate::Error::SessionNotFound(e.to_string()))?;
        let info = pane.info();
        self.panes.insert(new_pane_id.clone(), pane);
        self.active_pane_id = new_pane_id;
        Ok(info)
    }

    pub fn close_pane(&mut self, pane_id: &PaneId) -> Result<bool> {
        if !self.panes.contains_key(pane_id) {
            return Err(crate::Error::SessionNotFound(pane_id.clone()));
        }

        // Closing the last pane empties the window; the caller should terminate
        // the session instead of keeping an empty layout.
        if self.panes.len() == 1 {
            let pane = self.panes.remove(pane_id).expect("pane verified above");
            pane.native_session.kill_now();
            return Ok(true);
        }

        self.layout
            .remove_pane(pane_id)
            .map_err(|e| crate::Error::SessionNotFound(e.to_string()))?;
        let pane = self.panes.remove(pane_id).expect("pane verified above");
        pane.native_session.kill_now();

        if self.active_pane_id == *pane_id {
            self.active_pane_id = self
                .layout
                .first_pane()
                .cloned()
                .unwrap_or_else(|| self.panes.keys().next().unwrap().clone());
        }
        Ok(false)
    }

    pub fn focus_pane(&mut self, pane_id: &PaneId) -> Result<()> {
        if !self.panes.contains_key(pane_id) {
            return Err(crate::Error::SessionNotFound(pane_id.clone()));
        }
        self.active_pane_id = pane_id.clone();
        Ok(())
    }

    pub fn list_panes(&self) -> Vec<PaneInfo> {
        self.layout
            .list_panes()
            .into_iter()
            .filter_map(|id| {
                self.panes.get(id).map(|p| {
                    let mut info = p.info();
                    info.is_focused = id == &self.active_pane_id;
                    info
                })
            })
            .collect()
    }

    pub fn attach_active(&self) -> Option<Session> {
        self.active_pane().and_then(|p| p.attach())
    }

    pub async fn kill_all_panes(mut self) -> Result<()> {
        for (_, pane) in self.panes.drain() {
            pane.kill().await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::ProfileRegistry;

    fn test_registry(prefix: &str) -> ProfileRegistry {
        let user = format!("{}-{}", prefix, std::process::id());
        ProfileRegistry::new(&user)
    }

    fn test_scrollback_dir(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!("term2-window-{}-{}", prefix, std::process::id()))
    }

    #[tokio::test]
    async fn window_starts_with_one_pane() {
        let registry = test_registry("window-starts");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("starts");
        let window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        assert_eq!(window.list_panes().len(), 1);
        assert_eq!(window.active_pane_id, window.list_panes()[0].id);
        window.kill_all_panes().await.unwrap();
    }

    #[tokio::test]
    async fn split_active_pane_creates_second_pane() {
        let registry = test_registry("window-split");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("split");
        let mut window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        let original_id = window.active_pane_id.clone();

        let info = window
            .split_active_pane(SplitDirection::Vertical, &profile, &registry)
            .unwrap();

        assert_eq!(window.list_panes().len(), 2);
        assert!(window.panes.contains_key(&info.id));
        assert!(window.panes.contains_key(&original_id));
        assert_eq!(window.active_pane_id, info.id);
        window.kill_all_panes().await.unwrap();
    }

    #[tokio::test]
    async fn split_active_pane_updates_layout() {
        let registry = test_registry("window-layout");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("layout");
        let mut window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        let original_id = window.active_pane_id.clone();

        window
            .split_active_pane(SplitDirection::Horizontal, &profile, &registry)
            .unwrap();

        assert!(matches!(window.layout, LayoutNode::Split { .. }));
        assert!(window.layout.contains_pane(&original_id));
        assert!(window.layout.contains_pane(&window.active_pane_id));
        window.kill_all_panes().await.unwrap();
    }

    #[tokio::test]
    async fn close_pane_removes_pane_and_updates_focus() {
        let registry = test_registry("window-close");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("close");
        let mut window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        let original_id = window.active_pane_id.clone();
        let info = window
            .split_active_pane(SplitDirection::Vertical, &profile, &registry)
            .unwrap();
        let new_id = info.id;

        let empty = window.close_pane(&new_id).unwrap();
        assert!(!empty);
        assert_eq!(window.list_panes().len(), 1);
        assert!(!window.panes.contains_key(&new_id));
        assert!(window.panes.contains_key(&original_id));
        assert_eq!(window.active_pane_id, original_id);
        window.kill_all_panes().await.unwrap();
    }

    #[tokio::test]
    async fn close_pane_returns_true_when_window_becomes_empty() {
        let registry = test_registry("window-empty");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("empty");
        let mut window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        let pane_id = window.active_pane_id.clone();

        let empty = window.close_pane(&pane_id).unwrap();
        assert!(empty);
        assert!(window.panes.is_empty());
    }

    #[tokio::test]
    async fn focus_pane_changes_active_pane() {
        let registry = test_registry("window-focus");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("focus");
        let mut window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        let original_id = window.active_pane_id.clone();
        let info = window
            .split_active_pane(SplitDirection::Vertical, &profile, &registry)
            .unwrap();

        window.focus_pane(&original_id).unwrap();
        assert_eq!(window.active_pane_id, original_id);

        window.focus_pane(&info.id).unwrap();
        assert_eq!(window.active_pane_id, info.id);
        window.kill_all_panes().await.unwrap();
    }

    #[tokio::test]
    async fn focus_pane_fails_for_unknown_pane() {
        let registry = test_registry("window-focus-unknown");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("focus-unknown");
        let mut window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();

        let err = window.focus_pane(&"unknown-pane".to_string()).unwrap_err();
        assert!(matches!(err, crate::Error::SessionNotFound(id) if id == "unknown-pane"));
        window.kill_all_panes().await.unwrap();
    }

    #[tokio::test]
    async fn list_panes_marks_active_pane_focused() {
        let registry = test_registry("window-focused");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("focused");
        let mut window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        let original_id = window.active_pane_id.clone();
        let info = window
            .split_active_pane(SplitDirection::Vertical, &profile, &registry)
            .unwrap();

        let panes = window.list_panes();
        let active = panes.iter().find(|p| p.is_focused).unwrap();
        assert_eq!(active.id, info.id);
        assert_eq!(panes.iter().filter(|p| p.is_focused).count(), 1);

        window.focus_pane(&original_id).unwrap();
        let panes = window.list_panes();
        let active = panes.iter().find(|p| p.is_focused).unwrap();
        assert_eq!(active.id, original_id);
        window.kill_all_panes().await.unwrap();
    }

    #[tokio::test]
    async fn active_pane_returns_current_pane() {
        let registry = test_registry("window-active");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("active");
        let window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();
        let active_id = window.active_pane_id.clone();

        assert_eq!(window.active_pane().unwrap().id, active_id);
        assert_eq!(window.pane(&active_id).unwrap().id, active_id);
        assert!(window.pane(&"missing".to_string()).is_none());
        window.kill_all_panes().await.unwrap();
    }

    #[tokio::test]
    async fn attach_active_returns_session_for_active_pane() {
        let registry = test_registry("window-attach");
        let profile = registry.get("bash").unwrap();
        let dir = test_scrollback_dir("attach");
        let window = Window::new("session-1", "win-1", "main", &profile, &registry, dir)
            .await
            .unwrap();

        let session = window.attach_active().expect("attach active pane");
        assert_eq!(session.id, window.active_pane_id);
        window.kill_all_panes().await.unwrap();
    }
}
