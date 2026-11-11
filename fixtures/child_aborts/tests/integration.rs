#[cfg(test)]
mod tests {
    use std::process::{Command, abort};
    use test_hook::{test, test_hook};

    #[test_hook]
    fn test_hook(_command: &mut Command) {}

    #[test]
    #[should_panic]
    fn abort_in_child() {
        abort();
    }
}
