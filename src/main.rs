mod reports;
use reports::report::*;
mod options;
use options::option::*;
mod sources;
use sources::source::*;
use std::env;
use std::io::{stderr, stdout, ErrorKind, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let source: CcSource;
    let empty_path = String::new();
    if args.len() >= 3 {
        source = CcSource {
            source_type: &CcSourceType::File,
            file_path: &args[2],
        };
    } else if args.len() == 1 {
        source = CcSource {
            source_type: &CcSourceType::Stdin,
            file_path: &empty_path,
        };
    } else {
        if args[1].starts_with("-") {
            source = CcSource {
                source_type: &CcSourceType::Stdin,
                file_path: &empty_path,
            };
        } else {
            source = CcSource {
                source_type: &CcSourceType::File,
                file_path: &args[1],
            };
        }
    }
    match source.get_content() {
        Ok(content) => {
            let report = CcReport {
                content: &content,
                options: &CcOptions { raw: &args[1] },
            };
            return output(&report.count_string(), 0);
        }
        Err(error) => {
            return handle_error(error.kind());
        }
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
