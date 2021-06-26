#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPSPI
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::lpspi::Instance;
pub use crate::imxrt106::peripherals::lpspi::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::lpspi::{
    CCR, CFGR0, CFGR1, CR, DER, DMR0, DMR1, FCR, FSR, IER, PARAM, RDR, RSR, SR, TCR, TDR, VERID,
};

/// Access functions for the LPSPI1 peripheral instance
pub mod LPSPI1 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40394000,
        _marker: ::core::marker::PhantomData,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPSPI1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in LPSPI1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01020004,
        PARAM: 0x00040404,
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        DER: 0x00000000,
        CFGR0: 0x00000000,
        CFGR1: 0x00000000,
        DMR0: 0x00000000,
        DMR1: 0x00000000,
        CCR: 0x00000000,
        FCR: 0x00000000,
        FSR: 0x00000000,
        TCR: 0x0000001F,
        TDR: 0x00000000,
        RSR: 0x00000002,
        RDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static LPSPI1_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to LPSPI1
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
        let taken = LPSPI1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to LPSPI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = LPSPI1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPSPI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPSPI1_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }

    /// The interrupts associated with LPSPI1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPSPI1];

    /// The interrupts associated with LPSPI1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPSPI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPSPI1: *const RegisterBlock = 0x40394000 as *const _;

/// Access functions for the LPSPI2 peripheral instance
pub mod LPSPI2 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40398000,
        _marker: ::core::marker::PhantomData,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPSPI2],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in LPSPI2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01020004,
        PARAM: 0x00040404,
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        DER: 0x00000000,
        CFGR0: 0x00000000,
        CFGR1: 0x00000000,
        DMR0: 0x00000000,
        DMR1: 0x00000000,
        CCR: 0x00000000,
        FCR: 0x00000000,
        FSR: 0x00000000,
        TCR: 0x0000001F,
        TDR: 0x00000000,
        RSR: 0x00000002,
        RDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static LPSPI2_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to LPSPI2
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
        let taken = LPSPI2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to LPSPI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = LPSPI2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPSPI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPSPI2_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }

    /// The interrupts associated with LPSPI2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPSPI2];

    /// The interrupts associated with LPSPI2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPSPI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPSPI2: *const RegisterBlock = 0x40398000 as *const _;

/// Access functions for the LPSPI3 peripheral instance
pub mod LPSPI3 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4039c000,
        _marker: ::core::marker::PhantomData,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPSPI3],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in LPSPI3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01020004,
        PARAM: 0x00040404,
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        DER: 0x00000000,
        CFGR0: 0x00000000,
        CFGR1: 0x00000000,
        DMR0: 0x00000000,
        DMR1: 0x00000000,
        CCR: 0x00000000,
        FCR: 0x00000000,
        FSR: 0x00000000,
        TCR: 0x0000001F,
        TDR: 0x00000000,
        RSR: 0x00000002,
        RDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static LPSPI3_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to LPSPI3
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
        let taken = LPSPI3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to LPSPI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = LPSPI3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPSPI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPSPI3_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }

    /// The interrupts associated with LPSPI3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPSPI3];

    /// The interrupts associated with LPSPI3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPSPI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPSPI3: *const RegisterBlock = 0x4039c000 as *const _;

/// Access functions for the LPSPI4 peripheral instance
pub mod LPSPI4 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403a0000,
        _marker: ::core::marker::PhantomData,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPSPI4],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in LPSPI4
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01020004,
        PARAM: 0x00040404,
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        DER: 0x00000000,
        CFGR0: 0x00000000,
        CFGR1: 0x00000000,
        DMR0: 0x00000000,
        DMR1: 0x00000000,
        CCR: 0x00000000,
        FCR: 0x00000000,
        FSR: 0x00000000,
        TCR: 0x0000001F,
        TDR: 0x00000000,
        RSR: 0x00000002,
        RDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static LPSPI4_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to LPSPI4
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
        let taken = LPSPI4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to LPSPI4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = LPSPI4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPSPI4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPSPI4_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }

    /// The interrupts associated with LPSPI4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPSPI4];

    /// The interrupts associated with LPSPI4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPSPI4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPSPI4: *const RegisterBlock = 0x403a0000 as *const _;
