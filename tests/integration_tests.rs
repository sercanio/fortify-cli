use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_password_generation() {
    let mut cmd = Command::cargo_bin("fortify-cli").unwrap();
    cmd.arg("password")
        .arg("--length")
        .arg("20")
        .arg("--no-copy") // Ensure we don't try to copy in CI/Test env
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^.{20}\nStrength:").unwrap());
}

#[test]
fn test_guid_generation() {
    let mut cmd = Command::cargo_bin("fortify-cli").unwrap();
    cmd.arg("guid")
        .arg("--version")
        .arg("v4")
        .arg("--no-copy")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f-]{36}").unwrap());
}

#[test]
fn test_secret_generation() {
    let mut cmd = Command::cargo_bin("fortify-cli").unwrap();
    cmd.arg("secret")
        .arg("--length")
        .arg("10")
        .arg("--encoding")
        .arg("hex")
        .arg("--no-copy")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{20}").unwrap());
}
