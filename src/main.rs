mod args;
mod options;
mod reports;
mod sources;
mod utils;
use args::arg::CcArgs;
use reports::report::*;
use sources::source::*;
use std::env;
use std::process::ExitCode;
use utils::helpers::{handle_error, output};

fn main() -> ExitCode {
    let args = CcArgs::from(env::args());
    let source = Source::from(args.possible_file_path());
    match source.get_content() {
        Ok(content) => {
            let report = Report::from(content);
            output(report.count_string(&args.options()), 0)
        }
        Err(error) => handle_error(error.kind()),
    }
}
