#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB

#[cfg(not(feature = "nosync"))]
use crate::imxrt101::peripherals::usb::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::usb::{Instance, Valid};
pub use crate::imxrt101::peripherals::usb::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::usb::{
    ASYNCLISTADDR, BURSTSIZE, CAPLENGTH, CONFIGFLAG, DCCPARAMS, DCIVERSION, DEVICEADDR,
    ENDPTCOMPLETE, ENDPTCTRL0, ENDPTCTRL1, ENDPTCTRL2, ENDPTCTRL3, ENDPTCTRL4, ENDPTCTRL5,
    ENDPTCTRL6, ENDPTCTRL7, ENDPTFLUSH, ENDPTNAK, ENDPTNAKEN, ENDPTPRIME, ENDPTSETUPSTAT,
    ENDPTSTAT, FRINDEX, GPTIMER0CTRL, GPTIMER0LD, GPTIMER1CTRL, GPTIMER1LD, HCCPARAMS, HCIVERSION,
    HCSPARAMS, HWDEVICE, HWGENERAL, HWHOST, HWRXBUF, HWTXBUF, ID, OTGSC, PORTSC1, SBUSCFG,
    TXFILLTUNING, USBCMD, USBINTR, USBMODE, USBSTS,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The USB peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type USB = Instance<0>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for USB {}
#[cfg(not(feature = "nosync"))]
impl Valid for USB {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USB_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USB peripheral instance
#[cfg(not(feature = "nosync"))]
impl USB {
    const INSTANCE: Self = Self {
        addr: 0x402e0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::USB_OTG1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in USB
    pub const reset: ResetValues = ResetValues {
        ID: 0xE4A1FA05,
        HWGENERAL: 0x00000035,
        HWHOST: 0x10020001,
        HWDEVICE: 0x00000011,
        HWTXBUF: 0x80080B08,
        HWRXBUF: 0x00000808,
        GPTIMER0LD: 0x00000000,
        GPTIMER0CTRL: 0x00000000,
        GPTIMER1LD: 0x00000000,
        GPTIMER1CTRL: 0x00000000,
        SBUSCFG: 0x00000002,
        CAPLENGTH: 0x00000040,
        HCIVERSION: 0x00000100,
        HCSPARAMS: 0x00010011,
        HCCPARAMS: 0x00000006,
        DCIVERSION: 0x00000001,
        DCCPARAMS: 0x00000188,
        USBCMD: 0x00080000,
        USBSTS: 0x00000000,
        USBINTR: 0x00000000,
        FRINDEX: 0x00000000,
        DEVICEADDR: 0x00000000,
        ASYNCLISTADDR: 0x00000000,
        BURSTSIZE: 0x00000808,
        TXFILLTUNING: 0x00000000,
        ENDPTNAK: 0x00000000,
        ENDPTNAKEN: 0x00000000,
        CONFIGFLAG: 0x00000001,
        PORTSC1: 0x10000000,
        OTGSC: 0x00001120,
        USBMODE: 0x00005000,
        ENDPTSETUPSTAT: 0x00000000,
        ENDPTPRIME: 0x00000000,
        ENDPTFLUSH: 0x00000000,
        ENDPTSTAT: 0x00000000,
        ENDPTCOMPLETE: 0x00000000,
        ENDPTCTRL0: 0x00800080,
        ENDPTCTRL1: 0x00000000,
        ENDPTCTRL2: 0x00000000,
        ENDPTCTRL3: 0x00000000,
        ENDPTCTRL4: 0x00000000,
        ENDPTCTRL5: 0x00000000,
        ENDPTCTRL6: 0x00000000,
        ENDPTCTRL7: 0x00000000,
    };

    /// Safe access to USB
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
        let taken = USB_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USB_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USB_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with USB
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::USB_OTG1];

    /// The interrupts associated with USB
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB: *const RegisterBlock = 0x402e0000 as *const _;
