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
