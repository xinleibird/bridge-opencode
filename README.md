# bridge-opencode

Bridge between opencode and Neovim, inspired by [sidekick](https://github.com/NishantJoshi00/sidekick).

## Features

- **Buffer protection**: When you have unsaved changes in a buffer, opencode waits — edits are denied and your work is preserved
- **Auto-reload**: When opencode modifies a file you have open, the buffer is auto-reloaded with cursor position preserved
- **Visual selection context**: Visual selections in Neovim are sent to opencode as chat context

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

```json
{
  "plugin": ["@xinleibird/bridge-opencode"]
}
```

## Platforms

Currently only **macOS** and **Linux** (x64, ARM64) are supported. Windows is not supported.
