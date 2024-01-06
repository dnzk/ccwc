pub mod source {
    use std::fs::File;
    use std::io::{BufRead, Error, Read};

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
        fn read_file(file_path: &String) -> Result<String, Error> {
            match File::open(file_path) {
                Ok(mut file) => {
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;
                    Ok(content)
                }
                Err(error) => Err(error),
            }
        }

        fn read_from_stdin() -> Result<String, Error> {
            let stdin = std::io::stdin();
            let mut content = String::new();
            for line in stdin.lock().lines() {
                match line {
                    Ok(l) => {
                        content.push_str(l.as_str());
                        content.push('\n');
                    }
                    _ => (),
                }
            }
            Ok(content)
        }

        pub fn get_content(&self) -> Result<String, Error> {
            match self {
                Self::File(file_path) => Self::read_file(file_path),
                Self::Stdin => Self::read_from_stdin(),
            }
        }
    }
}
