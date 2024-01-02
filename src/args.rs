pub mod args {
    use super::super::options::option::*;
    use std::convert::*;
    use std::env::Args;
    use std::io::{Error, ErrorKind};
    use std::path::Path;

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

    impl<'a> CcArgs {
        fn find_by_pattern<T>(args: &Vec<T>, f: fn(&T) -> bool) -> Option<&T> {
            let mut r: Option<&T> = None;
            for a in args.iter() {
                if f(a) {
                    r = Some(a);
                }
            }
            r
        }

        pub fn file_path(&self) -> Result<Option<String>, Error> {
            let is_valid_path = |path: &String| -> bool { Path::new(path).is_file() };
            let path_list = self.raw[1..].to_vec().clone();
            match Self::find_by_pattern(&path_list, is_valid_path) {
                Some(path) => Ok(Some(path.clone())),
                _ => Err(Error::new(ErrorKind::NotFound, "File not found")),
            }
        }

        pub fn options(&self) -> Vec<Options> {
            let is_valid_options = |o: &String| -> bool { o.starts_with("-") };
            match Self::find_by_pattern(&self.raw, is_valid_options) {
                None => Options::from(None),
                Some(options) => Options::from(Some(String::from(options.clone()))),
            }
        }
    }
}
