This package provides a register access layer (RAL) for i.MX RT processors.
For more information, see [the README][README].

[README]: https://github.com/imxrt-rs/imxrt-ral

# Getting started

Select your chip and enable its feature flag. See [the README][README] for more
information on available chip features.

There are two ways to interact with peripherals and registers:

1. Fabricate a peripheral _instance_ with the unsafe `instance()` method.
2. Interact directly with the peripheral pointers, and mark all accesses as `unsafe`.

## Fabricate a peripheral instance

```no_run
use imxrt_ral as ral;
use ral::lpuart;

let mut lpuart2 = unsafe { lpuart::LPUART2::instance() };
let version = ral::read_reg!(lpuart, lpuart2, VERID);
# let byte = 0;
ral::modify_reg!(lpuart, lpuart2, CTRL, TE: 1, RE: 1);
ral::write_reg!(lpuart, lpuart2, DATA, byte);
```

Fabricating an instance is always `unsafe`. There are no checks that prevent
aliases to the same peripheral memory. If you're using this API, you need to
make sure that creating an instance is appropriate in your program's context.

It's helpful to design drivers to peripheral instances, since register accesses do
not need an `unsafe` block. The driver assumes that it has complete ownership
of the instance, and uses the instance to manage the hardware. See the [Usage](#usage) section
for more ideas.

## Interact directly with pointers

```no_run
use imxrt_ral as ral;
use ral::lpuart;

let version = unsafe { ral::read_reg!(lpuart, lpuart::LPUART2, VERID) };
# let byte = 0;
unsafe { ral::modify_reg!(lpuart, lpuart::LPUART2, CTRL, TE: 1, RE: 1) };
unsafe { ral::write_reg!(lpuart, lpuart::LPUART2, DATA, byte) };
```

If you're familiar with using C for embedded code, this is C mode. You're
responsible for making sure that register accesses are coordinated across all contexts.
You also need to coordinate with anyone who's using the instance API.

## Register access macros

`imxrt-ral` re-exports the [ral-registers](https://docs.rs/ral-registers/0.1.1/ral_registers/)
API. These macros make it easy to access register and register fields. For more information,
see [`read_reg`], [`write_reg`], and [`modify_reg`]. Note that the documentation assumes an
STM32 processor, and may demonstrate a different API for accessing instances.

> Note: `imxrt-ral` does not yet support the `reset_reg` macro, and it does not expose reset
> structs.

## Resource management

Unlike some peripheral access crates (PACs) or register access layers, `imxrt-ral` does not
provide a resource management policy for register blocks. Instead, the API uses `unsafe`
to signal that you may be mutably aliasing peripheral registers. This package expects
peripheral resource management to be handled by a higher-level crate, like a BSP or a custom
package that's aware of multi-core execution and resource management.

# Usage

imxrt-ral APIs use const generics to differentiate different peripheral instances.
This section describes some design techniques for using, or ignoring, these type
hints.

A function that accepts any GPIO instance:

```
use imxrt_ral::gpio;

fn any_gpio<const N: u8>(gpio: gpio::Instance<N>) { /* ... */ }

let gpio1 = unsafe { gpio::GPIO1::instance() };
let gpio2 = unsafe { gpio::GPIO2::instance() };

any_gpio(gpio1);
any_gpio(gpio2);
```

A function that *only* accepts GPIO1:

```
use imxrt_ral::gpio;

fn only_gpio1(gpio: &gpio::Instance<1>) { /* ... */ }
fn only_gpio1_alias(gpio: &gpio::GPIO1) { /* ... */ }

let gpio1 = unsafe { gpio::GPIO1::instance() };

only_gpio1(&gpio1);
only_gpio1_alias(&gpio1);
```

`only_gpio1` rejects GPIO2 at compile time:

```compile_fail
# use imxrt_ral::gpio; fn only_gpio1(gpio: &gpio::Instance<1>) { /* ... */ }
let gpio2 = unsafe { gpio::GPIO2::instance() };
only_gpio1(&gpio2);
```

These function designs apply to structures, too:

```
use imxrt_ral::gpio;

struct GpioDriver<const N: u8> {
    gpio: gpio::Instance<N>,
    // ...
}

impl<const N: u8> GpioDriver<N> {
    pub fn new(gpio: gpio::Instance<N>) -> Self {
        // ...
        # GpioDriver { gpio }
    }
}

let gpio1_driver = GpioDriver::new(unsafe { gpio::GPIO1::instance() });
```

Require that other resources, which are tagged with instance identifiers,
match their peripheral instance:

```
use imxrt_ral::gpio;

/// Type-level constant...
enum Const<const N: u8> {}

/// Represents a GPIO pin
trait GpioPin {
    /// The associated GPIO instance number
    type GpioInstance;
    const OFFSET: u32;
}

/// GPIO2[17]
struct AD_B1_00 { /* ... */ }

impl GpioPin for AD_B1_00 {
    type GpioInstance = Const<2>;
    const OFFSET: u32 = 17;
}
# struct GpioDriver<const N: u8> {
#     gpio: gpio::Instance<N>,
# }

impl<const N: u8> GpioDriver<N> {
    # pub fn new(gpio: gpio::Instance<N>) -> Self { Self { gpio } }
    pub fn set_high<P>(&mut self, pin: &mut P)
    where
        P: GpioPin<GpioInstance = Const<N>>, // <-- Requirement here
    {
        // ...
    }
}

let mut gpio2 = GpioDriver::new(unsafe { gpio::GPIO2::instance() });
let mut ad_b1_00 = // Ownership of pin...
    # AD_B1_00 {};
gpio2.set_high(&mut ad_b1_00);
```

If you provide a GPIO**1** pin to the GPIO**2** driver, it fails
at compile time:

```compile_fail
# use imxrt_ral::gpio;
# enum Const<const N: u8> {}
# trait GpioPin {
#     /// The associated GPIO instance number
#     type GpioInstance;
#     const OFFSET: u32;
# }
/// GPIO1[6]
struct SD_B0_03 { /* ... */ }

impl GpioPin for SD_B0_03 {
    type GpioInstance = Const<1>;
    const OFFSET: u32 = 6;
}
# struct GpioDriver<const N: u8> {
#     gpio: gpio::Instance<N>,
# }
# impl<const N: u8> GpioDriver<N> {
#     pub fn new(gpio: gpio::Instance<N>) -> Self { Self { gpio } }
#     pub fn set_high<P>(&mut self, pin: &mut P)
#     where
#         P: GpioPin<GpioInstance = Const<N>>, // <-- Requirement here
#     {
#         // ...
#     }
# }

let mut gpio2 = GpioDriver::new(unsafe { gpio::GPIO2::instance() });
let mut sd_b0_03 = // Ownership of pin...
    # SD_B0_03 {};
// Incorrect: GPIO1 pin with GPIO2 driver
gpio2.set_high(&mut sd_b0_03);
```

If you would like such a statement to compile, remove the `GpioPin` constraint
on the `set_high` function:

```
# use imxrt_ral::gpio;
# enum Const<const N: u8> {}
# trait GpioPin {
#     /// The associated GPIO instance number
#     type GpioInstance;
#     const OFFSET: u32;
# }
# /// GPIO1[6]
# struct SD_B0_03 { /* ... */ }
# impl GpioPin for SD_B0_03 {
#     type GpioInstance = Const<1>;
#     const OFFSET: u32 = 6;
# }
# struct GpioDriver<const N: u8> {
#     gpio: gpio::Instance<N>,
# }
impl<const N: u8> GpioDriver<N> {
    # pub fn new(gpio: gpio::Instance<N>) -> Self { Self { gpio } }
    pub fn set_high<P>(&mut self, pin: &mut P)
    where
        P: GpioPin/*<GpioInstance = Const<N>>*/, // No constraint that pin matches driver
    {
        // ...
    }
}

let mut gpio2 = GpioDriver::new(unsafe { gpio::GPIO2::instance() });
let mut sd_b0_03 = // Ownership of pin...
    # SD_B0_03 {};
// Now OK: GPIO1 pin with GPIO2 driver
gpio2.set_high(&mut sd_b0_03);
```

If you don't want to carry around a generic type for your driver struct,
you can still model peripheral ownership, and work with a pointer to the
register block. You'll need to use some `unsafe` code, as shown below:

```
use imxrt_ral::gpio;

struct GpioDriver {
    gpio: &'static gpio::RegisterBlock,
}

impl GpioDriver {
    pub fn new<const N: u8>(gpio: gpio::Instance<N>) -> GpioDriver {
        // Instance derefs to a register block
        let register: *const gpio::RegisterBlock = &*gpio;
        // Safety: pointer points to static peripheral memory,
        // which will outlive the gpio Instance.
        let register = unsafe { &*register };
        GpioDriver { gpio: register }
    }
}

let gpio2 = GpioDriver::new(unsafe { gpio::GPIO2::instance() });
```

This approach loses some of the compile-time checks, but may be simpler
for others to use.

## `[Peripheral name]` vs `Instance<N>`

Design to a concrete type when you know that there's only one, single instance
of that peripheral across all chips. This simplifies your driver API while still
supporting all i.MX RT chips. The CCM peripheral is an example of a peripheral with
one instance across all i.MX RT chips.

```
use imxrt_ral::ccm;

// A truly single instance:
fn new_ccm(_: &ccm::CCM) { /* ... */ }

// Still works, but more general (though the generality isn't
// necessary, since there's only one CCM instance)
fn new_ccm_explicit<const N: u8>(_: &ccm::Instance<N>) { /* ... */ }

let ccm = unsafe { ccm::CCM::instance() };
new_ccm(&ccm);
new_ccm_explicit(&ccm);
```

A `CCM` is actually an `Instance<imxrt_ral::SOLE_INSTANCE>`. So, you could
always design to a generic `Instance` type to be explicit, but it's not necessary.

When there's a chance for a peripheral to have multiple instances across
different chips, favor `Instance<N>` for maximal reuse. Since all instances
are generic, the same function should work no matter how many peripheral
instances exist on your chip. For example, this same function works for 1021
chips -- having only one USB instance -- and 1062 chips -- having two USB instances.

```
use imxrt_ral::usb;

fn new_usb_driver<const N: u8>(_: usb::Instance<N>) { /* ... */ }

#[cfg(feature = "imxrt1021")]
new_usb_driver(unsafe { usb::USB::instance() });

#[cfg(feature = "imxrt1062")]
{
    new_usb_driver(unsafe { usb::USB1::instance() });
    new_usb_driver(unsafe { usb::USB2::instance() });
}
```

When compared to the USB implementation, you would *not* want to use
the a concrete USB `Instance`, since there are chips that have multiple
USB instances:

```compile_fail
use imxrt_ral::usb;

/// A function that only takes the sole USB instance.
fn new_usb_driver(_: usb::Instance<0>) { /* ... */ }

#[cfg(feature = "imxrt1062")]
new_usb_driver(unsafe { usb::USB1::instance() }); // <-- Fails to compile! Instance<1> != Instance<0>

#[cfg(feature = "imxrt1021")]
new_usb_driver(unsafe { usb::USB::instance() }); // <-- Doesn't work here, either! USB == Instance<0> != Instance<1>
# #[cfg(feature = "imxrt1021")]
# compile_error!("Forced failure to meet test requirements");
```

## Valid instance numbers

Consider a function that needs to change behavior given only the const generic instance
number:

```should_panic
use imxrt_ral::ccm;

/// Enable the LPUART clock gate in the CCM.
///
/// # Panics
///
/// Panics if `LPUART_N` does not represent a valid LPUART instance.
fn ccm_enable_lpuart_clock_gate<const LPUART_N: u8>(ccm: &mut ccm::CCM) {
    match LPUART_N {
        1 => { /* ... */ }
        2 => { /* ... */ }
        3 => { /* ... */ }
        // 4..=8
        _ => panic!("Unhandled LPUART instance number"),
    }
}

# || -> Option<()> {
let mut ccm = unsafe { ccm::CCM::instance() };
ccm_enable_lpuart_clock_gate::<3>(&mut ccm); // OK: LPUART3 is valid.
ccm_enable_lpuart_clock_gate::<9>(&mut ccm); // panic! LPUART9 isn't valid
# Some(()) }();
```

To catch an invalid N at compile time, use `Valid`, a trait implemented on
all valid `Instance<N>` types.

```
# use imxrt_ral::ccm;
use imxrt_ral::lpuart;

fn ccm_enable_lpuart_clock_gate<const LPUART_N: u8>(ccm: &mut ccm::CCM)
where
    lpuart::Instance<LPUART_N>: imxrt_ral::Valid, // NEW: constrain LPUART_N to valid instance numbers.
{
    match LPUART_N {
        1 => { /* ... */ }
        2 => { /* ... */ }
        3 => { /* ... */ }
        // 4..=8
        _ => unreachable!("Handled all LPUART instances"),
    }
}

# || -> Option<()> {
let mut ccm = unsafe { ccm::CCM::instance() };
ccm_enable_lpuart_clock_gate::<3>(&mut ccm); // OK: LPUART3 is valid.
# Some(()) }();
```
```compile_fail
# use imxrt_ral::ccm;
# use imxrt_ral::lpuart;
# fn ccm_enable_lpuart_clock_gate<const LPUART_N: u8>(ccm: &mut ccm::CCM)
# where
#     lpuart::Instance<LPUART_N>: imxrt_ral::Valid,
# {}
# || -> Option<()> {
# let mut ccm = unsafe { ccm::CCM::instance() };
ccm_enable_lpuart_clock_gate::<9>(&mut ccm); // Does not compile!
# Some(()) }();
```

## Advanced usage

You can `unsafe`ly instantiate any `Instance` from a pointer using `new`. This
is helpful if your strongly-numbered adapter is only using a pointer / static
reference to a register block, yet you need to reconstruct the `Instance`
for a user.

```
use imxrt_ral::lpuart;

pub struct Lpuart<const N: u8> {
    /// Not holding lpuart::Instance<N>.
    /// Instead, we're just keeping a reference
    /// after taking ownership of the instance.
    ptr: &'static lpuart::RegisterBlock,
}

impl<const N: u8> Lpuart<N> {
    pub fn new(inst: lpuart::Instance<N>) -> Self {
        let ptr: *const lpuart::RegisterBlock = &*inst;
        // Safety: pointer truly points to static memory.
        Self { ptr: unsafe { &*ptr }}
    }
    pub fn release(self) -> lpuart::Instance<N> {
        // Safety: The N associated with this type
        // is still associated with its register block.
        // We're not accidentally returning Instance<1>
        // when we have a reference to Instance<2>.
        //
        // The pointer points to valid LPUART memory.
        unsafe { lpuart::Instance::new(self.ptr) }
    }
}
```

If you're fully discarding all type information, you can use
the `number` function in each peripheral module to acquire the
instance number for a register block. Note that this incurs a
small runtime cost of up to `N` pointer compares, where `N` is
the number of valid instances.

```
use imxrt_ral::lpuart;

/// Note that there's no `N` const generic,
/// so that information isn't in the type system.
pub struct AnyLpuart {
    ptr: &'static lpuart::RegisterBlock,
}

impl AnyLpuart {
    pub fn new<const N: u8>(inst: lpuart::Instance<N>) -> Self {
        let ptr: *const lpuart::RegisterBlock = &*inst;
        // Safety: pointer truly points to static memory.
        Self { ptr: unsafe { &*ptr }}
    }

    pub fn instance(&self) -> u8 {
        // Unwrap OK; `new` guarantees that it's one of
        // the N LPUART instances.
        lpuart::number(self.ptr).unwrap()
    }
}
```

```
use imxrt_ral::{ccm, lpuart};

assert_eq!(ccm::number(ccm::CCM), Some(0));
assert_eq!(lpuart::number(lpuart::LPUART2), Some(2));
assert_eq!(lpuart::number(ccm::CCM as _), None);
```
