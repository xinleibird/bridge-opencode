# bridge-opencode — AGENTS.md

## Project structure

Neovim Lua plugin + napi-rs Rust native addon that bridges opencode and Neovim.

- `plugin/bridge.lua` — auto-executed at nvim startup, runs `serverstart('/tmp/bridge-<pid>.sock')`
- `bridge.ts` — opencode plugin entry point (package main)
- `src/lib.rs` — re-exports `action`, `constants`, `handler`, `utils`; exports `#[napi]` functions
- `src/handler.rs` — action wrappers for Neovim RPC calls

## Commands

```sh
cargo build --release    # builds napi addon (also via `npm run build`)
cargo test               # tests without napi (uses --no-default-features internally)
cargo check              # lightweight check without napi
cargo check-all          # full check with napi (alias for check --features napi)
npm run build            # napi build --platform --release
```

## Architecture

- `rmp = "=0.8.14"` — pinned because 0.8.15 pulls in a breaking change that clashes with `rmpv`, which `neovim-lib` depends on.
- RPC to Neovim (`src/action/neovim/`) uses `neovim-lib` over msgpack-rpc.
- opencode loads `bridge-opencode` via `opencode.json`: `"plugin": ["bridge-opencode"]`
- napi features are behind `napi` feature gate to allow `cargo test` without Node.js linkage.
