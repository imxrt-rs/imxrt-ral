#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! XTALOSC24M
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
use crate::imxrt106::peripherals::xtalosc24m::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::xtalosc24m::{Instance, Valid};
pub use crate::imxrt106::peripherals::xtalosc24m::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::xtalosc24m::{
    LOWPWR_CTRL, LOWPWR_CTRL_CLR, LOWPWR_CTRL_SET, LOWPWR_CTRL_TOG, MISC0, MISC0_CLR, MISC0_SET,
    MISC0_TOG, OSC_CONFIG0, OSC_CONFIG0_CLR, OSC_CONFIG0_SET, OSC_CONFIG0_TOG, OSC_CONFIG1,
    OSC_CONFIG1_CLR, OSC_CONFIG1_SET, OSC_CONFIG1_TOG, OSC_CONFIG2, OSC_CONFIG2_CLR,
    OSC_CONFIG2_SET, OSC_CONFIG2_TOG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The XTALOSC24M peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type XTALOSC24M = Instance<0>;

/// The XTALOSC24M peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XTALOSC24M = Instance<0>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct XTALOSC24M {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for XTALOSC24M {}
#[cfg(not(feature = "nosync"))]
impl Valid for XTALOSC24M {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XTALOSC24M_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XTALOSC24M peripheral instance
#[cfg(not(feature = "nosync"))]
impl XTALOSC24M {
    const INSTANCE: Self = Self {
        addr: 0x400d8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in XTALOSC24M
    pub const reset: ResetValues = ResetValues {
        MISC0: 0x04000000,
        MISC0_SET: 0x04000000,
        MISC0_CLR: 0x04000000,
        MISC0_TOG: 0x04000000,
        LOWPWR_CTRL: 0x00004001,
        LOWPWR_CTRL_SET: 0x00004001,
        LOWPWR_CTRL_CLR: 0x00004001,
        LOWPWR_CTRL_TOG: 0x00004001,
        OSC_CONFIG0: 0x00001020,
        OSC_CONFIG0_SET: 0x00001020,
        OSC_CONFIG0_CLR: 0x00001020,
        OSC_CONFIG0_TOG: 0x00001020,
        OSC_CONFIG1: 0x000002EE,
        OSC_CONFIG1_SET: 0x000002EE,
        OSC_CONFIG1_CLR: 0x000002EE,
        OSC_CONFIG1_TOG: 0x000002EE,
        OSC_CONFIG2: 0x000102E2,
        OSC_CONFIG2_SET: 0x000102E2,
        OSC_CONFIG2_CLR: 0x000102E2,
        OSC_CONFIG2_TOG: 0x000102E2,
    };

    /// Safe access to XTALOSC24M
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
        let taken = XTALOSC24M_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XTALOSC24M
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XTALOSC24M_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XTALOSC24M
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XTALOSC24M_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with XTALOSC24M
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with XTALOSC24M
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XTALOSC24M
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XTALOSC24M: *const RegisterBlock = 0x400d8000 as *const _;
