#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CSU registers
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
use crate::imxrt105::peripherals::csu::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::csu::{Instance, Valid};
pub use crate::imxrt105::peripherals::csu::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::csu::{
    CSL0, CSL1, CSL10, CSL11, CSL12, CSL13, CSL14, CSL15, CSL16, CSL17, CSL18, CSL19, CSL2, CSL20,
    CSL21, CSL22, CSL23, CSL24, CSL25, CSL26, CSL27, CSL28, CSL29, CSL3, CSL30, CSL31, CSL4, CSL5,
    CSL6, CSL7, CSL8, CSL9, HP0, HPCONTROL0, SA,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CSU peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type CSU = Instance<0>;

/// The CSU peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CSU = Instance<0>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct CSU {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for CSU {}
#[cfg(not(feature = "nosync"))]
impl Valid for CSU {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CSU_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CSU peripheral instance
#[cfg(not(feature = "nosync"))]
impl CSU {
    const INSTANCE: Self = Self {
        addr: 0x400dc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CSU],
    };

    /// Reset values for each field in CSU
    pub const reset: ResetValues = ResetValues {
        CSL0: 0x00330033,
        CSL1: 0x00330033,
        CSL2: 0x00330033,
        CSL3: 0x00330033,
        CSL4: 0x00330033,
        CSL5: 0x00330033,
        CSL6: 0x00330033,
        CSL7: 0x00330033,
        CSL8: 0x00330033,
        CSL9: 0x00330033,
        CSL10: 0x00330033,
        CSL11: 0x00330033,
        CSL12: 0x00330033,
        CSL13: 0x00330033,
        CSL14: 0x00330033,
        CSL15: 0x00330033,
        CSL16: 0x00330033,
        CSL17: 0x00330033,
        CSL18: 0x00330033,
        CSL19: 0x00330033,
        CSL20: 0x00330033,
        CSL21: 0x00330033,
        CSL22: 0x00330033,
        CSL23: 0x00330033,
        CSL24: 0x00330033,
        CSL25: 0x00330033,
        CSL26: 0x00330033,
        CSL27: 0x00330033,
        CSL28: 0x00330033,
        CSL29: 0x00330033,
        CSL30: 0x00330033,
        CSL31: 0x00330033,
        HP0: 0x00000000,
        SA: 0x00000000,
        HPCONTROL0: 0x00000000,
    };

    /// Safe access to CSU
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
        let taken = CSU_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CSU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CSU_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CSU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CSU_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with CSU
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::CSU];

    /// The interrupts associated with CSU
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CSU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CSU: *const RegisterBlock = 0x400dc000 as *const _;
