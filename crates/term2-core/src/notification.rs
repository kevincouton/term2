//! Agent and system notifications.
//!
//! Warp surfaces notifications from coding agents, both in-app and via desktop
//! alerts, so you know exactly when an agent needs your attention.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Complete,
    Request,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Notification {
    pub id: String,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub session_id: String,
    pub read: bool,
}

impl Notification {
    pub fn new(
        id: impl Into<String>,
        notification_type: NotificationType,
        title: impl Into<String>,
        message: impl Into<String>,
        session_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            notification_type,
            title: title.into(),
            message: message.into(),
            session_id: session_id.into(),
            read: false,
        }
    }

    pub fn mark_read(&mut self) {
        self.read = true;
    }
}

#[derive(Debug, Default, Clone)]
pub struct NotificationMailbox {
    notifications: Vec<Notification>,
    filter: MailboxFilter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MailboxFilter {
    #[default]
    All,
    Unread,
    Errors,
}

impl NotificationMailbox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    pub fn set_filter(&mut self, filter: MailboxFilter) {
        self.filter = filter;
    }

    pub fn filtered(&self) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| match self.filter {
                MailboxFilter::All => true,
                MailboxFilter::Unread => !n.read,
                MailboxFilter::Errors => n.notification_type == NotificationType::Error,
            })
            .collect()
    }

    pub fn mark_all_read(&mut self) {
        for n in &mut self.notifications {
            n.read = true;
        }
    }

    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    pub fn error_count(&self) -> usize {
        self.notifications
            .iter()
            .filter(|n| n.notification_type == NotificationType::Error && !n.read)
            .count()
    }

    pub fn clear(&mut self) {
        self.notifications.clear();
    }
}

/// Per-tab status indicator for agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TabAgentState {
    Working,
    Blocked,
    Completed,
    Errored,
}

#[derive(Debug, Default, Clone)]
pub struct TabStatus {
    pub agent_state: Option<TabAgentState>,
    pub unread_notifications: bool,
}

impl TabStatus {
    pub fn set_agent_state(&mut self, state: TabAgentState) {
        self.agent_state = Some(state);
        if state == TabAgentState::Completed || state == TabAgentState::Errored {
            self.unread_notifications = true;
        }
    }

    pub fn mark_read(&mut self) {
        self.unread_notifications = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_mark_read() {
        let mut n = Notification::new(
            "1",
            NotificationType::Complete,
            "Done",
            "Task finished",
            "session-1",
        );
        assert!(!n.read);
        n.mark_read();
        assert!(n.read);
    }

    #[test]
    fn mailbox_filters() {
        let mut mailbox = NotificationMailbox::new();
        mailbox.push(Notification::new(
            "1",
            NotificationType::Complete,
            "Done",
            "",
            "s1",
        ));
        mailbox.push(Notification::new(
            "2",
            NotificationType::Request,
            "Input needed",
            "",
            "s1",
        ));
        mailbox.push(Notification::new(
            "3",
            NotificationType::Error,
            "Failed",
            "",
            "s2",
        ));

        assert_eq!(mailbox.unread_count(), 3);

        mailbox.set_filter(MailboxFilter::Errors);
        assert_eq!(mailbox.filtered().len(), 1);

        mailbox.set_filter(MailboxFilter::Unread);
        assert_eq!(mailbox.filtered().len(), 3);

        mailbox.mark_all_read();
        assert_eq!(mailbox.unread_count(), 0);
    }

    #[test]
    fn tab_status_reflects_agent_state() {
        let mut status = TabStatus::default();
        status.set_agent_state(TabAgentState::Working);
        assert_eq!(status.agent_state, Some(TabAgentState::Working));
        assert!(!status.unread_notifications);

        status.set_agent_state(TabAgentState::Errored);
        assert!(status.unread_notifications);

        status.mark_read();
        assert!(!status.unread_notifications);
    }

    // Warp-derived scenarios:

    #[test]
    fn toast_notification_types() {
        // Notifications are categorized as Complete, Request, or Error.
        let complete = Notification::new("1", NotificationType::Complete, "Done", "", "s1");
        assert_eq!(complete.notification_type, NotificationType::Complete);
    }

    #[test]
    fn mailbox_has_filter_tabs() {
        // Mailbox supports All, Unread, and Errors filters.
        let mut mailbox = NotificationMailbox::new();
        mailbox.set_filter(MailboxFilter::Unread);
        assert_eq!(mailbox.filtered().len(), 0);
    }

    #[test]
    #[ignore = "desktop notification bridge not yet implemented"]
    fn desktop_notifications_fire_when_warp_backgrounded() {
        // When Warp is in the background, notifications become desktop alerts.
    }
}
