#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Bus Encryption Engine
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::bee::Instance;
pub use crate::imxrt106::peripherals::bee::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::bee::{
    ADDR_OFFSET0, ADDR_OFFSET1, AES_KEY0_W0, AES_KEY0_W1, AES_KEY0_W2, AES_KEY0_W3, CTRL,
    CTR_NONCE0_W0, CTR_NONCE0_W1, CTR_NONCE0_W2, CTR_NONCE0_W3, CTR_NONCE1_W0, CTR_NONCE1_W1,
    CTR_NONCE1_W2, CTR_NONCE1_W3, REGION1_BOT, REGION1_TOP, STATUS,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The BEE peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type BEE = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static BEE_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the BEE peripheral instance
#[cfg(not(feature = "nosync"))]
impl BEE {
    const INSTANCE: Self = Self {
        addr: 0x403ec000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::BEE],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in BEE
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00007700,
        ADDR_OFFSET0: 0x0000F000,
        ADDR_OFFSET1: 0x0000F000,
        AES_KEY0_W0: 0x00000000,
        AES_KEY0_W1: 0x00000000,
        AES_KEY0_W2: 0x00000000,
        AES_KEY0_W3: 0x00000000,
        STATUS: 0x00000000,
        CTR_NONCE0_W0: 0x00000000,
        CTR_NONCE0_W1: 0x00000000,
        CTR_NONCE0_W2: 0x00000000,
        CTR_NONCE0_W3: 0x00000000,
        CTR_NONCE1_W0: 0x00000000,
        CTR_NONCE1_W1: 0x00000000,
        CTR_NONCE1_W2: 0x00000000,
        CTR_NONCE1_W3: 0x00000000,
        REGION1_TOP: 0x00000000,
        REGION1_BOT: 0x00000000,
    };

    /// Safe access to BEE
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
        let taken = BEE_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to BEE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = BEE_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal BEE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        BEE_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with BEE
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::BEE];

    /// The interrupts associated with BEE
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to BEE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const BEE: *const RegisterBlock = 0x403ec000 as *const _;
