//! Core terminal multiplexer, profile, and PTY abstractions for Term2.

pub mod profile;
pub mod session;

pub use profile::{Profile, ProfileRegistry, Shell};
pub use session::{Error, Result, Session, SessionInfo, SessionManager};
