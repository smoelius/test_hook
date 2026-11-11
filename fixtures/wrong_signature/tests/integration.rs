#[cfg(test)]
mod tests {
    use std::process::Command;
    use test_hook::{test, test_hook};

    #[test_hook]
    fn test_hook(_command: &Command) {}

    #[test]
    fn test() {}
}
