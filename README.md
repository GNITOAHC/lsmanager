# LSManager

## Description

LSManager stands for Language Server Manager, which manages [language servers](https://langserver.org/) for you via command line.

## TODO

- Source support
  - [x] GitHub
  - [ ] Npm
  - [ ] Pypi
- OS support
  - [x] MacOS
  - [ ] Windows
  - [ ] Linux
- Command support
  - [x] install
  - [x] list
  - [ ] uninstall
  - [ ] upgrade
  - [ ] search

## Structure

```
~/.local/ (or path to lsm)
└── lsm/
    ├── packages/
    │   ├── clangd/
    │   │   ├── clangd (bin)
    │   │   └── info.yaml
    │   └── stylua/
    │       ├── stylua (bin)
    │       └── info.yaml
    └── registries/
        └── github/
            ├── info.json (store version of registry.json)
            └── registry.json (store package name list of all languages)
~/.local/ (or path to binary)
└── bin/
    ├── stylua -> ~/.local/lsm/packages/sylua/stylua (bin)
    └── clangd -> ~/.local/lsm/packages/sylua/clangd (bin)
```
