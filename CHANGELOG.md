# Changelog

## [Unreleased]

**BREAKING** Remove the NVIC and SCB peripherals from the 1176. If you need an
API for Cortex-M registers, check out the
[`cortex-m`][https://crates.io/crates/cortex-m] crate.

**BREAKING** The 1176 GPU2D interrupt is marked as reserved, and it's not
available in the `Interrupt` enum.

**BREAKING** In the 1176 API, there is only one SAI module. All SAI instances,
including those that only have a single channel, now appear to support multiple
channels. The user is responsible for making sure their SAI instance can
truly support multiple channels.

**BREAKING** The `flexio1` module is now called `flexio` for the 1010,
1015, and 1020 families.

**BREAKING** Rename LPI2C.SIER field "AM1F" to "AM1IE".

**BREAKING** Rename SRC.SRSR field "LOCKUP" to "LOCKUP_SYSRESETREQ" for 1011,
1015, and 1051 MCUs.

**BREAKING** Rename CCM.CS1CDR field "FLEXIO1" to "FLEXIO2" for the 1050 and
1060 families.

**BREAKING** Expose the "TRNG_ACC" field in TRNG.MCTL on the 1015, 1021 MCUs.

**BREAKING** Rename IOMUXC_GPR.GPR26 field "GPIO_SEL" to "GPIO_MUX1_GPIO_SEL,"
and correct the documentation.

**BREAKING** Correct the name, documentation of the SNVS.LPCR field "LVD_EN" on
the 1176.

**BREAKING** Correct the symbols for most IOMUXC `*SELECT_INPUT*` enum variants.
As a result of this change, symbols for IOMUXC `SW_MUX_CTL*` and `*SW_PAD_CTL*`
enum variants may have also changed.

**BREAKING** Remove deprecated items from the 0.5.1 release.

Drop all register fields that are documented as "reserved" (first word of the
description, all lowercase). Dropping these fields changes the combiner's
approach for combining fieldsets, enabling correct documentation for non-
reserved fields.

Fix the documentation associated with IOMUXC field values.

## [0.5.3] 2023-08-21

Exclude the interrupt vector table when we're building for a target with an
operating system. This ensures you can build imxrt-ral in different contexts,
like build scripts.

## [0.5.2] 2023-08-02

Add missing register fields:

- SERCLKDIV in FlexSPI MCR0.
- FILT_PRSC in ENC FILT.

## [0.5.1] 2023-07-03

*Deprecated* The following items will be renamed in the next breaking release:

| Deprecated item | New item             | Notes                          |
| --------------- | -------------------- | -------------------------------|
|   `flexio1`     | `flexio`             | 1010, 1015, and 1020 families. |
|   `AM1F`        | `AM1IE`              | LPI2C.SIER field.              |
|   `LOCKUP`      | `LOCKUP_SYSRESETREQ` | SRC.SRSR field.                |
|   `FLEXIO1_*`   | `FLEXIO2_*`          | CCM.CS1CDR fields.             |
|   `GPIO_SEL`    | `GPIO_MUX1_GPIO_SEL` | 1060 IOMUXC_GPR.GPR26 field.   |
| `PWR_GLITCH_EN` | `LVD_EN`             | 1170 SNVS.LPCR field.          |

Fix the TRNG peripheral block for the 1015, 1021 families.

Fix FlexIO register fields that describe pin counts, ensuring that they can
represent the 32 pins available for FlexIO2 and FlexIO3. Note that this
increases the field width for FlexIO1, even though this instance only supports
16 pins. FlexIO1 users should take care to only access the lower four bits of
such fields.

Fix the FlexIO timer and shifter counts from four to eight, permitting access
to all available components.

## [0.5.0] 2022-12-27

Add support for NXP's i.MX RT 1176 dual-core MCUs. An `"imxrt1176_cm7"` feature
targets the Cortex-M7, and an `"imxrt1176_cm4"` feature targets the Cortex-M4.

**BREAKING** Peripheral instances are acquired with an unsafe `instance()`
method, and `imxrt-ral` does not provide a resource management strategy.
Replace usages of `take()` and `steal()` with `instance().` See the before and
after in the next note for an example.

**BREAKING** Peripherals that support multiple instances are now unique types.
This affects APIs that accept instances. See the before and after below for
a summary. The usage documentation contains more examples.

```rust
use imxrt_ral::{ccm, lpuart};

// Before
let _: lpuart::Instance = lpuart::LPUART4::take().unwrap();
let _: ccm::Instance = ccm::CCM::take().unwrap();

// After
let lpuart4: lpuart::Instance<4> = unsafe { lpuart::LPUART4::instance() };
// Equivalent:
let lpuart4: lpuart::LPUART4 = lpuart4;

// Use peripheral names when naming singleton instances:
let _: ccm::CCM = unsafe { ccm::CCM::instance() };
```

**BREAKING** Undo an SVD patch that renamed the USB module path for 1010,
1015, and 1020 chips. `imxrt-ral` users on those systems may now find USB
register APIS at the `imxrt_ral::usb` path, rather than `imxrt_ral::usb1`.
This maintains consistency with the other chip variants.

**BREAKING** This release removes the `cortex_m` module.  Additionally, this
release removes the NVIC and SystemControl peripherals, which were available
under chip-specific modules. If you need an API for Cortex-M registers,
consider using the [`cortex-m`][https://crates.io/crates/cortex-m] crate.

**BREAKING** The RAL depends on `cortex-m`, version `0.7`. All `Interrupt`
enumerations now implement `cortex_m::interrupt::InterruptNumber`, instead
of `bare_metal::Nr`.

**BREAKING** The following symbols have been renamed:

- (1011, 1015) The `adc1` module is now `adc`. Rename applies to the peripheral
  instance, too: `ADC1` is now `ADC`.
- (1011, 1015) The `pwm1` modules is now `pwm`; similar to the ADC rename, above.
- (all 1050, 1060 chips) `ENET` is `ENET1`, and `FLEXSPI` is `FLEXSPI1`. The
  instances that end with '2' are unchanged.

Depend on `ral-registers`, and expose the API through the `imxrt-ral` package.

## [0.4.4] 2022-08-21

Correct enumerated values that were specified with alternate binary values in
the SVD files.

## [0.4.3] 2022-07-06

Mark all registers as `#[repr(transparent)]`.

Fix builds with the `"nosync"` feature. Use the `"nosync"` feature to disable
owned peripheral instance. You may only access registers behind the `unsafe`
API, which directly manipulates global registers.

Associate interrupt signals with peripheral instances:

- `Instance`s now have an `interrupts()` method that returns zero or more
  interrupt signals.
- Peripheral instance modules have an associated constant, `INTERRUPTS`,
  that is an array of zero or more interrupt signals.

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

[Unreleased]: https://github.com/imxrt-rs/imxrt-ral/compare/0.5.0...HEAD
[0.5.3]: https://github.com/imxrt-rs/imxrt-ral/compare/0.5.2...0.5.3
[0.5.2]: https://github.com/imxrt-rs/imxrt-ral/compare/0.5.1...0.5.2
[0.5.1]: https://github.com/imxrt-rs/imxrt-ral/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.4...0.5.0
[0.4.4]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.3...0.4.4
[0.4.3]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.2...0.4.3
[0.4.2]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.1...0.4.2
[0.4.1]: https://github.com/imxrt-rs/imxrt-ral/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/imxrt-rs/imxrt-ral/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/imxrt-rs/imxrt-ral/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/imxrt-rs/imxrt-ral/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/imxrt-rs/imxrt-ral/compare/0.1.0...0.2.1
[0.1.0]: https://github.com/imxrt-rs/imxrt-ral/releases/tag/0.1.0
