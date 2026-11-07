# `test_hook`

Control how test executables are run

## Usage

Import `test_hook::{test, test_hook}` and add the `test_hook` attribute to a function that takes a `&mut std::process::Command` as an argument. Before tests are run, the function will be called with a command to launch the test executable in a child process. The function can modify the command, e.g., by [adding arguments], [changing its working directory], or [adding] or [removing environment variables].

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

## Common errors

- **``error[E0425]: cannot find value `__test_hook__` in this scope``**

  Check that you imported the `test_hook::test_hook` attribute and applied it to a function.

- **`error[E0308]: mismatched types`**

  Check the signature of the function annotated with `#[test_hook]`. It must take one `&mut std::process::Command` argument and return `()`.

## Known problems

- **"running \<n> tests" messages are printed twice.** E.g.:

  ```
  running 3 tests

  running 3 tests
  test tests::arg_was_added ... ok
  test tests::current_dir_changed ... ok
  test tests::environment_variable_was_set ... ok
  ```

- **Every test in the executable must use `test_hook::test`.** Tests that use the built-in `#[test]` attribute may start before the hook runs and may be terminated before they finish.

## Alternatives considered

- **An in-process `before_all` hook.** This would avoid printing the test count twice, but tests using the built-in `#[test]` attribute could still start before the hook runs. Modifying environment variables while other tests are running introduces safety concerns and requires [unsafe code]. So launching a separate process seems preferable.

## Credits

The `test_hook` attribute's syntax was inspired by [`ctor`].

[`ctor`]: https://docs.rs/ctor/latest/ctor/
[adding arguments]: https://doc.rust-lang.org/std/process/struct.Command.html#method.args
[adding]: https://doc.rust-lang.org/std/process/struct.Command.html#method.env
[changing its working directory]: https://doc.rust-lang.org/std/process/struct.Command.html#method.current_dir
[removing environment variables]: https://doc.rust-lang.org/std/process/struct.Command.html#method.env_remove
[unsafe code]: https://doc.rust-lang.org/std/env/fn.set_var.html
