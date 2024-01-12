use ccwc::{handle_error, output, CcArgs, Report, Source};
use std::env;
use std::process::ExitCode;

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
