# A Minimal Rust Kernel

When you turn on a computer, it loads the BIOS from some special flash memory located on the motherboard. The BIOS runs self test and initialization routines of the hardware, then it looks for bootable disks. If it finds one, the control is transferred to its bootloader, which is a 512-byte portion of executable code stored at the disk's beginning. Most bootloaders are larger than 512 bytes, so bootloaders are commonly split into a small first stage, which fits into 512 bytes, and a second stage, which is subsequently loaded by the first stage.

The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode, where 64-bit registers and the complete main memory are available. Its third job is to query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.

## Target Specifcation for our Kernel

We have to create a custom target for our kernel ("x86_64-blog_os.json").
The core library for Rust, however, comes precompiled with support for valid triple-hosts, but our compiler isn't one so we have to re-compile the core library for our custom target.
Thus, we build with `cargo xbuild --target x86_64-blog_os.json` whuch re-compiles the core library for our custom target and then compiles our `blog_os` create, but this only works with Rust's Nightly Compiler so we have to install the Nightly release of Rust.


# The VGA Text Buffer
VGA text buffer has a standard size and a character is two bytes where the first byte represents the ascii character and the second byte represents the colour of the character (first four bytes are foreground color, and last four bytes are background color).

To write to screen, we created a writer which contains the reference to the address of the VGA buffer, the current column position of the writer and the color code for all text written to screen. Like a type writter, we just move text up one row when the current column position exceeds the size of the VGA buffer's width (25 chars).

But if Rust compilers optimize aggressively, they may not allow us to write to an address that it can't verify (especailly since the compiler doesn't understand that writing to that address has the side affect of writing to the screen). Hence, we wrap our ScreenChar in a Volatile and use the write and read that come with volatile instead of assigning a value to a the VGA buffer.

Rust comes built-in with formatting macros (write! and writeln!) which handles printing integers and floats. Hence, we implement fmt::Write for our writer.

Now instead of carrying a `Writer` instance around, it makes sense to implement a global writer by creating a static writer. But since static objects are initalized at compile time whereas pointers and variables are initalized at run time, the raw pointers aren't converted to references which leads to errors and even if we convert the pointert to consts, Rust's const evaluator is unable to convert raw pointers to references at compile time. Hence, we use the crate `lazy_static` that initializes a static when it's accessed the first time.

But we have another requirement, `Writer` has to be a mutable since its we mutate the buffer, and column position when writing a byte. We could implement a mutable static but that's unsafe due to unsyncronized reads and writes from multiple threads. A better solution would be to implement an immutable static that has interior mutability in a thread-safe form. A spinlock Mutex will allow us to do this (i.e. the thread will keep on trying to acquire the lock endlessly until it succeeds).


# Testing

Since Rust's testing framework is linked to its standard library, we have to create a custom testing framework. We do so using `#![feature(custom_test_frameworks)]`, `#![test_runner(crate::test_runner)]`, and the `test_runner` function. Thus, all we have to do for tests now is create a function and enclose it in a `#[test_case]`. To run test we use `cargo xtest`.

Every time we run tests, we have to manually close QEMU which can get daunting. We solve this by adding a "-device" flag which specifies that if we write to the listed port of "0xf4", QEMU will exit with `(value << 1) | 1` where value is what we write to the port. QEMU's default failure code is 1 so we stay clear of that. Hence, we use the failure code as `0x11` (can be any non-zero number really) and success as `0x10` and state that the test success exit code is 33 (`(0x10 << 1) | 1`) in `cargo.toml`.

We also make testing quicker by passing the messages produced by the kernel to our terminal using a serial port.

Lastly, we'll move our the code for the test_runner and test_panic_handler to lib.rs so that we can use the common code to handle panics and run tests for both integration tests (located in the `tests` directory) and unit tests (in each file, annotated with `#[cfg(test)]`).


# CPU Exceptions

Every architecture has its defined exceptions and has a Interrupt Description Table that maps out a type of exception to the pointer of a function that handles that type of interrupt. When an exception occurs, the cpu pushes registers overwritten by the current function to the stack, the instruction pointer, and RFLAGS register to the stack, reads and jumps to the function that handles the raised interrupt. After the interrupt is handled, the cup then returns back to the function that was running and restores the registers, instruction pointer and RFLAGS register (possibly with some bits changed to indicate an interrupt has occured).

How are registers saved? What registers are saved? The saving of the registers depends on the calling conventions. The C calling convention, for example, states that certain registers (called the preserved registers) have to be saved by the callee-function (the function being called) and certain other registers (called the scratch registers) have to be saved by the caller function. The names preserved and scratch are used in relation to the callee function; the callee function can change the values of scratch registers without worrying about restoring them, but has to preserve the values of the preserved registers before it returns.
When the callee function returns, it must re-write the original values to the preserved register, and when the caller function resumes, it must re-write the origianl values to the scratch register. Note that the compiler only saves registers whose values have been written to by the function which prevents ineffiently saving all registers. The decisions as to when and what registers need to saved are decided by the compiler during compile time.

But interrupts occur during run-time and can occur on any instruction. The current function doesn't get enough notice to save any of its used registers. Hence, interrupt handler functions use the Interrupt Calling Convention (`extern x86-interrupt`). This convention gurantees that all registers used by the function when an exception happens are returned to their original state before the interrupt handler function returns.


