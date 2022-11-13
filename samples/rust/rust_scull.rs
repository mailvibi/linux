//! Scull module in Rust
use kernel::{file, miscdev};
use kernel::prelude::*;

module! {
    type: Scull,
    name : "Scull",
    license : "GPL",
}

struct Scull {
    _dev: Pin<Box<miscdev::Registration<Scull>>>,
}

#[vtable]
impl file::Operations for Scull {
    fn open(context: &Self::OpenData, file: &file::File) -> Result<Self::Data> {
        pr_info!("File was opened\n");
        Ok(())
    }
}

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world\n");
        let reg = miscdev::Registration::<Scull>::new_pinned(fmt!("Scull"),())?;
        Ok(Scull{_dev:reg})
    }
}