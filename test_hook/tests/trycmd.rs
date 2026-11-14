use test_hook::{test, test_hook};

#[test_hook]
fn test_hook(command: &mut std::process::Command) {
    command.env_remove("CARGO_TERM_COLOR");
}

#[test]
fn trycmd() {
    let cases = trycmd::TestCases::new();

    cases.default_bin_path(env!("CARGO"));

    cases.case("tests/cases/*.toml");

    // On Windows, abort causes the child to exit with a code rather than a signal.
    #[cfg(windows)]
    cases.skip("tests/cases/child_aborts.toml");
}
