#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Touch Screen Controller
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::tsc::Instance;
pub use crate::imxrt106::peripherals::tsc::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::tsc::{
    BASIC_SETTING, DEBUG_MODE, DEBUG_MODE2, FLOW_CONTROL, INT_EN, INT_SIG_EN, INT_STATUS,
    MEASEURE_VALUE, PRE_CHARGE_TIME,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The TSC peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type TSC = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static TSC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the TSC peripheral instance
#[cfg(not(feature = "nosync"))]
impl TSC {
    const INSTANCE: Self = Self {
        addr: 0x400e0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::TSC_DIG],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in TSC
    pub const reset: ResetValues = ResetValues {
        BASIC_SETTING: 0x00000000,
        PRE_CHARGE_TIME: 0x00000000,
        FLOW_CONTROL: 0x00000000,
        MEASEURE_VALUE: 0x00000000,
        INT_EN: 0x00000000,
        INT_SIG_EN: 0x00000000,
        INT_STATUS: 0x00000000,
        DEBUG_MODE: 0x00000000,
        DEBUG_MODE2: 0x00000000,
    };

    /// Safe access to TSC
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
        let taken = TSC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to TSC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = TSC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal TSC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        TSC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with TSC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::TSC_DIG];

    /// The interrupts associated with TSC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to TSC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TSC: *const RegisterBlock = 0x400e0000 as *const _;
