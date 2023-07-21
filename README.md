# Flow Updater JSON Creator

Create JSON file compatibles with Flow Updater from command line.

## TODO for 2.0.0

- [x] support local mods and generate .zip or other archive at build if local mods added (Can't be add to the project because we can profile a URL for access to files)
- [ ] add Modrinth (mod provider like Curseforge) support
- [ ] add a GUI and a option for use the cli `--nogui`

## Installation

Download from [releases](https://github.com/zuygui/flowupdater-json-creator/releases).

Or from source (require [cargo and rust](https://rust-lang.com)):

```bash
git clone https://github.com/zuygui/flowupdater-json-creator.git
cd flowupdater-json-creator
cargo build --release
```

## Usage

```bash
# in Linux; you need to set executable perm to file
chmod +x ./flowupdater-json-creator-{your version end}

# Run the executable
./flowupdater-json-creator-{your version end}
```

## Features

- [Eternal API (Curseforge api)](https://docs.curseforge.com/#getting-started) wrapper
- JSON Transpiler from mods list
- TUI with [requestty](https://github.com/Lutetium-Vanadium/requestty)
- Cross platform

## Documentation

- [Documentation(in French)](https://bricklou.github.io/launcher-tutorials)
- [Documentation(in English)](https://github.com/zuygui/flowupdater-json-creator/wiki)

## License

[MIT](https://github.com/zuygui/flowupdater-json-creator/blob/master/LICENSE)
