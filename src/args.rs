pub mod args {
    use super::super::options::option::*;
    use std::convert::*;
    use std::env::Args;
    use std::path::Path;

    pub struct CcArgs {
        raw: Vec<String>,
    }

    impl<'a> CcArgs {
        pub fn from(args: Args) -> CcArgs {
            return CcArgs {
                raw: args.collect(),
            };
        }

        fn find_by_pattern<T>(args: &Vec<T>, f: fn(&T) -> bool) -> Option<&T> {
            let mut r: Option<&T> = None;
            for a in args.iter() {
                if f(a) {
                    r = Some(a);
                }
            }
            r
        }

        pub fn file_path(&self) -> Option<String> {
            let is_valid_path = |path: &String| -> bool { Path::new(path).is_file() };
            let path_list = self.raw[1..].to_vec().clone();
            match Self::find_by_pattern(&path_list, is_valid_path) {
                Some(path) => Some(path.clone()),
                _ => None,
            }
        }

        pub fn options(&self) -> CcOptions {
            let is_valid_options = |o: &String| -> bool { o.starts_with("-") };
            match Self::find_by_pattern(&self.raw, is_valid_options) {
                None => CcOptions::from(String::from("")),
                Some(options) => CcOptions::from(options.clone()),
            }
        }
    }
}
