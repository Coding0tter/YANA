use crate::app::{App, Focus};
use crate::notes::count_todos;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let top_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(f.area());

    let title_line = Line::from(vec![
        Span::styled(
            " Yana ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" - Yet Another Note App"),
    ]);
    let title_block = Paragraph::new(title_line)
        .style(Style::default().bg(Color::Blue))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title_block, top_layout[0]);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(top_layout[1]);

    let notes_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(main_layout[0]);

    render_notes(f, app, &notes_chunks);

    let shortcuts_text = Line::from(vec![
        Span::styled("[q]", Style::default().fg(Color::Cyan)),
        Span::raw(" Quit  "),
        Span::styled("[c]", Style::default().fg(Color::Cyan)),
        Span::raw(" Create  "),
        Span::styled("[e]", Style::default().fg(Color::Cyan)),
        Span::raw(" Edit  "),
        Span::styled("[d]", Style::default().fg(Color::Cyan)),
        Span::raw(" Delete  "),
        Span::styled("[h/l]", Style::default().fg(Color::Cyan)),
        Span::raw(" Focus Left/Right  "),
        Span::styled("[j/k]", Style::default().fg(Color::Cyan)),
        Span::raw(" Move/Scroll  "),
        Span::styled("[g/G]", Style::default().fg(Color::Cyan)),
        Span::raw(" Top/Bottom  "),
        Span::styled("[space]", Style::default().fg(Color::Cyan)),
        Span::raw(" Toggle Todo"),
    ]);

    let shortcuts = Paragraph::new(shortcuts_text)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("Shortcuts")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        );
    f.render_widget(shortcuts, main_layout[1]);
}

fn render_notes(f: &mut Frame, app: &App, chunks: &[Rect]) {
    let left_focus = matches!(app.focus, Focus::Left);
    let right_focus = matches!(app.focus, Focus::Right);

    let left_block_style = if left_focus {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let right_block_style = if right_focus {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };

    let left_block = Block::default()
        .title(format!(
            " Notes ({}/{}) ",
            if app.notes.is_empty() {
                0
            } else {
                app.selected_note + 1
            },
            app.notes.len()
        ))
        .borders(Borders::ALL)
        .border_style(left_block_style);

    let mut note_items: Vec<ListItem> = app
        .notes
        .iter()
        .enumerate()
        .map(|(i, note)| {
            let style = if i == app.selected_note {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(note.title.clone(), style)))
        })
        .collect();

    let visible_limit = chunks[0].height.saturating_sub(2) as usize;
    if !note_items.is_empty() {
        if app.list_scroll + visible_limit > note_items.len() {
            let start = note_items.len().saturating_sub(visible_limit);
            note_items = note_items[start..].to_vec();
        } else {
            note_items = note_items
                [app.list_scroll..app.list_scroll + visible_limit.min(note_items.len())]
                .to_vec();
        }
    }

    let notes_list = List::new(note_items).block(left_block);
    f.render_widget(notes_list, chunks[0]);

    let right_block = Block::default()
        .title(if !app.notes.is_empty() {
            let curr_note = &app.notes[app.selected_note];
            let (open, closed) = count_todos(&curr_note.content);
            format!(
                " {} (Todos: {} open / {} done) ",
                curr_note.title, open, closed
            )
        } else {
            " No Notes ".to_string()
        })
        .borders(Borders::ALL)
        .border_style(right_block_style);

    if !app.notes.is_empty() {
        let curr_note = &app.notes[app.selected_note];
        let lines = parse_markdown_to_lines(&curr_note.content);

        let height = right_block.inner(chunks[1]).height;
        let visible_height = height as usize;

        // Clamp note_scroll and selected_line
        let line_count = lines.len();
        let note_scroll = app.note_scroll;
        let selected_line = app.selected_line.min(line_count.saturating_sub(1));

        let start = note_scroll as usize;
        let end = (note_scroll as usize)
            .saturating_add(visible_height)
            .min(line_count);

        let mut visible_lines = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if i >= start && i < end {
                let mut styled_line = line.clone();
                if i == selected_line {
                    // Highlight selected line
                    // Apply a bold, reversed style
                    styled_line = Line::from(
                        styled_line
                            .spans
                            .iter()
                            .map(|span| {
                                Span::styled(
                                    span.content.to_string(),
                                    Style::default()
                                        .add_modifier(Modifier::REVERSED | Modifier::BOLD),
                                )
                            })
                            .collect::<Vec<Span>>(),
                    );
                }
                visible_lines.push(styled_line);
            }
        }

        let paragraph = Paragraph::new(visible_lines)
            .wrap(Wrap { trim: false })
            .block(right_block);
        f.render_widget(paragraph, chunks[1]);
    } else {
        let paragraph = Paragraph::new("Create a note with 'c'").block(right_block);
        f.render_widget(paragraph, chunks[1]);
    }
}

fn parse_markdown_to_lines(input: &str) -> Vec<Line> {
    // Basic markdown parsing (similar to before)
    let parser = Parser::new_ext(input, Options::all());
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut in_heading = false;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { .. } => {
                    in_heading = true;
                    if !current_line.is_empty() {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    }
                }
                Tag::Item => {
                    if !current_line.is_empty() {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    }
                    current_line.push_str("• ");
                }
                _ => {}
            },
            Event::End(tagend) => match tagend {
                TagEnd::Heading { .. } => {
                    // end heading
                    if !current_line.is_empty() {
                        lines.push(Line::from(Span::styled(
                            current_line.clone(),
                            Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                        )));
                        current_line.clear();
                    }
                    in_heading = false;
                }
                TagEnd::Item => {}
                _ => {}
            },
            Event::Text(t) => {
                current_line.push_str(&t);
            }
            Event::SoftBreak | Event::HardBreak => {
                if !current_line.is_empty() {
                    if in_heading {
                        lines.push(Line::from(Span::styled(
                            current_line.clone(),
                            Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                        )));
                    } else {
                        lines.push(Line::from(current_line.clone()));
                    }
                    current_line.clear();
                } else {
                    lines.push(Line::from(""));
                }
            }
            Event::TaskListMarker(checked) => {
                if checked {
                    current_line.push_str("[x] ");
                } else {
                    current_line.push_str("[ ] ");
                }
            }
            _ => {}
        }
    }

    if !current_line.is_empty() {
        if in_heading {
            lines.push(Line::from(Span::styled(
                current_line,
                Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )));
        } else {
            lines.push(Line::from(current_line));
        }
    }

    lines
}
