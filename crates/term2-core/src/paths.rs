use std::path::PathBuf;

/// Return the base Term2 configuration directory.
///
/// Honors the `TERM2_CONFIG_DIR` environment variable when set. Falls back to
/// `~/.config/term2` (or `./term2` when the system config directory cannot be
/// determined).
pub fn term2_config_dir() -> PathBuf {
    std::env::var("TERM2_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("term2")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_dir_uses_env_override() {
        let expected = std::env::temp_dir().join("term2-config-override-test");
        std::env::set_var("TERM2_CONFIG_DIR", &expected);
        let dir = term2_config_dir();
        std::env::remove_var("TERM2_CONFIG_DIR");
        assert_eq!(dir, expected);
    }

    #[test]
    fn config_dir_falls_back_to_default() {
        std::env::remove_var("TERM2_CONFIG_DIR");
        let dir = term2_config_dir();
        let expected = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("term2");
        assert_eq!(dir, expected);
    }
}
