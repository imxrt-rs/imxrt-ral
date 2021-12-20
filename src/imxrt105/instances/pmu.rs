#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PMU
//!
//! Used by: imxrt1051, imxrt1052

use crate::imxrt105::peripherals::pmu::private;
pub use crate::imxrt105::peripherals::pmu::{Instance, Valid};
pub use crate::imxrt105::peripherals::pmu::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::pmu::{
    MISC0, MISC0_CLR, MISC0_SET, MISC0_TOG, MISC1, MISC1_CLR, MISC1_SET, MISC1_TOG, MISC2,
    MISC2_CLR, MISC2_SET, MISC2_TOG, REG_1P1, REG_1P1_CLR, REG_1P1_SET, REG_1P1_TOG, REG_2P5,
    REG_2P5_CLR, REG_2P5_SET, REG_2P5_TOG, REG_3P0, REG_3P0_CLR, REG_3P0_SET, REG_3P0_TOG,
    REG_CORE, REG_CORE_CLR, REG_CORE_SET, REG_CORE_TOG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PMU peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PMU = Instance<0>;

/// The PMU peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PMU = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct PMU {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl private::Sealed for PMU {}
impl Valid for PMU {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PMU_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PMU peripheral instance
#[cfg(not(feature = "nosync"))]
impl PMU {
    const INSTANCE: Self = Self {
        addr: 0x400d8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::PMU_EVENT],
    };

    /// Reset values for each field in PMU
    pub const reset: ResetValues = ResetValues {
        REG_1P1: 0x00001073,
        REG_1P1_SET: 0x00001073,
        REG_1P1_CLR: 0x00001073,
        REG_1P1_TOG: 0x00001073,
        REG_3P0: 0x00000F74,
        REG_3P0_SET: 0x00000F74,
        REG_3P0_CLR: 0x00000F74,
        REG_3P0_TOG: 0x00000F74,
        REG_2P5: 0x00001073,
        REG_2P5_SET: 0x00001073,
        REG_2P5_CLR: 0x00001073,
        REG_2P5_TOG: 0x00001073,
        REG_CORE: 0x00482012,
        REG_CORE_SET: 0x00482012,
        REG_CORE_CLR: 0x00482012,
        REG_CORE_TOG: 0x00482012,
        MISC0: 0x04000000,
        MISC0_SET: 0x04000000,
        MISC0_CLR: 0x04000000,
        MISC0_TOG: 0x04000000,
        MISC1: 0x00000000,
        MISC1_SET: 0x00000000,
        MISC1_CLR: 0x00000000,
        MISC1_TOG: 0x00000000,
        MISC2: 0x00272727,
        MISC2_SET: 0x00272727,
        MISC2_CLR: 0x00272727,
        MISC2_TOG: 0x00272727,
    };

    /// Safe access to PMU
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
        let taken = PMU_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PMU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PMU_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PMU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PMU_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PMU {
    /// The interrupts associated with PMU
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::PMU_EVENT];

    /// The interrupts associated with PMU
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PMU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PMU: *const RegisterBlock = 0x400d8000 as *const _;
