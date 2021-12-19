#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PIT
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
use crate::imxrt106::peripherals::pit::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::pit::{Instance, Valid};
pub use crate::imxrt106::peripherals::pit::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::pit::{
    CVAL0, CVAL1, CVAL2, CVAL3, LDVAL0, LDVAL1, LDVAL2, LDVAL3, LTMR64H, LTMR64L, MCR, TCTRL0,
    TCTRL1, TCTRL2, TCTRL3, TFLG0, TFLG1, TFLG2, TFLG3,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PIT peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type PIT = Instance<0>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for PIT {}
#[cfg(not(feature = "nosync"))]
impl Valid for PIT {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PIT_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PIT peripheral instance
#[cfg(not(feature = "nosync"))]
impl PIT {
    const INSTANCE: Self = Self {
        addr: 0x40084000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::PIT],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in PIT
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00000002,
        LTMR64H: 0x00000000,
        LTMR64L: 0x00000000,
        LDVAL0: 0x00000000,
        CVAL0: 0x00000000,
        TCTRL0: 0x00000000,
        TFLG0: 0x00000000,
        LDVAL1: 0x00000000,
        CVAL1: 0x00000000,
        TCTRL1: 0x00000000,
        TFLG1: 0x00000000,
        LDVAL2: 0x00000000,
        CVAL2: 0x00000000,
        TCTRL2: 0x00000000,
        TFLG2: 0x00000000,
        LDVAL3: 0x00000000,
        CVAL3: 0x00000000,
        TCTRL3: 0x00000000,
        TFLG3: 0x00000000,
    };

    /// Safe access to PIT
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
        let taken = PIT_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PIT
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PIT_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PIT
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PIT_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with PIT
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::PIT];

    /// The interrupts associated with PIT
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PIT
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PIT: *const RegisterBlock = 0x40084000 as *const _;
