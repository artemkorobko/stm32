# STM32 Examples
This repository contains scaffolding examples for STM32 MCU.

[stm32f103](stm32f103) - Basic peripheral setup

# Prerequisites
## Rust
- Install Rust toolchain by following the instructions on https://rustup.rs.
- Install the `rust-std` component `thumbv7em-none-eabihf` to cross-compile for ARM Cortex-M4 MCU using the following command:
```bash
rustup target add thumbv7em-none-eabihf
```
- Install `cargo-binutils` subcommands to invoke the LLVM tools shipped with the Rust toolchain.
```bash
cargo install cargo-binutils 
```
- Install `llvm-tools-preview` component for binary inspection.
```bash
rustup component add llvm-tools-preview
```

## ARM gcc extension for Mac
Before installing extension make sure you have updated [Homebrew](https://brew.sh) packages.
- Install ARM gcc extension and open on-chip debugger.
```bash
brew install armmbed/formulae/arm-none-eabi-gcc openocd
```
- Ensure extension has been installed
```
arm-none-eabi-gcc -v
```

## VS Build Tools for Windows
Download the Visual Studio 2019 Build tools from the Microsoft website: https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16

During installation in the `Workloads` tab select `Desktop development with C++`. Select the following items on the `Installation details` page:
- MSVC v142 - VS 2019 C++ ...
- Windows 10 SDK ...
- C++ CMake tools for Windows

You can find more information about the embedded toolchains here https://docs.rust-embedded.org/book/intro/index.html.
