# file_rat

RAT is my own archiving algorithm written in rust.

## CLI

You can now run the app with command line arguments:

```bash
cargo run -- <command> [args]
```

Commands:

- `add <archive.rat> <file> [--meta name1=value1] ... [--meta nameN=valueN] [--compression fast|best|default]`
- `list <archive.rat>`
- `extract <archive.rat> <id-or-name> <destination> [--remove]`
- `remove <archive.rat> <id-or-name>`
- `help`

Examples:

```bash
cargo run -- add ./test.rat ./1.txt --compression best --meta owner=alice --meta category=invoice --meta priority=1
cargo run -- list ./test.rat
cargo run -- extract ./test.rat 1.txt ./1.txt.extracted
cargo run -- remove ./test.rat 1.txt
```

## Goals

the main goal of this project are :

- [x] make a working file archives system
- [x] possibility to add metadata
- [ ] fast & optimized (we're far)
- [ ] make a crate (mainly to use it myself in my project "fd2")
