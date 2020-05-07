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
#![test_runner(blog_os::test_runner)] // use create test_runner as test runner
#![reexport_test_harness_main = "test_main"] // changes test function test_runner name to test_main
use core::panic::PanicInfo;
use blog_os::println;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }



    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
