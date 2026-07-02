//! Runtime pane that owns one native PTY session.

use std::path::PathBuf;

use crate::native_session::NativeSession;
use crate::profile::{Profile, ProfileRegistry};
use crate::{Result, Session};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaneInfo {
    pub id: String,
    pub session_id: String,
    pub profile: String,
    pub is_focused: bool,
}

pub struct Pane {
    pub id: String,
    pub session_id: String,
    pub native_session: NativeSession,
}

impl Pane {
    pub fn from_profile(
        pane_id: impl Into<String>,
        session_id: impl Into<String>,
        name: &str,
        profile: &Profile,
        registry: &ProfileRegistry,
        scrollback_dir: impl Into<Option<PathBuf>>,
    ) -> Result<Self> {
        let pane_id = pane_id.into();
        let session_id = session_id.into();
        let native = NativeSession::from_profile(
            &pane_id,
            "", // user is not stored per pane in this slice
            name,
            profile,
            registry,
            scrollback_dir,
        )?;
        Ok(Self {
            id: pane_id,
            session_id,
            native_session: native,
        })
    }

    pub fn info(&self) -> PaneInfo {
        PaneInfo {
            id: self.id.clone(),
            session_id: self.session_id.clone(),
            profile: self.native_session.info.profile.clone(),
            is_focused: false, // window sets this
        }
    }

    pub fn attach(&self) -> Option<Session> {
        self.native_session.attach()
    }

    pub async fn kill(self) -> Result<()> {
        self.native_session.kill().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::ProfileRegistry;

    #[tokio::test]
    async fn pane_info_includes_id_and_profile() {
        let registry = ProfileRegistry::new("pane-test-user");
        let profile = registry.get("bash").unwrap();
        let pane = Pane::from_profile(
            "pane-1",
            "session-1",
            "test-pane",
            &profile,
            &registry,
            std::env::temp_dir().join("term2-pane-test"),
        )
        .unwrap();
        let info = pane.info();
        assert_eq!(info.id, "pane-1");
        assert_eq!(info.session_id, "session-1");
        assert_eq!(info.profile, "bash");

        pane.kill().await.unwrap();
    }
}
