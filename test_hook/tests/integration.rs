use assert_cmd::{Command, assert::Assert};
use std::path::Path;
use test_hook::{test, test_hook};

#[test_hook]
fn test_hook(command: &mut std::process::Command) {
    command.env_remove("CARGO_TERM_COLOR");
}

#[test]
fn additional_attrs() {
    let assert = test_fixture("additional_attrs").success();
    let stderr = str::from_utf8(&assert.get_output().stderr).unwrap();
    let stderr_normalized = stderr.replace('\\', "/");
    assert!(stderr_normalized.contains(
        "\
warning: unreachable `pub` item
  --> tests/integration.rs:10:5
   |
10 |     pub mod foo {}
   |     ---^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates
note: the lint level is defined here
  --> tests/integration.rs:7:8
   |
 7 | #[warn(unreachable_pub)]
   |        ^^^^^^^^^^^^^^^

warning: unreachable `pub` item
  --> tests/integration.rs:16:5
   |
16 |     pub mod foo {}
   |     ---^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates
note: the lint level is defined here
  --> tests/integration.rs:14:8
   |
14 | #[warn(unreachable_pub)]
   |        ^^^^^^^^^^^^^^^

warning: `additional_attrs` (test \"integration\") generated 2 warnings (run `cargo fix --test \
         \"integration\" -p additional_attrs` to apply 2 suggestions)
",
    ));
}

#[test]
fn basic() {
    let prefix = test_result_prefix(3);
    let assert = test_fixture("basic").success();
    let stdout = str::from_utf8(&assert.get_output().stdout).unwrap();
    let lines = stdout.lines();
    let count = lines.filter(|&line| line.starts_with(&prefix)).count();
    assert_eq!(1, count);
}

#[cfg_attr(
    windows,
    ignore = "on Windows, `abort` causes the child to exit with a code rather than a signal"
)]
#[test]
fn child_aborts() {
    let assert = test_fixture("child_aborts").failure();
    assert_eq!(Some(101), assert.get_output().status.code());
    let stderr = str::from_utf8(&assert.get_output().stderr).unwrap();
    // Whether " (core dumped)" appears depends on the system's core dump settings.
    let stderr_normalized = stderr.replace(" (core dumped)", "");
    assert!(stderr_normalized.ends_with(
        "\
test_hook: test executable terminated abnormally: signal: 6 (SIGABRT)
error: test failed, to rerun pass `--test integration`
",
    ));
}

#[test]
fn multiple_test_hooks() {
    let assert = test_fixture("multiple_test_hooks").failure();
    let stderr = str::from_utf8(&assert.get_output().stderr).unwrap();
    let stderr_normalized = stderr.replace('\\', "/");
    assert!(stderr_normalized.ends_with(
        "\
error[E0428]: the name `__test_hook__` is defined multiple times
 --> tests/integration.rs:7:1
  |
4 | #[test_hook]
  | ------------ previous definition of the value `__test_hook__` here
...
7 | #[test_hook]
  | ^^^^^^^^^^^^ `__test_hook__` redefined here
  |
  = note: `__test_hook__` must be defined only once in the value namespace of this module
  = note: this error originates in the attribute macro `test_hook` (in Nightly builds, run with -Z \
         macro-backtrace for more info)

For more information about this error, try `rustc --explain E0428`.
error: could not compile `multiple_test_hooks` (test \"integration\") due to 1 previous error
",
    ));
}

#[test]
fn no_test_hook() {
    let assert = test_fixture("no_test_hook").failure();
    let stderr = str::from_utf8(&assert.get_output().stderr).unwrap();
    let stderr_normalized = stderr.replace('\\', "/");
    assert!(stderr_normalized.ends_with(
        "\
error[E0425]: cannot find value `__test_hook__` in the crate root
 --> tests/integration.rs:3:1
  |
3 | #[test]
  | ^^^^^^^ not found in the crate root
  |
  = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z \
         macro-backtrace for more info)

For more information about this error, try `rustc --explain E0425`.
error: could not compile `no_test_hook` (test \"integration\") due to 1 previous error
",
    ));
}

#[test]
fn not_at_crate_root() {
    let assert = test_fixture("not_at_crate_root").failure();
    let stderr = str::from_utf8(&assert.get_output().stderr).unwrap();
    let stderr_normalized = stderr.replace('\\', "/");
    assert!(stderr_normalized.ends_with(
        "\
error[E0425]: cannot find value `__test_hook__` in the crate root
 --> tests/integration.rs:8:5
  |
8 |     #[test]
  |     ^^^^^^^ not found in the crate root
  |
  = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z \
         macro-backtrace for more info)

For more information about this error, try `rustc --explain E0425`.
error: could not compile `not_at_crate_root` (test \"integration\") due to 1 previous error
",
    ));
}

#[test]
fn panic_in_test_hook() {
    let assert = test_fixture("panic_in_test_hook").failure();
    assert_eq!(Some(101), assert.get_output().status.code());
    let stderr = str::from_utf8(&assert.get_output().stderr).unwrap();
    let stderr_normalized = stderr.replace('\\', "/");
    assert!(stderr_normalized.ends_with(
        "\
test_hook: test hook panicked: something went wrong
error: test failed, to rerun pass `--test integration`
",
    ));
}

#[test]
fn unconventional_name() {
    let prefix = test_result_prefix(1);
    let assert = test_fixture("unconventional_name").success();
    let stdout = str::from_utf8(&assert.get_output().stdout).unwrap();
    let mut lines = stdout.lines();
    assert!(lines.any(|line| line.starts_with(&prefix)));
}

#[test]
fn wrong_signature() {
    let assert = test_fixture("wrong_signature").failure();
    let stderr = str::from_utf8(&assert.get_output().stderr).unwrap();
    let stderr_normalized = stderr.replace('\\', "/");
    assert!(stderr_normalized.ends_with(
        "\
error[E0308]: mismatched types
 --> tests/integration.rs:5:4
  |
4 | #[test_hook]
  | ------------ expected because of the type of the constant
5 | fn test_hook(_command: &Command) {}
  |    ^^^^^^^^^ types differ in mutability
  |
  = note: expected fn pointer `for<\'a> fn(&\'a mut Command)`
                found fn item `for<\'a> fn(&\'a Command) {test_hook}`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `wrong_signature` (test \"integration\") due to 1 previous error
",
    ));
}

/// Runs the fixture's `tests/integration.rs`. Building only that target prevents Cargo from
/// compiling other targets in parallel. Otherwise, depending on timing, a compile error could be
/// followed by "warning: build failed, waiting for other jobs to finish...", which would break the
/// `ends_with` assertions for fixtures that intentionally fail to compile.
fn test_fixture(name: &str) -> Assert {
    Command::new("cargo")
        .args(["test", "--test=integration"])
        .current_dir(Path::new("../fixtures").join(name))
        .assert()
}

fn test_result_prefix(n_passed: usize) -> String {
    format!(
        "test result: ok. {n_passed} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; \
         finished in "
    )
}
