#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! EWM
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::ewm::Instance;
pub use crate::imxrt106::peripherals::ewm::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::ewm::{CLKCTRL, CLKPRESCALER, CMPH, CMPL, CTRL, SERV};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The EWM peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type EWM = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static EWM_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the EWM peripheral instance
#[cfg(not(feature = "nosync"))]
impl EWM {
    const INSTANCE: Self = Self {
        addr: 0x400b4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::EWM],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in EWM
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        SERV: 0x00000000,
        CMPL: 0x00000000,
        CMPH: 0x000000FF,
        CLKCTRL: 0x00000000,
        CLKPRESCALER: 0x00000000,
    };

    /// Safe access to EWM
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
        let taken = EWM_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to EWM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = EWM_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal EWM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        EWM_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with EWM
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::EWM];

    /// The interrupts associated with EWM
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to EWM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EWM: *const RegisterBlock = 0x400b4000 as *const _;
