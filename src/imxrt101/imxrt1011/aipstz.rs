#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AIPSTZ Control Registers

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::aipstz::Instance;
pub use crate::imxrt101::peripherals::aipstz::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::aipstz::{MPR, OPACR, OPACR1, OPACR2, OPACR3, OPACR4};

/// Access functions for the AIPSTZ1 peripheral instance
pub mod AIPSTZ1 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4007c000,
        _marker: ::core::marker::PhantomData,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ1
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static AIPSTZ1_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to AIPSTZ1
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
        let taken = AIPSTZ1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = AIPSTZ1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AIPSTZ1_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }

    /// The interrupts associated with AIPSTZ1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ1: *const RegisterBlock = 0x4007c000 as *const _;

/// Access functions for the AIPSTZ2 peripheral instance
pub mod AIPSTZ2 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4017c000,
        _marker: ::core::marker::PhantomData,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ2
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static AIPSTZ2_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to AIPSTZ2
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
        let taken = AIPSTZ2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = AIPSTZ2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AIPSTZ2_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }

    /// The interrupts associated with AIPSTZ2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ2: *const RegisterBlock = 0x4017c000 as *const _;
