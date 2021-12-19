#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AND/OR/INVERT module
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::aoi::Instance;
pub use crate::imxrt106::peripherals::aoi::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::aoi::{
    BFCRT010, BFCRT011, BFCRT012, BFCRT013, BFCRT230, BFCRT231, BFCRT232, BFCRT233,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The AOI1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type AOI1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AOI1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AOI1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AOI1 {
    const INSTANCE: Self = Self {
        addr: 0x403b4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in AOI1
    pub const reset: ResetValues = ResetValues {
        BFCRT010: 0x00000000,
        BFCRT011: 0x00000000,
        BFCRT012: 0x00000000,
        BFCRT013: 0x00000000,
        BFCRT230: 0x00000000,
        BFCRT231: 0x00000000,
        BFCRT232: 0x00000000,
        BFCRT233: 0x00000000,
    };

    /// Safe access to AOI1
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
        let taken = AOI1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AOI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AOI1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AOI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AOI1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with AOI1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AOI1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AOI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AOI1: *const RegisterBlock = 0x403b4000 as *const _;

/// The AOI2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type AOI2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AOI2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AOI2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AOI2 {
    const INSTANCE: Self = Self {
        addr: 0x403b8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in AOI2
    pub const reset: ResetValues = ResetValues {
        BFCRT010: 0x00000000,
        BFCRT011: 0x00000000,
        BFCRT012: 0x00000000,
        BFCRT013: 0x00000000,
        BFCRT230: 0x00000000,
        BFCRT231: 0x00000000,
        BFCRT232: 0x00000000,
        BFCRT233: 0x00000000,
    };

    /// Safe access to AOI2
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
        let taken = AOI2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AOI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AOI2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AOI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AOI2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with AOI2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AOI2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AOI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AOI2: *const RegisterBlock = 0x403b8000 as *const _;
