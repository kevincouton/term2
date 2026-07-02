//! Tiling layout tree for windows and panes.

use serde::{Deserialize, Serialize};

pub type PaneId = String;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SplitDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LayoutNode {
    Pane(PaneId),
    Split {
        direction: SplitDirection,
        children: Vec<LayoutNode>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum LayoutError {
    #[error("pane not found: {0}")]
    PaneNotFound(PaneId),
}

impl LayoutNode {
    pub fn pane(pane_id: impl Into<PaneId>) -> Self {
        Self::Pane(pane_id.into())
    }

    pub fn split(
        direction: SplitDirection,
        children: impl IntoIterator<Item = LayoutNode>,
    ) -> Self {
        Self::Split {
            direction,
            children: children.into_iter().collect(),
        }
    }

    /// Replace `target` pane with a split containing the original pane and `new_pane_id`.
    pub fn split_pane(
        &mut self,
        target: &str,
        direction: SplitDirection,
        new_pane_id: PaneId,
    ) -> Result<(), LayoutError> {
        match self {
            LayoutNode::Pane(id) if id == target => {
                let old_id = std::mem::take(id);
                *self = LayoutNode::Split {
                    direction,
                    children: vec![LayoutNode::Pane(old_id), LayoutNode::Pane(new_pane_id)],
                };
                Ok(())
            }
            LayoutNode::Pane(_) => Err(LayoutError::PaneNotFound(target.to_string())),
            LayoutNode::Split { children, .. } => {
                for child in children {
                    if child.contains_pane(target) {
                        return child.split_pane(target, direction, new_pane_id);
                    }
                }
                Err(LayoutError::PaneNotFound(target.to_string()))
            }
        }
    }

    /// Remove `target` pane and collapse any split that becomes empty or single-child.
    pub fn remove_pane(&mut self, target: &str) -> Result<(), LayoutError> {
        match self {
            LayoutNode::Pane(id) if id == target => {
                // Caller must ensure the window is terminated if the root pane is removed.
                *self = LayoutNode::Pane(String::new());
                Ok(())
            }
            LayoutNode::Pane(_) => Err(LayoutError::PaneNotFound(target.to_string())),
            LayoutNode::Split { children, .. } => {
                let mut found = false;
                for i in 0..children.len() {
                    if children[i].contains_pane(target) {
                        children[i].remove_pane(target)?;
                        found = true;
                        if matches!(children[i], LayoutNode::Pane(ref id) if id.is_empty()) {
                            children.remove(i);
                        } else if let LayoutNode::Split { children: ref inner, .. } = children[i] {
                            if inner.is_empty() {
                                children.remove(i);
                            }
                        }
                        break;
                    }
                }
                if !found {
                    return Err(LayoutError::PaneNotFound(target.to_string()));
                }
                // Collapse a split with a single child to that child.
                if children.len() == 1 {
                    let only = children.remove(0);
                    *self = only;
                }
                Ok(())
            }
        }
    }

    pub fn contains_pane(&self, pane_id: &str) -> bool {
        match self {
            LayoutNode::Pane(id) => id == pane_id,
            LayoutNode::Split { children, .. } => children.iter().any(|c| c.contains_pane(pane_id)),
        }
    }

    pub fn list_panes(&self) -> Vec<&PaneId> {
        let mut out = Vec::new();
        self.collect_panes(&mut out);
        out
    }

    fn collect_panes<'a>(&'a self, out: &mut Vec<&'a PaneId>) {
        match self {
            LayoutNode::Pane(id) => out.push(id),
            LayoutNode::Split { children, .. } => {
                for child in children {
                    child.collect_panes(out);
                }
            }
        }
    }

    pub fn first_pane(&self) -> Option<&PaneId> {
        match self {
            LayoutNode::Pane(id) => Some(id),
            LayoutNode::Split { children, .. } => children.first().and_then(|c| c.first_pane()),
        }
    }

    pub fn next_pane(&self, pane_id: &str) -> Option<&PaneId> {
        let panes = self.list_panes();
        panes
            .iter()
            .position(|&id| id == pane_id)
            .and_then(|idx| panes.get(idx + 1).copied().or_else(|| panes.first().copied()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_leaf_creates_two_panes() {
        let mut layout = LayoutNode::Pane("p1".to_string());
        layout.split_pane("p1", SplitDirection::Vertical, "p2".to_string()).unwrap();
        match layout {
            LayoutNode::Split { direction, children } => {
                assert_eq!(direction, SplitDirection::Vertical);
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0], LayoutNode::Pane(id) if id == "p1"));
                assert!(matches!(&children[1], LayoutNode::Pane(id) if id == "p2"));
            }
            _ => panic!("expected split"),
        }
    }

    #[test]
    fn nested_split_preserves_direction() {
        let mut layout = LayoutNode::Pane("p1".to_string());
        layout.split_pane("p1", SplitDirection::Vertical, "p2".to_string()).unwrap();
        layout.split_pane("p2", SplitDirection::Horizontal, "p3".to_string()).unwrap();

        assert!(layout.contains_pane("p1"));
        assert!(layout.contains_pane("p2"));
        assert!(layout.contains_pane("p3"));
        assert!(!layout.contains_pane("p4"));

        let panes = layout.list_panes();
        assert_eq!(panes, vec!["p1", "p2", "p3"]);

        match &layout {
            LayoutNode::Split {
                direction: SplitDirection::Vertical,
                children,
            } => match &children[1] {
                LayoutNode::Split {
                    direction: SplitDirection::Horizontal,
                    children: inner,
                } => {
                    assert_eq!(inner.len(), 2);
                    assert!(matches!(&inner[0], LayoutNode::Pane(id) if id == "p2"));
                    assert!(matches!(&inner[1], LayoutNode::Pane(id) if id == "p3"));
                }
                _ => panic!("expected horizontal split"),
            },
            _ => panic!("expected vertical split"),
        }
    }

    #[test]
    fn remove_pane_collapses_split() {
        let mut layout = LayoutNode::Pane("p1".to_string());
        layout.split_pane("p1", SplitDirection::Vertical, "p2".to_string()).unwrap();
        layout.remove_pane("p2").unwrap();

        assert_eq!(layout, LayoutNode::Pane("p1".to_string()));
        assert!(layout.contains_pane("p1"));
        assert!(!layout.contains_pane("p2"));
    }

    #[test]
    fn remove_pane_cascades_in_nested_split() {
        let mut layout = LayoutNode::Pane("p1".to_string());
        layout.split_pane("p1", SplitDirection::Vertical, "p2".to_string()).unwrap();
        layout.split_pane("p2", SplitDirection::Horizontal, "p3".to_string()).unwrap();
        layout.remove_pane("p3").unwrap();

        // The horizontal split collapses to just p2, leaving a vertical split of p1 and p2.
        assert!(matches!(layout, LayoutNode::Split { .. }));
        assert!(layout.contains_pane("p1"));
        assert!(layout.contains_pane("p2"));
        assert!(!layout.contains_pane("p3"));
    }

    #[test]
    fn remove_pane_not_found() {
        let mut layout = LayoutNode::Pane("p1".to_string());
        let err = layout.remove_pane("p2").unwrap_err();
        assert!(matches!(err, LayoutError::PaneNotFound(id) if id == "p2"));
    }

    #[test]
    fn first_pane_returns_leftmost() {
        let mut layout = LayoutNode::Pane("p1".to_string());
        layout.split_pane("p1", SplitDirection::Vertical, "p2".to_string()).unwrap();
        layout.split_pane("p2", SplitDirection::Horizontal, "p3".to_string()).unwrap();

        assert_eq!(layout.first_pane(), Some(&"p1".to_string()));
    }

    #[test]
    fn first_pane_on_single_pane() {
        let layout = LayoutNode::Pane("p1".to_string());
        assert_eq!(layout.first_pane(), Some(&"p1".to_string()));
    }

    #[test]
    fn next_pane_advances_and_wraps() {
        let mut layout = LayoutNode::Pane("p1".to_string());
        layout.split_pane("p1", SplitDirection::Vertical, "p2".to_string()).unwrap();
        layout.split_pane("p2", SplitDirection::Horizontal, "p3".to_string()).unwrap();

        assert_eq!(layout.next_pane("p1"), Some(&"p2".to_string()));
        assert_eq!(layout.next_pane("p2"), Some(&"p3".to_string()));
        assert_eq!(layout.next_pane("p3"), Some(&"p1".to_string()));
    }

    #[test]
    fn next_pane_single_pane_wraps_to_self() {
        let layout = LayoutNode::Pane("p1".to_string());
        assert_eq!(layout.next_pane("p1"), Some(&"p1".to_string()));
    }
}
