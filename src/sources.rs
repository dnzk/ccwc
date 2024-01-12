use std::io::{BufRead, Error};

pub enum Source {
    File(String),
    Stdin,
}

impl Source {
    pub fn from(file_path: Option<String>) -> Self {
        match file_path {
            None => Self::Stdin,
            Some(file_path) => Self::File(file_path),
        }
    }
}

impl Source {
    fn read_from_stdin() -> Result<String, Error> {
        let stdin = std::io::stdin();
        let mut content = String::new();
        for line in stdin.lock().lines().flatten() {
            content.push_str(line.as_str());
            content.push('\n');
        }
        Ok(content)
    }

    pub fn get_content(&self) -> Result<String, Error> {
        match self {
            Self::File(file_path) => std::fs::read_to_string(file_path),
            Self::Stdin => Self::read_from_stdin(),
        }
    }
}
