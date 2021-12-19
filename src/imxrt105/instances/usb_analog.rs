#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB Analog
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::usb_analog::Instance;
pub use crate::imxrt105::peripherals::usb_analog::{RegisterBlock, ResetValues};

pub use crate::imxrt105::peripherals::usb_analog::{
    DIGPROG, USB1_CHRG_DETECT, USB1_CHRG_DETECT_CLR, USB1_CHRG_DETECT_SET, USB1_CHRG_DETECT_STAT,
    USB1_CHRG_DETECT_TOG, USB1_LOOPBACK, USB1_LOOPBACK_CLR, USB1_LOOPBACK_SET, USB1_LOOPBACK_TOG,
    USB1_MISC, USB1_MISC_CLR, USB1_MISC_SET, USB1_MISC_TOG, USB1_VBUS_DETECT, USB1_VBUS_DETECT_CLR,
    USB1_VBUS_DETECT_SET, USB1_VBUS_DETECT_STAT, USB1_VBUS_DETECT_TOG, USB2_CHRG_DETECT,
    USB2_CHRG_DETECT_CLR, USB2_CHRG_DETECT_SET, USB2_CHRG_DETECT_STAT, USB2_CHRG_DETECT_TOG,
    USB2_LOOPBACK, USB2_LOOPBACK_CLR, USB2_LOOPBACK_SET, USB2_LOOPBACK_TOG, USB2_MISC,
    USB2_MISC_CLR, USB2_MISC_SET, USB2_MISC_TOG, USB2_VBUS_DETECT, USB2_VBUS_DETECT_CLR,
    USB2_VBUS_DETECT_SET, USB2_VBUS_DETECT_STAT, USB2_VBUS_DETECT_TOG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The USB_ANALOG peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type USB_ANALOG = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USB_ANALOG_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USB_ANALOG peripheral instance
#[cfg(not(feature = "nosync"))]
impl USB_ANALOG {
    const INSTANCE: Self = Self {
        addr: 0x400d8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in USB_ANALOG
    pub const reset: ResetValues = ResetValues {
        USB1_VBUS_DETECT: 0x00100004,
        USB1_VBUS_DETECT_SET: 0x00100004,
        USB1_VBUS_DETECT_CLR: 0x00100004,
        USB1_VBUS_DETECT_TOG: 0x00100004,
        USB1_CHRG_DETECT: 0x00000000,
        USB1_CHRG_DETECT_SET: 0x00000000,
        USB1_CHRG_DETECT_CLR: 0x00000000,
        USB1_CHRG_DETECT_TOG: 0x00000000,
        USB1_VBUS_DETECT_STAT: 0x00000000,
        USB1_CHRG_DETECT_STAT: 0x00000000,
        USB1_LOOPBACK: 0x00000000,
        USB1_LOOPBACK_SET: 0x00000000,
        USB1_LOOPBACK_CLR: 0x00000000,
        USB1_LOOPBACK_TOG: 0x00000000,
        USB1_MISC: 0x00000002,
        USB1_MISC_SET: 0x00000002,
        USB1_MISC_CLR: 0x00000002,
        USB1_MISC_TOG: 0x00000002,
        USB2_VBUS_DETECT: 0x00100004,
        USB2_VBUS_DETECT_SET: 0x00100004,
        USB2_VBUS_DETECT_CLR: 0x00100004,
        USB2_VBUS_DETECT_TOG: 0x00100004,
        USB2_CHRG_DETECT: 0x00000000,
        USB2_CHRG_DETECT_SET: 0x00000000,
        USB2_CHRG_DETECT_CLR: 0x00000000,
        USB2_CHRG_DETECT_TOG: 0x00000000,
        USB2_VBUS_DETECT_STAT: 0x00000000,
        USB2_CHRG_DETECT_STAT: 0x00000000,
        USB2_LOOPBACK: 0x00000000,
        USB2_LOOPBACK_SET: 0x00000000,
        USB2_LOOPBACK_CLR: 0x00000000,
        USB2_LOOPBACK_TOG: 0x00000000,
        USB2_MISC: 0x00000002,
        USB2_MISC_SET: 0x00000002,
        USB2_MISC_CLR: 0x00000002,
        USB2_MISC_TOG: 0x00000002,
        DIGPROG: 0x006A0001,
    };

    /// Safe access to USB_ANALOG
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = USB_ANALOG_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USB_ANALOG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USB_ANALOG_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USB_ANALOG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USB_ANALOG_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with USB_ANALOG
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with USB_ANALOG
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USB_ANALOG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB_ANALOG: *const RegisterBlock = 0x400d8000 as *const _;
