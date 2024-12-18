use crate::notes::{load_notes, Note};

pub enum Mode {
    Normal,
}

pub enum Focus {
    Left,
    Right,
}

pub struct App {
    pub notes: Vec<Note>,
    pub selected_note: usize,
    pub note_scroll: u16,
    pub list_scroll: usize,
    pub mode: Mode,
    pub focus: Focus,
    pub needs_redraw: bool,
    pub selected_line: usize, // NEW
}

impl App {
    pub fn new() -> Self {
        App {
            notes: load_notes(),
            selected_note: 0,
            note_scroll: 0,
            list_scroll: 0,
            mode: Mode::Normal,
            focus: Focus::Left,
            needs_redraw: false,
            selected_line: 0,
        }
    }
}