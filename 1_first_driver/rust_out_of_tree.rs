// SPDX-License-Identifier: GPL-2.0

//! Rust printing macros sample.

use kernel::prelude::*;

module! {
    type: MyModule,
    name: "rust_print",
    author: "Rust for Linux Contributors",
    description: "Rust printing macros sample",
    license: "GPL",
}

struct MyModule;

impl kernel::Module for MyModule {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World from Rust!\n");

        Ok(MyModule)
    }
}

impl Drop for MyModule {
    fn drop(&mut self) {
        pr_info!("Rust printing macros sample (exit)\n");
    }
}
