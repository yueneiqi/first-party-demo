use demo_util::format_greeting;

pub fn greet(name: &str) -> String {
    format_greeting("Hello", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_greet_empty_name() {
        assert_eq!(greet(""), "Hello, !");
    }

    #[test]
    fn test_greet_special_characters() {
        assert_eq!(greet("世界"), "Hello, 世界!");
        assert_eq!(greet("🦀"), "Hello, 🦀!");
    }
}
