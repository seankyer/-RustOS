# µRustOS

An embedded micro RustOS for baremetal

# µRustOS Getting Started (ARM + QEMU)

Set up your minimal bare-metal Rust development environment for an ARM Cortex-M core using QEMU and `no_std`.

## 0. Install Required Toolchains

```bash
sudo apt-get install binutils-arm-none-eabi gdb-multiarch
```

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

## 2. Add required packages

```bash
rustup intsall nightly
rustup default nightly
rustup component add rust-src
rustup component add rust-std
```

## 3. Add the ARM Target

```bash
rustup target add thumbv7em-none-eabihf
```

This target supports Cortex-M4 with hardware floating point.

## 4. Install QEMU (Optional)

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
cargo build --release # Debug build not working yet
```

# Running µRustOS (QEMU)

Once you've built µRustOS, you can run it on QEMU with GDB debugging server:

```bash
qemu-system-arm \
  -machine lm3s6965evb \
  -kernel target/thumbv7em-none-eabihf/release/rustos \
  -S -gdb tcp::1234
```

# Connecting Debugger to See Output

You can then connect the debugger with `target extended-remote` to see hello-world

```bash
gdb-multiarch target/thumbv7em-none-eabihf/release/rustos -ex "target extended-remote :1234"
```

To see where the "Hello World!" message was printed:

```bash
c
x/s 0x20000000
> Hello World!
```

# Guide

Resources used:

- https://os.phil-opp.com/freestanding-rust-binary/
