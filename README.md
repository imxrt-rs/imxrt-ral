# imxrt-ral

A Rust register access layer (RAL), and SVD patches for NXP i.MX RT processors.

[![All Checks][all-checks-badge]][all-checks-url] [![Crates.io][imxrt-hal-badge]][imxrt-hal-url]

[all-checks-badge]: https://github.com/imxrt-rs/imxrt-ral/workflows/All%20Checks/badge.svg
[all-checks-url]: https://github.com/imxrt-rs/imxrt-ral/actions?query=workflow%3A%22All+Checks%22
[imxrt-hal-badge]: https://img.shields.io/crates/v/imxrt-ral
[imxrt-hal-url]: https://crates.io/crates/imxrt-ral

**[API Docs (main branch)][main-api-docs]**

[main-api-docs]: https://imxrt-rs.github.io/imxrt-ral/

## Goals

- Simple but useful register level access. It compiles quickly, and it's intuitive for existing embedded developers.
- RTIC support.

## Getting Started

The `imxrt-ral` is a lower-level interface for i.MX RT processor registers with useful macros. The `imxrt-ral` is modeled after the [`stm32ral` crate](https://github.com/adamgreig/stm32ral). It provides direct access to the processor's registers. Use the `imxrt-ral` if you'd like to create your own hardware abstraction layer, or a custom driver.

The `imxrt-ral` supports these i.MX RT processors:

- [x] `"imxrt1011"`
- [x] `"imxrt1015"`
- [x] `"imxrt1021"`
- [x] `"imxrt1051"`
- [x] `"imxrt1052"`
- [x] `"imxrt1061"`
- [x] `"imxrt1062"`
- [x] `"imxrt1064"`
- [x] `"imxrt1176_cm4"`
- [x] `"imxrt1176_cm7"`
- [x] `"imxrt1189_cm33"`
- [x] `"imxrt1189_cm7"`

The RAL also **requires** a feature flag to specify the processor variant. The RAL is [on crates.io](https://crates.io/crates/imxrt-ral). The RAL provides the `"rt"` feature flag, and the interrupt table definition, that's used by the HAL.

## Q/A

#### *Why not use [`svd2rust`](https://docs.rs/svd2rust/0.17.0/svd2rust/) to generate a crate for register access?*

See [here](https://github.com/mciantyre/teensy4-rs/issues/48) and [here](https://users.rust-lang.org/t/svd2rust-generates-an-enormous-crate/32372). `svd2rust` generates a crate that's nearly 1 million lines of Rust code, and it takes a few minutes to compile. On the other hand, the RAL compiles in a few seconds. Additionally, `svd2rust` only supports one SVD input, but the RAL auto-generation script accepts multiple SVD inputs, sharing the common peripherals across processor families. This means that we can more easily support all i.MX RT processor variants from a single crate.

## Contributing & Development

For contributions and development guidance, see [CONTRIBUTING.md](CONTRIBUTING.md)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
