#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WDOG

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::wdog::Instance;
pub use crate::imxrt101::peripherals::wdog::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::wdog::{WCR, WICR, WMCR, WRSR, WSR};

/// Access functions for the WDOG1 peripheral instance
pub mod WDOG1 {
    use super::ResetValues;
    use typenum::*;

    #[cfg(not(feature = "nosync"))]
    pub type Instance = super::Instance<U1>;

    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400b8000,
        _marker: ::core::marker::PhantomData,
        _inst: ::core::marker::PhantomData,
    };

    /// Reset values for each field in WDOG1
    pub const reset: ResetValues = ResetValues {
        WCR: 0x00000030,
        WSR: 0x00000000,
        WRSR: 0x00000010,
        WICR: 0x00000004,
        WMCR: 0x00000001,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static WDOG1_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to WDOG1
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
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        let taken = WDOG1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to WDOG1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = WDOG1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal WDOG1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        WDOG1_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }
}

/// Raw pointer to WDOG1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WDOG1: *const RegisterBlock = 0x400b8000 as *const _;

/// Access functions for the WDOG2 peripheral instance
pub mod WDOG2 {
    use super::ResetValues;
    use typenum::*;

    #[cfg(not(feature = "nosync"))]
    pub type Instance = super::Instance<U2>;

    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d0000,
        _marker: ::core::marker::PhantomData,
        _inst: ::core::marker::PhantomData,
    };

    /// Reset values for each field in WDOG2
    pub const reset: ResetValues = ResetValues {
        WCR: 0x00000030,
        WSR: 0x00000000,
        WRSR: 0x00000010,
        WICR: 0x00000004,
        WMCR: 0x00000001,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static WDOG2_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to WDOG2
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
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        let taken = WDOG2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to WDOG2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = WDOG2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal WDOG2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        WDOG2_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }
}

/// Raw pointer to WDOG2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WDOG2: *const RegisterBlock = 0x400d0000 as *const _;
