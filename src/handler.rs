use std::io::{self, Read, Write};

use crate::action::{Action, neovim::NeovimAction};
use crate::hook::{self, Hook, HookEvent, HookOutput, PermissionDecision, Tool};
use crate::utils;

pub fn handle_hook() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let hook = hook::parse_hook(&input)?;

    let socket_paths = utils::find_matching_sockets().unwrap_or_default();
    let nvim_action = if socket_paths.is_empty() {
        None
    } else {
        Some(NeovimAction::new(socket_paths))
    };

    let output = match hook {
        Hook::Tool(h) => match h.hook_event_name {
            HookEvent::PreToolUse => handle_pre_tool_use(&h, nvim_action.as_ref()),
            HookEvent::PostToolUse => handle_post_tool_use(&h, nvim_action.as_ref()),
        },
        Hook::UserPrompt => handle_user_prompt_submit(nvim_action.as_ref()),
    };

    io::stdout().write_all(output.to_json()?.as_bytes())?;
    Ok(())
}

fn handle_pre_tool_use(h: &hook::ToolHook, nvim_action: Option<&NeovimAction>) -> HookOutput {
    let Some(file_path) = tool_to_mutation(&h.tool) else {
        return HookOutput::new();
    };
    check_buffer_modifications(nvim_action, file_path)
}

fn handle_post_tool_use(h: &hook::ToolHook, nvim_action: Option<&NeovimAction>) -> HookOutput {
    let Some(file_path) = tool_to_mutation(&h.tool) else {
        return HookOutput::new();
    };
    refresh_buffer(nvim_action, file_path)
}

fn handle_user_prompt_submit(nvim_action: Option<&NeovimAction>) -> HookOutput {
    let Some(action) = nvim_action else {
        return HookOutput::new();
    };

    let Ok(selections) = action.get_visual_selections() else {
        return HookOutput::new();
    };

    if selections.is_empty() {
        return HookOutput::new();
    }

    let context = selections
        .iter()
        .map(|ctx| {
            format!("[{}] {}:{}-{}", ctx.cwd, ctx.file_path, ctx.start_line, ctx.end_line)
        })
        .collect::<Vec<_>>()
        .join("\n");

    HookOutput::new().with_additional_context(context)
}

fn check_buffer_modifications(
    nvim_action: Option<&NeovimAction>,
    file_path: &str,
) -> HookOutput {
    let Some(action) = nvim_action else {
        return HookOutput::new();
    };

    let Ok(status) = action.buffer_status(file_path) else {
        return HookOutput::new();
    };

    if status.has_unsaved_changes && status.is_current {
        if let Err(e) = action.send_message("Edit blocked — file has unsaved changes") {
            eprintln!("Warning: {}", e);
        }

        HookOutput::new().with_permission_decision(
            PermissionDecision::Deny,
            Some("The file is being edited by the user, try again later".to_string()),
        )
    } else {
        HookOutput::new()
    }
}

fn refresh_buffer(nvim_action: Option<&NeovimAction>, file_path: &str) -> HookOutput {
    let Some(action) = nvim_action else {
        return HookOutput::new();
    };

    if let Err(e) = action.refresh_buffer(file_path) {
        eprintln!("Warning: {}", e);
    }

    HookOutput::new()
}

fn tool_to_mutation(tool: &Tool) -> Option<&str> {
    match tool {
        Tool::Edit(f) | Tool::Write(f) | Tool::MultiEdit(f) => Some(f.file_path.as_str()),
        _ => None,
    }
}
