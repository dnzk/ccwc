use assert_cmd::Command;

#[test]
fn shows_lines_count_with_l() {
    let mut cmd = Command::cargo_bin("ccwc").unwrap();
    cmd.arg("-l")
        .arg("./test_files/one.txt")
        .assert()
        .stdout("6 lines\n");
}

#[test]
fn shows_bytes_count_with_c() {
    let mut cmd = Command::cargo_bin("ccwc").unwrap();
    cmd.arg("-c")
        .arg("./test_files/one.txt")
        .assert()
        .stdout("158 bytes\n");
}

#[test]
fn shows_characters_count_with_m() {
    let mut cmd = Command::cargo_bin("ccwc").unwrap();
    cmd.arg("-m")
        .arg("./test_files/one.txt")
        .assert()
        .stdout("158 characters\n");
}

#[test]
fn shows_words_count_with_w() {
    let mut cmd = Command::cargo_bin("ccwc").unwrap();
    cmd.arg("-w")
        .arg("./test_files/one.txt")
        .assert()
        .stdout("33 words\n");
}

#[test]
fn shows_clw_by_default() {
    let mut cmd = Command::cargo_bin("ccwc").unwrap();
    cmd.arg("./test_files/one.txt")
        .assert()
        .stdout("158 bytes 6 lines 33 words\n");
}

#[test]
fn does_not_duplicate_options() {
    let mut cmd = Command::cargo_bin("ccwc").unwrap();
    cmd.arg("./test_files/one.txt")
        .arg("-lllll")
        .assert()
        .stdout("6 lines\n");

    cmd.arg("./test_files/one.txt")
        .arg("-llwwllcccclll")
        .assert()
        .stdout("158 bytes 6 lines 33 words\n");
}

#[test]
fn errors_when_file_not_found() {
    let mut cmd = Command::cargo_bin("ccwc").unwrap();
    cmd.arg("./test_files/doesnt_exist.txt")
        .assert()
        .stderr("File not found\n");
}

#[test]
fn supports_piping() {
    Command::new("./target/debug/ccwc")
        .pipe_stdin("./test_files/one.txt")
        .unwrap()
        .assert()
        .stdout("158 bytes 6 lines 33 words\n");
}
