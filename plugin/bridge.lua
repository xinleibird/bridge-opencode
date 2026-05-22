local pid = vim.fn.getpid()
local socket_path = "/tmp/sidekick-" .. pid .. ".sock"
vim.fn.serverstart(socket_path)
