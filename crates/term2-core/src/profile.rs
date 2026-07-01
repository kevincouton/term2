use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Shell {
    #[default]
    Bash,
    Zsh,
    Nushell,
}

impl Shell {
    pub fn as_str(&self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Nushell => "nu",
        }
    }

    /// Returns true if the shell binary exists on this system.
    pub fn is_available(&self) -> bool {
        which::which(self.as_str()).is_ok()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub shell: Shell,
    /// Extra files written into the profile directory before a session starts.
    /// For custom profiles this is where the user-provided dotfile/init content lives.
    #[serde(default)]
    pub files: HashMap<String, String>,
}

impl Profile {
    pub fn new(name: impl Into<String>, shell: Shell) -> Self {
        Self {
            name: name.into(),
            shell,
            files: HashMap::new(),
        }
    }

    pub fn with_file(mut self, name: impl Into<String>, content: impl Into<String>) -> Self {
        self.files.insert(name.into(), content.into());
        self
    }
}

/// Manages per-user profile directories under `~/.config/term2/profiles/<user>`.
#[derive(Clone, Debug)]
pub struct ProfileRegistry {
    base_dir: PathBuf,
}

impl ProfileRegistry {
    pub fn new(user: &str) -> Self {
        let base_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("term2")
            .join("profiles")
            .join(sanitize(user));
        Self { base_dir }
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// List the built-in profiles plus any custom profiles the user created.
    pub fn list(&self) -> Vec<Profile> {
        let mut profiles = vec![
            self.bash_profile(),
            self.zsh_profile(),
            self.nushell_profile(),
        ];

        if let Ok(entries) = std::fs::read_dir(&self.base_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.is_empty() || profiles.iter().any(|p| p.name == name) {
                    continue;
                }
                if let Some(custom) = self.load_custom(name) {
                    profiles.push(custom);
                }
            }
        }

        profiles
    }

    pub fn get(&self, name: &str) -> Option<Profile> {
        match name {
            "bash" => Some(self.bash_profile()),
            "zsh" => Some(self.zsh_profile()),
            "nushell" => Some(self.nushell_profile()),
            _ => self.load_custom(name),
        }
    }

    pub fn ensure(&self, profile: &Profile) -> std::io::Result<PathBuf> {
        let dir = self.base_dir.join(&profile.name);
        std::fs::create_dir_all(&dir)?;
        for (file_name, content) in &profile.files {
            let path = dir.join(file_name);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, content)?;
        }
        Ok(dir)
    }

    /// Returns the argv and environment needed by portable-pty to launch this profile.
    /// The caller is responsible for running this inside tmux.
    pub fn launch_args(&self, profile: &Profile) -> LaunchArgs {
        let dir = self.base_dir.join(&profile.name);
        match profile.shell {
            Shell::Bash => {
                let rcfile = dir.join(".bashrc");
                let mut args = vec!["bash".to_string(), "-l".to_string()];
                if rcfile.exists() {
                    args.push("--rcfile".to_string());
                    args.push(rcfile.to_string_lossy().to_string());
                }
                LaunchArgs {
                    command: args[0].clone(),
                    args,
                    env: HashMap::new(),
                    cwd: dir,
                }
            }
            Shell::Zsh => {
                let mut env = HashMap::new();
                env.insert("ZDOTDIR".to_string(), dir.to_string_lossy().to_string());
                LaunchArgs {
                    command: "zsh".to_string(),
                    args: vec!["zsh".to_string(), "-l".to_string()],
                    env,
                    cwd: dir,
                }
            }
            Shell::Nushell => {
                let config = dir.join("config.nu");
                let env_config = dir.join("env.nu");
                let mut args = vec!["nu".to_string()];
                if config.exists() {
                    args.push("--config".to_string());
                    args.push(config.to_string_lossy().to_string());
                }
                if env_config.exists() {
                    args.push("--env-config".to_string());
                    args.push(env_config.to_string_lossy().to_string());
                }
                LaunchArgs {
                    command: "nu".to_string(),
                    args,
                    env: HashMap::new(),
                    cwd: dir,
                }
            }
        }
    }

    fn bash_profile(&self) -> Profile {
        Profile::new("bash", Shell::Bash)
    }

    fn zsh_profile(&self) -> Profile {
        let zshrc = default_zshrc();
        Profile::new("zsh", Shell::Zsh).with_file(".zshrc", zshrc)
    }

    fn nushell_profile(&self) -> Profile {
        let (config, env_config) = default_nushell_configs();
        Profile::new("nushell", Shell::Nushell)
            .with_file("config.nu", config)
            .with_file("env.nu", env_config)
    }

    fn load_custom(&self, name: &str) -> Option<Profile> {
        let dir = self.base_dir.join(name);
        if !dir.exists() {
            return None;
        }
        let mut files = HashMap::new();
        for entry in std::fs::read_dir(&dir).ok()?.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let (Some(name), Ok(content)) = (
                    path.file_name().and_then(|n| n.to_str()),
                    std::fs::read_to_string(&path),
                ) {
                    files.insert(name.to_string(), content);
                }
            }
        }
        Some(Profile {
            name: name.to_string(),
            shell: Shell::Bash,
            files,
        })
    }
}

#[derive(Debug, Clone)]
pub struct LaunchArgs {
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub cwd: PathBuf,
}

impl LaunchArgs {
    pub fn apply(&self, builder: &mut portable_pty::CommandBuilder) {
        for (k, v) in &self.env {
            builder.env(k, v);
        }
        builder.cwd(&self.cwd);
        // CommandBuilder::new takes the program; we clear and re-add args.
        // portable-pty stores argv[0] as the program and additional args separately.
        builder.arg(&self.command);
        for arg in self.args.iter().skip(1) {
            builder.arg(arg);
        }
    }
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn default_zshrc() -> String {
    r#"# Term2 default zshrc
export ZSH="/usr/share/oh-my-zsh"
ZSH_THEME="robbyrussell"
plugins=(git)
if [ -d "$ZSH" ]; then
  source "$ZSH/oh-my-zsh.sh"
else
  PROMPT='%n@%m:%~%# '
fi
"#
    .to_string()
}

fn default_nushell_configs() -> (String, String) {
    let config = r#"# Term2 default Nushell config
$env.config = {
  show_banner: false,
  edit_mode: emacs,
}
"#
    .to_string();
    let env_config = r#"# Term2 default Nushell env
$env.PROMPT_COMMAND = {|| "~> " }
"#
    .to_string();
    (config, env_config)
}
