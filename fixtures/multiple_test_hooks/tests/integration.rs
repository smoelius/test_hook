use std::process::Command;
use test_hook::{test, test_hook};

#[test_hook]
fn first_test_hook(_command: &mut Command) {}

#[test_hook]
fn second_test_hook(_command: &mut Command) {}

#[test]
fn test() {}
