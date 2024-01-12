use ccwc::{handle_error, output, Config, Report, Source};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let config = Config::from(env::args());
    let source = Source::from(config.possible_file_path());
    match source.get_content() {
        Ok(content) => {
            let report = Report::from(content);
            output(report.count_string(&config.options()), 0)
        }
        Err(error) => handle_error(error.kind()),
    }
}
