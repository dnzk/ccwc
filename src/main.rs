mod reports;
use reports::report::*;
mod options;
use options::option::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stderr, stdin, stdout, BufRead, Write};
use std::io::{Error, ErrorKind};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        return generate_report_from_file(&args[2], &args[1]);
    } else if args.len() == 1 {
        return generate_report_from_stdin(&String::new());
    } else {
        if args[1].starts_with("-") {
            return generate_report_from_stdin(&args[1]);
        } else {
            return generate_report_from_file(&args[1], &String::new());
        }
    }
}

fn generate_report_from_file(file_path: &String, options: &String) -> ExitCode {
    match get_content_from_file(file_path) {
        Ok(content) => {
            let report = CcReport {
                content: &content,
                options: &CcOptionsBuilder { raw: options },
            };
            return output(&report.count_string(), 0);
        }
        Err(error) => return handle_file_error(error.kind()),
    }
}

fn generate_report_from_stdin(options: &String) -> ExitCode {
    let content = get_content_from_stdin();
    let report = CcReport {
        content: &content,
        options: &CcOptionsBuilder { raw: options },
    };
    return output(&report.count_string(), 0);
}

fn get_content_from_file(file_path: &String) -> Result<String, Error> {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            return Ok(content);
        }
        Err(error) => return Err(error),
    }
}

fn get_content_from_stdin() -> String {
    let stdin = stdin();
    let mut content = String::new();
    for line in stdin.lock().lines() {
        content.push_str(line.unwrap().as_str());
        content.push('\n');
    }
    content
}

fn handle_file_error(error: ErrorKind) -> ExitCode {
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
