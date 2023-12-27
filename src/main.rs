use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::{stderr, stdout, Write};
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
    let args = parse_arguments(&args);
    let options = args.0;
    let file_path = args.1;
    match File::open(&file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut report: Vec<String> = vec![];
            for o in options.iter() {
                match o {
                    CcOptions::Lines => {
                        let result = format!("{} lines", count_lines(&contents));
                        report.push(result);
                    }
                    CcOptions::Characters => {
                        let result = format!("{} characters", count_characters(&contents));
                        report.push(result);
                    }
                    CcOptions::Words => {
                        let result = format!("{} words", count_words(&contents));
                        report.push(result);
                    }
                    CcOptions::Bytes => {
                        let result = format!("{} bytes", count_bytes(&contents));
                        report.push(result);
                    }
                }
            }
            let report = report.join(" ");
            return output(&report, 0);
        }
        Err(error) => handle_file_error(error.kind()),
    }
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

fn parse_arguments(args: &Vec<String>) -> (Vec<CcOptions>, &String) {
    if args[1].starts_with("-") {
        return (parse_options(&args[1]), &args[2]);
    }
    return (parse_options(&String::new()), &args[1]);
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
