#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXCAN
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
use crate::imxrt106::peripherals::can::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::can::{Instance, Valid};
pub use crate::imxrt106::peripherals::can::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::can::{
    CRCR, CTRL1, CTRL2, DBG1, DBG2, ECR, ESR1, ESR2, GFWR, IFLAG1, IFLAG2, IMASK1, IMASK2, MCR,
    RX14MASK, RX15MASK, RXFGMASK, RXFIR, RXIMR0, RXIMR1, RXIMR10, RXIMR11, RXIMR12, RXIMR13,
    RXIMR14, RXIMR15, RXIMR16, RXIMR17, RXIMR18, RXIMR19, RXIMR2, RXIMR20, RXIMR21, RXIMR22,
    RXIMR23, RXIMR24, RXIMR25, RXIMR26, RXIMR27, RXIMR28, RXIMR29, RXIMR3, RXIMR30, RXIMR31,
    RXIMR32, RXIMR33, RXIMR34, RXIMR35, RXIMR36, RXIMR37, RXIMR38, RXIMR39, RXIMR4, RXIMR40,
    RXIMR41, RXIMR42, RXIMR43, RXIMR44, RXIMR45, RXIMR46, RXIMR47, RXIMR48, RXIMR49, RXIMR5,
    RXIMR50, RXIMR51, RXIMR52, RXIMR53, RXIMR54, RXIMR55, RXIMR56, RXIMR57, RXIMR58, RXIMR59,
    RXIMR6, RXIMR60, RXIMR61, RXIMR62, RXIMR63, RXIMR7, RXIMR8, RXIMR9, RXMGMASK, TIMER,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CAN1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type CAN1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for CAN1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for CAN1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CAN1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CAN1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CAN1 {
    const INSTANCE: Self = Self {
        addr: 0x401d0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CAN1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in CAN1
    pub const reset: ResetValues = ResetValues {
        MCR: 0x5980000F,
        CTRL1: 0x00000000,
        TIMER: 0x00000000,
        RXMGMASK: 0xFFFFFFFF,
        RX14MASK: 0xFFFFFFFF,
        RX15MASK: 0xFFFFFFFF,
        ECR: 0x00000000,
        ESR1: 0x00000000,
        IMASK2: 0x00000000,
        IMASK1: 0x00000000,
        IFLAG2: 0x00000000,
        IFLAG1: 0x00000000,
        CTRL2: 0x00000000,
        ESR2: 0x00000000,
        CRCR: 0x00000000,
        RXFGMASK: 0xFFFFFFFF,
        RXFIR: 0x00000000,
        DBG1: 0x00010000,
        DBG2: 0x00000000,
        RXIMR0: 0x00000000,
        RXIMR1: 0x00000000,
        RXIMR2: 0x00000000,
        RXIMR3: 0x00000000,
        RXIMR4: 0x00000000,
        RXIMR5: 0x00000000,
        RXIMR6: 0x00000000,
        RXIMR7: 0x00000000,
        RXIMR8: 0x00000000,
        RXIMR9: 0x00000000,
        RXIMR10: 0x00000000,
        RXIMR11: 0x00000000,
        RXIMR12: 0x00000000,
        RXIMR13: 0x00000000,
        RXIMR14: 0x00000000,
        RXIMR15: 0x00000000,
        RXIMR16: 0x00000000,
        RXIMR17: 0x00000000,
        RXIMR18: 0x00000000,
        RXIMR19: 0x00000000,
        RXIMR20: 0x00000000,
        RXIMR21: 0x00000000,
        RXIMR22: 0x00000000,
        RXIMR23: 0x00000000,
        RXIMR24: 0x00000000,
        RXIMR25: 0x00000000,
        RXIMR26: 0x00000000,
        RXIMR27: 0x00000000,
        RXIMR28: 0x00000000,
        RXIMR29: 0x00000000,
        RXIMR30: 0x00000000,
        RXIMR31: 0x00000000,
        RXIMR32: 0x00000000,
        RXIMR33: 0x00000000,
        RXIMR34: 0x00000000,
        RXIMR35: 0x00000000,
        RXIMR36: 0x00000000,
        RXIMR37: 0x00000000,
        RXIMR38: 0x00000000,
        RXIMR39: 0x00000000,
        RXIMR40: 0x00000000,
        RXIMR41: 0x00000000,
        RXIMR42: 0x00000000,
        RXIMR43: 0x00000000,
        RXIMR44: 0x00000000,
        RXIMR45: 0x00000000,
        RXIMR46: 0x00000000,
        RXIMR47: 0x00000000,
        RXIMR48: 0x00000000,
        RXIMR49: 0x00000000,
        RXIMR50: 0x00000000,
        RXIMR51: 0x00000000,
        RXIMR52: 0x00000000,
        RXIMR53: 0x00000000,
        RXIMR54: 0x00000000,
        RXIMR55: 0x00000000,
        RXIMR56: 0x00000000,
        RXIMR57: 0x00000000,
        RXIMR58: 0x00000000,
        RXIMR59: 0x00000000,
        RXIMR60: 0x00000000,
        RXIMR61: 0x00000000,
        RXIMR62: 0x00000000,
        RXIMR63: 0x00000000,
        GFWR: 0x0000007F,
    };

    /// Safe access to CAN1
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
        let taken = CAN1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CAN1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CAN1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CAN1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with CAN1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::CAN1];

    /// The interrupts associated with CAN1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CAN1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN1: *const RegisterBlock = 0x401d0000 as *const _;

/// The CAN2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type CAN2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for CAN2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for CAN2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CAN2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CAN2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CAN2 {
    const INSTANCE: Self = Self {
        addr: 0x401d4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CAN2],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in CAN2
    pub const reset: ResetValues = ResetValues {
        MCR: 0x5980000F,
        CTRL1: 0x00000000,
        TIMER: 0x00000000,
        RXMGMASK: 0xFFFFFFFF,
        RX14MASK: 0xFFFFFFFF,
        RX15MASK: 0xFFFFFFFF,
        ECR: 0x00000000,
        ESR1: 0x00000000,
        IMASK2: 0x00000000,
        IMASK1: 0x00000000,
        IFLAG2: 0x00000000,
        IFLAG1: 0x00000000,
        CTRL2: 0x00000000,
        ESR2: 0x00000000,
        CRCR: 0x00000000,
        RXFGMASK: 0xFFFFFFFF,
        RXFIR: 0x00000000,
        DBG1: 0x00010000,
        DBG2: 0x00000000,
        RXIMR0: 0x00000000,
        RXIMR1: 0x00000000,
        RXIMR2: 0x00000000,
        RXIMR3: 0x00000000,
        RXIMR4: 0x00000000,
        RXIMR5: 0x00000000,
        RXIMR6: 0x00000000,
        RXIMR7: 0x00000000,
        RXIMR8: 0x00000000,
        RXIMR9: 0x00000000,
        RXIMR10: 0x00000000,
        RXIMR11: 0x00000000,
        RXIMR12: 0x00000000,
        RXIMR13: 0x00000000,
        RXIMR14: 0x00000000,
        RXIMR15: 0x00000000,
        RXIMR16: 0x00000000,
        RXIMR17: 0x00000000,
        RXIMR18: 0x00000000,
        RXIMR19: 0x00000000,
        RXIMR20: 0x00000000,
        RXIMR21: 0x00000000,
        RXIMR22: 0x00000000,
        RXIMR23: 0x00000000,
        RXIMR24: 0x00000000,
        RXIMR25: 0x00000000,
        RXIMR26: 0x00000000,
        RXIMR27: 0x00000000,
        RXIMR28: 0x00000000,
        RXIMR29: 0x00000000,
        RXIMR30: 0x00000000,
        RXIMR31: 0x00000000,
        RXIMR32: 0x00000000,
        RXIMR33: 0x00000000,
        RXIMR34: 0x00000000,
        RXIMR35: 0x00000000,
        RXIMR36: 0x00000000,
        RXIMR37: 0x00000000,
        RXIMR38: 0x00000000,
        RXIMR39: 0x00000000,
        RXIMR40: 0x00000000,
        RXIMR41: 0x00000000,
        RXIMR42: 0x00000000,
        RXIMR43: 0x00000000,
        RXIMR44: 0x00000000,
        RXIMR45: 0x00000000,
        RXIMR46: 0x00000000,
        RXIMR47: 0x00000000,
        RXIMR48: 0x00000000,
        RXIMR49: 0x00000000,
        RXIMR50: 0x00000000,
        RXIMR51: 0x00000000,
        RXIMR52: 0x00000000,
        RXIMR53: 0x00000000,
        RXIMR54: 0x00000000,
        RXIMR55: 0x00000000,
        RXIMR56: 0x00000000,
        RXIMR57: 0x00000000,
        RXIMR58: 0x00000000,
        RXIMR59: 0x00000000,
        RXIMR60: 0x00000000,
        RXIMR61: 0x00000000,
        RXIMR62: 0x00000000,
        RXIMR63: 0x00000000,
        GFWR: 0x0000007F,
    };

    /// Safe access to CAN2
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
        let taken = CAN2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CAN2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CAN2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CAN2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with CAN2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::CAN2];

    /// The interrupts associated with CAN2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CAN2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN2: *const RegisterBlock = 0x401d4000 as *const _;
