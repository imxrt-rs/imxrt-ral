#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ROMC
//!
//! Used by: imxrt1011, imxrt1015

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::romc::Instance;
pub use crate::imxrt101::peripherals::romc::{RegisterBlock, ResetValues};

pub use crate::imxrt101::peripherals::romc::{
    ROMPATCH0A, ROMPATCH0D, ROMPATCH10A, ROMPATCH11A, ROMPATCH12A, ROMPATCH13A, ROMPATCH14A,
    ROMPATCH15A, ROMPATCH1A, ROMPATCH1D, ROMPATCH2A, ROMPATCH2D, ROMPATCH3A, ROMPATCH3D,
    ROMPATCH4A, ROMPATCH4D, ROMPATCH5A, ROMPATCH5D, ROMPATCH6A, ROMPATCH6D, ROMPATCH7A, ROMPATCH7D,
    ROMPATCH8A, ROMPATCH9A, ROMPATCHCNTL, ROMPATCHENH, ROMPATCHENL, ROMPATCHSR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ROMC peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type ROMC = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ROMC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ROMC peripheral instance
#[cfg(not(feature = "nosync"))]
impl ROMC {
    const INSTANCE: Self = Self {
        addr: 0x40180000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in ROMC
    pub const reset: ResetValues = ResetValues {
        ROMPATCH7D: 0x00000000,
        ROMPATCH6D: 0x00000000,
        ROMPATCH5D: 0x00000000,
        ROMPATCH4D: 0x00000000,
        ROMPATCH3D: 0x00000000,
        ROMPATCH2D: 0x00000000,
        ROMPATCH1D: 0x00000000,
        ROMPATCH0D: 0x00000000,
        ROMPATCHCNTL: 0x08400000,
        ROMPATCHENH: 0x00000000,
        ROMPATCHENL: 0x00000000,
        ROMPATCH0A: 0x00000000,
        ROMPATCH1A: 0x00000000,
        ROMPATCH2A: 0x00000000,
        ROMPATCH3A: 0x00000000,
        ROMPATCH4A: 0x00000000,
        ROMPATCH5A: 0x00000000,
        ROMPATCH6A: 0x00000000,
        ROMPATCH7A: 0x00000000,
        ROMPATCH8A: 0x00000000,
        ROMPATCH9A: 0x00000000,
        ROMPATCH10A: 0x00000000,
        ROMPATCH11A: 0x00000000,
        ROMPATCH12A: 0x00000000,
        ROMPATCH13A: 0x00000000,
        ROMPATCH14A: 0x00000000,
        ROMPATCH15A: 0x00000000,
        ROMPATCHSR: 0x00000000,
    };

    /// Safe access to ROMC
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
        let taken = ROMC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ROMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ROMC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ROMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ROMC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with ROMC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with ROMC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ROMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ROMC: *const RegisterBlock = 0x40180000 as *const _;
