//! Core terminal multiplexer, profile, and PTY abstractions for Term2.

pub mod agent_mode;
pub mod block;
pub mod command_palette;
pub mod completions;
pub mod input_editor;
pub mod keybinding;
pub mod launch_config;
pub mod layout;
pub mod native_session;
pub mod notification;
pub mod pane;
pub mod paths;
pub mod profile;
pub mod pty_manager;
pub mod scrollback;
pub mod session;
pub mod theme;
pub mod warp_drive;
pub mod window;

pub use agent_mode::{AgentConversation, AgentStatus, InputMode};
pub use block::{Block, BlockId, BlockStatus, BlockStore};
pub use command_palette::{CommandPalette, PaletteItem, PaletteItemKind, PaletteScope};
pub use completions::{CompletionEngine, CompletionItem, CompletionKind, HistoryStore};
pub use input_editor::InputEditor;
pub use keybinding::{KeybindingSet, Shortcut};
pub use launch_config::{LaunchConfig, TabColor};
pub use layout::{LayoutNode, SplitDirection};
pub use native_session::NativeSession;
pub use notification::{Notification, NotificationMailbox, NotificationType, TabStatus};
pub use pane::{Pane, PaneInfo};
pub use profile::{Profile, ProfileRegistry, Shell};
pub use pty_manager::{PtyHandle, PtyManager};
pub use scrollback::{ReplayReceiver, ReplaySender, Scrollback, DEFAULT_MAX_SCROLLBACK_BYTES};
pub use session::{Backend, Error, Result, Session, SessionInfo, SessionManager};
pub use theme::{AppearanceConfig, Color, CursorStyle, InputPosition, Theme};
pub use warp_drive::{
    ArgumentType, DynamicSource, EnvVar, EnvVarSet, Notebook, NotebookCell, Prompt, WarpDrive,
    Workflow, WorkflowArgument,
};
pub use window::Window;
pub use window::WindowInfo;
