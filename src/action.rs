pub mod neovim;

#[derive(Debug, Clone)]
pub struct BufferStatus {
    pub is_current: bool,
    pub has_unsaved_changes: bool,
}

#[derive(Debug, Clone)]
pub struct EditorContext {
    pub file_path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub cwd: String,
    pub content: String,
}

pub trait Action {
    fn buffer_status(&self, file_path: &str) -> anyhow::Result<BufferStatus>;
    fn refresh_buffer(&self, file_path: &str) -> anyhow::Result<()>;
    fn send_message(&self, message: &str, level: &str) -> anyhow::Result<()>;
    fn get_visual_selections(&self) -> anyhow::Result<Vec<EditorContext>>;
}
