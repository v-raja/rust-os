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
use core::panic::PanicInfo;

mod vga_buffer; // module to safely write to vga buffer

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        // Write letter and set color to cyan
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
