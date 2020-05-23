#![no_std] // Don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};

#[cfg(not(test))]
#[panic_handler]
///This function is called on panic.
fn panic(_info: &PanicInfo) -> ! {
	println!("{}", _info);
	rust_os::hlt_loop();
}

///This function is called on panic during testing
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	rust_os::test_panic_handler(info)
}

entry_point!(kernel_main);

///////////////////////////////////////////////////////////////////////////////
/// Entry point
///////////////////////////////////////////////////////////////////////////////
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	// This function is the entry point, since the linker looks for a fucntion
	// named `_start` by default

	use rust_os::memory::active_level_4_table;
	use x86_64::VirtAddr;

	println!("Hello World{}", "!");

	rust_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

	for (i, entry) in l4_table.iter().enumerate() {
		if !entry.is_unused() {
			println!("L4 Entry {}: {:?}", i, entry);

			use x86_64::structures::paging::PageTable;

			let phys = entry.frame().unwrap().start_address();
			let virt = phys.as_u64() + boot_info.physical_memory_offset;
			let ptr = VirtAddr::new(virt).as_mut_ptr();
			let l3_table: &PageTable = unsafe { &*ptr };

			for (i, entry) in l3_table.iter().enumerate() {
				if !entry.is_unused() {
					println!("	L3 Entry {}: {:?}", i, entry);
				}
			}
		}
	}

	#[cfg(test)]
	test_main();

	println!("It did not crash!");

	rust_os::hlt_loop();
}
