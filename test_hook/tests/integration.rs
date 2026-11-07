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
  --> src/lib.rs:12:9
   |
12 |         pub mod foo {}
   |         ---^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates
note: the lint level is defined here
  --> src/lib.rs:9:12
   |
 9 |     #[warn(unreachable_pub)]
   |            ^^^^^^^^^^^^^^^

warning: unreachable `pub` item
  --> src/lib.rs:18:9
   |
18 |         pub mod foo {}
   |         ---^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates
note: the lint level is defined here
  --> src/lib.rs:16:12
   |
16 |     #[warn(unreachable_pub)]
   |            ^^^^^^^^^^^^^^^

warning: `additional_attrs` (lib test) generated 2 warnings (run `cargo fix --lib -p \
         additional_attrs --tests` to apply 2 suggestions)
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
error: test failed, to rerun pass `--lib`
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
error[E0425]: cannot find value `__test_hook__` in this scope
 --> src/lib.rs:5:5
  |
5 |     #[test]
  |     ^^^^^^^ not found in this scope
  |
  = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z \
         macro-backtrace for more info)

For more information about this error, try `rustc --explain E0425`.
error: could not compile `no_test_hook` (lib test) due to 1 previous error
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
error: test failed, to rerun pass `--lib`
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
 --> src/lib.rs:7:8
  |
6 |     #[test_hook]
  |     ------------ expected because of the type of the constant
7 |     fn test_hook(_command: &Command) {}
  |        ^^^^^^^^^ types differ in mutability
  |
  = note: expected fn pointer `for<\'a> fn(&\'a mut Command)`
                found fn item `for<\'a> fn(&\'a Command) {test_hook}`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `wrong_signature` (lib test) due to 1 previous error
",
    ));
}

fn test_fixture(name: &str) -> Assert {
    Command::new("cargo")
        .arg("test")
        .current_dir(Path::new("../fixtures").join(name))
        .assert()
}

fn test_result_prefix(n_passed: usize) -> String {
    format!(
        "test result: ok. {n_passed} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; \
         finished in "
    )
}
