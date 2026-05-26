use crate::action::{neovim::NeovimAction, Action, BufferStatus, EditorContext};
use crate::utils;

pub fn connect() -> anyhow::Result<NeovimAction> {
    let socket_paths = utils::find_matching_sockets()?;
    if socket_paths.is_empty() {
        anyhow::bail!("no Neovim instances found");
    }
    Ok(NeovimAction::new(socket_paths))
}

pub fn check_buffer(action: &NeovimAction, file_path: &str) -> anyhow::Result<BufferStatus> {
    action.buffer_status(file_path)
}

pub fn refresh_buffer(action: &NeovimAction, file_path: &str) -> anyhow::Result<()> {
    action.refresh_buffer(file_path)
}

pub fn get_visual_selections(action: &NeovimAction) -> anyhow::Result<Vec<EditorContext>> {
    action.get_visual_selections()
}

pub fn send_message(action: &NeovimAction, message: &str, level: &str) -> anyhow::Result<()> {
    action.send_message(message, level)
}
