use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Focus};

const TEXT: Color = Color::Gray;
const BORDER_ACTIVE: Color = Color::White;
const BORDER_IDLE: Color = Color::DarkGray;

pub fn draw(f: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    draw_main(f, app, layout[0]);
    draw_status_bar(f, app, layout[1]);

    match app.focus {
        Focus::Search => draw_search_popup(f, app),
        Focus::NewNote => draw_new_note_popup(f, app),
        Focus::Help => draw_help_popup(f),
        _ => {}
    }
}

fn draw_main(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    draw_files(f, app, chunks[0]);
    draw_editor(f, app, chunks[1]);
}

fn draw_files(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let name = p.file_name().unwrap().to_string_lossy();
            let style = if i == app.selected {
                Style::default().reversed()
            } else {
                Style::default().fg(TEXT)
            };
            ListItem::new(name).style(style)
        })
        .collect();

    let block = Block::default()
        .title(" Files ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.focus == Focus::Files {
            BORDER_ACTIVE
        } else {
            BORDER_IDLE
        }));

    f.render_widget(List::new(items).block(block), area);
}

fn draw_editor(f: &mut Frame, app: &App, area: Rect) {
    let visible = &app.lines[app.scroll..];

    let block = Block::default()
        .title(format!(" Editor — {} ", app.current_file_name()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.focus == Focus::Editor {
            BORDER_ACTIVE
        } else {
            BORDER_IDLE
        }));

    f.render_widget(
        Paragraph::new(visible.join("\n"))
            .style(Style::default().fg(TEXT))
            .block(block),
        area,
    );

    if app.focus == Focus::Editor {
        let x = area.x + 1 + app.cursor_col as u16;
        let y = area.y + 1 + (app.cursor_row - app.scroll) as u16;
        f.set_cursor(x, y);
    }
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let text = format!(" {} ", app.current_file_name());

    f.render_widget(
        Paragraph::new(text)
            .style(Style::default().fg(Color::Black).bg(Color::Gray)),
        area,
    );
}

fn draw_help_popup(f: &mut Frame) {
    let area = centered_rect(50, 50, f.size());
    f.render_widget(Clear, area);

    let text = r#"
Tab        Switch Files / Editor
Ctrl+P     Search notes
Ctrl+S     Save
Ctrl+Q     Quit
Ctrl+H     Toggle help
n          New note
"#;

    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().title(" Help ").borders(Borders::ALL)),
        area,
    );
}

fn draw_search_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 60, f.size());
    f.render_widget(Clear, area);

    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .enumerate()
        .map(|(i, &idx)| {
            let name = app.files[idx].file_name().unwrap().to_string_lossy();
            let style = if i == app.search_selected {
                Style::default().reversed()
            } else {
                Style::default().fg(TEXT)
            };
            ListItem::new(name).style(style)
        })
        .collect();

    f.render_widget(
        List::new(items).block(
            Block::default()
                .title(format!(" Search: {} ", app.search_input))
                .borders(Borders::ALL),
        ),
        area,
    );
}

fn draw_new_note_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(40, 20, f.size());
    f.render_widget(Clear, area);

    f.render_widget(
        Paragraph::new(format!("File name:\n\n{}", app.new_note_input))
            .block(Block::default().title(" New Note ").borders(Borders::ALL)),
        area,
    );

    f.set_cursor(area.x + 2 + app.new_note_input.len() as u16, area.y + 3);
}

fn centered_rect(px: u16, py: u16, r: Rect) -> Rect {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - py) / 2),
            Constraint::Percentage(py),
            Constraint::Percentage((100 - py) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - px) / 2),
            Constraint::Percentage(px),
            Constraint::Percentage((100 - px) / 2),
        ])
        .split(v[1])[1]
}
