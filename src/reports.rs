pub mod report {
    use super::super::options::option::{CcOptions, CcOptionsType};

    pub struct CcReport<'a> {
        pub content: &'a String,
        pub options: &'a CcOptions,
    }

    impl<'a> CcReport<'a> {
        pub fn count(&self) -> Vec<(usize, CcOptionsType)> {
            let mut result: Vec<(usize, CcOptionsType)> = vec![];
            for option in self.options.encode().iter() {
                match option {
                    CcOptionsType::Bytes => result.push((self.content.len(), CcOptionsType::Bytes)),
                    CcOptionsType::Characters => {
                        result.push((self.content.chars().count(), CcOptionsType::Characters))
                    }
                    CcOptionsType::Lines => {
                        result.push((self.content.lines().count(), CcOptionsType::Lines))
                    }
                    CcOptionsType::Words => result.push((
                        self.content.split_whitespace().count(),
                        CcOptionsType::Words,
                    )),
                }
            }
            result
        }

        pub fn count_string(&self) -> String {
            let mut result: Vec<String> = vec![];
            for c in self.count().iter() {
                match c.1 {
                    CcOptionsType::Bytes => result.push(format!("{} bytes", c.0)),
                    CcOptionsType::Characters => result.push(format!("{} characters", c.0)),
                    CcOptionsType::Lines => result.push(format!("{} lines", c.0)),
                    CcOptionsType::Words => result.push(format!("{} words", c.0)),
                }
            }
            result.join(" ")
        }
    }
}
