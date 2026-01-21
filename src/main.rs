mod app;
mod ui;
mod fs;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use app::App;
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

    // ALWAYS restore terminal
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
                match key.code {
                    // Quit cleanly
                    KeyCode::Char('q') => return Ok(()),

                    // File navigation
                    KeyCode::Up => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if app.selected + 1 < app.files.len() {
                            app.selected += 1;
                        }
                    }

                    // Open file
                    KeyCode::Enter => {
                        if let Some(path) = app.files.get(app.selected) {
                            app.content = fs::load_file(path);
                            app.cursor = app.content.len();
                        }
                    }

                    // Editing
                    KeyCode::Backspace => {
                        app.content.pop();
                    }

                    KeyCode::Char(c) => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) && c == 's' {
                            if let Some(path) = app.files.get(app.selected) {
                                fs::save_file(path, &app.content);
                            }
                        } else {
                            app.content.push(c);
                        }
                    }

                    _ => {}
                }
            }
        }
    }
}
