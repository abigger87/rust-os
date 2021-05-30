# rust-os

Rust-powered OS following the https://os.phil-opp.com/ tutorial

## Building

Requires

-   [Rust Nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html)
    -   In short, `rustup toolchain install nightly` installs the nightly build and `rustup override set nightly` sets the toolchain as nightly for this project directory
-   `rust-src` component (install with `rustup component add rust-src`)
-   `bootimage` tool to compile kernal and bootloader and then link the two together after, creating a bootable disk image.
    -   Install with `cargo install bootimage`
    -   Requires `llvm-tools-preview` rustup component (install with `rustup component add llvm-tools-preview`).

Build: `cargo build` or specify the verbose build target with `cargo build --target x86-64-blog-os.json`
Create a bootable disk: `cargo bootimage`

## Booting

Boot in QEMU: `qemu-system-x86_64 -drive format=raw,file=target/x86-64-blog-os/debug/bootimage-rust-os.bin`
