# A Minimal Rust Kernel

When you turn on a computer, it loads the BIOS from some special flash memory located on the motherboard. The BIOS runs self test and initialization routines of the hardware, then it looks for bootable disks. If it finds one, the control is transferred to its bootloader, which is a 512-byte portion of executable code stored at the disk's beginning. Most bootloaders are larger than 512 bytes, so bootloaders are commonly split into a small first stage, which fits into 512 bytes, and a second stage, which is subsequently loaded by the first stage.

The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode, where 64-bit registers and the complete main memory are available. Its third job is to query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.

## Target Specifcation for our Kernel

We have to create a custom target for our kernel ("x86_64-blog_os.json").
The core library for Rust, however, comes precompiled with support for valid triple-hosts, but our compiler isn't one so we have to re-compile the core library for our custom target.
Thus, we build with `cargo xbuild --target x86_64-blog_os.json` whuch re-compiles the core library for our custom target and then compiles our `blog_os` create, but this only works with Rust's Nightly Compiler so we have to install the Nightly release of Rust.


