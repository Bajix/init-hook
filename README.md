![License](https://img.shields.io/badge/license-MIT-green.svg)
![License](https://img.shields.io/badge/license-Apache-green.svg)
[![Cargo](https://img.shields.io/crates/v/init-hook.svg)](https://crates.io/crates/init-hook)
[![Documentation](https://docs.rs/init-hook/badge.svg)](https://docs.rs/init-hook)

This crate allows registering functions for guaranteed initialization during main

# Example

```rust
use std::sync::atomic::{AtomicBool, Ordering};
static INIT_CALLED: AtomicBool = AtomicBool::new(false);

#[init_hook::call_on_init]
fn init() {
    INIT_CALLED.store(true, Ordering::Release);
}

fn main() {
    // This is enforced by a pre-main assertion to always be included exactly once
    init_hook::init!();

    assert!(INIT_CALLED.load(Ordering::Acquire));
}
```