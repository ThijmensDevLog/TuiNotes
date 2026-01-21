use std::path::PathBuf;

#[derive(PartialEq, Eq)]
pub enum Focus {
    Files,
    Editor,
    Help,
}

pub struct App {
    pub notes_dir: PathBuf,
    pub files: Vec<PathBuf>,
    pub selected: usize,

    pub content: String,
    pub cursor: usize,

    pub focus: Focus,
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
        }
    }
}
