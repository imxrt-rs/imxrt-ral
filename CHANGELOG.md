# Changelog

## [Unreleased]

## [0.4.4] 2022-08-21

Correct enumerated values that were specified with alternate binary values in
the SVD files.

## [0.4.3] 2022-07-06

Mark all registers as `#[repr(transparent)]`.

Fix builds with the `"nosync"` feature. Use the `"nosync"` feature to disable
owned peripheral instance. You may only access registers behind the `unsafe`
API, which directly manipulates global registers.

Change how reserved peripheral memory is computed, resolving incorrect
register offsets for the following peripherals and chips:

- FlexPWM for the 1011.
- Quad timers for all applicable chips.

## [0.4.2] 2021-03-11

This release may let you use `imxrt-ral` without including additional
dependencies. `imxrt-ral` dependencies are still required when you specify a
chip-specific feature, like `"imxrt1062"`. But as of this release, those
dependencies aren't necessary when building without chip-specific features.

Use `imxrt-ral` without features to access

- the read, write, modify, and reset macros
- the register definitions

You may use these APIS when designing an `imxrt-ral`-like API in an i.MX RT
driver crate.

## [0.4.1] 2021-02-12

This release corrects for missing, or incomplete, information in the i.MX RT
SVD files. The changes manifest in the `imxrt-ral` crate.

* Change USB's `ENDPTSTAT` access to read-write, supporting the access required
  for USB bus resets.
* Add RIDMAE field to the BAUD register of i.MX RT 1015 and 1021 LPUART
  peripherals.
* Correct USBCMD\[ATDTW\] bit offset for 1021, 1051, 1052, 1061, 1062, and 1064
  chips. SVD identifies the offset as 12, when it's 14. Refer to the reference
  manuals for more information.
* Correct the LDVAL bitwidth for PIT peripherals on 1015 and 1021 chips. The
  SVDs indicate that the field is 24 bits, when it's 32 bits.

This release also removes mention of 'stm32ral' in the API documentation.

## [0.4.0] 2020-08-29

* **BREAKING** The RAL's `"rtfm"` feature is changed to `"rtic"`, reflecting the framework's
  new name. Users who are relying on the `"rtfm"` feature should now use the `"rtic"` feature.

## [0.3.0] 2020-06-18

* Only emit link section for `__INTERRUPTS` when compiling for ARM targets
* Fix RAL's documentation to refer to i.MX RT registers

## [0.2.1] 2020-04-10

* Fixes cargo release, adds release building documentation

## [0.2.0] 2020-04-08

* Port of ccm, iomuxc, uart, i2c, and spi peripherals from teensy4-rs!
* Support for imxrt1060evk board as well as teensy4

## [0.1.0] 2020-02-06

Initial build and release of imxrt family of peripheral access crates

[Unreleased]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.0...HEAD
[0.4.4]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.3...0.4.4
[0.4.3]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.2...0.4.3
[0.4.2]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.1...0.4.2
[0.4.1]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/imxrt-rs/imxrt-ral/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/imxrt-rs/imxrt-ral/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/imxrt-rs/imxrt-ral/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/imxrt-rs/imxrt-ral/compare/0.1.0...0.2.1
[0.1.0]: https://github.com/imxrt-rs/imxrt-ral/releases/tag/0.1.0
