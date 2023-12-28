pub mod option {
    const C: &str = "c";
    const L: &str = "l";
    const M: &str = "m";
    const W: &str = "w";

    pub enum CcOptions {
        Bytes,
        Characters,
        Lines,
        Words,
    }

    pub struct CcOptionsBuilder<'a> {
        pub raw: &'a String,
    }

    impl<'a> CcOptionsBuilder<'a> {
        pub fn encode(&self) -> Vec<CcOptions> {
            if self.raw.trim().len() > 0 && self.raw.starts_with("-") {
                let options: Vec<&str> = self.raw.trim().split("").collect();
                let mut cc_options: Vec<CcOptions> = vec![];
                for o in options.iter() {
                    match o.to_lowercase().as_str() {
                        C => cc_options.push(CcOptions::Bytes),
                        L => cc_options.push(CcOptions::Lines),
                        M => cc_options.push(CcOptions::Characters),
                        W => cc_options.push(CcOptions::Words),
                        _ => {}
                    }
                }
                return cc_options;
            }
            vec![CcOptions::Bytes, CcOptions::Lines, CcOptions::Words]
        }
    }
}
