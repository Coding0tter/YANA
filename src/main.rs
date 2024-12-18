mod app;
mod notes;
mod ui;
mod utils;

use app::App;
use crossterm::{
    cursor::MoveTo,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use notes::save_notes;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};
use utils::handle_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    loop {
        // If we need a full redraw (e.g., after exiting nvim)
        if app.needs_redraw {
            terminal.clear()?;
            app.needs_redraw = false;
        }

        terminal.draw(|f| ui::ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        let mut should_quit = false;
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                should_quit = handle_input(key, &mut app)?;
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if should_quit {
            save_notes(&app.notes);
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    terminal.show_cursor()?;
    Ok(())
}
