use ratatui::style::Stylize;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(f.size());

    // File list
    let items: Vec<ListItem> = app.files.iter().enumerate().map(|(i, p)| {
        let name = p.file_name().unwrap().to_string_lossy();
        let style = if i == app.selected {
            Style::default().reversed()
        } else {
            Style::default()
        };
        ListItem::new(name).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Notes"));

    f.render_widget(list, chunks[0]);

    // Editor
    let editor = Paragraph::new(app.content.as_str())
        .block(Block::default().borders(Borders::ALL).title("Editor"));

    f.render_widget(editor, chunks[1]);
}
