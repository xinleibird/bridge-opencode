# bridge — AGENTS.md

## Project structure

Neovim Lua plugin + Rust CLI binary that bridges opencode and Neovim.

- `plugin/bridge.lua` — auto-executed at nvim startup, runs `serverstart('/tmp/sidekick-<pid>.sock')`
- `lua/bridge/init.lua` — plugin entry point, `require('bridge').setup()`
- `src/main.rs` — binary entry, single subcommand `bridge hook`
- `src/lib.rs` — re-exports `action`, `constants`, `handler`, `hook`, `utils` for integration tests
- `bin/README.md` — documents the build output directory

## Commands

```sh
cargo build --release && cp target/release/bridge bin/
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

- Socket discovery (`src/utils.rs`) globs `/tmp/sidekick-*.sock` — the socket name still says "sidekick", a legacy from the upstream fork. Both ends must agree on the prefix.
- `rmp = "=0.8.14"` — pinned because 0.8.15 pulls in a breaking change that clashes with `rmpv`, which `neovim-lib` depends on.
- RPC to Neovim (`src/action/neovim/`) uses `neovim-lib` over msgpack-rpc.
- lazy.nvim: `build = 'cargo build --release && cp target/release/bridge bin/'`
