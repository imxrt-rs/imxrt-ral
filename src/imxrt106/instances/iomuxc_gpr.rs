#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC_GPR
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::iomuxc_gpr::Instance;
pub use crate::imxrt106::peripherals::iomuxc_gpr::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::iomuxc_gpr::{
    GPR0, GPR1, GPR10, GPR11, GPR12, GPR13, GPR14, GPR15, GPR16, GPR17, GPR18, GPR19, GPR2, GPR20,
    GPR21, GPR22, GPR23, GPR24, GPR25, GPR26, GPR27, GPR28, GPR29, GPR3, GPR30, GPR31, GPR32,
    GPR33, GPR34, GPR4, GPR5, GPR6, GPR7, GPR8, GPR9,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The IOMUXC_GPR peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type IOMUXC_GPR = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static IOMUXC_GPR_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the IOMUXC_GPR peripheral instance
#[cfg(not(feature = "nosync"))]
impl IOMUXC_GPR {
    const INSTANCE: Self = Self {
        addr: 0x400ac000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPR_IRQ],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in IOMUXC_GPR
    pub const reset: ResetValues = ResetValues {
        GPR0: 0x00000000,
        GPR1: 0x00000000,
        GPR2: 0x00000000,
        GPR3: 0x0000F0F0,
        GPR4: 0x00000000,
        GPR5: 0x00000000,
        GPR6: 0x00000000,
        GPR7: 0x00000000,
        GPR8: 0x00000000,
        GPR9: 0x00000000,
        GPR10: 0x00000007,
        GPR11: 0x00000000,
        GPR12: 0x00000000,
        GPR13: 0x00000000,
        GPR14: 0x00000000,
        GPR15: 0xFFFFFFFF,
        GPR16: 0x00200003,
        GPR17: 0x00000000,
        GPR18: 0x00000000,
        GPR19: 0x00000000,
        GPR20: 0x00000000,
        GPR21: 0x00000000,
        GPR22: 0x00000000,
        GPR23: 0x00000000,
        GPR24: 0x00000000,
        GPR25: 0x00000000,
        GPR26: 0x00000000,
        GPR27: 0x00000000,
        GPR28: 0x00000000,
        GPR29: 0x00000000,
        GPR30: 0x00000000,
        GPR31: 0x00000000,
        GPR32: 0x00000000,
        GPR33: 0x00000007,
        GPR34: 0x00000000,
    };

    /// Safe access to IOMUXC_GPR
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
        let taken = IOMUXC_GPR_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to IOMUXC_GPR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = IOMUXC_GPR_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal IOMUXC_GPR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        IOMUXC_GPR_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with IOMUXC_GPR
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPR_IRQ];

    /// The interrupts associated with IOMUXC_GPR
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to IOMUXC_GPR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC_GPR: *const RegisterBlock = 0x400ac000 as *const _;
