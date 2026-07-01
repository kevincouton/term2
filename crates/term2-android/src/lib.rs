//! Android JNI bridge to the Term2 Rust core.

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub use android::*;
