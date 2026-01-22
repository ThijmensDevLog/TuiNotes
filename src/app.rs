use std::path::PathBuf;

#[derive(PartialEq, Eq)]
pub enum Focus {
    Files,
    Editor,
    Help,
    NewNote,
    Search,
}

pub struct App {
    pub notes_dir: PathBuf,
    pub files: Vec<PathBuf>,
    pub selected: usize,

    // Editor
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll: usize,

    pub focus: Focus,

    // Status bar
    pub status: String,

    // New note
    pub new_note_input: String,

    // Search
    pub search_input: String,
    pub search_results: Vec<usize>,
    pub search_selected: usize,
}

impl App {
    pub fn new(notes_dir: PathBuf) -> Self {
        Self {
            notes_dir,
            files: Vec::new(),
            selected: 0,
            lines: vec![String::new()],
            cursor_row: 0,
            cursor_col: 0,
            scroll: 0,
            focus: Focus::Files,
            status: "Ready".into(),
            new_note_input: String::new(),
            search_input: String::new(),
            search_results: Vec::new(),
            search_selected: 0,
        }
    }

    pub fn current_file_name(&self) -> String {
        self.files
            .get(self.selected)
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "-".into())
    }

    pub fn load_content(&mut self, content: String) {
        self.lines = content.lines().map(|l| l.to_string()).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.scroll = 0;
    }

    pub fn content_as_string(&self) -> String {
        self.lines.join("\n")
    }
}
