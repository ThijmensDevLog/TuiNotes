use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Focus};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(f.size());

    draw_files(f, app, chunks[0]);
    draw_editor(f, app, chunks[1]);

    if app.focus == Focus::Help {
        draw_help_popup(f);
    }
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
                Style::default()
            };
            ListItem::new(name).style(style)
        })
        .collect();

    let block = Block::default()
        .title(" Files ")
        .borders(Borders::ALL)
        .border_style(if app.focus == Focus::Files {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn draw_editor(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Editor ")
        .borders(Borders::ALL)
        .border_style(if app.focus == Focus::Editor {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let paragraph = Paragraph::new(app.content.as_str()).block(block);
    f.render_widget(paragraph, area);

    // Cursor (only when editor focused)
    if app.focus == Focus::Editor {
        let x = area.x + 1 + app.cursor as u16;
        let y = area.y + 1;
        f.set_cursor(x, y);
    }
}

fn draw_help_popup(f: &mut Frame) {
    let area = centered_rect(60, 60, f.size());

    f.render_widget(Clear, area);

    let text = r#"
Ctrl+Q   Quit application
Tab      Switch pane
Ctrl+S   Save file
Ctrl+H   Toggle help
Enter    Open file
Esc      Close help
"#;

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
