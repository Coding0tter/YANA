# Yana (Yet Another Note App)

**Yana** is a terminal-based note-taking application that uses Markdown for notes and supports a Vim-like navigation interface. It allows you to keep your notes organized, toggle TODO items, and navigate between multiple notes easily.

## Features

- **Markdown Support:** Write notes in Markdown for better formatting.
- **Vim-like Navigation:**
  - **h/l:** Switch focus between the notes list (left) and the note content (right).
  - **j/k:** Move selection (in the notes list) or move through lines (in the note).
  - **g/G:** Jump to the top/bottom of the current note.
- **Create, Edit, and Delete Notes:** Quickly create new notes, edit existing ones, and remove notes you no longer need.
- **Todo Lists:** Insert `[ ]` or `[x]` items to keep track of tasks. Press space to toggle them.
- **Persistent Storage:** Notes are stored in a fixed location (e.g., `~/.local/share/yana`) and remain available between sessions.

## Installation

```bash
cargo install --path .
```

Ensure `~/.cargo/bin` is in your `PATH`. For example, add this to your `~/.bashrc` or `~/.zshrc`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage

Run the application:

```bash
yana
```

- Press `c` to create a new note (opens in `nvim`).
- Press `e` to edit the selected note.
- Press `d` to delete the selected note.
- Press `q` to quit.

**Enjoy your efficient, keyboard-driven note-taking experience!**
