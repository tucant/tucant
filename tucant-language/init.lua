--  nvim -u init.lua README.tucant
-- vim.lsp.set_log_level("TRACE")

-- https://github.com/neovim/neovim/pull/15723

-- https://github.com/theHamsta/nvim-semantic-tokens

local function on_attach(client, bufnr)
  -- Find the clients capabilities
  local cap = client.resolved_capabilities

  if cap.document_highlight then
      vim.cmd('augroup LspHighlight')
      vim.cmd('autocmd!')
      vim.cmd('autocmd <buffer> CursorHold lua vim.lsp.buf.document_highlight()')
      vim.cmd('autocmd <buffer> CursorMoved lua vim.lsp.buf.clear_references()')
      vim.cmd('augroup END')
  else 
    vim.cmd("dsfsdf")
  end 
end

vim.lsp.start({ name = 'TUCaNt Language Server', cmd = vim.lsp.rpc.connect('127.0.0.1', 6008), on_attach = on_attach })


-- :lua vim.lsp.buf.document_highlight()

-- https://neovim.io/doc/user/cmdline.html
-- https://sbulav.github.io/til/til-neovim-highlight-references/

--[[
vim.lsp.start({
  name = 'TUCaNt Language Server',
  cmd = {'/home/moritz/Documents/tucant/tucant-vscode-language-client/tucant-language-server/target/debug/tucant-language-server'},
  root_dir = vim.fs.dirname(vim.fs.find({'init.lua'}, { upward = true })[1]),
})
]]
