#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPUART
//!
//! Used by: imxrt1011, imxrt1015

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::lpuart::Instance;
pub use crate::imxrt101::peripherals::lpuart::{RegisterBlock, ResetValues};

pub use crate::imxrt101::peripherals::lpuart::{
    BAUD, CTRL, DATA, FIFO, GLOBAL, MATCH, MODIR, PARAM, PINCFG, STAT, VERID, WATER,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The LPUART1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type LPUART1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART1 {
    const INSTANCE: Self = Self {
        addr: 0x40184000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in LPUART1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART1
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
        let taken = LPUART1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART1
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

        let taken = LPUART1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with LPUART1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART1];

    /// The interrupts associated with LPUART1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART1: *const RegisterBlock = 0x40184000 as *const _;

/// The LPUART2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type LPUART2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART2 {
    const INSTANCE: Self = Self {
        addr: 0x40188000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART2],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in LPUART2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART2
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
        let taken = LPUART2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART2
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

        let taken = LPUART2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with LPUART2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART2];

    /// The interrupts associated with LPUART2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART2: *const RegisterBlock = 0x40188000 as *const _;

/// The LPUART3 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type LPUART3 = Instance<3>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART3 {
    const INSTANCE: Self = Self {
        addr: 0x4018c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART3],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in LPUART3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART3
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
        let taken = LPUART3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART3
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

        let taken = LPUART3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with LPUART3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART3];

    /// The interrupts associated with LPUART3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART3: *const RegisterBlock = 0x4018c000 as *const _;

/// The LPUART4 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type LPUART4 = Instance<4>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART4 {
    const INSTANCE: Self = Self {
        addr: 0x40190000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART4],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in LPUART4
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART4
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
        let taken = LPUART4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART4
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

        let taken = LPUART4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with LPUART4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART4];

    /// The interrupts associated with LPUART4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART4: *const RegisterBlock = 0x40190000 as *const _;
