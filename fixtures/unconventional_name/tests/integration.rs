#[cfg(test)]
mod tests {
    use std::process::Command;
    use test_hook::{test, test_hook};

    #[test_hook]
    fn unconventional_name(_command: &mut Command) {}

    #[test]
    fn test() {}
}
