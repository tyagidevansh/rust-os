# Simple OS

This is a hobby operating system project built from scratch, following [Philipp Oppermann’s "Writing an OS in Rust" blog series](https://os.phil-opp.com/). The project is written in Rust and runs on x86_64 architecture.

## Features

- 64-bit kernel written in Rust
- Basic VGA text output
- Interrupt handling
- Minimal bootloader (via `bootloader` crate)
- Runs in QEMU
- Can access the page tables
- Can allocate memory on the heap (all Rust allocators such as Box, Rc, etc are supported)

## Requirements

- Rust (nightly toolchain, 15th June 2025's nightly worked great for me, others may work too)
- QEMU

## Running it
cargo run
That's it. If you have QEMU installed it'll run the OS in the VM too. You can also take the binary, load it to disk and run it on a real computer too. 