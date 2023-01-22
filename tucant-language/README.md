# Language design

Performance shouldn't matter at all at the start only epic features.

Like reverse execution, adjusting code at runtime, macros, etc.

The function first typechecks and then returns the code that needs to run. E.g.:

```tucant
(+ (number 1) (number 1)
```

Maybe switch the parsing to everything is an element and elements are separated by whitespace. The you can use escaping `\ ` to create larger strings.

Starts with calling the typechecking `+` which gets the AST as parameter. This function then
calls the same on the subasts and then gets a value returned that contains the type and a function to actually run the code and return the value. This value can then be used by the function to return its own type and a function to run it.

The hard thing there is how to implement all the language server functionalities.

e.g. call hierarchy. this needs forward and reverse analysis, maybe this can be done by "debugging execution" and using tracepoints?
if method calls are explicit and not simply (method args) but instead (apply method args) and maybe later we create a shorthand for that. then apply is an internal compiler method that we can use.

how can types be shown in the ide? maybe every typecheck function calls an internal compiler tracing method and we can use that
to record these things.

# Supported Editors

https://langserver.org/

<!-- https://microsoft.github.io/debug-adapter-protocol/specification -->

## VSCode

- doesn't to document highlighting according to specification (https://github.com/microsoft/vscode/issues/127007)

## Neovim

- install neovim-git

- TODO switch to https://github.com/wbthomason/packer.nvim because then you can use lua only

```
sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'

mkdir ~/.config/nvim
nano ~/.config/nvim/init.vim

" Plugins will be downloaded under the specified directory.
call plug#begin(has('nvim') ? stdpath('data') . '/plugged' : '~/.vim/plugged')

" Declare the list of plugins.
Plug 'https://github.com/theHamsta/nvim-semantic-tokens.git'

" List ends here. Plugins become visible to Vim after this call.
call plug#end()


nano ~/.config/nvim/zinit.lua

vim.lsp.start({ name = 'TUCaNt Language Server', cmd = vim.lsp.rpc.connect('127.0.0.1', 6008), on_attach = on_attach })

```

- currently it doesn't support semantic tokens (https://github.com/theHamsta/nvim-semantic-tokens)
- https://github.com/neovim/nvim-lspconfig

## Kate

- I failed to make it work
- https://docs.kde.org/stable5/en/kate/kate/kate-application-plugin-lspclient.html
- Settings -> Configure Kate -> LSP Client ->

```json
{
  "servers": {
    "tucant": {
      "command": ["/usr/bin/nc", "127.0.0.1", "6008"],
      "url": "https://github.com/mohe2015/tucant",
      "highlightingModeRegex": "^tucant$" // maybe just try for all text files
    }
  }
}
```

- Settings -> Configure Kate -> Open/Save -> Modes & Filetypes -> New
  Name: tucant
  File extensions: \*.tucant
  Section: sources
  Priority: 15

## Jupyterlab

https://github.com/krassowski/jupyterlab-lsp

## CodeMirror

https://github.com/furqansoftware/codemirror-languageserver

https://github.com/qualified/lsps#readme

## Eclipse

https://projects.eclipse.org/projects/technology.lsp4e

- Install it
- Preferences -> General -> Content Types -> Add child to "Text" -> tucant, File associations -> \*.tucant, Editor -> Generic Text Editor, Encoding utf-8

- Run -> External tools configuration
  /usr/bin/nc
  127.0.0.1 6008
- Preferences -> Language Servers -> Add
- omg this actually works (if you reopen the open file it works)
- no semantic tokens support it seems

## Gnome Builder

https://gitlab.gnome.org/GNOME/gnome-builder/-/tree/main/src/libide/lsp

probably needs an extension

## Sublime Text 4

- Seems like it has no folding support

https://github.com/sublimelsp/LSP

Command Palette
Package Control: Install Package
LSP

https://lsp.sublimetext.io/guides/client_configuration/

```json
// Settings in here override those in "LSP/LSP.sublime-settings"
{
  "semantic_highlighting": true,
  "show_inlay_hints": true,
  "clients": {
    "tucant": {
      "enabled": true,
      "command": ["nc", "127.0.0.1", "6008"],
      "selector": "source.tucant"
    }
  }
}
```

http://www.sublimetext.com/docs/syntax.html

Tools -> Developer -> New Syntax

```yaml
%YAML 1.2
---
# http://www.sublimetext.com/docs/syntax.html
name: tucant
file_extensions:
  - tucant
scope: source.tucant
contexts:
  main: []
```
