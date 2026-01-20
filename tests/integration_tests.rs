use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_password_generation() {
    let mut cmd = Command::cargo_bin("fortify-cli").unwrap();
    cmd.arg("password")
        .arg("--length")
        .arg("20")
        .arg("--no-copy")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^.{20}\nStrength:").unwrap());
}

#[test]
fn test_password_specific_flags() {
    let mut cmd = Command::cargo_bin("fortify-cli").unwrap();
    cmd.arg("password")
        .arg("--length")
        .arg("10")
        .arg("--numbers")
        .arg("--no-copy")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d{10}\nStrength:").unwrap());
}

#[test]
fn test_guid_generation() {
    let mut cmd = Command::cargo_bin("fortify-cli").unwrap();
    cmd.arg("guid")
        .arg("--v4")
        .arg("--no-copy")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f-]{36}").unwrap());
}

#[test]
fn test_guid_v7_generation() {
    let mut cmd = Command::cargo_bin("fortify-cli").unwrap();
    cmd.arg("guid")
        .arg("--v7")
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
        .arg("--hex")
        .arg("--no-copy")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{20}").unwrap());
}

#[test]
fn test_secret_base64_generation() {
    let mut cmd = Command::cargo_bin("fortify-cli").unwrap();
    cmd.arg("secret")
        .arg("--length")
        .arg("10")
        .arg("--base64")
        .arg("--no-copy")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[A-Za-z0-9+/=]+").unwrap());
}
