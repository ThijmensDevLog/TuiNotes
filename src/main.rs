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
                /* ---------- GLOBAL ---------- */

                // Quit
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('q')
                {
                    return Ok(());
                }

                // Switch pane
                if key.code == KeyCode::Tab {
                    app.focus = match app.focus {
                        Focus::Files => Focus::Editor,
                        Focus::Editor => Focus::Files,
                        _ => Focus::Files,
                    };
                    continue;
                }

                // Search
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('p')
                {
                    app.focus = Focus::Search;
                    app.search_input.clear();
                    app.search_results = (0..app.files.len()).collect();
                    app.search_selected = 0;
                    continue;
                }

                /* ---------- SEARCH ---------- */

                if app.focus == Focus::Search {
                    match key.code {
                        KeyCode::Esc => {
                            app.focus = Focus::Files;
                        }
                        KeyCode::Enter => {
                            let idx = app.search_results[app.search_selected];
                            app.selected = idx;
                            let path = &app.files[idx];
                            app.load_content(fs::load_file(path));
                            app.focus = Focus::Editor;
                        }
                        KeyCode::Up => {
                            app.search_selected =
                                app.search_selected.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            if app.search_selected + 1 < app.search_results.len() {
                                app.search_selected += 1;
                            }
                        }
                        KeyCode::Backspace => {
                            app.search_input.pop();
                        }
                        KeyCode::Char(c) => {
                            app.search_input.push(c);
                        }
                        _ => {}
                    }

                    app.search_results = app
                        .files
                        .iter()
                        .enumerate()
                        .filter(|(_, p)| {
                            p.file_name()
                                .unwrap()
                                .to_string_lossy()
                                .contains(&app.search_input)
                        })
                        .map(|(i, _)| i)
                        .collect();

                    continue;
                }

                /* ---------- FILES ---------- */

                if app.focus == Focus::Files {
                    match key.code {
                        KeyCode::Up => {
                            app.selected = app.selected.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            if app.selected + 1 < app.files.len() {
                                app.selected += 1;
                            }
                        }
                        KeyCode::Enter => {
                            let path = &app.files[app.selected];
                            app.load_content(fs::load_file(path));
                            app.focus = Focus::Editor;
                        }
                        _ => {}
                    }
                    continue;
                }

                /* ---------- EDITOR ---------- */

                if app.focus == Focus::Editor {
                    match key.code {
                        KeyCode::Left => {
                            if app.cursor_col > 0 {
                                app.cursor_col -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if app.cursor_col < app.lines[app.cursor_row].len() {
                                app.cursor_col += 1;
                            }
                        }
                        KeyCode::Up => {
                            if app.cursor_row > 0 {
                                app.cursor_row -= 1;
                                app.cursor_col =
                                    app.cursor_col.min(app.lines[app.cursor_row].len());
                            }
                        }
                        KeyCode::Down => {
                            if app.cursor_row + 1 < app.lines.len() {
                                app.cursor_row += 1;
                                app.cursor_col =
                                    app.cursor_col.min(app.lines[app.cursor_row].len());
                            }
                        }
                        KeyCode::Enter => {
                            let current = app.lines[app.cursor_row].split_off(app.cursor_col);
                            app.lines.insert(app.cursor_row + 1, current);
                            app.cursor_row += 1;
                            app.cursor_col = 0;
                        }
                        KeyCode::Backspace => {
                            if app.cursor_col > 0 {
                                app.lines[app.cursor_row].remove(app.cursor_col - 1);
                                app.cursor_col -= 1;
                            } else if app.cursor_row > 0 {
                                let prev_len = app.lines[app.cursor_row - 1].len();
                                let line = app.lines.remove(app.cursor_row);
                                app.cursor_row -= 1;
                                app.cursor_col = prev_len;
                                app.lines[app.cursor_row].push_str(&line);
                            }
                        }
                        KeyCode::Char(c) => {
                            app.lines[app.cursor_row].insert(app.cursor_col, c);
                            app.cursor_col += 1;
                        }
                        _ => {}
                    }

                    // Scroll logic
                    let visible_height = terminal.size()?.height as usize - 3;
                    if app.cursor_row < app.scroll {
                        app.scroll = app.cursor_row;
                    } else if app.cursor_row >= app.scroll + visible_height {
                        app.scroll = app.cursor_row - visible_height + 1;
                    }
                }
            }
        }
    }
}
