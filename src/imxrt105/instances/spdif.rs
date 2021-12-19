#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SPDIF
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
use crate::imxrt105::peripherals::spdif::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::spdif::{Instance, Valid};
pub use crate::imxrt105::peripherals::spdif::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::spdif::{
    SCR, SI, SIE, SRCD, SRCSH, SRCSL, SRFM, SRL, SRPC, SRQ, SRR, SRU, STC, STCSCH, STCSCL, STL, STR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The SPDIF peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type SPDIF = Instance<0>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for SPDIF {}
#[cfg(not(feature = "nosync"))]
impl Valid for SPDIF {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SPDIF_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SPDIF peripheral instance
#[cfg(not(feature = "nosync"))]
impl SPDIF {
    const INSTANCE: Self = Self {
        addr: 0x40380000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SPDIF],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in SPDIF
    pub const reset: ResetValues = ResetValues {
        SCR: 0x00000400,
        SRCD: 0x00000000,
        SRPC: 0x00000000,
        SIE: 0x00000000,
        SI: 0x00000002,
        SRL: 0x00000000,
        SRR: 0x00000000,
        SRCSH: 0x00000000,
        SRCSL: 0x00000000,
        SRU: 0x00000000,
        SRQ: 0x00000000,
        STL: 0x00000000,
        STR: 0x00000000,
        STCSCH: 0x00000000,
        STCSCL: 0x00000000,
        SRFM: 0x00000000,
        STC: 0x00020F00,
    };

    /// Safe access to SPDIF
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
        let taken = SPDIF_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SPDIF
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SPDIF_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SPDIF
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SPDIF_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with SPDIF
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::SPDIF];

    /// The interrupts associated with SPDIF
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SPDIF
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPDIF: *const RegisterBlock = 0x40380000 as *const _;
