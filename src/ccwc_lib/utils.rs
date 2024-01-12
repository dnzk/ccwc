use std::io::ErrorKind;
use std::process::ExitCode;

pub fn handle_error(error: ErrorKind) -> ExitCode {
    let msg = match error {
        ErrorKind::NotFound => "File not found",
        ErrorKind::PermissionDenied => "Insufficient access to file",
        ErrorKind::InvalidInput => "Invalid input",
        _ => panic!("Fatal Error"),
    };
    output(msg.to_string(), 1)
}

pub fn output(msg: String, status: u8) -> ExitCode {
    if status == 0 {
        println!("{}", msg);
        return ExitCode::SUCCESS;
    }
    eprintln!("{}", msg);
    ExitCode::from(status)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "Fatal Error")]
    fn unknown_error_kind() {
        handle_error(std::io::ErrorKind::AddrInUse);
    }
}
