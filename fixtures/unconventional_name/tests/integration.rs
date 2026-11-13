use std::{env::var, process::Command};
use test_hook::{test, test_hook};

const TEST_HOOK_UNCONVENTIONAL_NAME: &str = "TEST_HOOK_UNCONVENTIONAL_NAME";

#[test_hook]
fn unconventional_name(command: &mut Command) {
    command.env(TEST_HOOK_UNCONVENTIONAL_NAME, "1");
}

#[test]
fn test() {
    assert_eq!("1", var(TEST_HOOK_UNCONVENTIONAL_NAME).unwrap());
}
