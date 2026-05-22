use anyhow::Context;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HookEvent {
    PreToolUse,
    PostToolUse,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolHook {
    pub session_id: String,
    pub transcript_path: String,
    pub cwd: String,
    pub hook_event_name: HookEvent,
    #[serde(flatten)]
    pub tool: Tool,
}

#[derive(Debug)]
pub enum Hook {
    Tool(ToolHook),
    UserPrompt,
}

#[non_exhaustive]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "tool_name", content = "tool_input")]
pub enum Tool {
    Read(FileToolInput),
    Write(FileToolInput),
    Edit(FileToolInput),
    MultiEdit(FileToolInput),
    Bash(BashToolInput),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileToolInput {
    pub file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_string: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BashToolInput {
    pub command: String,
    pub description: String,
}

pub fn parse_hook(input: &str) -> anyhow::Result<Hook> {
    let value: serde_json::Value =
        serde_json::from_str(input).context("couldn't parse hook input")?;
    let event_name = value
        .get("hook_event_name")
        .and_then(|v| v.as_str())
        .context("hook is missing event name")?;

    match event_name {
        "UserPromptSubmit" => Ok(Hook::UserPrompt),
        "PreToolUse" | "PostToolUse" => {
            let hook: ToolHook =
                serde_json::from_str(input).context("unrecognized tool in hook")?;
            Ok(Hook::Tool(hook))
        }
        _ => {
            anyhow::bail!("unrecognized hook event")
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionDecision {
    Allow,
    Deny,
    Ask,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookSpecificOutput {
    pub hook_event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_decision: Option<PermissionDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_decision_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_context: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_specific_output: Option<HookSpecificOutput>,
}

impl HookOutput {
    pub fn new() -> Self {
        Self {
            hook_specific_output: None,
        }
    }

    pub fn with_permission_decision(
        mut self,
        decision: PermissionDecision,
        reason: Option<String>,
    ) -> Self {
        self.hook_specific_output = Some(HookSpecificOutput {
            hook_event_name: "PreToolUse".to_string(),
            permission_decision: Some(decision),
            permission_decision_reason: reason,
            additional_context: None,
        });
        self
    }

    pub fn with_additional_context(mut self, context: impl Into<String>) -> Self {
        self.hook_specific_output = Some(HookSpecificOutput {
            hook_event_name: "UserPromptSubmit".to_string(),
            permission_decision: None,
            permission_decision_reason: None,
            additional_context: Some(context.into()),
        });
        self
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        serde_json::to_string(self).context("couldn't serialize hook output")
    }

    #[allow(dead_code)]
    pub fn to_json_pretty(&self) -> anyhow::Result<String> {
        serde_json::to_string_pretty(self).context("couldn't serialize hook output")
    }
}

impl Default for HookOutput {
    fn default() -> Self {
        Self::new()
    }
}
