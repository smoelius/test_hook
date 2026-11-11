#[cfg(test)]
const TEST_HOOK_UNIT_TESTS: &str = "TEST_HOOK_UNIT_TESTS";

#[cfg(test)]
#[test_hook::test_hook]
fn test_hook(command: &mut std::process::Command) {
    command.env(TEST_HOOK_UNIT_TESTS, "1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::var;
    use test_hook::test;

    #[test]
    fn outer() {
        assert_eq!("1", var(TEST_HOOK_UNIT_TESTS).unwrap());
    }

    mod inner {
        use super::*;
        use test_hook::test;

        #[test]
        fn inner() {
            assert_eq!("1", var(TEST_HOOK_UNIT_TESTS).unwrap());
        }
    }
}
