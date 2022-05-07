use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn fail_if_no_params_provided() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn fail_if_file_not_found() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-t", "file_that_doesnt_exist.tar"])
        .assert()
        .failure();
}

#[test]
fn succeed_if_file_found() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["-t", "test-files/single-text-file.tar"])
        .assert()
        .success();
}
