#![no_std]
#![no_main]
#![feature(
    panic_info_message,
    llvm_asm,
    allocator_api,
    alloc_error_handler,
    alloc_prelude,
    const_raw_ptr_to_usize_cast
)]

#[macro_use]
extern crate alloc;
// This is experimental and requires alloc_prelude as a feature
use alloc::prelude::v1::*;

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
			});
}
#[macro_export]
macro_rules! println
{
	() => ({
		   print!("\r\n")
		   });
	($fmt:expr) => ({
			print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			print!(concat!($fmt, "\r\n"), $($args)+)
			});
}

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////
#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }
    abort();
}
#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        unsafe {
            llvm_asm!("wfi"::::"volatile");
        }
    }
}

// ///////////////////////////////////
// / CONSTANTS
// ///////////////////////////////////
// const STR_Y: &str = "\x1b[38;2;79;221;13m✓\x1b[m";
// const STR_N: &str = "\x1b[38;2;221;41;13m✘\x1b[m";

// The following symbols come from asm/mem.S. We can use
// the symbols directly, but the address of the symbols
// themselves are their values, which can cause issues.
// Instead, I created doubleword values in mem.S in the .rodata and .data
// sections.
extern "C" {
    static TEXT_START: usize;
    static TEXT_END: usize;
    static DATA_START: usize;
    static DATA_END: usize;
    static RODATA_START: usize;
    static RODATA_END: usize;
    static BSS_START: usize;
    static BSS_END: usize;
    static KERNEL_STACK_START: usize;
    static KERNEL_STACK_END: usize;
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
    static mut KERNEL_TABLE: usize;
}
/// Identity map range
/// Takes a contiguous allocation of memory and maps it using PAGE_SIZE
/// This assumes that start <= end
pub fn id_map_range(root: &mut page::Table, start: usize, end: usize, bits: i64) {
    let mut memaddr = start & !(page::PAGE_SIZE - 1);
    let num_kb_pages = (page::align_val(end, 12) - memaddr) / page::PAGE_SIZE;

    // I named this num_kb_pages for future expansion when
    // I decide to allow for GiB (2^30) and 2MiB (2^21) page
    // sizes. However, the overlapping memory regions are causing
    // nightmares.
    for _ in 0..num_kb_pages {
        page::map(root, memaddr, memaddr, bits, 0);
        memaddr += 1 << 12;
    }
}
// ///////////////////////////////////
// / ENTRY POINT
// ///////////////////////////////////
#[no_mangle]
extern "C" fn kinit() -> usize {
    // We created kinit, which runs in super-duper mode
    // 3 called "machine mode".
    // The job of kinit() is to get us into supervisor mode
    // as soon as possible.
    // Interrupts are disabled for the duration of kinit()
    uart::Uart::new(0x1000_0000).init();
    page::init();
    kmem::init();

    // Map heap allocations
    let root_ptr = kmem::get_page_table();
    let root_u = root_ptr as usize;
    let mut root = unsafe { root_ptr.as_mut().unwrap() };
    let kheap_head = kmem::get_head() as usize;
    let total_pages = kmem::get_num_allocations();
    println!();
    println!();
    unsafe {
        println!("TEXT:   0x{:x} -> 0x{:x}", TEXT_START, TEXT_END);
        println!("RODATA: 0x{:x} -> 0x{:x}", RODATA_START, RODATA_END);
        println!("DATA:   0x{:x} -> 0x{:x}", DATA_START, DATA_END);
        println!("BSS:    0x{:x} -> 0x{:x}", BSS_START, BSS_END);
        println!(
            "STACK:  0x{:x} -> 0x{:x}",
            KERNEL_STACK_START, KERNEL_STACK_END
        );
        println!(
            "HEAP:   0x{:x} -> 0x{:x}",
            kheap_head,
            kheap_head + total_pages * 4096
        );
    }
    id_map_range(
        &mut root,
        kheap_head,
        kheap_head + total_pages * 4096,
        page::EntryBits::ReadWrite.val(),
    );
    unsafe {
        // Map heap descriptors
        let num_pages = HEAP_SIZE / page::PAGE_SIZE;
        id_map_range(
            &mut root,
            HEAP_START,
            HEAP_START + num_pages,
            page::EntryBits::ReadWrite.val(),
        );
        // Map executable section
        id_map_range(
            &mut root,
            TEXT_START,
            TEXT_END,
            page::EntryBits::ReadExecute.val(),
        );
        // Map rodata section
        // We put the ROdata section into the text section, so they can
        // potentially overlap however, we only care that it's read
        // only.
        id_map_range(
            &mut root,
            RODATA_START,
            RODATA_END,
            page::EntryBits::ReadExecute.val(),
        );
        // Map data section
        id_map_range(
            &mut root,
            DATA_START,
            DATA_END,
            page::EntryBits::ReadWrite.val(),
        );
        // Map bss section
        id_map_range(
            &mut root,
            BSS_START,
            BSS_END,
            page::EntryBits::ReadWrite.val(),
        );
        // Map kernel stack
        id_map_range(
            &mut root,
            KERNEL_STACK_START,
            KERNEL_STACK_END,
            page::EntryBits::ReadWrite.val(),
        );
    }

    // UART
    page::map(
        &mut root,
        0x1000_0000,
        0x1000_0000,
        page::EntryBits::ReadWrite.val(),
        0,
    );

    // CLINT
    //  -> MSIP
    page::map(
        &mut root,
        0x0200_0000,
        0x0200_0000,
        page::EntryBits::ReadWrite.val(),
        0,
    );
    //  -> MTIMECMP
    page::map(
        &mut root,
        0x0200_b000,
        0x0200_b000,
        page::EntryBits::ReadWrite.val(),
        0,
    );
    //  -> MTIME
    page::map(
        &mut root,
        0x0200_c000,
        0x0200_c000,
        page::EntryBits::ReadWrite.val(),
        0,
    );
    // PLIC
    id_map_range(
        &mut root,
        0x0c00_0000,
        0x0c00_2000,
        page::EntryBits::ReadWrite.val(),
    );
    id_map_range(
        &mut root,
        0x0c20_0000,
        0x0c20_8000,
        page::EntryBits::ReadWrite.val(),
    );
    page::print_page_allocations();
    // The following shows how we're going to walk to translate a virtual
    // address into a physical address. We will use this whenever a user
    // space application requires services. Since the user space application
    // only knows virtual addresses, we have to translate silently behind
    // the scenes.
    let p = 0x8005_7000 as usize;
    let m = page::virt_to_phys(&root, p).unwrap_or(0);
    println!("Walk 0x{:x} = 0x{:x}", p, m);
    // When we return from here, we'll go back to boot.S and switch into
    // supervisor mode We will return the SATP register to be written when
    // we return. root_u is the root page table's address. When stored into
    // the SATP register, this is divided by 4 KiB (right shift by 12 bits).
    // We enable the MMU by setting mode 8. Bits 63, 62, 61, 60 determine
    // the mode.
    // 0 = Bare (no translation)
    // 8 = Sv39
    // 9 = Sv48
    unsafe {
        // We have to store the kernel's table. The tables will be moved back
        // and forth between the kernel's table and user applicatons' tables.
        KERNEL_TABLE = root_u;
    }
    // table / 4096    Sv39 mode
    (root_u >> 12) | (8 << 60)
}

#[no_mangle]
extern "C" fn kmain() {
    // kmain() starts in supervisor mode. So, we should have the trap
    // vector setup and the MMU turned on when we get here.

    // We initialized my_uart in machine mode under kinit for debugging
    // prints, but this just grabs a pointer to it.
    let mut my_uart = uart::Uart::new(0x1000_0000);
    println!("init");
    // Create a new scope so that we can test the global allocator and
    // deallocator
    {
        // We have the global allocator, so let's see if that works!
        let k = Box::<u32>::new(100);
        println!("Boxed value = {}", *k);
        kmem::print_table();
        // The following comes from the Rust documentation:
        // some bytes, in a vector
        let sparkle_heart = vec![240, 159, 146, 150];
        // We know these bytes are valid, so we'll use `unwrap()`.
        let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
        println!("String = {}", sparkle_heart);
    }
    // If we get here, the Box, vec, and String should all be freed since
    // they go out of scope. This calls their "Drop" trait.
    // Now see if we can read stuff:
    // Usually we can use #[test] modules in Rust, but it would convolute
    // the task at hand, and it requires us to create the testing harness
    // since the embedded testing system is part of the "std" library.
    loop {
        if let Some(c) = my_uart.get() {
            match c {
                8 => {
                    // This is a backspace, so we
                    // essentially have to write a space and
                    // backup again:
                    print!("{} {}", 8 as char, 8 as char);
                }
                10 | 13 => {
                    // Newline or carriage-return
                    println!();
                }
                _ => {
                    print!("{}", c as char);
                }
            }
        }
    }
}

// ///////////////////////////////////
// / RUST MODULES
// ///////////////////////////////////

pub mod kmem;
pub mod page;
pub mod uart;
