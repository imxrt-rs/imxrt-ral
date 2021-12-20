#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! I2S
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
use crate::imxrt105::peripherals::sai::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::sai::{Instance, Valid};
pub use crate::imxrt105::peripherals::sai::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::sai::{
    PARAM, RCR1, RCR2, RCR3, RCR4, RCR5, RCSR, RDR0, RDR1, RDR2, RDR3, RFR0, RFR1, RFR2, RFR3, RMR,
    TCR1, TCR2, TCR3, TCR4, TCR5, TCSR, TDR0, TDR1, TDR2, TDR3, TFR0, TFR1, TFR2, TFR3, TMR, VERID,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The SAI1 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type SAI1 = Instance<1>;

/// The SAI1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SAI1 = Instance<1>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct SAI1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for SAI1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for SAI1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SAI1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SAI1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl SAI1 {
    const INSTANCE: Self = Self {
        addr: 0x40384000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SAI1],
    };

    /// Reset values for each field in SAI1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03000000,
        PARAM: 0x00050504,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TDR1: 0x00000000,
        TDR2: 0x00000000,
        TDR3: 0x00000000,
        TFR0: 0x00000000,
        TFR1: 0x00000000,
        TFR2: 0x00000000,
        TFR3: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RDR1: 0x00000000,
        RDR2: 0x00000000,
        RDR3: 0x00000000,
        RFR0: 0x00000000,
        RFR1: 0x00000000,
        RFR2: 0x00000000,
        RFR3: 0x00000000,
        RMR: 0x00000000,
    };

    /// Safe access to SAI1
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
        let taken = SAI1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SAI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SAI1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SAI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SAI1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with SAI1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::SAI1];

    /// The interrupts associated with SAI1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SAI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI1: *const RegisterBlock = 0x40384000 as *const _;

/// The SAI2 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type SAI2 = Instance<2>;

/// The SAI2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SAI2 = Instance<2>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct SAI2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for SAI2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for SAI2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SAI2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SAI2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl SAI2 {
    const INSTANCE: Self = Self {
        addr: 0x40388000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SAI2],
    };

    /// Reset values for each field in SAI2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03000000,
        PARAM: 0x00050504,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TDR1: 0x00000000,
        TDR2: 0x00000000,
        TDR3: 0x00000000,
        TFR0: 0x00000000,
        TFR1: 0x00000000,
        TFR2: 0x00000000,
        TFR3: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RDR1: 0x00000000,
        RDR2: 0x00000000,
        RDR3: 0x00000000,
        RFR0: 0x00000000,
        RFR1: 0x00000000,
        RFR2: 0x00000000,
        RFR3: 0x00000000,
        RMR: 0x00000000,
    };

    /// Safe access to SAI2
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
        let taken = SAI2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SAI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SAI2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SAI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SAI2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with SAI2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::SAI2];

    /// The interrupts associated with SAI2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SAI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI2: *const RegisterBlock = 0x40388000 as *const _;

/// The SAI3 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type SAI3 = Instance<3>;

/// The SAI3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SAI3 = Instance<3>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct SAI3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for SAI3 {}
#[cfg(not(feature = "nosync"))]
impl Valid for SAI3 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SAI3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SAI3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl SAI3 {
    const INSTANCE: Self = Self {
        addr: 0x4038c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SAI3_RX, crate::interrupt::SAI3_TX],
    };

    /// Reset values for each field in SAI3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03000000,
        PARAM: 0x00050504,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TDR1: 0x00000000,
        TDR2: 0x00000000,
        TDR3: 0x00000000,
        TFR0: 0x00000000,
        TFR1: 0x00000000,
        TFR2: 0x00000000,
        TFR3: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RDR1: 0x00000000,
        RDR2: 0x00000000,
        RDR3: 0x00000000,
        RFR0: 0x00000000,
        RFR1: 0x00000000,
        RFR2: 0x00000000,
        RFR3: 0x00000000,
        RMR: 0x00000000,
    };

    /// Safe access to SAI3
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
        let taken = SAI3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SAI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SAI3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SAI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SAI3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with SAI3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::SAI3_RX, crate::interrupt::SAI3_TX];

    /// The interrupts associated with SAI3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SAI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI3: *const RegisterBlock = 0x4038c000 as *const _;
