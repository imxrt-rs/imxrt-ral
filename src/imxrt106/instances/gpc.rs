#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::gpc::Instance;
pub use crate::imxrt106::peripherals::gpc::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::gpc::{
    CNTR, IMR1, IMR2, IMR3, IMR4, IMR5, ISR1, ISR2, ISR3, ISR4, ISR5,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The GPC peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type GPC = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPC peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPC {
    const INSTANCE: Self = Self {
        addr: 0x400f4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPC],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in GPC
    pub const reset: ResetValues = ResetValues {
        CNTR: 0x00520000,
        IMR1: 0x00000000,
        IMR2: 0x00000000,
        IMR3: 0x00000000,
        IMR4: 0x00000000,
        ISR1: 0x00000000,
        ISR2: 0x00000000,
        ISR3: 0x00000000,
        ISR4: 0x00000000,
        IMR5: 0x00000000,
        ISR5: 0x00000000,
    };

    /// Safe access to GPC
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
        let taken = GPC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Self) {
        assert!(
            inst.addr == Self::INSTANCE.addr,
            "Released the wrong instance"
        );

        let taken = GPC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPC];

    /// The interrupts associated with GPC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPC: *const RegisterBlock = 0x400f4000 as *const _;
