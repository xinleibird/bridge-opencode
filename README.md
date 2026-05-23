# bridge

Bridge between opencode and Neovim.

Inspired by / forked from [sidekick](https://github.com/NishantJoshi00/sidekick) — thanks [@NishantJoshi00](https://github.com/NishantJoshi00).

## Install with lazy.nvim

```lua
---@module "lazy"
---@type LazySpec
local M = {
  "xinleibird/bridge.opencode",
  -- enabled = false,
  priority = 1000,
  lazy = false,
  build = function(plugin)
    local plugin_dir = plugin.dir or (vim.fn.stdpath("data") .. "/lazy/bridge.opencode")
    local opencode_home = vim.fn.expand("~/.config/opencode")

    vim.fn.mkdir(opencode_home .. "/bin", "p")
    vim.fn.mkdir(opencode_home .. "/plugins", "p")

    vim.fn.system({ "cargo", "build", "--release", "--manifest-path", plugin_dir .. "/Cargo.toml" })
    vim.fn.system({ "cp", plugin_dir .. "/target/release/bridge", plugin_dir .. "/bin" })

    local bin_target = opencode_home .. "/bin/bridge"
    local plugin_target = opencode_home .. "/plugins/bridge.ts"
    vim.fn.delete(bin_target)
    vim.fn.delete(plugin_target)

    vim.fn.system({ "ln", "-sf", plugin_dir .. "/bin/bridge", bin_target })
    vim.fn.system({ "ln", "-sf", plugin_dir .. "/opencode/bridge.ts", plugin_target })
  end,
}

return M
```

The RPC socket starts automatically when Neovim launches — no manual setup required.
