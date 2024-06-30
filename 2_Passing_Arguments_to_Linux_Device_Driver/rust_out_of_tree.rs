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
            permissions: 0o644,
            description: "integer value",
        },
        nameETX: str {
            default: b"example string",
            permissions: 0o644,
            description: "string value",
        },
        arr_valueETX: ArrayParam<i32,4> {
            default: [0, 0, 0, 0],
            permissions: 0o644,
            description: "array value",
        },
    },
}

struct MyModule;

impl kernel::Module for MyModule {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        // If the parameter is writeable, then the kparam lock must be
        // taken to read the parameter:
        let lock = THIS_MODULE.kernel_param_lock();
        pr_info!("Kernel Module Inserted Successfully...\n");
        pr_info!("ValueETX = {}\n", valueETX.read(&lock));
        pr_info!("NameETX = {}\n", core::str::from_utf8(nameETX.read(&lock))?); // try to handle error
        pr_info!("ArrayETX = {:?}\n", arr_valueETX.read(&lock));

        Ok(MyModule)
    }
}

impl Drop for MyModule {
    fn drop(&mut self) {
        pr_info!("Kernel Module Removed Successfully...\n");
    }
}
