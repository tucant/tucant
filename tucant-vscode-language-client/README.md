```
ln --symbolic --verbose --force $PWD ~/.vscode-insiders/extensions/tucant-vscode-language-client
```

https://code.visualstudio.com/api/language-extensions/language-server-extension-guide

https://neovim.io/doc/user/lsp.html
https://neovim.io/doc/user/lsp.html#vim.lsp.start_client()

```
 nvim -u init.lua README.tucant

nvim
:luafile init.lua
:lua =vim.lsp.get_log_path()
:lua =vim.lsp.set_log_level("TRACE")
```

tail -f ~/.local/state/nvim/lsp.log
