#![no_std] // Don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

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
	use rust_os::allocator;
	use rust_os::memory::{self, BootInfoFrameAllocator};
	use x86_64::VirtAddr;

	println!("Hello World{}", "!");
	rust_os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	allocator::init_heap(&mut mapper, &mut frame_allocator)
		.expect("heap initialization failed");

	let heap_value = Box::new(41);
	println!("heap_value at {:p}", heap_value);

	let mut vec = Vec::new();
	for i in 0..500 {
		vec.push(i);
	}
	println!("vec at {:p}", vec.as_slice());

	let reference_counted = Rc::new(vec![1,2,3]);
	let cloned_reference = reference_counted.clone();
	println!("Current reference coutn is {}", Rc::strong_count(&cloned_reference));
	core::mem::drop(reference_counted);
	println!("reference count is {} now", Rc::strong_count(&cloned_reference));
 

	#[cfg(test)]
	test_main();

	println!("It did not crash!");

	rust_os::hlt_loop();
}
