```
first-party-demo/
├── Cargo.toml          # demo-root (depends on demo-lib, demo-util)
├── src/lib.rs
├── examples/
│   └── platform_demo.rs  # demonstrates platform-specific functionality
└── crates/
    ├── demo-lib/       # depends on demo-util + serde
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── demo-util/      # leaf crate with platform-specific deps
        ├── Cargo.toml
        └── src/lib.rs

Dependency graph:
demo-root
├── demo-lib (first-party)
│   ├── demo-util (first-party, with platform-specific deps)
│   │   ├── libc (third-party, Linux only)
│   │   ├── core-foundation (third-party, macOS only)
│   │   └── winapi (third-party, Windows only)
│   └── serde (third-party)
└── demo-util (first-party, with platform-specific deps)
    ├── libc (third-party, Linux only)
    ├── core-foundation (third-party, macOS only)
    └── winapi (third-party, Windows only)
```

## Features

- **Platform-specific dependencies**: `demo-util` uses different dependencies based on the target platform:
  - **Linux**: Uses `libc` to get system information (e.g., process ID)
  - **macOS**: Uses `core-foundation` to access macOS system frameworks
  - **Windows**: Uses `winapi` to access Windows API functions

## Examples

Run the platform demo:
```bash
cargo run --example platform_demo
```

This example demonstrates the `get_platform_info()` function which returns platform-specific information using the appropriate platform dependencies.
