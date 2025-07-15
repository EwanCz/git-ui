# Git-UI 🦀

A Terminal User Interface for Git written in Rust using [Ratatui](https://github.com/ratatui-org/ratatui).

## 🎯 Purpose

This project serves a dual purpose:

1. **Learning Rust**: A hands-on project to explore Rust's systems programming capabilities, error handling, and ecosystem
2. **Git TUI**: A functional terminal interface for Git operations, designed for personal use

## 🚀 Installation

### Prerequisites

- Rust 1.70+ installed ([rustup.rs](https://rustup.rs/))
- Git installed and configured
- Git-delta package for diff ([Git-delta](https://github.com/dandavison/delta))

### From Source

```bash
git clone https://github.com/EwanCz/git-ui.git
cd git-ui
cargo build --release
cargo install --path .
```

## 🎮 Usage

Navigate to a Git repository and run:

```bash
git-ui
```

Maximun deepth is set to 5

## ⌨️ Keybindings

### 🌐 Global
- `q` - Quit
- `1-2-3` - Switch between panels
- `?` - Show help
- `Up/Down` - Navigate in blocks
- `Control direction` - Navigate between blocks

### 📝 Status panels

- `c` - launch commit mode
#### commit mode
- `ESC` - quit commit mode
- `Enter` - commit
- `Character` -  Write commit message (can move with arrow)


- `p` - launch push mode
#### commit mode
- `ESC` - quit push mode
- `Enter` - push file (might take time before finishing push)

#### staged block
- `r` - restore the selected File

#### unstaged block
- `a` - add the selected file

### 🌳 Branch Panel
Checkout on remote branch make a local version of it

- `c` - Checkout branch

## 🛠️ Technologies Used

- **[Ratatui](https://github.com/ratatui-org/ratatui)**: Terminal UI framework
- **[git2](https://docs.rs/git2/)**: Git operations library
- **[crossterm](https://docs.rs/crossterm/)**: Cross-platform terminal handling

## 🎯 Development Status

### ✅ Completed Features

- [x] Basic repository detection and status display
- [x] Branch name and status detection
- [x] File staging/unstaging
- [x] Basic TUI layout with panels
- [x] Commit entry with message
- [x] simple push handling
- [x] diff viewer (if no diff show raw file)
- [x] multi threading to get push loading bar

### 🚧 In Progress
- [ ] push fonctionnality that might be unstable on branch
- [ ] Branch switching and creation

### 📋 Planned Features

- [ ] complete push option
- [ ] pull option
- [ ] help menu
- [ ] clone / init / branch creation / checkout
- [ ] Configuration file support
- [ ] Remote repository operations
- [ ] Stash management

## 📚 Learning Resources

If you're also learning Rust, here are some resources that helped with this project:

- [Rust Documentation](https://doc.rust-lang.org/stable/rust-by-example/)
- [Ratatui Documentation](https://ratatui.rs/)
- [Git2 Documentation](https://docs.rs/git2/)

## other information
Still on my journey to learn rust. If you find out bad practice or bugs in the program I would love to have it reported.
