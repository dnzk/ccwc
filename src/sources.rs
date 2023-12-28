pub mod source {
    use std::fs::File;
    use std::io::{BufRead, Error, Read};

    pub enum CcSourceType {
        File,
        Stdin,
    }

    pub struct CcSource<'a> {
        pub source_type: &'a CcSourceType,
        pub file_path: &'a String,
    }

    impl<'a> CcSource<'a> {
        pub fn get_content(&self) -> Result<String, Error> {
            match self.source_type {
                CcSourceType::File => match File::open(self.file_path) {
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
