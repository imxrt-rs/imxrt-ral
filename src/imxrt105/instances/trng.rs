#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TRNG
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::trng::Instance;
pub use crate::imxrt105::peripherals::trng::{RegisterBlock, ResetValues};

pub use crate::imxrt105::peripherals::trng::{
    ENT0, ENT1, ENT10, ENT11, ENT12, ENT13, ENT14, ENT15, ENT2, ENT3, ENT4, ENT5, ENT6, ENT7, ENT8,
    ENT9, FRQ, FRQMIN, INT_CTRL, INT_MASK, INT_STATUS, MCTL, PKR, PKRCNT10, PKRCNT32, PKRCNT54,
    PKRCNT76, PKRCNT98, PKRCNTBA, PKRCNTDC, PKRCNTFE, PKRRNG, SBLIM, SCM, SCMISC, SCR1, SCR2, SCR3,
    SCR4, SCR5, SCR6P, SDCTL, SEC_CFG, STATUS, VID1, VID2,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The TRNG peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type TRNG = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static TRNG_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the TRNG peripheral instance
#[cfg(not(feature = "nosync"))]
impl TRNG {
    const INSTANCE: Self = Self {
        addr: 0x400cc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::TRNG],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in TRNG
    pub const reset: ResetValues = ResetValues {
        MCTL: 0x00012001,
        SCMISC: 0x00010022,
        PKRRNG: 0x000009A3,
        PKR: 0x00000000,
        SDCTL: 0x0C8009C4,
        SBLIM: 0x00000000,
        FRQMIN: 0x00000640,
        FRQ: 0x00006400,
        SCM: 0x010C0568,
        SCR1: 0x00B20195,
        SCR2: 0x007A00DC,
        SCR3: 0x0058007D,
        SCR4: 0x0040004B,
        SCR5: 0x002E002F,
        SCR6P: 0x002E002F,
        STATUS: 0x00000000,
        ENT0: 0x00000000,
        ENT1: 0x00000000,
        ENT2: 0x00000000,
        ENT3: 0x00000000,
        ENT4: 0x00000000,
        ENT5: 0x00000000,
        ENT6: 0x00000000,
        ENT7: 0x00000000,
        ENT8: 0x00000000,
        ENT9: 0x00000000,
        ENT10: 0x00000000,
        ENT11: 0x00000000,
        ENT12: 0x00000000,
        ENT13: 0x00000000,
        ENT14: 0x00000000,
        ENT15: 0x00000000,
        PKRCNT10: 0x00000000,
        PKRCNT32: 0x00000000,
        PKRCNT54: 0x00000000,
        PKRCNT76: 0x00000000,
        PKRCNT98: 0x00000000,
        PKRCNTBA: 0x00000000,
        PKRCNTDC: 0x00000000,
        PKRCNTFE: 0x00000000,
        SEC_CFG: 0x00000000,
        INT_CTRL: 0x00000007,
        INT_MASK: 0x00000000,
        INT_STATUS: 0x00000000,
        VID1: 0x00300301,
        VID2: 0x00000000,
    };

    /// Safe access to TRNG
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
        let taken = TRNG_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to TRNG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = TRNG_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal TRNG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        TRNG_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with TRNG
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::TRNG];

    /// The interrupts associated with TRNG
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to TRNG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TRNG: *const RegisterBlock = 0x400cc000 as *const _;
