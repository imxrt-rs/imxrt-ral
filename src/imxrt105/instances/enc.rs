#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Quadrature Decoder
//!
//! Used by: imxrt1051, imxrt1052

pub use crate::imxrt105::peripherals::enc::Instance;
pub use crate::imxrt105::peripherals::enc::{RegisterBlock, ResetValues};

pub use crate::imxrt105::peripherals::enc::{
    CTRL, CTRL2, FILT, IMR, LCOMP, LINIT, LMOD, LPOS, LPOSH, POSD, POSDH, REV, REVH, TST, UCOMP,
    UINIT, UMOD, UPOS, UPOSH, WTR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ENC1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ENC1 = Instance<1>;

/// The ENC1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ENC1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct ENC1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ENC1 {}
impl crate::Valid for ENC1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ENC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ENC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl ENC1 {
    const INSTANCE: Self = Self {
        addr: 0x403c8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ENC1],
    };

    /// Reset values for each field in ENC1
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        FILT: 0x00000000,
        WTR: 0x00000000,
        POSD: 0x00000000,
        POSDH: 0x00000000,
        REV: 0x00000000,
        REVH: 0x00000000,
        UPOS: 0x00000000,
        LPOS: 0x00000000,
        UPOSH: 0x00000000,
        LPOSH: 0x00000000,
        UINIT: 0x00000000,
        LINIT: 0x00000000,
        IMR: 0x00000000,
        TST: 0x00000000,
        CTRL2: 0x00000000,
        UMOD: 0x00000000,
        LMOD: 0x00000000,
        UCOMP: 0x0000FFFF,
        LCOMP: 0x0000FFFF,
    };

    /// Safe access to ENC1
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
        let taken = ENC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ENC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ENC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ENC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ENC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ENC1 {
    /// The interrupts associated with ENC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ENC1];

    /// The interrupts associated with ENC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ENC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENC1: *const RegisterBlock = 0x403c8000 as *const _;

/// The ENC2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ENC2 = Instance<2>;

/// The ENC2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ENC2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct ENC2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ENC2 {}
impl crate::Valid for ENC2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ENC2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ENC2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl ENC2 {
    const INSTANCE: Self = Self {
        addr: 0x403cc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ENC2],
    };

    /// Reset values for each field in ENC2
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        FILT: 0x00000000,
        WTR: 0x00000000,
        POSD: 0x00000000,
        POSDH: 0x00000000,
        REV: 0x00000000,
        REVH: 0x00000000,
        UPOS: 0x00000000,
        LPOS: 0x00000000,
        UPOSH: 0x00000000,
        LPOSH: 0x00000000,
        UINIT: 0x00000000,
        LINIT: 0x00000000,
        IMR: 0x00000000,
        TST: 0x00000000,
        CTRL2: 0x00000000,
        UMOD: 0x00000000,
        LMOD: 0x00000000,
        UCOMP: 0x0000FFFF,
        LCOMP: 0x0000FFFF,
    };

    /// Safe access to ENC2
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
        let taken = ENC2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ENC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ENC2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ENC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ENC2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ENC2 {
    /// The interrupts associated with ENC2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ENC2];

    /// The interrupts associated with ENC2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ENC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENC2: *const RegisterBlock = 0x403cc000 as *const _;

/// The ENC3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ENC3 = Instance<3>;

/// The ENC3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ENC3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct ENC3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ENC3 {}
impl crate::Valid for ENC3 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ENC3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ENC3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl ENC3 {
    const INSTANCE: Self = Self {
        addr: 0x403d0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ENC3],
    };

    /// Reset values for each field in ENC3
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        FILT: 0x00000000,
        WTR: 0x00000000,
        POSD: 0x00000000,
        POSDH: 0x00000000,
        REV: 0x00000000,
        REVH: 0x00000000,
        UPOS: 0x00000000,
        LPOS: 0x00000000,
        UPOSH: 0x00000000,
        LPOSH: 0x00000000,
        UINIT: 0x00000000,
        LINIT: 0x00000000,
        IMR: 0x00000000,
        TST: 0x00000000,
        CTRL2: 0x00000000,
        UMOD: 0x00000000,
        LMOD: 0x00000000,
        UCOMP: 0x0000FFFF,
        LCOMP: 0x0000FFFF,
    };

    /// Safe access to ENC3
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
        let taken = ENC3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ENC3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ENC3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ENC3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ENC3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ENC3 {
    /// The interrupts associated with ENC3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ENC3];

    /// The interrupts associated with ENC3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ENC3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENC3: *const RegisterBlock = 0x403d0000 as *const _;

/// The ENC4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ENC4 = Instance<4>;

/// The ENC4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ENC4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct ENC4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ENC4 {}
impl crate::Valid for ENC4 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ENC4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ENC4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl ENC4 {
    const INSTANCE: Self = Self {
        addr: 0x403d4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ENC4],
    };

    /// Reset values for each field in ENC4
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        FILT: 0x00000000,
        WTR: 0x00000000,
        POSD: 0x00000000,
        POSDH: 0x00000000,
        REV: 0x00000000,
        REVH: 0x00000000,
        UPOS: 0x00000000,
        LPOS: 0x00000000,
        UPOSH: 0x00000000,
        LPOSH: 0x00000000,
        UINIT: 0x00000000,
        LINIT: 0x00000000,
        IMR: 0x00000000,
        TST: 0x00000000,
        CTRL2: 0x00000000,
        UMOD: 0x00000000,
        LMOD: 0x00000000,
        UCOMP: 0x0000FFFF,
        LCOMP: 0x0000FFFF,
    };

    /// Safe access to ENC4
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
        let taken = ENC4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ENC4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ENC4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ENC4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ENC4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ENC4 {
    /// The interrupts associated with ENC4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ENC4];

    /// The interrupts associated with ENC4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ENC4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENC4: *const RegisterBlock = 0x403d4000 as *const _;
