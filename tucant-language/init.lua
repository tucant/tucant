vim.lsp.set_log_level("TRACE")
vim.lsp.start({ name = 'TUCaNt Language Server', cmd = vim.lsp.rpc.connect('127.0.0.1', 6008) })
--[[
vim.lsp.start({
  name = 'TUCaNt Language Server',
  cmd = {'/home/moritz/Documents/tucant/tucant-vscode-language-client/tucant-language-server/target/debug/tucant-language-server'},
  root_dir = vim.fs.dirname(vim.fs.find({'.nvimrc'}, { upward = true })[1]),
})
]]