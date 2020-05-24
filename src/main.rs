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

	use rust_os::memory;
	use rust_os::memory::BootInfoFrameAllocator;;
	use x86_64::{structures::paging::Page, VirtAddr};

	println!("Hello World{}", "!");

	rust_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
	memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

	#[cfg(test)]
	test_main();

	println!("It did not crash!");

	rust_os::hlt_loop();
}
