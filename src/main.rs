mod args;
mod options;
mod reports;
mod sources;
use args::args::CcArgs;
use reports::report::*;
use sources::source::*;
use std::env;
use std::io::ErrorKind;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = CcArgs::from(env::args());
    match args.file_path() {
        Ok(file_path) => {
            let source = Source::from(file_path);
            match source.get_content() {
                Ok(content) => {
                    let report = Report::from(content);
                    output(report.count_string(&args.options()), 0)
                }
                Err(error) => handle_error(error.kind()),
            }
        }
        Err(error) => handle_error(error.kind()),
    }
}

fn handle_error(error: ErrorKind) -> ExitCode {
    let msg = match error {
        ErrorKind::NotFound => "File not found",
        ErrorKind::PermissionDenied => "Insufficient access to file",
        _ => "Fatal error",
    };
    output(msg.to_string(), 1)
}

fn output(msg: String, status: u8) -> ExitCode {
    if status == 0 {
        println!("{}", msg);
    } else {
        eprintln!("{}", msg);
    }
    ExitCode::from(status)
}
