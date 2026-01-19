use demo_root::{greet, get_platform_info, format_greeting};

#[test]
fn test_greet_integration() {
    let result = greet("Integration");
    assert_eq!(result, "Hello, Integration!");
}

#[test]
fn test_format_greeting_integration() {
    let result = format_greeting("Hi", "Test");
    assert_eq!(result, "Hi, Test!");
}

#[test]
fn test_platform_info_integration() {
    let info = get_platform_info();

    // Verify that platform info is not empty
    assert!(!info.is_empty(), "Platform info should not be empty");

    // Verify platform-specific information
    #[cfg(target_os = "linux")]
    {
        assert!(info.contains("Linux"), "Should identify Linux platform");
        assert!(info.contains("PID"), "Should contain PID on Linux");
    }

    #[cfg(target_os = "macos")]
    {
        assert!(info.contains("macOS"), "Should identify macOS platform");
        assert!(info.contains("CoreFoundation"), "Should mention CoreFoundation on macOS");
    }

    #[cfg(target_os = "windows")]
    {
        assert!(info.contains("Windows"), "Should identify Windows platform");
        assert!(info.contains("computer name"), "Should mention computer name on Windows");
    }
}

#[test]
fn test_cross_crate_dependency() {
    // Test that demo-lib correctly uses demo-util
    let greeting = greet("CrossCrate");
    assert!(greeting.contains("Hello"), "Should use Hello prefix from demo-lib");
    assert!(greeting.contains("CrossCrate"), "Should contain the name parameter");
    assert!(greeting.ends_with("!"), "Should end with exclamation mark from format_greeting");
}

#[test]
fn test_multiple_calls() {
    // Ensure functions can be called multiple times safely
    for i in 0..10 {
        let name = format!("User{}", i);
        let greeting = greet(&name);
        assert_eq!(greeting, format!("Hello, {}!", name));
    }
}

#[test]
fn test_platform_info_consistency() {
    // Platform info should be consistent across calls
    let info1 = get_platform_info();
    let info2 = get_platform_info();

    // Both should identify the same platform
    #[cfg(target_os = "linux")]
    {
        assert!(info1.contains("Linux"));
        assert!(info2.contains("Linux"));
    }

    #[cfg(target_os = "macos")]
    {
        assert!(info1.contains("macOS"));
        assert!(info2.contains("macOS"));
    }

    #[cfg(target_os = "windows")]
    {
        assert!(info1.contains("Windows"));
        assert!(info2.contains("Windows"));
    }
}
