#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA_CH_MUX
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::dmamux::Instance;
pub use crate::imxrt106::peripherals::dmamux::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::dmamux::{
    CHCFG0, CHCFG1, CHCFG10, CHCFG11, CHCFG12, CHCFG13, CHCFG14, CHCFG15, CHCFG16, CHCFG17,
    CHCFG18, CHCFG19, CHCFG2, CHCFG20, CHCFG21, CHCFG22, CHCFG23, CHCFG24, CHCFG25, CHCFG26,
    CHCFG27, CHCFG28, CHCFG29, CHCFG3, CHCFG30, CHCFG31, CHCFG4, CHCFG5, CHCFG6, CHCFG7, CHCFG8,
    CHCFG9,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DMAMUX peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type DMAMUX = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DMAMUX_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DMAMUX peripheral instance
#[cfg(not(feature = "nosync"))]
impl DMAMUX {
    const INSTANCE: Self = Self {
        addr: 0x400ec000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in DMAMUX
    pub const reset: ResetValues = ResetValues {
        CHCFG0: 0x00000000,
        CHCFG1: 0x00000000,
        CHCFG2: 0x00000000,
        CHCFG3: 0x00000000,
        CHCFG4: 0x00000000,
        CHCFG5: 0x00000000,
        CHCFG6: 0x00000000,
        CHCFG7: 0x00000000,
        CHCFG8: 0x00000000,
        CHCFG9: 0x00000000,
        CHCFG10: 0x00000000,
        CHCFG11: 0x00000000,
        CHCFG12: 0x00000000,
        CHCFG13: 0x00000000,
        CHCFG14: 0x00000000,
        CHCFG15: 0x00000000,
        CHCFG16: 0x00000000,
        CHCFG17: 0x00000000,
        CHCFG18: 0x00000000,
        CHCFG19: 0x00000000,
        CHCFG20: 0x00000000,
        CHCFG21: 0x00000000,
        CHCFG22: 0x00000000,
        CHCFG23: 0x00000000,
        CHCFG24: 0x00000000,
        CHCFG25: 0x00000000,
        CHCFG26: 0x00000000,
        CHCFG27: 0x00000000,
        CHCFG28: 0x00000000,
        CHCFG29: 0x00000000,
        CHCFG30: 0x00000000,
        CHCFG31: 0x00000000,
    };

    /// Safe access to DMAMUX
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
        let taken = DMAMUX_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DMAMUX
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

        let taken = DMAMUX_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DMAMUX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DMAMUX_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with DMAMUX
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with DMAMUX
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DMAMUX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX: *const RegisterBlock = 0x400ec000 as *const _;
