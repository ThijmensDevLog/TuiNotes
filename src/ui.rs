use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Focus};

pub fn draw(f: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.size());

    draw_main(f, app, layout[0]);
    draw_status_bar(f, app, layout[1]);

    match app.focus {
        Focus::Help => draw_help_popup(f),
        Focus::NewNote => draw_new_note_popup(f, app),
        _ => {}
    }
}

fn draw_main(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
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

    f.render_widget(List::new(items).block(block), area);
}

fn draw_editor(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(format!(" Editor — {} ", app.current_file_name()))
        .borders(Borders::ALL)
        .border_style(if app.focus == Focus::Editor {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    f.render_widget(
        Paragraph::new(app.content.as_str()).block(block),
        area,
    );

    if app.focus == Focus::Editor {
        f.set_cursor(area.x + 1 + app.cursor as u16, area.y + 1);
    }
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let focus = match app.focus {
        Focus::Files => "FILES",
        Focus::Editor => "EDITOR",
        Focus::Help => "HELP",
        Focus::NewNote => "NEW NOTE",
    };

    let text = format!(
        " {} | {} | {} | Ctrl+Q quit | Ctrl+S save | Ctrl+H help | n new ",
        focus,
        app.current_file_name(),
        app.status
    );

    let bar = Paragraph::new(text)
        .style(Style::default().fg(Color::Black).bg(Color::White));

    f.render_widget(bar, area);
}

fn draw_help_popup(f: &mut Frame) {
    let area = centered_rect(60, 60, f.size());
    f.render_widget(Clear, area);

    let text = r#"
Ctrl+Q   Quit
Ctrl+S   Save
Ctrl+H   Toggle help
Tab      Switch pane
n        New note
Esc      Close popup
"#;

    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().title(" Help ").borders(Borders::ALL)),
        area,
    );
}

fn draw_new_note_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, f.size());
    f.render_widget(Clear, area);

    let text = format!("File name:\n\n{}", app.new_note_input);

    f.render_widget(
        Paragraph::new(text)
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
