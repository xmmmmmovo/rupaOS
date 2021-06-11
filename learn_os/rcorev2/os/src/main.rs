#![no_std]
#![no_main]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(lang_items)]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![allow(unused_imports)]

use log::{debug, info};

#[macro_use]
mod console;
mod lang_items;
mod sbi;
#[macro_use]
mod logger;
mod config;
mod loader;
mod syscall;
mod task;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    logger::init();
    info!("log init finished!");
    trap::init();
    info!("log init finished!");
    batch::init();
    info!("log init finished!");
    batch::run_next_app();
    debug!("run next app!!!");
}
