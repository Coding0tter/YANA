use crate::app::{App, Focus, Mode, SelectedButton};
use crate::notes::{create_note, delete_note, edit_note, toggle_todo};
use crossterm::event::{KeyCode, KeyEvent};
use std::env;
use std::path::PathBuf;

pub fn handle_input(key: KeyEvent, app: &mut App) -> std::io::Result<bool> {
    if app.confirm_delete {
        match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                app.selected_button = SelectedButton::Yes;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                app.selected_button = SelectedButton::No;
            }
            KeyCode::Enter => match app.selected_button {
                SelectedButton::Yes => {
                    delete_note(app);
                    app.confirm_delete = false;
                }
                SelectedButton::No => {
                    app.confirm_delete = false;
                }
            },
            _ => {}
        }
        return Ok(false);
    }

    match app.mode {
        Mode::Normal => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('c') => {
                create_note(app)?;
            }
            KeyCode::Char('d') => {
                if !app.notes.is_empty() {
                    app.confirm_delete = true;
                    app.selected_button = SelectedButton::No;
                }
            }
            KeyCode::Char('e') => {
                edit_note(app)?;
            }
            KeyCode::Char('h') => {
                app.focus = Focus::Left;
            }
            KeyCode::Char('l') => {
                app.focus = Focus::Right;
            }
            KeyCode::Char('g') => {
                if let Focus::Right = app.focus {
                    app.selected_line = 0;
                    app.note_scroll = 0;
                }
            }
            KeyCode::Char('G') => {
                if let Focus::Right = app.focus {
                    if !app.notes.is_empty() {
                        let content = &app.notes[app.selected_note].content;
                        let line_count = parse_note_lines(content);
                        app.selected_line = line_count.saturating_sub(1);

                        let height = 10;

                        let visible_height = height as usize;

                        if line_count > visible_height {
                            app.note_scroll = (line_count - visible_height) as u16;
                        } else {
                            app.note_scroll = 0;
                        }
                    }
                }
            }
            KeyCode::Char('j') => match app.focus {
                Focus::Left => {
                    if app.selected_note + 1 < app.notes.len() {
                        app.selected_note += 1;
                        adjust_list_scroll_down(app);
                        app.note_scroll = 0;
                        app.selected_line = 0;
                    }
                }
                Focus::Right => {
                    if !app.notes.is_empty() {
                        let content = &app.notes[app.selected_note].content;
                        let line_count = parse_note_lines(content);
                        if app.selected_line + 1 < line_count - 1 {
                            app.selected_line += 1;
                            adjust_note_scroll_down(app, line_count);
                        }
                    }
                }
            },
            KeyCode::Char('k') => match app.focus {
                Focus::Left => {
                    if app.selected_note > 0 {
                        app.selected_note -= 1;
                        adjust_list_scroll_up(app);
                        app.note_scroll = 0;
                        app.selected_line = 0;
                    }
                }
                Focus::Right => {
                    if app.selected_line > 0 {
                        app.selected_line -= 1;
                        adjust_note_scroll_up(app);
                    }
                }
            },
            KeyCode::Char(' ') => {
                toggle_todo(app);
            }
            _ => {}
        },
    }
    Ok(false)
}

fn parse_note_lines(content: &str) -> usize {
    content.lines().count()
}

fn adjust_list_scroll_down(app: &mut App) {
    let visible_height = 10;
    if app.selected_note >= app.list_scroll + visible_height {
        app.list_scroll = app.selected_note.saturating_sub(visible_height - 1);
    }
}

fn adjust_list_scroll_up(app: &mut App) {
    if app.selected_note < app.list_scroll {
        app.list_scroll = app.selected_note;
    }
}

fn adjust_note_scroll_down(app: &mut App, line_count: usize) {
    let visible_height = 10;
    let bottom_line_visible = app.note_scroll as usize + visible_height - 1;
    if app.selected_line > bottom_line_visible {
        app.note_scroll = (app.selected_line - (visible_height - 1)) as u16;
    }
    if app.selected_line >= line_count {
        app.selected_line = line_count.saturating_sub(1);
    }
}

fn adjust_note_scroll_up(app: &mut App) {
    if app.selected_line < app.note_scroll as usize {
        app.note_scroll = app.selected_line as u16;
    }
}

pub fn notes_path() -> PathBuf {
    let home = env::var("HOME").expect("HOME not set");
    let path = PathBuf::from(home).join(".local/share/yana");
    std::fs::create_dir_all(&path).expect("Failed to create notes directory");
    path.join("notes.json")
}
