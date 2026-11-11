use std::{
    env::{args, current_dir, temp_dir, var},
    process::Command,
};
use test_hook::{test, test_hook};

const TEST_HOOK_BASIC: &str = "TEST_HOOK_BASIC";

#[test_hook]
fn test_hook(command: &mut Command) {
    command.arg("--");
    command.env(TEST_HOOK_BASIC, "1");
    command.current_dir(temp_dir());
}

#[test]
fn arg_was_added() {
    assert_eq!("--", args().last().unwrap());
}

#[test]
fn environment_variable_was_set() {
    assert_eq!("1", var(TEST_HOOK_BASIC).unwrap());
}

#[test]
fn current_dir_changed() {
    assert_eq!(
        temp_dir().canonicalize().unwrap(),
        current_dir().unwrap().canonicalize().unwrap()
    );
}
