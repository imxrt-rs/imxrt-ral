#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Quad Timer
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::tmr::Instance;
pub use crate::imxrt106::peripherals::tmr::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::tmr::{
    CAPT0, CAPT1, CAPT2, CAPT3, CMPLD10, CMPLD11, CMPLD12, CMPLD13, CMPLD20, CMPLD21, CMPLD22,
    CMPLD23, CNTR0, CNTR1, CNTR2, CNTR3, COMP10, COMP11, COMP12, COMP13, COMP20, COMP21, COMP22,
    COMP23, CSCTRL0, CSCTRL1, CSCTRL2, CSCTRL3, CTRL0, CTRL1, CTRL2, CTRL3, DMA0, DMA1, DMA2, DMA3,
    ENBL, FILT0, FILT1, FILT2, FILT3, HOLD0, HOLD1, HOLD2, HOLD3, LOAD0, LOAD1, LOAD2, LOAD3,
    SCTRL0, SCTRL1, SCTRL2, SCTRL3,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The TMR1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type TMR1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static TMR1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the TMR1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl TMR1 {
    const INSTANCE: Self = Self {
        addr: 0x401dc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::TMR1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in TMR1
    pub const reset: ResetValues = ResetValues {
        COMP10: 0x00000000,
        COMP11: 0x00000000,
        COMP12: 0x00000000,
        COMP13: 0x00000000,
        COMP20: 0x00000000,
        COMP21: 0x00000000,
        COMP22: 0x00000000,
        COMP23: 0x00000000,
        CAPT0: 0x00000000,
        CAPT1: 0x00000000,
        CAPT2: 0x00000000,
        CAPT3: 0x00000000,
        LOAD0: 0x00000000,
        LOAD1: 0x00000000,
        LOAD2: 0x00000000,
        LOAD3: 0x00000000,
        HOLD0: 0x00000000,
        HOLD1: 0x00000000,
        HOLD2: 0x00000000,
        HOLD3: 0x00000000,
        CNTR0: 0x00000000,
        CNTR1: 0x00000000,
        CNTR2: 0x00000000,
        CNTR3: 0x00000000,
        CTRL0: 0x00000000,
        CTRL1: 0x00000000,
        CTRL2: 0x00000000,
        CTRL3: 0x00000000,
        SCTRL0: 0x00000000,
        SCTRL1: 0x00000000,
        SCTRL2: 0x00000000,
        SCTRL3: 0x00000000,
        CMPLD10: 0x00000000,
        CMPLD11: 0x00000000,
        CMPLD12: 0x00000000,
        CMPLD13: 0x00000000,
        CMPLD20: 0x00000000,
        CMPLD21: 0x00000000,
        CMPLD22: 0x00000000,
        CMPLD23: 0x00000000,
        CSCTRL0: 0x00000000,
        CSCTRL1: 0x00000000,
        CSCTRL2: 0x00000000,
        CSCTRL3: 0x00000000,
        FILT0: 0x00000000,
        FILT1: 0x00000000,
        FILT2: 0x00000000,
        FILT3: 0x00000000,
        DMA0: 0x00000000,
        DMA1: 0x00000000,
        DMA2: 0x00000000,
        DMA3: 0x00000000,
        ENBL: 0x0000000F,
    };

    /// Safe access to TMR1
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
        let taken = TMR1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to TMR1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = TMR1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal TMR1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        TMR1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with TMR1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::TMR1];

    /// The interrupts associated with TMR1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to TMR1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TMR1: *const RegisterBlock = 0x401dc000 as *const _;

/// The TMR2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type TMR2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static TMR2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the TMR2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl TMR2 {
    const INSTANCE: Self = Self {
        addr: 0x401e0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::TMR2],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in TMR2
    pub const reset: ResetValues = ResetValues {
        COMP10: 0x00000000,
        COMP11: 0x00000000,
        COMP12: 0x00000000,
        COMP13: 0x00000000,
        COMP20: 0x00000000,
        COMP21: 0x00000000,
        COMP22: 0x00000000,
        COMP23: 0x00000000,
        CAPT0: 0x00000000,
        CAPT1: 0x00000000,
        CAPT2: 0x00000000,
        CAPT3: 0x00000000,
        LOAD0: 0x00000000,
        LOAD1: 0x00000000,
        LOAD2: 0x00000000,
        LOAD3: 0x00000000,
        HOLD0: 0x00000000,
        HOLD1: 0x00000000,
        HOLD2: 0x00000000,
        HOLD3: 0x00000000,
        CNTR0: 0x00000000,
        CNTR1: 0x00000000,
        CNTR2: 0x00000000,
        CNTR3: 0x00000000,
        CTRL0: 0x00000000,
        CTRL1: 0x00000000,
        CTRL2: 0x00000000,
        CTRL3: 0x00000000,
        SCTRL0: 0x00000000,
        SCTRL1: 0x00000000,
        SCTRL2: 0x00000000,
        SCTRL3: 0x00000000,
        CMPLD10: 0x00000000,
        CMPLD11: 0x00000000,
        CMPLD12: 0x00000000,
        CMPLD13: 0x00000000,
        CMPLD20: 0x00000000,
        CMPLD21: 0x00000000,
        CMPLD22: 0x00000000,
        CMPLD23: 0x00000000,
        CSCTRL0: 0x00000000,
        CSCTRL1: 0x00000000,
        CSCTRL2: 0x00000000,
        CSCTRL3: 0x00000000,
        FILT0: 0x00000000,
        FILT1: 0x00000000,
        FILT2: 0x00000000,
        FILT3: 0x00000000,
        DMA0: 0x00000000,
        DMA1: 0x00000000,
        DMA2: 0x00000000,
        DMA3: 0x00000000,
        ENBL: 0x0000000F,
    };

    /// Safe access to TMR2
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
        let taken = TMR2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to TMR2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = TMR2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal TMR2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        TMR2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with TMR2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::TMR2];

    /// The interrupts associated with TMR2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to TMR2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TMR2: *const RegisterBlock = 0x401e0000 as *const _;

/// The TMR3 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type TMR3 = Instance<3>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static TMR3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the TMR3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl TMR3 {
    const INSTANCE: Self = Self {
        addr: 0x401e4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::TMR3],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in TMR3
    pub const reset: ResetValues = ResetValues {
        COMP10: 0x00000000,
        COMP11: 0x00000000,
        COMP12: 0x00000000,
        COMP13: 0x00000000,
        COMP20: 0x00000000,
        COMP21: 0x00000000,
        COMP22: 0x00000000,
        COMP23: 0x00000000,
        CAPT0: 0x00000000,
        CAPT1: 0x00000000,
        CAPT2: 0x00000000,
        CAPT3: 0x00000000,
        LOAD0: 0x00000000,
        LOAD1: 0x00000000,
        LOAD2: 0x00000000,
        LOAD3: 0x00000000,
        HOLD0: 0x00000000,
        HOLD1: 0x00000000,
        HOLD2: 0x00000000,
        HOLD3: 0x00000000,
        CNTR0: 0x00000000,
        CNTR1: 0x00000000,
        CNTR2: 0x00000000,
        CNTR3: 0x00000000,
        CTRL0: 0x00000000,
        CTRL1: 0x00000000,
        CTRL2: 0x00000000,
        CTRL3: 0x00000000,
        SCTRL0: 0x00000000,
        SCTRL1: 0x00000000,
        SCTRL2: 0x00000000,
        SCTRL3: 0x00000000,
        CMPLD10: 0x00000000,
        CMPLD11: 0x00000000,
        CMPLD12: 0x00000000,
        CMPLD13: 0x00000000,
        CMPLD20: 0x00000000,
        CMPLD21: 0x00000000,
        CMPLD22: 0x00000000,
        CMPLD23: 0x00000000,
        CSCTRL0: 0x00000000,
        CSCTRL1: 0x00000000,
        CSCTRL2: 0x00000000,
        CSCTRL3: 0x00000000,
        FILT0: 0x00000000,
        FILT1: 0x00000000,
        FILT2: 0x00000000,
        FILT3: 0x00000000,
        DMA0: 0x00000000,
        DMA1: 0x00000000,
        DMA2: 0x00000000,
        DMA3: 0x00000000,
        ENBL: 0x0000000F,
    };

    /// Safe access to TMR3
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
        let taken = TMR3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to TMR3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = TMR3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal TMR3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        TMR3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with TMR3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::TMR3];

    /// The interrupts associated with TMR3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to TMR3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TMR3: *const RegisterBlock = 0x401e4000 as *const _;

/// The TMR4 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type TMR4 = Instance<4>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static TMR4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the TMR4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl TMR4 {
    const INSTANCE: Self = Self {
        addr: 0x401e8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::TMR4],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in TMR4
    pub const reset: ResetValues = ResetValues {
        COMP10: 0x00000000,
        COMP11: 0x00000000,
        COMP12: 0x00000000,
        COMP13: 0x00000000,
        COMP20: 0x00000000,
        COMP21: 0x00000000,
        COMP22: 0x00000000,
        COMP23: 0x00000000,
        CAPT0: 0x00000000,
        CAPT1: 0x00000000,
        CAPT2: 0x00000000,
        CAPT3: 0x00000000,
        LOAD0: 0x00000000,
        LOAD1: 0x00000000,
        LOAD2: 0x00000000,
        LOAD3: 0x00000000,
        HOLD0: 0x00000000,
        HOLD1: 0x00000000,
        HOLD2: 0x00000000,
        HOLD3: 0x00000000,
        CNTR0: 0x00000000,
        CNTR1: 0x00000000,
        CNTR2: 0x00000000,
        CNTR3: 0x00000000,
        CTRL0: 0x00000000,
        CTRL1: 0x00000000,
        CTRL2: 0x00000000,
        CTRL3: 0x00000000,
        SCTRL0: 0x00000000,
        SCTRL1: 0x00000000,
        SCTRL2: 0x00000000,
        SCTRL3: 0x00000000,
        CMPLD10: 0x00000000,
        CMPLD11: 0x00000000,
        CMPLD12: 0x00000000,
        CMPLD13: 0x00000000,
        CMPLD20: 0x00000000,
        CMPLD21: 0x00000000,
        CMPLD22: 0x00000000,
        CMPLD23: 0x00000000,
        CSCTRL0: 0x00000000,
        CSCTRL1: 0x00000000,
        CSCTRL2: 0x00000000,
        CSCTRL3: 0x00000000,
        FILT0: 0x00000000,
        FILT1: 0x00000000,
        FILT2: 0x00000000,
        FILT3: 0x00000000,
        DMA0: 0x00000000,
        DMA1: 0x00000000,
        DMA2: 0x00000000,
        DMA3: 0x00000000,
        ENBL: 0x0000000F,
    };

    /// Safe access to TMR4
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
        let taken = TMR4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to TMR4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = TMR4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal TMR4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        TMR4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with TMR4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::TMR4];

    /// The interrupts associated with TMR4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to TMR4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TMR4: *const RegisterBlock = 0x401e8000 as *const _;
