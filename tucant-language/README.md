# Supported Editors

<!-- https://microsoft.github.io/debug-adapter-protocol/specification -->

## VSCode

- doesn't to document highlighting according to specification (https://github.com/microsoft/vscode/issues/127007)

## Neovim

- if you are able to set it up go for it
- currently it doesn't support semantic tokens (https://github.com/theHamsta/nvim-semantic-tokens)

## Kate

- I failed to make it work
- https://docs.kde.org/stable5/en/kate/kate/kate-application-plugin-lspclient.html
- Settings -> Configure Kate -> LSP Client -> 
```json
{
    "servers": {
        "tucant": {
            "command": ["nc", "127.0.0.1", "6008"],
            "url": "https://github.com/mohe2015/tucant",
            "highlightingModeRegex": "^tucant$"
        }
    }
}
```
- Seettings -> Configure Kate -> Open/Save -> Modes & Filetypes -> New
Name: tucant
File extensions: *.tucant
Section: sources
Priority: 15

## Jupyterlab

https://github.com/krassowski/jupyterlab-lsp

## CodeMirror

https://github.com/furqansoftware/codemirror-languageserver

## Eclipse

https://projects.eclipse.org/projects/technology.lsp4e

- Install it
- Preferences -> General -> Content Types -> Add child to "Text" -> tucant, File associations -> *.tucant, Editor -> Generic Text Editor, Encoding utf-8

- Run -> External tools configuration
/usr/bin/nc
127.0.0.1 6008
- Preferences -> Language Servers -> Add
- omg this actually works (if you reopen the open file it works!!!)
- no semantic tokens support it seems

## Gnome Builder

https://gitlab.gnome.org/GNOME/gnome-builder/-/tree/main/src/libide/lsp

probably needs an extension