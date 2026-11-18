# AeroTxt

A lightweight native text editor with aero theming, built in Rust with [iced](https://iced.rs).

![AeroTxt Screenshot](https://via.placeholder.com/800x450?text=AeroTxt)

## Features

- **Native & Lightweight** - No web engine, no Electron. Pure Rust with iced. ~5MB binary.
- **Aero Theme** - Windows Aero-inspired glass aesthetic with light/dark mode toggle
- **Syntax Highlighting** - Built-in syntax highlighting for common languages
- **Multi-Tab Editing** - Open and edit multiple files simultaneously
- **File Operations** - Open, save, and create files with native file dialogs
- **Keyboard Shortcuts** - Ctrl+O (open), Ctrl+S (save), Ctrl+N (new tab)
- **Cross-Platform** - Windows, macOS, and Linux

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/iluxz/aerotxt.git
cd aerotxt

# Build (requires Rust toolchain)
cargo build --release

# Run
cargo run --release
```

### Pre-built Binaries

Download the latest release from the [Releases page](https://github.com/iluxz/aerotxt/releases).

## Building

Requires Rust 1.70+ and the iced dependencies:

```bash
cargo build --release
```

The binary will be in `target/release/aerotxt`.

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| Ctrl+O | Open file |
| Ctrl+S | Save file |
| Ctrl+N | New tab |
| Ctrl+W | Close tab |
| Ctrl+Tab | Next tab |
| Ctrl+Shift+Tab | Previous tab |
| Ctrl+F | Find |
| Ctrl+H | Find and replace |
| Ctrl+/ | Toggle comment |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [iced](https://iced.rs) - The GUI library
- [rfd](https://github.com/PolyMeilex/rfd) - Native file dialogs
- Sublime Text - Inspiration for the feature set
- Windows Aero - Inspiration for the theme