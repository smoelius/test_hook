use std::process::Command;
use test_hook::{test, test_hook};

#[test_hook]
fn test_hook(_command: &mut Command) {}

#[warn(unreachable_pub)]
#[test]
fn additional_attr_before() {
    pub mod foo {}
}

#[test]
#[warn(unreachable_pub)]
fn additional_attr_after() {
    pub mod foo {}
}
