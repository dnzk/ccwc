pub mod option {
    const C: &str = "c";
    const L: &str = "l";
    const M: &str = "m";
    const W: &str = "w";

    #[cfg_attr(test, derive(PartialEq, Debug))]
    pub enum Options {
        Bytes,
        Characters,
        Lines,
        Words,
    }

    impl Options {
        pub fn from(raw: Option<String>) -> Vec<Self> {
            match raw {
                None => Self::default_options(),
                Some(options) => {
                    if Self::is_valid_options(&options) {
                        let mut options: Vec<&str> = options.trim().split("").collect();
                        options.sort();
                        options.dedup();
                        let mut res: Vec<Self> = vec![];
                        for o in options.iter() {
                            match o.to_lowercase().as_str() {
                                C => res.push(Self::Bytes),
                                L => res.push(Self::Lines),
                                M => res.push(Self::Characters),
                                W => res.push(Self::Words),
                                _ => (),
                            }
                        }
                        return res;
                    }
                    Self::default_options()
                }
            }
        }

        fn default_options() -> Vec<Self> {
            vec![Self::Bytes, Self::Lines, Self::Words]
        }

        fn is_valid_options(options: &String) -> bool {
            options.trim().len() > 0 && options.starts_with("-")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::option::*;

    fn assert_default_options(options: Option<String>) {
        let options = Options::from(options);
        assert_eq!(options.len(), 3);
        assert!(options.contains(&Options::Bytes));
        assert!(options.contains(&Options::Lines));
        assert!(options.contains(&Options::Words));
        assert!(!options.contains(&Options::Characters));
    }

    #[test]
    fn generates_default_options_with_none() {
        assert_default_options(None);
    }

    #[test]
    fn generates_default_options_when_arguments_has_no_dash_prefix() {
        assert_default_options(Some("clw".to_string()));
    }

    #[test]
    fn generates_bytes_from_c() {
        let options = Options::from(Some("-c".to_string()));
        assert_eq!(options.len(), 1);
        assert_eq!(options[0], Options::Bytes);
    }

    #[test]
    fn generates_lines_from_l() {
        let options = Options::from(Some("-l".to_string()));
        assert_eq!(options.len(), 1);
        assert_eq!(options[0], Options::Lines);
    }

    #[test]
    fn generates_words_from_w() {
        let options = Options::from(Some("-w".to_string()));
        assert_eq!(options.len(), 1);
        assert_eq!(options[0], Options::Words);
    }

    #[test]
    fn generates_default_options_when_passed_in_empty_string() {
        assert_default_options(Some("".to_string()));
    }

    #[test]
    fn generates_default_options_when_passed_in_unrecognized_characters() {
        assert_default_options(Some("asdf".to_string()));
    }
}
