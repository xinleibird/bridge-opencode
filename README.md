# bridge

Bridge between opencode and Neovim.

Inspired by / forked from [sidekick](https://github.com/NishantJoshi00/sidekick) — thanks [@NishantJoshi00](https://github.com/NishantJoshi00).

## Install with lazy.nvim

```lua
{
  'xinleibird/bridge.opencode',
  priority = 1000,
  lazy = false,
  build = function()
    vim.fn.system('cargo build --release && cp target/release/bridge bin/')
    local opencode_home = vim.fn.expand('~/.config/opencode')
    vim.fn.mkdir(opencode_home .. '/bin', 'p')
    vim.fn.mkdir(opencode_home .. '/plugins', 'p')
    vim.fn.system({ 'ln', '-sf', vim.fn.getcwd() .. '/bin/bridge', opencode_home .. '/bin/bridge' })
    vim.fn.system({ 'ln', '-sf', vim.fn.getcwd() .. '/opencode/bridge.ts', opencode_home .. '/plugins/bridge.ts' })
  end,
}
```

The RPC socket starts automatically when Neovim launches — no manual setup required.
