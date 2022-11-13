//! Scull module in Rust
use kernel::prelude::*;

module! {
    type: Scull,
    name : "Scull",
    license : "GPL",
}

struct Scull;

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        Ok(Scull)
    }
}