// smoelius: `call_test_hook` is called at the beginning of every test. If `call_test_hook` were to
// panic, the panic would be indistinguishable from one originating in the test itself. So a test
// marked `should_panic` could be considered successful, for example. Furthermore, the parent's test
// harness would continue running rather than exit, and if the panicking test held `APERTURE`,
// another test could be let through. Report errors with `fail` instead.
#![deny(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use std::{
    env::{args_os, current_exe, var_os},
    ffi::{OsStr, OsString},
    io::{Write, stderr, stdout},
    panic::{AssertUnwindSafe, catch_unwind},
    path::Path,
    process::{Command, exit},
    sync::Mutex,
};

// https://github.com/rust-lang/rust/blob/12397e9dd5a97460d76c884d449ca1c2d26da8ed/src/libtest/lib.rs#L93-L94
// Process exit code to be used to indicate test failures.
const ERROR_EXIT_CODE: i32 = 101;

pub type TestHook = fn(&mut Command);

/// Allows the first caller through; all others wait and are eventually killed.
static APERTURE: Mutex<()> = Mutex::new(());

pub fn call_test_hook(test_hook: TestHook) {
    let current_exe = current_exe()
        .unwrap_or_else(|error| fail(&format!("failed to get current executable: {error}")));

    let test_hook_child_key = test_hook_child_key(&current_exe);

    if enabled(&test_hook_child_key) {
        return;
    }

    let _lock = APERTURE
        .lock()
        .unwrap_or_else(|error| fail(&format!("failed to acquire lock: {error}")));

    let mut command = Command::new(&current_exe);
    let args = args_os().skip(1);
    command.args(args);

    // smoelius: `AssertUnwindSafe` is appropriate here because `command` is exclusively borrowed,
    // and the process exits immediately without reusing it.
    if let Err(payload) = catch_unwind(AssertUnwindSafe(|| test_hook(&mut command))) {
        let message = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
            .unwrap_or("<non-string payload>");
        fail(&format!("test hook panicked: {message}"));
    }

    // smoelius: Enable the child key _after_ the `test_hook` function is called so that the
    // function cannot remove the key.
    command.env(test_hook_child_key, "1");

    // smoelius: Take the stdout lock to prevent the parent's test harness from printing "test ...
    // has been running for over 60 seconds" warnings while the child runs. The child has its own
    // stdout, so it is unaffected. The lock is acquired _after_ the `test_hook` function returns so
    // that a thread spawned by the function cannot cause a deadlock by printing to stdout. Note
    // that if the `test_hook` function takes more than 60 seconds, the warnings may be printed
    // nonetheless.
    let _stdout = stdout().lock();

    let status = command.status().unwrap_or_else(|error| {
        fail(&format!(
            "failed to run `{}`: {error}",
            current_exe.display()
        ))
    });

    // smoelius: `code` returns `None` if the child was terminated by a signal.
    let code = status
        .code()
        .unwrap_or_else(|| fail(&format!("test executable terminated abnormally: {status}")));

    // smoelius: Exit the parent process to prevent its test messages from being printed.
    exit(code);
}

/// The returned `OsString` names an environment variable to indicate whether the current process is
/// a child process. Tests should be run in the child only.
fn test_hook_child_key(current_exe: &Path) -> OsString {
    let filename = current_exe.file_name().unwrap_or_else(|| {
        fail(&format!(
            "current executable has no file name: {}",
            current_exe.display()
        ))
    });
    let mut key = OsString::from("TEST_HOOK_CHILD_");
    key.push(filename);
    key
}

/// Reports an error and exits without returning to the test harness, so that a calling test
/// marked `should_panic` cannot be considered successful.
fn fail(message: &str) -> ! {
    // smoelius: Write to `stderr()` directly rather than use `eprintln!`, whose output libtest
    // captures.
    let _ = writeln!(stderr(), "test_hook: {message}");
    exit(ERROR_EXIT_CODE);
}

fn enabled(key: &OsStr) -> bool {
    var_os(key).is_some_and(|value| value != "0")
}
