#[cfg(test)]
mod tests {
    use std::process::Command;
    use test_hook::{test, test_hook};

    #[test_hook]
    fn test_hook(_command: &mut Command) {
        panic!("something went wrong");
    }

    #[test]
    #[should_panic]
    fn test_that_should_panic() {}
}
