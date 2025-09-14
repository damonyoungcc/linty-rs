use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn prints_location_when_console_log_exists() {
    // 临时文件当作输入
    let file = assert_fs::NamedTempFile::new("a.js").unwrap();
    file.write_str("let a=1;\nconsole.log('x');\n").unwrap();

    let mut cmd = Command::cargo_bin("linty").unwrap();
    cmd.arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(":2:1  no-console"));
}

#[test]
fn prints_ok_when_no_console_log() {
    let file = assert_fs::NamedTempFile::new("b.js").unwrap();
    file.write_str("let a=1;\nlet b=2;\n").unwrap();

    let mut cmd = Command::cargo_bin("linty").unwrap();
    cmd.arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ok (no console.log)"));
}
