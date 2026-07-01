//! Core terminal multiplexer and PTY abstractions for Term2.

pub mod pty;
pub mod session;

pub use session::{Error, Result, SessionManager};
