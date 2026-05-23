pub mod action;
pub mod constants;
pub mod handler;
pub mod utils;

#[cfg(feature = "napi")]
mod bindings {
    use crate::action::neovim::NeovimAction;
    use crate::action::EditorContext as InternalEditorContext;
    use crate::handler;
    use napi::bindgen_prelude::*;
    use napi_derive::napi;

    fn connect_nvim() -> Result<NeovimAction> {
        handler::connect().map_err(|e| Error::from_reason(e.to_string()))
    }

    fn with_action<F, T>(f: F) -> Result<T>
    where
        F: FnOnce(&NeovimAction) -> anyhow::Result<T>,
    {
        let action = connect_nvim()?;
        f(&action).map_err(|e| Error::from_reason(e.to_string()))
    }

    #[napi(object)]
    #[derive(Clone)]
    pub struct BufferStatus {
        pub is_current: bool,
        pub has_unsaved_changes: bool,
    }

    impl From<crate::action::BufferStatus> for BufferStatus {
        fn from(s: crate::action::BufferStatus) -> Self {
            Self {
                is_current: s.is_current,
                has_unsaved_changes: s.has_unsaved_changes,
            }
        }
    }

    #[napi(object)]
    #[derive(Clone)]
    pub struct EditorContext {
        pub file_path: String,
        pub start_line: u32,
        pub end_line: u32,
        pub cwd: String,
        pub content: String,
    }

    impl From<InternalEditorContext> for EditorContext {
        fn from(ctx: InternalEditorContext) -> Self {
            Self {
                file_path: ctx.file_path,
                start_line: ctx.start_line,
                end_line: ctx.end_line,
                cwd: ctx.cwd,
                content: ctx.content,
            }
        }
    }

    #[napi]
    pub fn check_buffer(file_path: String) -> Result<BufferStatus> {
        with_action(|action| {
            handler::check_buffer(action, &file_path).map(BufferStatus::from)
        })
    }

    #[napi]
    pub fn refresh_buffer(file_path: String) -> Result<()> {
        with_action(|action| handler::refresh_buffer(action, &file_path))
    }

    #[napi]
    pub fn get_visual_selections() -> Result<Vec<EditorContext>> {
        with_action(|action| {
            handler::get_visual_selections(action)
                .map(|v| v.into_iter().map(EditorContext::from).collect())
        })
    }

    #[napi]
    pub fn send_message(message: String) -> Result<()> {
        with_action(|action| handler::send_message(action, &message))
    }
}
