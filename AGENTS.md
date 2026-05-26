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

## CI/CD

**GitHub Actions** (`.github/workflows/`):

- **CI** (`ci.yml`) — on push to `main` and PRs:
  - `verify` job: `cargo check` + `cargo test --no-default-features` on Ubuntu
  - `build` job: builds napi addon for all 4 targets (macOS aarch64/x86_64, Linux x86_64/arm64), uploads `.node` artifacts (retention: 5 days)

- **Release** (`release.yml`) — on tag push (`v*`):
  - `verify-clean` job: ensures working tree is clean
  - `build` job: same matrix as CI, uploads `.node` + `index.cjs` + `index.d.ts`
  - `publish` job: downloads artifacts, runs `npm publish`

**No manual `npm publish`** — release is fully automated via GitHub Actions on tag push.

## Release workflow

```sh
# 1. bump version in package.json (e.g. 0.2.1 → 0.2.2)

# 2. stage all changes
git add .

# 3. call conventional-commit tool generate commit message

# 4. commit with conventional-commit message
git commit

# 5. tag matching the new version
git tag v0.2.2

# 6. push commit and tag
git push origin main
git push origin v0.2.2
```

GitHub Actions `release.yml` automatically builds napi addon for all 4 targets and runs `npm publish`. No manual publish needed.

## Architecture

- `rmp = "=0.8.14"` — pinned because 0.8.15 pulls in a breaking change that clashes with `rmpv`, which `neovim-lib` depends on.
- RPC to Neovim (`src/action/neovim/`) uses `neovim-lib` over msgpack-rpc.
- opencode loads `bridge-opencode` via `opencode.json`: `"plugin": ["bridge-opencode"]`
- napi features are behind `napi` feature gate to allow `cargo test` without Node.js linkage.
