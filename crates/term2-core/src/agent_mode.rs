//! Agent mode and agent conversation model.
//!
//! Warp supports Terminal mode and Agent conversation view for multi-turn agent
//! workflows.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputMode {
    Terminal,
    Agent,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Idle,
    Working,
    Blocked,
    Completed,
    Errored,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentMessage {
    pub role: String,
    pub content: String,
    #[serde(default)]
    pub attached_block_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentConversation {
    pub id: String,
    pub status: AgentStatus,
    pub messages: Vec<AgentMessage>,
    #[serde(default)]
    pub context: HashMap<String, String>,
}

impl AgentConversation {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: AgentStatus::Idle,
            messages: Vec::new(),
            context: HashMap::new(),
        }
    }

    pub fn add_message(&mut self, role: impl Into<String>, content: impl Into<String>) {
        self.messages.push(AgentMessage {
            role: role.into(),
            content: content.into(),
            attached_block_ids: Vec::new(),
        });
    }

    pub fn attach_block(&mut self, block_id: impl Into<String>) {
        let block_id = block_id.into();
        if let Some(last) = self.messages.last_mut() {
            if !last.attached_block_ids.contains(&block_id) {
                last.attached_block_ids.push(block_id);
            }
        }
    }

    pub fn set_status(&mut self, status: AgentStatus) {
        self.status = status;
    }
}

/// Slash commands available in agent mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlashCommand {
    pub name: String,
    pub description: String,
    pub action: String,
}

pub fn default_slash_commands() -> Vec<SlashCommand> {
    vec![
        SlashCommand {
            name: "/new".to_string(),
            description: "Start a new conversation".to_string(),
            action: "conversation:new".to_string(),
        },
        SlashCommand {
            name: "/agent".to_string(),
            description: "Enter agent mode".to_string(),
            action: "mode:agent".to_string(),
        },
        SlashCommand {
            name: "/plan".to_string(),
            description: "Create a plan for a complex task".to_string(),
            action: "agent:plan".to_string(),
        },
        SlashCommand {
            name: "/fork".to_string(),
            description: "Fork the current conversation".to_string(),
            action: "conversation:fork".to_string(),
        },
        SlashCommand {
            name: "/queue".to_string(),
            description: "Queue a prompt".to_string(),
            action: "prompt:queue".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_stores_messages() {
        let mut conv = AgentConversation::new("conv-1");
        conv.add_message("user", "fix the build");
        conv.add_message("agent", "I'll investigate.");
        assert_eq!(conv.messages.len(), 2);
        assert_eq!(conv.messages[0].role, "user");
    }

    #[test]
    fn attach_block_to_last_message() {
        let mut conv = AgentConversation::new("conv-1");
        conv.add_message("user", "explain this error");
        conv.attach_block("block-123");
        assert_eq!(conv.messages[0].attached_block_ids, vec!["block-123"]);
    }

    #[test]
    fn status_transitions() {
        let mut conv = AgentConversation::new("conv-1");
        assert_eq!(conv.status, AgentStatus::Idle);
        conv.set_status(AgentStatus::Working);
        conv.set_status(AgentStatus::Completed);
        assert_eq!(conv.status, AgentStatus::Completed);
    }

    #[test]
    fn default_slash_commands_include_essentials() {
        let commands = default_slash_commands();
        let names: Vec<_> = commands.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"/new"));
        assert!(names.contains(&"/plan"));
        assert!(names.contains(&"/fork"));
        assert!(names.contains(&"/queue"));
    }

    // Warp-derived scenarios:

    #[test]
    fn terminal_and_agent_modes_are_distinct() {
        // Terminal mode runs shell commands; Agent mode is for multi-turn workflows.
        assert_ne!(InputMode::Terminal, InputMode::Agent);
    }

    #[test]
    fn agent_conversation_blocks_are_separate_from_terminal_blocks() {
        // Agent conversation blocks live alongside but separate from terminal blocks.
        let conv = AgentConversation::new("conv-1");
        assert!(conv.messages.is_empty());
    }

    #[test]
    #[ignore = "auto-detection not yet implemented"]
    fn auto_mode_routes_input_to_terminal_or_agent() {
        // Auto-detection classifies input as shell command or agent prompt.
    }

    #[test]
    #[ignore = "conversation forking not yet implemented"]
    fn conversation_can_be_forked() {
        // /fork creates a new conversation branch from the current point.
    }

    #[test]
    #[ignore = "prompt queueing not yet implemented"]
    fn prompts_can_be_queued_while_agent_works() {
        // /queue queues a prompt while the agent is busy.
    }
}
