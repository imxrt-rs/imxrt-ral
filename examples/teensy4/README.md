# imxrt-ral examples for the Teensy 4

The programs in this package run on a Teensy 4 (4.0 and 4.1), and demonstrate
how to use `imxrt-ral` on an embedded system.

## Dependencies

You'll need all of the dependencies for the `imxrt-ral` project. See the
project documentation for more details.

You'll also need

- A capable `objcopy` for transforming Rust binaries into hex files. The
documentation and tooling in the guide uses the LLVM `objcopy` provided by
[`cargo binutils`]. Install [`cargo binutils`] if you want to precisely follow
this documentation.

[`cargo binutils`]: https://github.com/rust-embedded/cargo-binutils

- To download programs to your Teensy 4, you'll need either a build of
[`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli), or
the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html). The
latter is available with the Teensyduino add-ons.

## Building and running examples

From this directory, use `cargo objcopy` to build a release binary, and output
a hex file:

```
cargo objcopy --target thumbv7em-none-eabihf --release --bin blink -- -O ihex blink.hex
```

Flash the hex file to your Teensy 4!

## See also

Note the `.cargo/config.toml` configuration, which specifies the linker script.
You'll need a similar Cargo configuration for your project. To understand these
linking requirements, study the [`cortex-m-rt` documentation][cmrt].

[cmrt]: https://docs.rs/cortex-m-rt/0.6.13/cortex_m_rt/

[Cargo.toml](./Cargo.toml) describes all `imxrt-ral` feature selections, and
other dependencies required for these examples.
