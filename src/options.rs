pub mod option {
    const C: &str = "c";
    const L: &str = "l";
    const M: &str = "m";
    const W: &str = "w";

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
                    if options.trim().len() > 0 && options.starts_with("-") {
                        let options: Vec<&str> = options.trim().split("").collect();
                        let mut res: Vec<Self> = vec![];
                        for o in options.iter() {
                            match o.to_lowercase().as_str() {
                                C => res.push(Self::Bytes),
                                L => res.push(Self::Lines),
                                M => res.push(Self::Characters),
                                W => res.push(Self::Words),
                                _ => {}
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
    }
}
