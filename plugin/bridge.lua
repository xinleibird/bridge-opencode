local pid = vim.fn.getpid()
local socket_path = "/tmp/bridge-" .. pid .. ".sock"
vim.fn.serverstart(socket_path)
