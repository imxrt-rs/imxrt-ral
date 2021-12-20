#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGC
//!
//! Used by: imxrt1011, imxrt1015

use crate::imxrt101::peripherals::pgc::private;
pub use crate::imxrt101::peripherals::pgc::{Instance, Valid};
pub use crate::imxrt101::peripherals::pgc::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::pgc::{
    CPU_CTRL, CPU_PDNSCR, CPU_PUPSCR, CPU_SR, MEGA_CTRL, MEGA_PDNSCR, MEGA_PUPSCR, MEGA_SR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PGC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGC = Instance<0>;

/// The PGC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct PGC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl private::Sealed for PGC {}
impl Valid for PGC {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGC peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGC {
    const INSTANCE: Self = Self {
        addr: 0x400f4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGC
    pub const reset: ResetValues = ResetValues {
        MEGA_CTRL: 0x00000000,
        MEGA_PUPSCR: 0x00000F01,
        MEGA_PDNSCR: 0x00000101,
        MEGA_SR: 0x00000000,
        CPU_CTRL: 0x00000000,
        CPU_PUPSCR: 0x00000F01,
        CPU_PDNSCR: 0x00000101,
        CPU_SR: 0x00000000,
    };

    /// Safe access to PGC
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
        let taken = PGC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGC {
    /// The interrupts associated with PGC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGC: *const RegisterBlock = 0x400f4000 as *const _;
