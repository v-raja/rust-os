// Freestanding Rust binary
// - [no_std] means that we need a way to handle panic; we do so by implementing
// our own panic. Further, stack unwinding required eh_personality language item.
// We disable by setting panic to abort in our target configuration file
// - Compiler searches for _start function which isn't there since we didn't link
// the Rust standard library. We implement our own and use no_mangle to force the
// compiler to keep the function name as "_start"
// - Compiler assumes C runtime, but since we don't implement various things
// implemented in _start in the standard library, it is not a good idea to compile
// assuming C runtime by default. Hence, build with
// `cargo build --target thumbv7em-none-eabihf`


#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // use custom test framework since standard framework depends on the standard library
#![test_runner(crate::test_runner)] // use create test_runner as test runner
#![reexport_test_harness_main = "test_main"] // changes test function test_runner name to test_main
use core::panic::PanicInfo;

mod vga_buffer; // module to safely write to vga buffer

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {

    // Only add test_main when running tests
    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert_eq!(1,1);
    println!("[ok]");
}
