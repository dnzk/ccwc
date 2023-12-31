mod args;
mod options;
mod reports;
mod sources;
use args::args::CcArgs;
use reports::report::*;
use sources::source::*;
use std::env;
use std::io::{stderr, stdout, ErrorKind, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = CcArgs::from(env::args());
    let source = CcSource::from(args.file_path());
    match source.get_content() {
        Ok(content) => {
            let report = CcReport {
                content: &content,
                options: &args.options(),
            };
            return output(&report.count_string(), 0);
        }
        Err(error) => return handle_error(error.kind()),
    }
}

fn handle_error(error: ErrorKind) -> ExitCode {
    let msg = match error {
        ErrorKind::NotFound => "File not found",
        ErrorKind::PermissionDenied => "Insufficient access to file",
        _ => "Fatal error",
    };
    return output(msg, 1);
}

fn output(msg: &str, status: u8) -> ExitCode {
    if status == 0 {
        write_to_stdout(msg);
    } else {
        write_to_stderr(msg);
    }
    return ExitCode::from(status);
}

fn write_to_stdout(msg: &str) {
    let mut lock = stdout().lock();
    writeln!(lock, "{}", msg).expect("Fatal error");
}

fn write_to_stderr(msg: &str) {
    let mut lock = stderr().lock();
    writeln!(lock, "{}", msg).expect("Fatal error");
}
