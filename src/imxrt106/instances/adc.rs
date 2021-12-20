#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
use crate::imxrt106::peripherals::adc::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::adc::{Instance, Valid};
pub use crate::imxrt106::peripherals::adc::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::adc::{
    CAL, CFG, CV, GC, GS, HC0, HC1, HC2, HC3, HC4, HC5, HC6, HC7, HS, OFS, R0, R1, R2, R3, R4, R5,
    R6, R7,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ADC1 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type ADC1 = Instance<1>;

/// The ADC1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ADC1 = Instance<1>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct ADC1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for ADC1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for ADC1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ADC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ADC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl ADC1 {
    const INSTANCE: Self = Self {
        addr: 0x400c4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ADC1],
    };

    /// Reset values for each field in ADC1
    pub const reset: ResetValues = ResetValues {
        HC0: 0x0000001F,
        HC1: 0x0000001F,
        HC2: 0x0000001F,
        HC3: 0x0000001F,
        HC4: 0x0000001F,
        HC5: 0x0000001F,
        HC6: 0x0000001F,
        HC7: 0x0000001F,
        HS: 0x00000000,
        R0: 0x00000000,
        R1: 0x00000000,
        R2: 0x00000000,
        R3: 0x00000000,
        R4: 0x00000000,
        R5: 0x00000000,
        R6: 0x00000000,
        R7: 0x00000000,
        CFG: 0x00000200,
        GC: 0x00000000,
        GS: 0x00000000,
        CV: 0x00000000,
        OFS: 0x00000000,
        CAL: 0x00000000,
    };

    /// Safe access to ADC1
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
        let taken = ADC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ADC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ADC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ADC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ADC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with ADC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ADC1];

    /// The interrupts associated with ADC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ADC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC1: *const RegisterBlock = 0x400c4000 as *const _;

/// The ADC2 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type ADC2 = Instance<2>;

/// The ADC2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ADC2 = Instance<2>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct ADC2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for ADC2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for ADC2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ADC2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ADC2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl ADC2 {
    const INSTANCE: Self = Self {
        addr: 0x400c8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ADC2],
    };

    /// Reset values for each field in ADC2
    pub const reset: ResetValues = ResetValues {
        HC0: 0x0000001F,
        HC1: 0x0000001F,
        HC2: 0x0000001F,
        HC3: 0x0000001F,
        HC4: 0x0000001F,
        HC5: 0x0000001F,
        HC6: 0x0000001F,
        HC7: 0x0000001F,
        HS: 0x00000000,
        R0: 0x00000000,
        R1: 0x00000000,
        R2: 0x00000000,
        R3: 0x00000000,
        R4: 0x00000000,
        R5: 0x00000000,
        R6: 0x00000000,
        R7: 0x00000000,
        CFG: 0x00000200,
        GC: 0x00000000,
        GS: 0x00000000,
        CV: 0x00000000,
        OFS: 0x00000000,
        CAL: 0x00000000,
    };

    /// Safe access to ADC2
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
        let taken = ADC2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ADC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ADC2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ADC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ADC2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with ADC2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ADC2];

    /// The interrupts associated with ADC2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ADC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC2: *const RegisterBlock = 0x400c8000 as *const _;
