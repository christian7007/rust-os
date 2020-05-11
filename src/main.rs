#![no_std] // Don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;


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

///////////////////////////////////////////////////////////////////////////////
/// Entry point
///////////////////////////////////////////////////////////////////////////////
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
	// This function is the entry point, since the linker looks for a fucntion
	// named `_start` by default
	println!("Hello World{}", "!");

	rust_os::init();

	#[cfg(test)]
	test_main();

	println!("It did not crash!");

	rust_os::hlt_loop();
}
