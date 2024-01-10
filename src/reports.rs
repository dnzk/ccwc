pub mod report {
    use crate::options::option::*;

    pub struct Report {
        content: String,
    }

    impl Report {
        pub fn from(content: String) -> Self {
            Self { content }
        }
    }

    impl Report {
        pub fn count(&self, options: &Vec<Options>) -> Vec<(usize, Options)> {
            let mut result: Vec<(usize, Options)> = vec![];
            for option in options.iter() {
                match option {
                    Options::Bytes => result.push((self.content.len(), Options::Bytes)),
                    Options::Characters => {
                        result.push((self.content.chars().count(), Options::Characters))
                    }
                    Options::Lines => result.push((self.content.lines().count(), Options::Lines)),
                    Options::Words => {
                        result.push((self.content.split_whitespace().count(), Options::Words))
                    }
                }
            }
            result
        }

        pub fn count_string(&self, options: &Vec<Options>) -> String {
            let mut result: Vec<String> = vec![];
            for c in self.count(options).iter() {
                match c.1 {
                    Options::Bytes => result.push(format!("{} bytes", c.0)),
                    Options::Characters => result.push(format!("{} characters", c.0)),
                    Options::Lines => result.push(format!("{} lines", c.0)),
                    Options::Words => result.push(format!("{} words", c.0)),
                }
            }
            result.join(" ")
        }
    }
}
