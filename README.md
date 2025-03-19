![License](https://img.shields.io/badge/license-MIT-green.svg)
![License](https://img.shields.io/badge/license-Apache-green.svg)
[![Cargo](https://img.shields.io/crates/v/init-hook.svg)](https://crates.io/crates/init-hook)
[![Documentation](https://docs.rs/init-hook/badge.svg)](https://docs.rs/init-hook)

# Init Hook

This crate allows registering functions for guaranteed initialization during main

### Example

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
static COUNTER: AtomicUsize = AtomicUsize::new(0);

// Register function to be called exactly once during init
#[init_hook::call_on_init]
fn init_once() {
    COUNTER.fetch_add(1, Ordering::Release);
}

// Registered functions can also be unsafe
#[init_hook::call_on_init]
unsafe fn init_once_unchecked() {
    COUNTER.fetch_add(1, Ordering::Release);
}

fn main() {
    // This is enforced by a pre-main assertion to always be included exactly once
    init_hook::init!();
    assert_eq!(COUNTER.load(Ordering::Acquire), 2);
}
```

### Platform support

| Linux | macOS | Windows | FreeBSD | OpenBSD | illumos | Other |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 💚 | 💚 | 💚 | 💚 | 💚 | 💚 | ❌ |

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>