pub mod report {
    use super::super::options::option::CcOptions;

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
            let mut result: Vec<String> = vec![];
            for c in self.count().iter() {
                match c.1 {
                    CcOptions::Bytes => result.push(format!("{} bytes", c.0)),
                    CcOptions::Characters => result.push(format!("{} characters", c.0)),
                    CcOptions::Lines => result.push(format!("{} lines", c.0)),
                    CcOptions::Words => result.push(format!("{} words", c.0)),
                }
            }
            result.join(" ")
        }
    }
}
