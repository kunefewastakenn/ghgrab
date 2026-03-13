# ghgrab - "grab anything you want"

> A simple, pretty terminal tool that lets you search and download files from GitHub without leaving your CLI.

![Rust](https://img.shields.io/badge/rust-1.70%20%7C%201.75%20%7C%20stable-blue) ![crates.io](https://img.shields.io/crates/v/ghgrab.svg?color=blue) ![npm version](https://img.shields.io/npm/v/@ghgrab/ghgrab.svg?color=blue) ![PyPI version](https://img.shields.io/pypi/v/ghgrab.svg?color=blue) ![license](https://img.shields.io/badge/license-MIT-blue)

![ghgrab demo](assets/ghgrab.gif)

**ghgrab** provides a streamlined command-line interface for cherry-picking specific files or folders from any GitHub repository, powered by the Rust `tokio` and `ratatui` ecosystem. Focused on speed and ease of use, it offers a beautiful TUI that lets you grab exactly what you need; all without the wait times of a full `git clone`.

## Why use ghgrab?

- **No more clone-and-delete**: Grab exactly what you need, when you need it.
- **Easy on the eyes**: A clean terminal interface that makes browsing feel smooth.
- **Works where you are**: Installs quickly via NPM, Cargo, or PIP.
- **Find things fast**: Quickly search and navigate through any repo's folders.
- **Handles the big stuff**: Built-in support for GitHub LFS (Large File Storage).
- **Batch mode**: Select a bunch of files and folders to download them all at once.

---

## Installation

### NPM
```bash
npm install -g @ghgrab/ghgrab
```

### Cargo
```bash
cargo install ghgrab
```

### pipx (Recommended for Python)
```bash
pipx install ghgrab
```

---

### Quick Start

Just type `ghgrab` to start browsing:

```bash
ghgrab
```

Or, if you already have a link, just paste it in:

```bash
ghgrab https://github.com/rust-lang/rust
```

### GitHub Token (Private Repos & Rate Limits)

GitHub limits how many requests you can make without a token and hides private repositories. To download from **private repositories** or increase your search rate limits, it's highly recommended to set a Personal Access Token:

```bash
# Set your token
ghgrab config set --token YOUR_TOKEN

# View masked token
ghgrab config list

# Remove token
ghgrab config unset
```

### Keyboard Shortcuts (How to move around)

We've kept it pretty standard, but here's a quick cheat sheet:

| Key | Action |
|-----|--------|
| `Enter` / `l` / `Right` | Enter directory or Submit URL |
| `Backspace` / `h` / `Left` | Go back to previous folder or Input screen |
| `Space` | Toggle selection for the current item |
| `a` | Select All items |
| `u` | Unselect all items |
| `d` / `D` | Download selected items |
| `i` | Toggle Icons (Emoji / ASCII) |
| `q` / `Q` | Exit application |
| `g` / `Home` | Jump to Top |
| `G` / `End` | Jump to Bottom |
| `Esc` | Clear input or Cancel search |

---



## Join the community

If you find a bug, have an idea for a cool new feature, or just want to help out, we'd love to hear from you! Check out our [Contributing Guide](CONTRIBUTING.md) to see how you can get involved.

## License

Distributed under the MIT License. It's open, free, and yours to play with. See [LICENSE](LICENSE) for the fine print.

