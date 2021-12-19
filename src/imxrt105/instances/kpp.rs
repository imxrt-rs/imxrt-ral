#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! KPP Registers
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::kpp::Instance;
pub use crate::imxrt105::peripherals::kpp::{RegisterBlock, ResetValues};

pub use crate::imxrt105::peripherals::kpp::{KDDR, KPCR, KPDR, KPSR};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The KPP peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type KPP = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static KPP_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the KPP peripheral instance
#[cfg(not(feature = "nosync"))]
impl KPP {
    const INSTANCE: Self = Self {
        addr: 0x401fc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::KPP],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in KPP
    pub const reset: ResetValues = ResetValues {
        KPCR: 0x00000000,
        KPSR: 0x00000400,
        KDDR: 0x00000000,
        KPDR: 0x00000000,
    };

    /// Safe access to KPP
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
        let taken = KPP_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to KPP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = KPP_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal KPP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        KPP_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with KPP
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::KPP];

    /// The interrupts associated with KPP
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to KPP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const KPP: *const RegisterBlock = 0x401fc000 as *const _;
