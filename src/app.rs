use std::path::PathBuf;

#[derive(PartialEq, Eq)]
pub enum Focus {
    Files,
    Editor,
    Help,
    NewNote,
}

pub struct App {
    pub notes_dir: PathBuf,
    pub files: Vec<PathBuf>,
    pub selected: usize,

    pub content: String,
    pub cursor: usize,

    pub focus: Focus,

    // Status bar
    pub status: String,

    // New note popup
    pub new_note_input: String,
}

impl App {
    pub fn new(notes_dir: PathBuf) -> Self {
        Self {
            notes_dir,
            files: Vec::new(),
            selected: 0,
            content: String::new(),
            cursor: 0,
            focus: Focus::Files,
            status: String::from("Ready"),
            new_note_input: String::new(),
        }
    }

    pub fn current_file_name(&self) -> String {
        self.files
            .get(self.selected)
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "-".into())
    }
}
