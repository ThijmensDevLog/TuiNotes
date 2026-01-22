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

    if let Some(path) = app.current_file() {
        app.load_content(fs::load_file(path));
    }

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                /* ---------- GLOBAL ---------- */

                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('q')
                {
                    return Ok(());
                }

                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('h')
                {
                    app.focus = if app.focus == Focus::Help {
                        Focus::Files
                    } else {
                        Focus::Help
                    };
                    continue;
                }

                if key.code == KeyCode::Tab {
                    app.focus = match app.focus {
                        Focus::Files => Focus::Editor,
                        Focus::Editor => Focus::Files,
                        other => other,
                    };
                    continue;
                }

                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('p')
                {
                    app.focus = Focus::Search;
                    app.search_input.clear();
                    app.search_results = (0..app.files.len()).collect();
                    app.search_selected = 0;
                    continue;
                }

                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('s')
                {
                    if let Some(path) = app.current_file() {
                        fs::save_file(path, &app.content_as_string());
                    }
                    continue; // ⬅ IMPORTANT
                }

                /* ---------- HELP ---------- */

                if app.focus == Focus::Help {
                    if key.code == KeyCode::Esc
                        || (key.modifiers.contains(KeyModifiers::CONTROL)
                            && key.code == KeyCode::Char('h'))
                    {
                        app.focus = Focus::Files;
                    }
                    continue;
                }

                /* ---------- FILES ---------- */

                if app.focus == Focus::Files {
                    match key.code {
                        KeyCode::Up => {
                            app.selected = app.selected.saturating_sub(1);
                            let path = app.current_file().unwrap();
                            app.load_content(fs::load_file(path));
                        }
                        KeyCode::Down => {
                            if app.selected + 1 < app.files.len() {
                                app.selected += 1;
                                let path = app.current_file().unwrap();
                                app.load_content(fs::load_file(path));
                            }
                        }
                        KeyCode::Char('n') => {
                            app.focus = Focus::NewNote;
                            app.new_note_input.clear();
                        }
                        _ => {}
                    }
                    continue;
                }

                /* ---------- NEW NOTE ---------- */

                if app.focus == Focus::NewNote {
                    match key.code {
                        KeyCode::Esc => app.focus = Focus::Files,
                        KeyCode::Enter => {
                            let mut name = app.new_note_input.clone();
                            if !name.ends_with(".md") {
                                name.push_str(".md");
                            }
                            let path = notes_dir.join(name);
                            fs::save_file(&path, "");
                            app.files = fs::list_md_files(&notes_dir);
                            app.selected = app.files.len() - 1;
                            app.load_content(String::new());
                            app.focus = Focus::Editor;
                        }
                        KeyCode::Backspace => {
                            app.new_note_input.pop();
                        }
                        KeyCode::Char(c)
                            if key.modifiers.is_empty() =>
                        {
                            app.new_note_input.push(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                /* ---------- SEARCH ---------- */

                if app.focus == Focus::Search {
                    match key.code {
                        KeyCode::Esc => app.focus = Focus::Files,
                        KeyCode::Enter => {
                            app.selected = app.search_results[app.search_selected];
                            let path = app.current_file().unwrap();
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
                        KeyCode::Char(c)
                            if key.modifiers.is_empty() =>
                        {
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
                            let rest =
                                app.lines[app.cursor_row].split_off(app.cursor_col);
                            app.lines.insert(app.cursor_row + 1, rest);
                            app.cursor_row += 1;
                            app.cursor_col = 0;
                        }
                        KeyCode::Backspace => {
                            if app.cursor_col > 0 {
                                app.lines[app.cursor_row].remove(app.cursor_col - 1);
                                app.cursor_col -= 1;
                            } else if app.cursor_row > 0 {
                                let line = app.lines.remove(app.cursor_row);
                                app.cursor_row -= 1;
                                app.cursor_col = app.lines[app.cursor_row].len();
                                app.lines[app.cursor_row].push_str(&line);
                            }
                        }
                        KeyCode::Char(c)
                            if key.modifiers.is_empty() =>
                        {
                            app.lines[app.cursor_row].insert(app.cursor_col, c);
                            app.cursor_col += 1;
                        }
                        _ => {}
                    }

                    let visible = terminal.size()?.height as usize - 3;
                    if app.cursor_row < app.scroll {
                        app.scroll = app.cursor_row;
                    } else if app.cursor_row >= app.scroll + visible {
                        app.scroll = app.cursor_row - visible + 1;
                    }
                }
            }
        }
    }
}
