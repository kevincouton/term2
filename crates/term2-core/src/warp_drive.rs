//! Warp Drive objects: Workflows, Notebooks, Prompts, and Environment Variables.
//!
//! Warp Drive is a built-in workspace for saving and sharing reusable objects
//! across a team.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Workflows
// ---------------------------------------------------------------------------

/// A reusable parameterized command workflow.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Workflow {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub shells: Vec<String>,
    #[serde(default)]
    pub arguments: Vec<WorkflowArgument>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowArgument {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub arg_type: ArgumentType,
    #[serde(default)]
    pub enum_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArgumentType {
    #[default]
    Text,
    Enum,
}

impl Workflow {
    pub fn from_yaml(yaml: &str) -> Result<Self, String> {
        serde_yaml::from_str(yaml).map_err(|e| e.to_string())
    }

    pub fn to_yaml(&self) -> Result<String, String> {
        serde_yaml::to_string(self).map_err(|e| e.to_string())
    }

    /// Extract argument names from the command using `{{name}}` placeholders.
    pub fn argument_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let mut remaining = self.command.as_str();
        while let Some(start) = remaining.find("{{") {
            if let Some(end) = remaining[start + 2..].find("}}") {
                let name = &remaining[start + 2..start + 2 + end];
                if !name.is_empty() && !names.contains(&name.to_string()) {
                    names.push(name.to_string());
                }
                remaining = &remaining[start + 4 + end..];
            } else {
                break;
            }
        }
        names
    }

    /// Substitute arguments into the command template.
    pub fn render(&self, values: &HashMap<String, String>) -> Result<String, String> {
        let mut command = self.command.clone();
        for arg in &self.arguments {
            let placeholder = format!("{{{{{}}}}}", arg.name);
            let value = values
                .get(&arg.name)
                .cloned()
                .or_else(|| arg.default_value.clone())
                .ok_or_else(|| format!("missing argument: {}", arg.name))?;
            command = command.replace(&placeholder, &value);
        }
        // Ensure all placeholders were filled.
        if command.contains("{{") {
            return Err("unfilled argument placeholders remain".to_string());
        }
        Ok(command)
    }
}

// ---------------------------------------------------------------------------
// Notebooks
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Notebook {
    pub title: String,
    #[serde(default)]
    pub cells: Vec<NotebookCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum NotebookCell {
    Text {
        content: String,
    },
    Heading {
        level: u8,
        content: String,
    },
    List {
        items: Vec<String>,
    },
    Code {
        language: String,
        content: String,
    },
    Command {
        content: String,
        arguments: Vec<WorkflowArgument>,
    },
}

impl Notebook {
    pub fn command_cells(&self) -> Vec<&NotebookCell> {
        self.cells
            .iter()
            .filter(|c| matches!(c, NotebookCell::Command { .. }))
            .collect()
    }

    pub fn render_markdown(&self) -> String {
        let mut md = format!("# {}\n\n", self.title);
        for cell in &self.cells {
            match cell {
                NotebookCell::Text { content } => {
                    md.push_str(content);
                    md.push_str("\n\n");
                }
                NotebookCell::Heading { level, content } => {
                    md.push_str(&"#".repeat((*level).min(6) as usize));
                    md.push(' ');
                    md.push_str(content);
                    md.push_str("\n\n");
                }
                NotebookCell::List { items } => {
                    for item in items {
                        md.push_str(&format!("- {}\n", item));
                    }
                    md.push('\n');
                }
                NotebookCell::Code { language, content } => {
                    md.push_str(&format!("```{language}\n{content}\n```\n\n"));
                }
                NotebookCell::Command { content, .. } => {
                    md.push_str(&format!("```shell\n{content}\n```\n\n"));
                }
            }
        }
        md
    }
}

// ---------------------------------------------------------------------------
// Prompts
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Prompt {
    pub name: String,
    pub query: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<WorkflowArgument>,
}

impl Prompt {
    /// Validate prompt argument names: A-Za-z0-9, hyphens, underscores; first char not a digit.
    pub fn validate_arguments(&self) -> Result<(), String> {
        for arg in &self.arguments {
            let mut chars = arg.name.chars();
            let first = chars
                .next()
                .ok_or_else(|| "empty argument name".to_string())?;
            if first.is_ascii_digit() {
                return Err(format!("argument '{}' cannot start with a digit", arg.name));
            }
            if !arg
                .name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                return Err(format!(
                    "argument '{}' contains invalid characters",
                    arg.name
                ));
            }
        }
        Ok(())
    }

    pub fn render(&self, values: &HashMap<String, String>) -> Result<String, String> {
        let mut query = self.query.clone();
        for arg in &self.arguments {
            let placeholder = format!("{{{{{}}}}}", arg.name);
            let value = values
                .get(&arg.name)
                .cloned()
                .or_else(|| arg.default_value.clone())
                .ok_or_else(|| format!("missing argument: {}", arg.name))?;
            query = query.replace(&placeholder, &value);
        }
        if query.contains("{{") {
            return Err("unfilled argument placeholders remain".to_string());
        }
        Ok(query)
    }
}

// ---------------------------------------------------------------------------
// Environment Variables
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvVarSet {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub variables: Vec<EnvVar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum EnvVar {
    Static { key: String, value: String },
    Dynamic { key: String, source: DynamicSource },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DynamicSource {
    OnePassword { reference: String },
    LastPass { reference: String },
    Command { command: String },
}

impl EnvVarSet {
    /// Return the static values as a map (dynamic values are not resolved here).
    pub fn static_values(&self) -> HashMap<String, String> {
        self.variables
            .iter()
            .filter_map(|v| match v {
                EnvVar::Static { key, value } => Some((key.clone(), value.clone())),
                _ => None,
            })
            .collect()
    }

    pub fn export_dotenv(&self) -> String {
        let mut out = String::new();
        for v in &self.variables {
            if let EnvVar::Static { key, value } = v {
                out.push_str(&format!("{}={}\n", key, value));
            }
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Drive store
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WarpDrive {
    workflows: HashMap<String, Workflow>,
    notebooks: HashMap<String, Notebook>,
    prompts: HashMap<String, Prompt>,
    env_var_sets: HashMap<String, EnvVarSet>,
}

impl WarpDrive {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_workflow(&mut self, workflow: Workflow) {
        self.workflows.insert(workflow.name.clone(), workflow);
    }

    pub fn add_notebook(&mut self, notebook: Notebook) {
        self.notebooks.insert(notebook.title.clone(), notebook);
    }

    pub fn add_prompt(&mut self, prompt: Prompt) {
        self.prompts.insert(prompt.name.clone(), prompt);
    }

    pub fn add_env_var_set(&mut self, set: EnvVarSet) {
        self.env_var_sets.insert(set.name.clone(), set);
    }

    pub fn workflows(&self) -> &HashMap<String, Workflow> {
        &self.workflows
    }

    pub fn notebooks(&self) -> &HashMap<String, Notebook> {
        &self.notebooks
    }

    pub fn prompts(&self) -> &HashMap<String, Prompt> {
        &self.prompts
    }

    pub fn env_var_sets(&self) -> &HashMap<String, EnvVarSet> {
        &self.env_var_sets
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let q = query.to_lowercase();
        let mut results = Vec::new();
        for (name, wf) in &self.workflows {
            if name.to_lowercase().contains(&q)
                || wf.tags.iter().any(|t| t.to_lowercase().contains(&q))
            {
                results.push(format!("workflow:{}", name));
            }
        }
        for name in self.notebooks.keys() {
            if name.to_lowercase().contains(&q) {
                results.push(format!("notebook:{}", name));
            }
        }
        for (name, p) in &self.prompts {
            if name.to_lowercase().contains(&q)
                || p.description
                    .as_ref()
                    .map(|d| d.to_lowercase())
                    .unwrap_or_default()
                    .contains(&q)
            {
                results.push(format!("prompt:{}", name));
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_parse_yaml() {
        let yaml = r#"
name: Deploy
description: Deploy the service
command: kubectl set image deployment/{{deployment}} {{container}}={{image}}
tags: ["k8s", "deploy"]
arguments:
  - name: deployment
    description: Deployment name
    default_value: api
  - name: container
    default_value: app
  - name: image
"#;
        let wf = Workflow::from_yaml(yaml).unwrap();
        assert_eq!(wf.name, "Deploy");
        assert_eq!(wf.tags, vec!["k8s", "deploy"]);
        assert_eq!(wf.arguments.len(), 3);
    }

    #[test]
    fn workflow_extract_argument_names() {
        let wf = Workflow {
            name: "Example".into(),
            command: "for {{variable}} in {{sequence}}; do {{command}}; done".into(),
            tags: vec![],
            description: None,
            shells: vec![],
            arguments: vec![],
        };
        let names = wf.argument_names();
        assert_eq!(names, vec!["variable", "sequence", "command"]);
    }

    #[test]
    fn workflow_render_substitutes_arguments() {
        let yaml = r#"
name: Echo
description: Echo a string
command: echo {{string}}
arguments:
  - name: string
    default_value: hello
"#;
        let wf = Workflow::from_yaml(yaml).unwrap();
        let rendered = wf.render(&HashMap::new()).unwrap();
        assert_eq!(rendered, "echo hello");

        let mut values = HashMap::new();
        values.insert("string".to_string(), "world".to_string());
        let rendered = wf.render(&values).unwrap();
        assert_eq!(rendered, "echo world");
    }

    #[test]
    fn workflow_render_errors_on_missing_argument() {
        let wf = Workflow {
            name: "Test".into(),
            command: "echo {{missing}}".into(),
            tags: vec![],
            description: None,
            shells: vec![],
            arguments: vec![WorkflowArgument {
                name: "missing".into(),
                description: None,
                default_value: None,
                arg_type: ArgumentType::Text,
                enum_values: None,
            }],
        };
        assert!(wf.render(&HashMap::new()).is_err());
    }

    #[test]
    fn notebook_render_markdown() {
        let notebook = Notebook {
            title: "Onboarding".into(),
            cells: vec![
                NotebookCell::Heading {
                    level: 2,
                    content: "Setup".into(),
                },
                NotebookCell::Text {
                    content: "Run the commands below.".into(),
                },
                NotebookCell::Command {
                    content: "cargo build".into(),
                    arguments: vec![],
                },
            ],
        };
        let md = notebook.render_markdown();
        assert!(md.contains("# Onboarding"));
        assert!(md.contains("## Setup"));
        assert!(md.contains("```shell\ncargo build\n```"));
    }

    #[test]
    fn notebook_command_cells() {
        let notebook = Notebook {
            title: "Runbook".into(),
            cells: vec![
                NotebookCell::Text {
                    content: "Intro".into(),
                },
                NotebookCell::Command {
                    content: "cargo test".into(),
                    arguments: vec![],
                },
            ],
        };
        assert_eq!(notebook.command_cells().len(), 1);
    }

    #[test]
    fn prompt_argument_validation() {
        let good = Prompt {
            name: "PR".into(),
            query: "Review {{branch}}".into(),
            description: None,
            arguments: vec![WorkflowArgument {
                name: "branch".into(),
                description: None,
                default_value: None,
                arg_type: ArgumentType::Text,
                enum_values: None,
            }],
        };
        assert!(good.validate_arguments().is_ok());

        let bad_digit = Prompt {
            name: "Bad".into(),
            query: "Do {{1bad}}".into(),
            description: None,
            arguments: vec![WorkflowArgument {
                name: "1bad".into(),
                description: None,
                default_value: None,
                arg_type: ArgumentType::Text,
                enum_values: None,
            }],
        };
        assert!(bad_digit.validate_arguments().is_err());

        let bad_char = Prompt {
            name: "Bad".into(),
            query: "Do {{bad name}}".into(),
            description: None,
            arguments: vec![WorkflowArgument {
                name: "bad name".into(),
                description: None,
                default_value: None,
                arg_type: ArgumentType::Text,
                enum_values: None,
            }],
        };
        assert!(bad_char.validate_arguments().is_err());
    }

    #[test]
    fn prompt_render() {
        let prompt = Prompt {
            name: "Summarize".into(),
            query: "Summarize PR {{pr_number}}".into(),
            description: None,
            arguments: vec![WorkflowArgument {
                name: "pr_number".into(),
                description: None,
                default_value: None,
                arg_type: ArgumentType::Text,
                enum_values: None,
            }],
        };
        let mut values = HashMap::new();
        values.insert("pr_number".to_string(), "42".to_string());
        assert_eq!(prompt.render(&values).unwrap(), "Summarize PR 42");
    }

    #[test]
    fn env_var_set_static_and_dynamic() {
        let set = EnvVarSet {
            name: "Project".into(),
            description: None,
            variables: vec![
                EnvVar::Static {
                    key: "API_URL".into(),
                    value: "https://api.example.com".into(),
                },
                EnvVar::Dynamic {
                    key: "API_KEY".into(),
                    source: DynamicSource::OnePassword {
                        reference: "op://vault/item/field".into(),
                    },
                },
            ],
        };
        let statics = set.static_values();
        assert_eq!(
            statics.get("API_URL"),
            Some(&"https://api.example.com".to_string())
        );
        assert!(!statics.contains_key("API_KEY"));
        assert!(set.export_dotenv().contains("API_URL="));
        assert!(!set.export_dotenv().contains("API_KEY"));
    }

    #[test]
    fn drive_search_covers_workflows_notebooks_prompts() {
        let mut drive = WarpDrive::new();
        drive.add_workflow(Workflow {
            name: "Deploy".into(),
            command: "kubectl apply".into(),
            tags: vec!["k8s".into()],
            description: None,
            shells: vec![],
            arguments: vec![],
        });
        drive.add_notebook(Notebook {
            title: "Onboarding".into(),
            cells: vec![],
        });
        drive.add_prompt(Prompt {
            name: "Review PR".into(),
            query: "Review".into(),
            description: Some("PR review prompt".into()),
            arguments: vec![],
        });

        let results = drive.search("deploy");
        assert!(results.iter().any(|r| r == "workflow:Deploy"));

        let results = drive.search("onboarding");
        assert!(results.iter().any(|r| r == "notebook:Onboarding"));

        let results = drive.search("review");
        assert!(results.iter().any(|r| r == "prompt:Review PR"));
    }

    // Warp-derived scenarios:

    #[test]
    fn workflows_are_parameterized_and_searchable() {
        // Workflows are easily parameterized and searchable by name, description,
        // or command arguments.
        let mut drive = WarpDrive::new();
        let wf = Workflow::from_yaml(
            r#"
name: Git pull
command: git pull origin {{branch}}
arguments:
  - name: branch
    default_value: main
"#,
        )
        .unwrap();
        drive.add_workflow(wf);
        assert!(drive.search("git").iter().any(|r| r == "workflow:Git pull"));
    }

    #[test]
    fn notebooks_combine_markdown_and_runnable_commands() {
        // Notebooks combine markdown text with executable shell command blocks.
        let nb = Notebook {
            title: "Runbook".into(),
            cells: vec![
                NotebookCell::Text {
                    content: "Build the project.".into(),
                },
                NotebookCell::Command {
                    content: "cargo build".into(),
                    arguments: vec![],
                },
            ],
        };
        assert_eq!(nb.command_cells().len(), 1);
    }

    #[test]
    fn prompts_are_reusable_parameterized_queries() {
        // Prompts are parameterized natural language queries saved for reuse.
        let prompt = Prompt {
            name: "Commit".into(),
            query: "Write a commit message for {{files}}".into(),
            description: None,
            arguments: vec![WorkflowArgument {
                name: "files".into(),
                description: None,
                default_value: None,
                arg_type: ArgumentType::Text,
                enum_values: None,
            }],
        };
        let mut values = HashMap::new();
        values.insert("files".to_string(), "src/main.rs".to_string());
        assert!(prompt.render(&values).unwrap().contains("src/main.rs"));
    }

    #[test]
    fn env_vars_load_with_click() {
        // Static env vars can be exported to a dotenv-compatible format.
        let set = EnvVarSet {
            name: "Project".into(),
            description: None,
            variables: vec![EnvVar::Static {
                key: "FOO".into(),
                value: "bar".into(),
            }],
        };
        assert!(set.export_dotenv().contains("FOO=bar"));
    }

    #[test]
    #[ignore = "team sync not yet implemented"]
    fn drive_objects_sync_in_real_time() {
        // Objects sync immediately as they are updated.
    }

    #[test]
    #[ignore = "permissions not yet implemented"]
    fn drive_permissions_are_inherited_from_parent_folders() {
        // Permissions are inherited from parent folders.
    }
}
