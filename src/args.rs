pub mod arg {
    use crate::options::option::*;
    use std::env::Args;

    pub struct CcArgs {
        raw: Vec<String>,
    }

    impl CcArgs {
        pub fn from(args: Args) -> Self {
            CcArgs {
                raw: args.collect(),
            }
        }
    }

    impl CcArgs {
        fn find_by_pattern<T>(args: &[T], f: fn(&T) -> bool) -> Option<&T> {
            let mut r: Option<&T> = None;
            for a in args.iter() {
                if f(a) {
                    r = Some(a);
                }
            }
            r
        }

        pub fn possible_file_path(&self) -> Option<String> {
            let maybe_file_path =
                |path: &String| -> bool { path.contains(std::path::MAIN_SEPARATOR) };
            let path_list = self.raw[1..].to_vec().clone();
            if let Some(path) = Self::find_by_pattern(&path_list, maybe_file_path) {
                return Some(path.clone());
            }
            None
        }

        pub fn options(&self) -> Vec<Options> {
            let is_valid_options = |o: &String| -> bool { o.starts_with('-') };
            match Self::find_by_pattern(&self.raw, is_valid_options) {
                None => Options::from(None),
                Some(options) => Options::from(Some(options.clone())),
            }
        }
    }
}
