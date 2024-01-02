pub mod source {
    use std::fs::File;
    use std::io::{BufRead, Error, Read};

    pub enum CcSourceType {
        File,
        Stdin,
    }

    pub struct CcSource {
        pub source_type: CcSourceType,
        pub file_path: Box<String>,
    }

    impl<'a> CcSource {
        pub fn from(file_path: Option<String>) -> Self {
            match file_path {
                None => CcSource {
                    source_type: CcSourceType::Stdin,
                    file_path: Box::from(String::from("")),
                },
                Some(path) => CcSource {
                    source_type: CcSourceType::File,
                    file_path: Box::from(path),
                },
            }
        }

        pub fn get_content(&self) -> Result<String, Error> {
            let file_path = *self.file_path.clone();
            match self.source_type {
                CcSourceType::File => match File::open(file_path) {
                    Ok(mut file) => {
                        let mut content = String::new();
                        file.read_to_string(&mut content)?;
                        return Ok(content);
                    }
                    Err(error) => return Err(error),
                },
                CcSourceType::Stdin => {
                    let stdin = std::io::stdin();
                    let mut content = String::new();
                    for line in stdin.lock().lines() {
                        content.push_str(line.unwrap().as_str());
                        content.push('\n');
                    }
                    return Ok(content);
                }
            }
        }
    }
}
