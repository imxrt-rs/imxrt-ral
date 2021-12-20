#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPT
//!
//! Used by: imxrt1011, imxrt1015

#[cfg(not(feature = "nosync"))]
use crate::imxrt101::peripherals::gpt::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::gpt::{Instance, Valid};
pub use crate::imxrt101::peripherals::gpt::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::gpt::{CNT, CR, ICR1, ICR2, IR, OCR1, OCR2, OCR3, PR, SR};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The GPT1 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPT1 = Instance<1>;

/// The GPT1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPT1 = Instance<1>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPT1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPT1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPT1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPT1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPT1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPT1 {
    const INSTANCE: Self = Self {
        addr: 0x401ec000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPT1],
    };

    /// Reset values for each field in GPT1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    /// Safe access to GPT1
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
        let taken = GPT1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPT1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPT1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPT1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPT1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPT1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPT1];

    /// The interrupts associated with GPT1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPT1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT1: *const RegisterBlock = 0x401ec000 as *const _;

/// The GPT2 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPT2 = Instance<2>;

/// The GPT2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPT2 = Instance<2>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPT2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPT2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPT2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPT2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPT2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPT2 {
    const INSTANCE: Self = Self {
        addr: 0x401f0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPT2],
    };

    /// Reset values for each field in GPT2
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    /// Safe access to GPT2
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
        let taken = GPT2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPT2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPT2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPT2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPT2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPT2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPT2];

    /// The interrupts associated with GPT2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPT2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT2: *const RegisterBlock = 0x401f0000 as *const _;
