pub mod args {
    use super::super::options::option::*;
    use std::convert::*;
    use std::env::Args;
    use std::path::Path;

    pub struct CcArgs {
        raw: Vec<String>,
    }

    impl<'a> CcArgs {
        fn is_valid_path(file_path: &String) -> bool {
            Path::new(file_path).exists()
        }

        pub fn from(args: Args) -> CcArgs {
            return CcArgs {
                raw: args.collect(),
            };
        }

        fn find_file_path(args: &Vec<String>) -> Option<String> {
            let mut r: Option<String> = None;
            for a in args.iter() {
                if Self::is_valid_path(&a) {
                    r = Some(a.to_string());
                }
            }
            r
        }

        fn find_options(args: &Vec<String>) -> Option<String> {
            let mut r: Option<String> = None;
            for a in args.iter() {
                if a.starts_with("-") {
                    r = Some(a.to_string());
                }
            }
            r
        }

        pub fn file_path(&self) -> Option<String> {
            Self::find_file_path(&self.raw[1..].to_vec())
        }

        pub fn options(&self) -> CcOptions {
            let options = Self::find_options(&self.raw);
            match options {
                None => return CcOptions::from(String::from("")),
                Some(options) => return CcOptions::from(options),
            }
        }
    }
}
