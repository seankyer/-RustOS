# µRustOS
An embedded Rust project

# µRustOS Getting Started (ARM + QEMU)

Set up your minimal bare-metal Rust development environment for an ARM Cortex-M core using QEMU and `no_std`.

## 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow options for default install. Then reload your shell or run:

```bash
source $HOME/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
```

## 2. Add the ARM Target

```bash
rustup target add thumbv7em-none-eabihf
```

This target supports Cortex-M4/M7 with hardware floating point.

## 3. Install Required Cargo Tools

```bash
cargo install cargo-binutils flip-link
rustup component add llvm-tools-preview
```

## 4.: Install QEMU (Optional)

```bash
sudo apt update
sudo apt install qemu-system-arm
```

Verify installation:

```bash
qemu-system-arm --version
```

You're now ready to move on to building your Rust kernel and running it in QEMU!

# Building µRustOS

Switch into the `rustos` directory

```bash
cd rustos
```

Run the rust build command:

```bash
cargo build --release
```
