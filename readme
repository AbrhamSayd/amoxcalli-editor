# Amoxcalli

A terminal-based text editor written in Rust, inspired by Vim.

**Amoxcalli** (from Nahuatl: *āmoxtli* "book" + *calli* "house") means "house of books" or "library" in the Aztec language. In pre-Columbian Mexico, the amoxcalli was the place where codices were stored and knowledge was preserved. This editor carries that spirit — a simple tool for writing and preserving text.

## About

This is a personal project built to learn Rust while creating something useful. It's a clone-style editor flavored by my own needs and preferences, implementing vim-like modal editing with a focus on simplicity and learning.

## Features

- **Modal Editing** — Normal and Insert modes, just like Vim
- **Vim-style Commands** — `:w`, `:q`, `:wq`, `:q!`, and more
- **Unicode Support** — Proper handling of multi-width and special characters
- **Minimal Dependencies** — Only what's necessary
- **Cross-platform** — Works on Linux, macOS, and Windows

### Roadmap

- [x] Vim controls style
- [ ] File explorer built in
- [ ] Code syntax highlighting
- [ ] Themes

## Installation

```bash
# Clone the repository
git clone https://github.com/AbrhamSayd/amoxcalli-editor.git
cd amoxcalli-editor

# Build release version
cargo build --release

# The binary will be at ./target/release/amoxcalli
```

## Usage

```bash
# Open a new empty document
./target/release/amoxcalli

# Open an existing file
./target/release/amoxcalli path/to/file.txt
```

### Keybindings

| Key | Mode | Action |
|-----|------|--------|
| `i` | Normal | Enter Insert mode |
| `I` | Normal | Enter Insert mode at line start |
| `Esc` | Insert | Return to Normal mode |
| `Arrow keys` | Any | Navigate |
| `Home` / `End` | Any | Jump to line start/end |
| `Page Up` / `Page Down` | Any | Scroll viewport |
| `Backspace` | Insert | Delete character before cursor |
| `Delete` | Insert | Delete character at cursor |

### Commands

Press `:` in Normal mode to enter command mode:

| Command | Action |
|---------|--------|
| `:w` | Save file |
| `:w <filename>` | Save as |
| `:q` | Quit (fails if unsaved changes) |
| `:q!` | Force quit without saving |
| `:wq` or `:x` | Save and quit |
| `:help` | Show help |

## Project Structure

```
src/
├── main.rs              # Entry point
└── editor/
    ├── mod.rs           # Main Editor logic
    ├── mode.rs          # Normal/Insert modes
    ├── command.rs       # Command definitions
    ├── commandparser.rs # Vim-style command parsing
    ├── commandbar.rs    # Command input UI
    ├── messagebar.rs    # Status messages
    ├── statusbar.rs     # File info display
    ├── terminal.rs      # Terminal I/O
    ├── view.rs          # Main editing buffer
    ├── line.rs          # Line/text handling
    └── ...
```

## Dependencies

- [crossterm](https://crates.io/crates/crossterm) — Cross-platform terminal manipulation
- [unicode-segmentation](https://crates.io/crates/unicode-segmentation) — Grapheme cluster handling
- [unicode-width](https://crates.io/crates/unicode-width) — Character width calculation

## Learning Goals

This project is primarily a learning exercise. Through building it, I'm exploring:

- Rust ownership and borrowing
- Terminal raw mode and escape sequences
- Building a TUI from scratch
- Vim's modal editing philosophy
- Unicode text handling complexities

## License

MIT

---