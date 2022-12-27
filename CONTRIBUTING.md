# Contributing

Thanks for helping us build embedded Rust support for NXP's i.MX RT processors! Please open an issue if

- you find a bug in the RAL
- you have an idea for a feature
- something isn't clear in our documentation

## Development

The steps below are useful for developers who want to build and modify this repository's crates. All steps assume that you've cloned the repository.

### Dependencies

You'll need  a Rust installation, at least Rust 1.64, possibly later. To be safe, use the latest, stable Rust compiler.

### Generate

To generate the RAL,

1. Install `svdtools`: `cargo install svdtools`.
2. Run `make`.

If everything went well, you should find that the `src` directory is updated with Rust files. If you made changes to SVD patches, `raltool` transforms, or `raltool` itself, you should see those changes reflected in the Rust files. The RAL can build by itself: `cargo check --features imxrt1062`.

### Modify

The `imxrt-ral` crate is auto-generated from the checked-in SVD files, available in `svd`. It's checked into git, and you should always have whatever represents the latest auto-generated RAL. Generally, you should **not** manually change RAL source files; rather, you should describe changes

- as patches to one or more SVDs.
- as a `raltool` transform, described in `raltool-cfg.yaml`.
- as a change in `raltool`.

Prefer SVD patches when you're correcting a defect in the SVD, or if your change could be use beyond this project. SVD patches are located under `devices` in various YAML files.

Prefer an existing or new `raltool` transform over manual `raltool` extensions, since they're easier to study and test. See `raltool/src/transform` to understand the available transforms.

### SVD Patches

To modify the RAL, you'll need to describe your change as an SVD patch. If you'd like to add patches to the i.MX RT SVD files, learn about [svdtools](https://github.com/stm32-rs/svdtools). Use some of the existing SVD patches as a guide.

### Testing

Our CI system ensures that the RAL and HAL(s) build for all processor variants. But, we can't automatically test against hardware! To test your changes on hardware, you'll need to test the RAL and the HAL(s) using another project, like a Rust BSP crate. Some BSP crates that use the `imxrt-hal` include

- [the `imxrt1060evk-bsp` crate](https://github.com/imxrt-rs/imxrt1060evk-bsp)
- [the `teensy4-bsp` crate](https://github.com/mciantyre/teensy4-rs)

Follow the instructions in those projects to prepare an environment for testing HAL changes. You may also consider contributing to those projects. The `teensy4-rs` project, in particular, maintains a set of [examples](https://github.com/mciantyre/teensy4-rs/tree/master/teensy4-examples/src) that may help you test changes.

## Resources

- i.MX RT reference manuals are available from NXP. The reference manuals describe the i.MX RT registers and peripheral capabilities. Go [here](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/i-mx-rt-crossover-mcus:IMX-RT-SERIES), and select your processor. Then, go to "Documentation," and scroll down to "Reference Manual." You'll need a free NXP account to access the reference manuals.
- i.MX RT data sheets are available as free downloads [here](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/i-mx-rt-crossover-mcus:IMX-RT-SERIES). The data sheets are useful for understanding high-level capabilities of the i.MX RT processors. Select your processor, then go to "Documentation," then "Data Sheet."
- For other code references, consider studying
  - the [Zephyr Project](https://www.zephyrproject.org/).
  - the ARM CMSIS Packs. Here's the [MIMXRT1062 pack](https://developer.arm.com/embedded/cmsis/cmsis-packs/devices/NXP/MIMXRT1062XXXXA); NXP and ARM also provide CMSIS packs for the other i.MX RT variants.
  - NXP's MCUXpresso SDK, available [here](https://www.nxp.com/design/software/development-software/mcuxpresso-software-and-tools/mcuxpresso-software-development-kit-sdk:MCUXpresso-SDK).

## Release Steps

To create a release of the RAL, see [RELEASE.md](docs/RELEASE.md).
