# bridge-opencode

Bridge between opencode and Neovim.

Inspired by / forked from [sidekick](https://github.com/NishantJoshi00/sidekick) — thanks [@NishantJoshi00](https://github.com/NishantJoshi00).

## Structure

- `plugin/bridge.lua` — Neovim plugin, starts msgpack-rpc socket
- `bridge.ts` — opencode plugin entry (npm package main)
- `src/` — Rust source (napi-rs native addon)

## Install

### Neovim plugin (lazy.nvim)

```lua
{
  "xinleibird/bridge-opencode",
  priority = 1000,
  lazy = false,
}
```

`plugin/bridge.lua` auto-starts the RPC socket when Neovim launches.

### opencode plugin

```sh
cd ~/.config/opencode && npm install @xinleibird/bridge-opencode
```

```json
{
  "plugin": ["@xinleibird/bridge-opencode"]
}
```

The `postinstall` script compiles the Rust native addon via napi-rs.
