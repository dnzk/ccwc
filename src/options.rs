pub mod option {
    const C: &str = "c";
    const L: &str = "l";
    const M: &str = "m";
    const W: &str = "w";

    pub enum CcOptionsType {
        Bytes,
        Characters,
        Lines,
        Words,
    }

    pub struct CcOptions {
        raw: String,
    }

    impl<'a> CcOptions {
        pub fn from(option: String) -> CcOptions {
            CcOptions { raw: option }
        }
        fn default_options() -> Vec<CcOptionsType> {
            vec![
                CcOptionsType::Bytes,
                CcOptionsType::Lines,
                CcOptionsType::Words,
            ]
        }
        pub fn encode(&self) -> Vec<CcOptionsType> {
            if self.raw.trim().len() > 0 && self.raw.starts_with("-") {
                let options: Vec<&str> = self.raw.trim().split("").collect();
                let mut cc_options: Vec<CcOptionsType> = vec![];
                for o in options.iter() {
                    match o.to_lowercase().as_str() {
                        C => cc_options.push(CcOptionsType::Bytes),
                        L => cc_options.push(CcOptionsType::Lines),
                        M => cc_options.push(CcOptionsType::Characters),
                        W => cc_options.push(CcOptionsType::Words),
                        _ => {}
                    }
                }
                return cc_options;
            }
            Self::default_options()
        }
    }
}
