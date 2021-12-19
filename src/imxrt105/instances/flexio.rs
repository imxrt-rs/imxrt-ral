#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXIO
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::flexio::Instance;
pub use crate::imxrt105::peripherals::flexio::{RegisterBlock, ResetValues};

pub use crate::imxrt105::peripherals::flexio::{
    CTRL, PARAM, PIN, SHIFTBUF0, SHIFTBUF1, SHIFTBUF2, SHIFTBUF3, SHIFTBUFBBS0, SHIFTBUFBBS1,
    SHIFTBUFBBS2, SHIFTBUFBBS3, SHIFTBUFBIS0, SHIFTBUFBIS1, SHIFTBUFBIS2, SHIFTBUFBIS3,
    SHIFTBUFBYS0, SHIFTBUFBYS1, SHIFTBUFBYS2, SHIFTBUFBYS3, SHIFTBUFHWS0, SHIFTBUFHWS1,
    SHIFTBUFHWS2, SHIFTBUFHWS3, SHIFTBUFNBS0, SHIFTBUFNBS1, SHIFTBUFNBS2, SHIFTBUFNBS3,
    SHIFTBUFNIS0, SHIFTBUFNIS1, SHIFTBUFNIS2, SHIFTBUFNIS3, SHIFTCFG0, SHIFTCFG1, SHIFTCFG2,
    SHIFTCFG3, SHIFTCTL0, SHIFTCTL1, SHIFTCTL2, SHIFTCTL3, SHIFTEIEN, SHIFTERR, SHIFTSDEN,
    SHIFTSIEN, SHIFTSTAT, SHIFTSTATE, TIMCFG0, TIMCFG1, TIMCFG2, TIMCFG3, TIMCMP0, TIMCMP1,
    TIMCMP2, TIMCMP3, TIMCTL0, TIMCTL1, TIMCTL2, TIMCTL3, TIMIEN, TIMSTAT, VERID,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The FLEXIO1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type FLEXIO1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static FLEXIO1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the FLEXIO1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl FLEXIO1 {
    const INSTANCE: Self = Self {
        addr: 0x401ac000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::FLEXIO1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in FLEXIO1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01010001,
        PARAM: 0x02200404,
        CTRL: 0x00000000,
        PIN: 0x00000000,
        SHIFTSTAT: 0x00000000,
        SHIFTERR: 0x00000000,
        TIMSTAT: 0x00000000,
        SHIFTSIEN: 0x00000000,
        SHIFTEIEN: 0x00000000,
        TIMIEN: 0x00000000,
        SHIFTSDEN: 0x00000000,
        SHIFTSTATE: 0x00000000,
        SHIFTCTL0: 0x00000000,
        SHIFTCTL1: 0x00000000,
        SHIFTCTL2: 0x00000000,
        SHIFTCTL3: 0x00000000,
        SHIFTCFG0: 0x00000000,
        SHIFTCFG1: 0x00000000,
        SHIFTCFG2: 0x00000000,
        SHIFTCFG3: 0x00000000,
        SHIFTBUF0: 0x00000000,
        SHIFTBUF1: 0x00000000,
        SHIFTBUF2: 0x00000000,
        SHIFTBUF3: 0x00000000,
        SHIFTBUFBIS0: 0x00000000,
        SHIFTBUFBIS1: 0x00000000,
        SHIFTBUFBIS2: 0x00000000,
        SHIFTBUFBIS3: 0x00000000,
        SHIFTBUFBYS0: 0x00000000,
        SHIFTBUFBYS1: 0x00000000,
        SHIFTBUFBYS2: 0x00000000,
        SHIFTBUFBYS3: 0x00000000,
        SHIFTBUFBBS0: 0x00000000,
        SHIFTBUFBBS1: 0x00000000,
        SHIFTBUFBBS2: 0x00000000,
        SHIFTBUFBBS3: 0x00000000,
        TIMCTL0: 0x00000000,
        TIMCTL1: 0x00000000,
        TIMCTL2: 0x00000000,
        TIMCTL3: 0x00000000,
        TIMCFG0: 0x00000000,
        TIMCFG1: 0x00000000,
        TIMCFG2: 0x00000000,
        TIMCFG3: 0x00000000,
        TIMCMP0: 0x00000000,
        TIMCMP1: 0x00000000,
        TIMCMP2: 0x00000000,
        TIMCMP3: 0x00000000,
        SHIFTBUFNBS0: 0x00000000,
        SHIFTBUFNBS1: 0x00000000,
        SHIFTBUFNBS2: 0x00000000,
        SHIFTBUFNBS3: 0x00000000,
        SHIFTBUFHWS0: 0x00000000,
        SHIFTBUFHWS1: 0x00000000,
        SHIFTBUFHWS2: 0x00000000,
        SHIFTBUFHWS3: 0x00000000,
        SHIFTBUFNIS0: 0x00000000,
        SHIFTBUFNIS1: 0x00000000,
        SHIFTBUFNIS2: 0x00000000,
        SHIFTBUFNIS3: 0x00000000,
    };

    /// Safe access to FLEXIO1
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
        let taken = FLEXIO1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to FLEXIO1
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

        let taken = FLEXIO1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal FLEXIO1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        FLEXIO1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with FLEXIO1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::FLEXIO1];

    /// The interrupts associated with FLEXIO1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to FLEXIO1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLEXIO1: *const RegisterBlock = 0x401ac000 as *const _;

/// The FLEXIO2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type FLEXIO2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static FLEXIO2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the FLEXIO2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl FLEXIO2 {
    const INSTANCE: Self = Self {
        addr: 0x401b0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::FLEXIO2],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in FLEXIO2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01010001,
        PARAM: 0x02200404,
        CTRL: 0x00000000,
        PIN: 0x00000000,
        SHIFTSTAT: 0x00000000,
        SHIFTERR: 0x00000000,
        TIMSTAT: 0x00000000,
        SHIFTSIEN: 0x00000000,
        SHIFTEIEN: 0x00000000,
        TIMIEN: 0x00000000,
        SHIFTSDEN: 0x00000000,
        SHIFTSTATE: 0x00000000,
        SHIFTCTL0: 0x00000000,
        SHIFTCTL1: 0x00000000,
        SHIFTCTL2: 0x00000000,
        SHIFTCTL3: 0x00000000,
        SHIFTCFG0: 0x00000000,
        SHIFTCFG1: 0x00000000,
        SHIFTCFG2: 0x00000000,
        SHIFTCFG3: 0x00000000,
        SHIFTBUF0: 0x00000000,
        SHIFTBUF1: 0x00000000,
        SHIFTBUF2: 0x00000000,
        SHIFTBUF3: 0x00000000,
        SHIFTBUFBIS0: 0x00000000,
        SHIFTBUFBIS1: 0x00000000,
        SHIFTBUFBIS2: 0x00000000,
        SHIFTBUFBIS3: 0x00000000,
        SHIFTBUFBYS0: 0x00000000,
        SHIFTBUFBYS1: 0x00000000,
        SHIFTBUFBYS2: 0x00000000,
        SHIFTBUFBYS3: 0x00000000,
        SHIFTBUFBBS0: 0x00000000,
        SHIFTBUFBBS1: 0x00000000,
        SHIFTBUFBBS2: 0x00000000,
        SHIFTBUFBBS3: 0x00000000,
        TIMCTL0: 0x00000000,
        TIMCTL1: 0x00000000,
        TIMCTL2: 0x00000000,
        TIMCTL3: 0x00000000,
        TIMCFG0: 0x00000000,
        TIMCFG1: 0x00000000,
        TIMCFG2: 0x00000000,
        TIMCFG3: 0x00000000,
        TIMCMP0: 0x00000000,
        TIMCMP1: 0x00000000,
        TIMCMP2: 0x00000000,
        TIMCMP3: 0x00000000,
        SHIFTBUFNBS0: 0x00000000,
        SHIFTBUFNBS1: 0x00000000,
        SHIFTBUFNBS2: 0x00000000,
        SHIFTBUFNBS3: 0x00000000,
        SHIFTBUFHWS0: 0x00000000,
        SHIFTBUFHWS1: 0x00000000,
        SHIFTBUFHWS2: 0x00000000,
        SHIFTBUFHWS3: 0x00000000,
        SHIFTBUFNIS0: 0x00000000,
        SHIFTBUFNIS1: 0x00000000,
        SHIFTBUFNIS2: 0x00000000,
        SHIFTBUFNIS3: 0x00000000,
    };

    /// Safe access to FLEXIO2
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
        let taken = FLEXIO2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to FLEXIO2
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

        let taken = FLEXIO2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal FLEXIO2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        FLEXIO2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with FLEXIO2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::FLEXIO2];

    /// The interrupts associated with FLEXIO2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to FLEXIO2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLEXIO2: *const RegisterBlock = 0x401b0000 as *const _;
