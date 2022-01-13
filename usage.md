# Usage

imxrt-ral APIs use const generics to differentiate different peripheral instances.
This section describes some design techniques for using, or ignoring, these type
hints.

A function that accepts any GPIO instance:

```no_run
use imxrt_ral::gpio;

fn any_gpio<const N: u8>(gpio: gpio::Instance<N>) { /* ... */ }

let gpio1 = gpio::GPIO1::take().unwrap();
let gpio2 = gpio::GPIO2::take().unwrap();

any_gpio(gpio1);
any_gpio(gpio2);
```

A function that *only* accepts GPIO1:

```no_run
use imxrt_ral::gpio;

fn only_gpio1(gpio: gpio::Instance<1>) { /* ... */ }

let gpio1 = gpio::GPIO1::take().unwrap();

only_gpio1(gpio1);
```

`only_gpio1` rejects GPIO2 at compile time:

```compile_fail
# use imxrt_ral::gpio; fn only_gpio1(gpio: gpio::Instance<1>) { /* ... */ }
let gpio2 = gpio::GPIO2::take().unwrap();
only_gpio1(gpio2);
```

In fact, rejection applies to the `release` functions, too:

```compile_fail
# use imxrt_ral::gpio;
let gpio2 = gpio::GPIO2::take().unwrap();
gpio::GPIO1::release(gpio2);
```

These function designs apply to structures, too:

```no_run
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

let gpio1_driver = GpioDriver::new(gpio::GPIO1::take().unwrap());
```

Require that other resources, which are tagged with instance identifiers,
match their peripheral instance:

```no_run
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

let mut gpio2 = GpioDriver::new(gpio::GPIO2::take().unwrap());
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

let mut gpio2 = GpioDriver::new(gpio::GPIO2::take().unwrap());
let mut sd_b0_03 = // Ownership of pin...
    # SD_B0_03 {};
// Incorrect: GPIO1 pin with GPIO2 driver
gpio2.set_high(&mut sd_b0_03);
```

If you would like such a statement to compile, remove the `GpioPin` constraint
on the `set_high` function:

```no_run
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

let mut gpio2 = GpioDriver::new(gpio::GPIO2::take().unwrap());
let mut sd_b0_03 = // Ownership of pin...
    # SD_B0_03 {};
// Now OK: GPIO1 pin with GPIO2 driver
gpio2.set_high(&mut sd_b0_03);
```

If you don't want to carry around a generic type for your driver struct,
you can still model peripheral ownership, and work with a pointer to the
register block. You'll need to use some `unsafe` code, as shown below:

```no_run
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
        // gpio::Instance dropped, but it's still maked as
        // "taken." So it appears that we own it.
    }
}

let gpio2 = GpioDriver::new(gpio::GPIO2::take().unwrap());
// This would fail, since the instance is still "taken" by the
// driver. Users would need an unsafe steal() to get another handle.
// gpio::GPIO2::take().unwrap();
```

This approach loses some of the compile-time checks, but may be simpler
for others to use.

## `[Peripheral name]` vs `Instance<N>`

Design to a concrete type when you know that there's only one, single instance
of that peripheral across all chips. This simplifies your driver API while still
supporting all i.MX RT chips. The CCM peripheral is an example of a peripheral with
one instance across all i.MX RT chips.

```no_run
use imxrt_ral::ccm;

// A truly single instance:
fn new_ccm(_: &ccm::CCM) { /* ... */ }

// Still works, but more general (though the generality isn't
// necessary, since there's only one CCM instance)
fn new_ccm_explicit<const N: u8>(_: &ccm::Instance<N>) { /* ... */ }

let ccm = ccm::CCM::take().unwrap();
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

```no_run
use imxrt_ral::usb;

fn new_usb_driver<const N: u8>(_: usb::Instance<N>) { /* ... */ }

#[cfg(feature = "imxrt1021")]
new_usb_driver(usb::USB::take().unwrap());

#[cfg(feature = "imxrt1062")]
{
    new_usb_driver(usb::USB1::take().unwrap());
    new_usb_driver(usb::USB2::take().unwrap());
}
```

When compared to the USB implementation, you would *not* want to use
the a concrete USB `Instance`, since there are chips that have multiple
USB instances:

```compile_fail
use imxrt_ral::usb;

/// A function that only takes the sole USB instance, Instance<0>.
fn new_usb_driver(_: usb::USB) { /* ... */ }

#[cfg(feature = "imxrt1062")]
new_usb_driver(usb::USB1::take().unwrap()); // <-- Fails to compile! Instance<1> != Instance<0>

#[cfg(feature = "imxrt1021")]
new_usb_driver(usb::USB::take().unwrap()); // <-- Doesn't work here, either! USB == Instance<0> != Instance<1>
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
let mut ccm = ccm::CCM::take()?;
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
let mut ccm = ccm::CCM::take()?;
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
# let mut ccm = ccm::CCM::take()?;
ccm_enable_lpuart_clock_gate::<9>(&mut ccm); // Does not compile!
# Some(()) }();
```

## Disable strongly-typed instances

If you don't want strongly-typed peripheral instances, enable the `nosync` feature.
`nosync` disables all synchronised access functions, like `take()` and `release()`,
as well as all the types associated with that API. `nosync` requires direct, unsafe
access to peripherals. This is "C" mode, where you're responsible for maintaining
synchronization. `nosync` is a negative feature; enabling the feature may cause other
dependencies to break, especially if they rely on owning strongly-typed instances.

```
use imxrt_ral::gpio;
use core::sync::atomic::{AtomicBool, Ordering};

struct GpioDriver {
    gpio: &'static gpio::RegisterBlock,
}

impl GpioDriver {
    /// Acquire the GPIO1 driver, if it exists
    pub fn gpio1() -> Option<GpioDriver> {
        static TAKEN: AtomicBool = AtomicBool::new(false);
        if !TAKEN.swap(true, Ordering::SeqCst) {
            // Safety: GPIO1 pointes to static memory
            Some(unsafe { Self { gpio: &*gpio::GPIO1 } })
        } else {
            None
        }
    }
}

let gpio1 = GpioDriver::gpio1().unwrap();
assert!(GpioDriver::gpio1().is_none());
```
