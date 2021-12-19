#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBPHY Register Reference Index
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
use crate::imxrt106::peripherals::usbphy::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::usbphy::{Instance, Valid};
pub use crate::imxrt106::peripherals::usbphy::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::usbphy::{
    CTRL, CTRL_CLR, CTRL_SET, CTRL_TOG, DEBUG, DEBUG0_STATUS, DEBUG1, DEBUG1_CLR, DEBUG1_SET,
    DEBUG1_TOG, DEBUG_CLR, DEBUG_SET, DEBUG_TOG, PWD, PWD_CLR, PWD_SET, PWD_TOG, RX, RX_CLR,
    RX_SET, RX_TOG, STATUS, TX, TX_CLR, TX_SET, TX_TOG, VERSION,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The USBPHY1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type USBPHY1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for USBPHY1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for USBPHY1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBPHY1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBPHY1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBPHY1 {
    const INSTANCE: Self = Self {
        addr: 0x400d9000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::USB_PHY1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in USBPHY1
    pub const reset: ResetValues = ResetValues {
        PWD: 0x001E1C00,
        PWD_SET: 0x001E1C00,
        PWD_CLR: 0x001E1C00,
        PWD_TOG: 0x001E1C00,
        TX: 0x10060607,
        TX_SET: 0x10060607,
        TX_CLR: 0x10060607,
        TX_TOG: 0x10060607,
        RX: 0x00000000,
        RX_SET: 0x00000000,
        RX_CLR: 0x00000000,
        RX_TOG: 0x00000000,
        CTRL: 0xC0200000,
        CTRL_SET: 0xC0200000,
        CTRL_CLR: 0xC0200000,
        CTRL_TOG: 0xC0200000,
        STATUS: 0x00000000,
        DEBUG: 0x7F180000,
        DEBUG_SET: 0x7F180000,
        DEBUG_CLR: 0x7F180000,
        DEBUG_TOG: 0x7F180000,
        DEBUG0_STATUS: 0x00000000,
        DEBUG1: 0x00001000,
        DEBUG1_SET: 0x00001000,
        DEBUG1_CLR: 0x00001000,
        DEBUG1_TOG: 0x00001000,
        VERSION: 0x04030000,
    };

    /// Safe access to USBPHY1
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
        let taken = USBPHY1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBPHY1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBPHY1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBPHY1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBPHY1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with USBPHY1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::USB_PHY1];

    /// The interrupts associated with USBPHY1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBPHY1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBPHY1: *const RegisterBlock = 0x400d9000 as *const _;

/// The USBPHY2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type USBPHY2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for USBPHY2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for USBPHY2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBPHY2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBPHY2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBPHY2 {
    const INSTANCE: Self = Self {
        addr: 0x400da000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::USB_PHY2],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in USBPHY2
    pub const reset: ResetValues = ResetValues {
        PWD: 0x001E1C00,
        PWD_SET: 0x001E1C00,
        PWD_CLR: 0x001E1C00,
        PWD_TOG: 0x001E1C00,
        TX: 0x10060607,
        TX_SET: 0x10060607,
        TX_CLR: 0x10060607,
        TX_TOG: 0x10060607,
        RX: 0x00000000,
        RX_SET: 0x00000000,
        RX_CLR: 0x00000000,
        RX_TOG: 0x00000000,
        CTRL: 0xC0200000,
        CTRL_SET: 0xC0200000,
        CTRL_CLR: 0xC0200000,
        CTRL_TOG: 0xC0200000,
        STATUS: 0x00000000,
        DEBUG: 0x7F180000,
        DEBUG_SET: 0x7F180000,
        DEBUG_CLR: 0x7F180000,
        DEBUG_TOG: 0x7F180000,
        DEBUG0_STATUS: 0x00000000,
        DEBUG1: 0x00001000,
        DEBUG1_SET: 0x00001000,
        DEBUG1_CLR: 0x00001000,
        DEBUG1_TOG: 0x00001000,
        VERSION: 0x04030000,
    };

    /// Safe access to USBPHY2
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
        let taken = USBPHY2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBPHY2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBPHY2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBPHY2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBPHY2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with USBPHY2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::USB_PHY2];

    /// The interrupts associated with USBPHY2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBPHY2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBPHY2: *const RegisterBlock = 0x400da000 as *const _;
