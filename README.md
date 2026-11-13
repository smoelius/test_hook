# `test_hook`

Control how test executables are run

## Usage

In an integration test[^integration-test], import `test_hook::{test, test_hook}` and add the `test_hook` attribute to a function that takes a `&mut std::process::Command` as an argument at the crate root[^crate-root]. Before tests are run, the function will be called with a command to launch the test executable in a child process. The function can modify the command, e.g., by [adding arguments], [changing its working directory], or [adding] or [removing environment variables].

## Example

```rust
use std::process::Command;
use test_hook::{test, test_hook};

#[test_hook]
fn test_hook(command: &mut Command) {
    command.current_dir("..");
    command.env_remove("CARGO_TERM_COLOR");
}
```

To enable calling the hook, `test_hook` provides its own version of the `#[test]` attribute. The `test_hook` version must be used in place of the built-in attribute for every test in the executable.

## Unit tests

For unit tests, the hook must be placed in `src/lib.rs` or `src/main.rs`, outside of any module. If `test_hook` is a dev-dependency (as intended), the hook must also be preceded by `#[cfg(test)]`. Example:

```rust
#[cfg(test)]
#[test_hook::test_hook]
fn test_hook(command: &mut std::process::Command) {
    // ...
}
```

Like for integration tests, the `test_hook` version of the `#[test]` attribute must be used for every test in the executable, i.e., every unit test in the crate.

## Common errors

- **``error[E0425]: cannot find value `__test_hook__` in the crate root``**

  Check that the `test_hook` attribute is applied to a function at the crate root.

- **``error[E0428]: the name `__test_hook__` is defined multiple times``**

  Check that the `test_hook` attribute is applied to only one function.

- **`error[E0308]: mismatched types`**

  Check the signature of the function annotated with `#[test_hook]`. It must take one `&mut std::process::Command` argument and return `()`.

## Known problems

- **"running \<n> tests" messages are printed twice.** E.g.:

  ```
  running 3 tests

  running 3 tests
  test arg_was_added ... ok
  test current_dir_changed ... ok
  test environment_variable_was_set ... ok
  ```

- **With `--test-threads=1`, a "test \<name> ... " line is left incomplete.** When tests run one at a time, [`libtest`] prints each test's name before running it and the result afterward. When `test_hook` is used, the first test causes a child process to be launched and the parent exits without printing the first test's result. E.g.:

  ```
  running 3 tests
  test arg_was_added ...
  running 3 tests
  test arg_was_added ... ok
  test current_dir_changed ... ok
  test environment_variable_was_set ... ok
  ```

- **With [`cargo-nextest`], the hook is called once per test.** `cargo-nextest` runs each test in its own process.

- **Every test in the executable must use `test_hook::test`.** Tests that use the built-in `#[test]` attribute may start before the hook runs and may be terminated before they finish.

## Alternatives considered

- **An in-process `before_all` hook.** This would avoid printing the test count twice, but tests using the built-in `#[test]` attribute could still start before the hook runs. Modifying environment variables while other tests are running introduces safety concerns and requires [unsafe code]. So launching a separate process seems preferable.

## Credits

The `test_hook` attribute's syntax was inspired by [`ctor`].

[^integration-test]: That is, a file in the `tests` directory.

[^crate-root]: That is, not inside a module.

[`cargo-nextest`]: https://nexte.st/
[`ctor`]: https://docs.rs/ctor/latest/ctor/
[`libtest`]: https://github.com/rust-lang/rust/tree/main/library/test
[adding arguments]: https://doc.rust-lang.org/std/process/struct.Command.html#method.args
[adding]: https://doc.rust-lang.org/std/process/struct.Command.html#method.env
[changing its working directory]: https://doc.rust-lang.org/std/process/struct.Command.html#method.current_dir
[removing environment variables]: https://doc.rust-lang.org/std/process/struct.Command.html#method.env_remove
[unsafe code]: https://doc.rust-lang.org/std/env/fn.set_var.html
