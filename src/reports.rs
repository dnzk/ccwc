pub mod report {
    pub enum CcOptions {
        Bytes,
        Characters,
        Lines,
        Words,
    }

    pub struct CcReport<'a> {
        pub options: &'a Vec<CcOptions>,
        pub content: &'a String,
    }

    impl<'a> CcReport<'a> {
        pub fn count(&self) -> Vec<(usize, CcOptions)> {
            let mut result: Vec<(usize, CcOptions)> = vec![];
            for option in self.options.iter() {
                match option {
                    CcOptions::Bytes => result.push((self.content.len(), CcOptions::Bytes)),
                    CcOptions::Characters => {
                        result.push((self.content.chars().count(), CcOptions::Characters))
                    }
                    CcOptions::Lines => {
                        result.push((self.content.lines().count(), CcOptions::Lines))
                    }
                    CcOptions::Words => {
                        result.push((self.content.split_whitespace().count(), CcOptions::Words))
                    }
                }
            }
            result
        }

        pub fn count_string(&self) -> String {
            let mut result = String::new();
            for c in self.count().iter() {
                match c.1 {
                    CcOptions::Bytes => result.push_str(format!("{} bytes", c.0).as_str()),
                    CcOptions::Characters => {
                        result.push_str(format!("{} characters", c.0).as_str())
                    }
                    CcOptions::Lines => result.push_str(format!("{} lines", c.0).as_str()),
                    CcOptions::Words => result.push_str(format!("{} words", c.0).as_str()),
                }
            }
            result
        }
    }
}
