mod app;
mod ui;
mod fs;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use app::{App, Focus};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), io::Error> {
    let notes_dir = PathBuf::from("./notes");
    std::fs::create_dir_all(&notes_dir).ok();

    let mut app = App::new(notes_dir.clone());
    app.files = fs::list_md_files(&notes_dir);

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                // HELP POPUP HANDLING
                if app.focus == Focus::Help {
                    match key.code {
                        KeyCode::Esc
                        | KeyCode::Char('h')
                            if key.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            app.focus = Focus::Files;
                        }
                        _ => {}
                    }
                    continue;
                }

                match key.code {
                    // Quit
                    KeyCode::Char('q')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        return Ok(());
                    }

                    // Help
                    KeyCode::Char('h')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        app.focus = Focus::Help;
                    }

                    // Switch pane
                    KeyCode::Tab => {
                        app.focus = match app.focus {
                            Focus::Files => Focus::Editor,
                            Focus::Editor => Focus::Files,
                            Focus::Help => Focus::Files,
                        };
                    }

                    // FILE PANE
                    KeyCode::Up if app.focus == Focus::Files => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Down if app.focus == Focus::Files => {
                        if app.selected + 1 < app.files.len() {
                            app.selected += 1;
                        }
                    }
                    KeyCode::Enter if app.focus == Focus::Files => {
                        if let Some(path) = app.files.get(app.selected) {
                            app.content = fs::load_file(path);
                            app.cursor = app.content.len();
                            app.focus = Focus::Editor;
                        }
                    }

                    // EDITOR
                    KeyCode::Left if app.focus == Focus::Editor => {
                        if app.cursor > 0 {
                            app.cursor -= 1;
                        }
                    }
                    KeyCode::Right if app.focus == Focus::Editor => {
                        if app.cursor < app.content.len() {
                            app.cursor += 1;
                        }
                    }
                    KeyCode::Backspace if app.focus == Focus::Editor => {
                        if app.cursor > 0 {
                            app.cursor -= 1;
                            app.content.remove(app.cursor);
                        }
                    }
                    KeyCode::Char(c) if app.focus == Focus::Editor => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) && c == 's' {
                            if let Some(path) = app.files.get(app.selected) {
                                fs::save_file(path, &app.content);
                            }
                        } else {
                            app.content.insert(app.cursor, c);
                            app.cursor += 1;
                        }
                    }

                    _ => {}
                }
            }
        }
    }
}
