pub fn format_greeting(greeting: &str, name: &str) -> String {
    format!("{}, {}!", greeting, name)
}

/// Get platform-specific information using different dependencies per platform
pub fn get_platform_info() -> String {
    #[cfg(target_os = "linux")]
    {
        // Use libc to get system information on Linux
        let pid = unsafe { libc::getpid() };
        format!("Linux platform (PID: {})", pid)
    }

    #[cfg(target_os = "macos")]
    {
        // Use core-foundation to get macOS version
        use core_foundation::base::TCFType;
        use core_foundation::string::CFString;

        format!("macOS platform (using CoreFoundation)")
    }

    #[cfg(target_os = "windows")]
    {
        // Use winapi to get Windows computer name
        use winapi::um::winbase::GetComputerNameW;
        use winapi::shared::minwindef::DWORD;

        let mut buffer = [0u16; 256];
        let mut size: DWORD = buffer.len() as DWORD;

        unsafe {
            GetComputerNameW(buffer.as_mut_ptr(), &mut size);
        }

        format!("Windows platform (computer name available)")
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        format!("Unknown platform")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_greeting() {
        assert_eq!(format_greeting("Hello", "World"), "Hello, World!");
        assert_eq!(format_greeting("Hi", "Rust"), "Hi, Rust!");
        assert_eq!(format_greeting("Greetings", "Alice"), "Greetings, Alice!");
    }

    #[test]
    fn test_format_greeting_empty() {
        assert_eq!(format_greeting("", ""), ", !");
        assert_eq!(format_greeting("Hello", ""), "Hello, !");
    }

    #[test]
    fn test_get_platform_info_returns_string() {
        let info = get_platform_info();
        assert!(!info.is_empty(), "Platform info should not be empty");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_platform_info() {
        let info = get_platform_info();
        assert!(info.starts_with("Linux platform"), "Should identify as Linux");
        assert!(info.contains("PID:"), "Should contain PID information");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_platform_info() {
        let info = get_platform_info();
        assert!(info.starts_with("macOS platform"), "Should identify as macOS");
        assert!(info.contains("CoreFoundation"), "Should mention CoreFoundation");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_platform_info() {
        let info = get_platform_info();
        assert!(info.starts_with("Windows platform"), "Should identify as Windows");
        assert!(info.contains("computer name"), "Should mention computer name");
    }

    #[test]
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    fn test_unknown_platform_info() {
        let info = get_platform_info();
        assert_eq!(info, "Unknown platform", "Should identify as Unknown platform");
    }
}
