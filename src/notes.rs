use crate::app::App;
use crate::utils::notes_path;
use crossterm::event::EnableMouseCapture;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::{execute, terminal::enable_raw_mode};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
}

pub fn load_notes() -> Vec<Note> {
    let notes_file = notes_path();
    if let Ok(data) = std::fs::read_to_string(&notes_file) {
        if let Ok(notes) = serde_json::from_str(&data) {
            return notes;
        }
    }
    Vec::new()
}

pub fn save_notes(notes: &Vec<Note>) {
    let notes_file = notes_path();
    if let Ok(data) = serde_json::to_string_pretty(notes) {
        let _ = std::fs::write(notes_file, data);
    }
}

pub fn create_note(app: &mut App) -> io::Result<()> {
    let tmpfile = "tmp_new_note.md";
    std::fs::write(tmpfile, "# Title\n\n- [ ] New item")?;
    std::process::Command::new("nvim").arg(tmpfile).status()?;

    let content = std::fs::read_to_string(tmpfile)?;
    let title_line = content
        .lines()
        .next()
        .unwrap_or("Untitled")
        .trim()
        .trim_start_matches('#')
        .trim();
    let title = if title_line.is_empty() {
        "Untitled".to_string()
    } else {
        title_line.to_string()
    };
    app.notes.push(Note { title, content });
    std::fs::remove_file(tmpfile)?;

    save_notes(&app.notes);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // Re-enter alternate screen and mouse capture
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // Indicate we need a full redraw
    app.needs_redraw = true;

    Ok(())
}

pub fn edit_note(app: &mut App) -> io::Result<()> {
    if app.notes.is_empty() {
        return Ok(());
    }
    let tmpfile = "tmp_edit_note.md";
    let curr = &app.notes[app.selected_note];
    std::fs::write(tmpfile, &curr.content)?;
    std::process::Command::new("nvim").arg(tmpfile).status()?;

    let content = std::fs::read_to_string(tmpfile)?;
    let title_line = content
        .lines()
        .next()
        .unwrap_or("Untitled")
        .trim()
        .trim_start_matches('#')
        .trim();
    let title = if title_line.is_empty() {
        "Untitled".to_string()
    } else {
        title_line.to_string()
    };
    app.notes[app.selected_note] = Note { title, content };
    std::fs::remove_file(tmpfile)?;

    save_notes(&app.notes);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // Indicate we need a full redraw
    app.needs_redraw = true;

    Ok(())
}

pub fn delete_note(app: &mut App) {
    if app.notes.is_empty() {
        return;
    }
    app.notes.remove(app.selected_note);
    if app.selected_note >= app.notes.len() && !app.notes.is_empty() {
        app.selected_note = app.notes.len() - 1;
    }
    if app.notes.is_empty() {
        app.selected_note = 0;
        app.list_scroll = 0;
    }
    save_notes(&app.notes);
}

pub fn toggle_todo(app: &mut App) {
    if app.notes.is_empty() {
        return;
    }
    let mut lines = app.notes[app.selected_note]
        .content
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();

    let curr_line = app.selected_line + 1;

    if curr_line < lines.len() {
        let line = &mut lines[curr_line];
        if line.contains("[ ]") {
            *line = line.replacen("[ ]", "[x]", 1);
        } else if line.contains("[x]") {
            *line = line.replacen("[x]", "[ ]", 1);
        }
        app.notes[app.selected_note].content = lines.join("\n");
        crate::notes::save_notes(&app.notes);
    }
}

pub fn count_todos(content: &str) -> (usize, usize) {
    let mut open = 0;
    let mut closed = 0;
    for line in content.lines() {
        if line.contains("[ ]") {
            open += 1;
        } else if line.contains("[x]") {
            closed += 1;
        }
    }
    (open, closed)
}
