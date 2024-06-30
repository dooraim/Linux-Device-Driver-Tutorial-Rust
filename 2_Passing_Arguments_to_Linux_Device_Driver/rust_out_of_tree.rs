// SPDX-License-Identifier: GPL-2.0

//! Rust printing macros sample.
// sudo dmesg -C && \
// make LLVM=1 clean && \
// make LLVM=1 && \
// sudo insmod rust_out_of_tree.ko && \
// sudo rmmod rust_out_of_tree && \
// sudo dmesg | tail -n 10

use kernel::prelude::*;

module! {
    type: MyModule,
    name: "rust_print",
    author: "Rust for Linux Contributors",
    description: "Rust printing macros sample",
    license: "GPL",
    params: { // see rust/rust/macros/lib.rs for more info
        valueETX: i32 {
            default: 0,
            permissions: 0o004,
            description: "integer value",
        },
    },
}

struct MyModule;

impl kernel::Module for MyModule {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Kernel Module Inserted Successfully...\n");
        pr_info!("ValueETX = {}\n", valueETX.read());

        Ok(MyModule)
    }
}

impl Drop for MyModule {
    fn drop(&mut self) {
        pr_info!("Kernel Module Removed Successfully...\n");
    }
}
