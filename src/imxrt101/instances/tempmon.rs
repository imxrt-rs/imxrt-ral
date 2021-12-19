#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Temperature Monitor
//!
//! Used by: imxrt1011, imxrt1015

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::tempmon::Instance;
pub use crate::imxrt101::peripherals::tempmon::{RegisterBlock, ResetValues};

pub use crate::imxrt101::peripherals::tempmon::{
    TEMPSENSE0, TEMPSENSE0_CLR, TEMPSENSE0_SET, TEMPSENSE0_TOG, TEMPSENSE1, TEMPSENSE1_CLR,
    TEMPSENSE1_SET, TEMPSENSE1_TOG, TEMPSENSE2, TEMPSENSE2_CLR, TEMPSENSE2_SET, TEMPSENSE2_TOG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The TEMPMON peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type TEMPMON = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static TEMPMON_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the TEMPMON peripheral instance
#[cfg(not(feature = "nosync"))]
impl TEMPMON {
    const INSTANCE: Self = Self {
        addr: 0x400d8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::TEMP_LOW_HIGH,
            crate::interrupt::TEMP_PANIC,
        ],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in TEMPMON
    pub const reset: ResetValues = ResetValues {
        TEMPSENSE0: 0x00000001,
        TEMPSENSE0_SET: 0x00000001,
        TEMPSENSE0_CLR: 0x00000001,
        TEMPSENSE0_TOG: 0x00000001,
        TEMPSENSE1: 0x00000001,
        TEMPSENSE1_SET: 0x00000001,
        TEMPSENSE1_CLR: 0x00000001,
        TEMPSENSE1_TOG: 0x00000001,
        TEMPSENSE2: 0x00000000,
        TEMPSENSE2_SET: 0x00000000,
        TEMPSENSE2_CLR: 0x00000000,
        TEMPSENSE2_TOG: 0x00000000,
    };

    /// Safe access to TEMPMON
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
        let taken = TEMPMON_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to TEMPMON
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = TEMPMON_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal TEMPMON
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        TEMPMON_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with TEMPMON
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::TEMP_LOW_HIGH,
        crate::interrupt::TEMP_PANIC,
    ];

    /// The interrupts associated with TEMPMON
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to TEMPMON
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TEMPMON: *const RegisterBlock = 0x400d8000 as *const _;
