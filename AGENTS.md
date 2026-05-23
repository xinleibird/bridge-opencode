# bridge — AGENTS.md

## Project structure

Neovim Lua plugin + Rust CLI binary that bridges opencode and Neovim.

- `plugin/bridge.lua` — auto-executed at nvim startup, runs `serverstart('/tmp/bridge-<pid>.sock')`
- `src/main.rs` — binary entry, single subcommand `bridge hook`
- `src/lib.rs` — re-exports `action`, `constants`, `handler`, `hook`, `utils` for integration tests
- `bin/README.md` — documents the build output directory

## Commands

```sh
cargo build --release
cargo test
cargo check
```

## Protocol

`bridge hook` reads JSON from stdin and writes JSON to stdout, following the Claude Code hook protocol:

```json
// PreToolUse / PostToolUse
{"hook_event_name":"PreToolUse","tool_name":"Edit","tool_input":{"file_path":"..."}}
// UserPromptSubmit
{"hook_event_name":"UserPromptSubmit","prompt":"..."}
```

Response: `{ hookSpecificOutput: { permissionDecision, permissionDecisionReason, additionalContext } }`.

## Architecture notes

- `rmp = "=0.8.14"` — pinned because 0.8.15 pulls in a breaking change that clashes with `rmpv`, which `neovim-lib` depends on.
- RPC to Neovim (`src/action/neovim/`) uses `neovim-lib` over msgpack-rpc.
- lazy.nvim: `build` function compiles binary and symlinks into `~/.config/opencode/{bin,plugins}/`
