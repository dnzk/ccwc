use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stderr, stdin, stdout, BufRead, Write};
use std::io::{Error, ErrorKind};
use std::process::ExitCode;

#[derive(Debug)]
enum CcOptions {
    Bytes,
    Words,
    Lines,
    Characters,
}

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
            let report = generate_report(&content, &parse_options(options)).join(" ");
            return output(&report, 0);
        }
        Err(error) => return handle_file_error(error.kind()),
    }
}

fn generate_report_from_stdin(options: &String) -> ExitCode {
    let content = get_content_from_stdin();
    let options = parse_options(options);
    let report = generate_report(&content, &options).join(" ");
    return output(&report, 0);
}

fn generate_report(content: &String, options: &Vec<CcOptions>) -> Vec<String> {
    let mut report: Vec<String> = vec![];
    for option in options.iter() {
        match option {
            CcOptions::Lines => report.push(format!("{} lines", count_lines(&content))),
            CcOptions::Characters => {
                report.push(format!("{} characters", count_characters(&content)))
            }
            CcOptions::Words => report.push(format!("{} words", count_words(&content))),
            CcOptions::Bytes => report.push(format!("{} bytes", count_bytes(&content))),
        }
    }
    report
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

fn count_bytes(contents: &String) -> usize {
    contents.len()
}

fn count_lines(contents: &String) -> usize {
    let lines: Vec<&str> = contents.lines().collect();
    lines.len()
}

fn count_characters(contents: &String) -> usize {
    contents.chars().count()
}

fn count_words(contents: &String) -> usize {
    contents.split_whitespace().count()
}

fn parse_options(options: &String) -> Vec<CcOptions> {
    if options.trim().len() > 0 {
        let options: Vec<&str> = options.trim().split("").collect();
        let mut cc_options: Vec<CcOptions> = vec![];
        for i in options.iter() {
            match i.to_lowercase().as_str() {
                "c" => cc_options.push(CcOptions::Bytes),
                "w" => cc_options.push(CcOptions::Words),
                "l" => cc_options.push(CcOptions::Lines),
                "m" => cc_options.push(CcOptions::Characters),
                _ => {}
            }
        }
        return cc_options;
    }
    return vec![CcOptions::Bytes, CcOptions::Lines, CcOptions::Words];
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
