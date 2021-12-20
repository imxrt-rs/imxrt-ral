#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)
//!
//! Used by: imxrt1051, imxrt1052

use crate::imxrt105::peripherals::cmp::private;
pub use crate::imxrt105::peripherals::cmp::{Instance, Valid};
pub use crate::imxrt105::peripherals::cmp::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::cmp::{CR0, CR1, DACCR, FPR, MUXCR, SCR};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CMP1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CMP1 = Instance<1>;

/// The CMP1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CMP1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct CMP1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl private::Sealed for CMP1 {}
impl Valid for CMP1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CMP1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CMP1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CMP1 {
    const INSTANCE: Self = Self {
        addr: 0x40094000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ACMP1],
    };

    /// Reset values for each field in CMP1
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    /// Safe access to CMP1
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
        let taken = CMP1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CMP1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CMP1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CMP1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CMP1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CMP1 {
    /// The interrupts associated with CMP1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ACMP1];

    /// The interrupts associated with CMP1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CMP1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP1: *const RegisterBlock = 0x40094000 as *const _;

/// The CMP2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CMP2 = Instance<2>;

/// The CMP2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CMP2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct CMP2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl private::Sealed for CMP2 {}
impl Valid for CMP2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CMP2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CMP2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CMP2 {
    const INSTANCE: Self = Self {
        addr: 0x40094008,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ACMP2],
    };

    /// Reset values for each field in CMP2
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    /// Safe access to CMP2
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
        let taken = CMP2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CMP2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CMP2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CMP2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CMP2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CMP2 {
    /// The interrupts associated with CMP2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ACMP2];

    /// The interrupts associated with CMP2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CMP2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP2: *const RegisterBlock = 0x40094008 as *const _;

/// The CMP3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CMP3 = Instance<3>;

/// The CMP3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CMP3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct CMP3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl private::Sealed for CMP3 {}
impl Valid for CMP3 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CMP3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CMP3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CMP3 {
    const INSTANCE: Self = Self {
        addr: 0x40094010,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ACMP3],
    };

    /// Reset values for each field in CMP3
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    /// Safe access to CMP3
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
        let taken = CMP3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CMP3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CMP3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CMP3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CMP3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CMP3 {
    /// The interrupts associated with CMP3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ACMP3];

    /// The interrupts associated with CMP3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CMP3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP3: *const RegisterBlock = 0x40094010 as *const _;

/// The CMP4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CMP4 = Instance<4>;

/// The CMP4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CMP4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct CMP4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl private::Sealed for CMP4 {}
impl Valid for CMP4 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CMP4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CMP4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CMP4 {
    const INSTANCE: Self = Self {
        addr: 0x40094018,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ACMP4],
    };

    /// Reset values for each field in CMP4
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    /// Safe access to CMP4
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
        let taken = CMP4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CMP4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CMP4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CMP4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CMP4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CMP4 {
    /// The interrupts associated with CMP4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ACMP4];

    /// The interrupts associated with CMP4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CMP4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP4: *const RegisterBlock = 0x40094018 as *const _;
