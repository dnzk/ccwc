use crate::ccwc_lib::options::Options;
use std::env::Args;

pub struct Config {
    raw: Vec<String>,
}

impl Config {
    /// Creates a `Config` struct from `Args`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ccwc::Config;
    /// use std::env;
    ///
    /// let config = Config::from(env::args());
    /// ```
    pub fn from(args: Args) -> Self {
        Config {
            raw: args.collect(),
        }
    }
}

impl Config {
    fn find_by_pattern<T>(args: &[T], f: fn(&T) -> bool) -> Option<&T> {
        let mut r: Option<&T> = None;
        for a in args.iter() {
            if f(a) {
                r = Some(a);
            }
        }
        r
    }

    /// Returns a possible file path from args, discounting index 0.
    /// It uses the OS's main separator ("\" for Windows, "/" for Unix-like systems, etc).
    pub fn possible_file_path(&self) -> Option<String> {
        let path_list = self.raw[1..].to_vec().clone();
        if let Some(path) = Self::find_by_pattern(&path_list, |p: &String| {
            p.contains(std::path::MAIN_SEPARATOR)
        }) {
            return Some(path.clone());
        }
        None
    }

    /// Returns the options supplied in arguments.
    pub fn options(&self) -> Vec<Options> {
        if let Some(options) = Self::find_by_pattern(&self.raw, |o: &String| o.starts_with('-')) {
            return Options::from(Some(options.clone()));
        }
        Options::from(None)
    }
}
