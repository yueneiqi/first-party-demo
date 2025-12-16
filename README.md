```
first-party-demo/
├── Cargo.toml          # demo-root (depends on demo-lib, demo-util)
├── src/lib.rs
└── crates/
    ├── demo-lib/       # depends on demo-util + serde
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── demo-util/      # leaf crate (no deps)
        ├── Cargo.toml
        └── src/lib.rs

Dependency graph:
demo-root
├── demo-lib (first-party)
│   ├── demo-util (first-party)
│   └── serde (third-party)
└── demo-util (first-party)
```
