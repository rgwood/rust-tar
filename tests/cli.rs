use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn fail_if_no_params_provided() {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap().assert().failure();
}