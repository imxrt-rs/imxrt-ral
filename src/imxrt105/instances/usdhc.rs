#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! uSDHC
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
use crate::imxrt105::peripherals::usdhc::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::usdhc::{Instance, Valid};
pub use crate::imxrt105::peripherals::usdhc::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::usdhc::{
    ADMA_ERR_STATUS, ADMA_SYS_ADDR, AUTOCMD12_ERR_STATUS, BLK_ATT, CLK_TUNE_CTRL_STATUS, CMD_ARG,
    CMD_RSP0, CMD_RSP1, CMD_RSP2, CMD_RSP3, CMD_XFR_TYP, DATA_BUFF_ACC_PORT, DLL_CTRL, DLL_STATUS,
    DS_ADDR, FORCE_EVENT, HOST_CTRL_CAP, INT_SIGNAL_EN, INT_STATUS, INT_STATUS_EN, MIX_CTRL,
    MMC_BOOT, PRES_STATE, PROT_CTRL, SYS_CTRL, TUNING_CTRL, VEND_SPEC, VEND_SPEC2, WTMK_LVL,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The USDHC1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type USDHC1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for USDHC1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for USDHC1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USDHC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USDHC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USDHC1 {
    const INSTANCE: Self = Self {
        addr: 0x402c0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::USDHC1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in USDHC1
    pub const reset: ResetValues = ResetValues {
        DS_ADDR: 0x00000000,
        BLK_ATT: 0x00000000,
        CMD_ARG: 0x00000000,
        CMD_XFR_TYP: 0x00000000,
        CMD_RSP0: 0x00000000,
        CMD_RSP1: 0x00000000,
        CMD_RSP2: 0x00000000,
        CMD_RSP3: 0x00000000,
        DATA_BUFF_ACC_PORT: 0x00000000,
        PRES_STATE: 0x00008080,
        PROT_CTRL: 0x08800020,
        SYS_CTRL: 0x0080800F,
        INT_STATUS: 0x00000000,
        INT_STATUS_EN: 0x00000000,
        INT_SIGNAL_EN: 0x00000000,
        AUTOCMD12_ERR_STATUS: 0x00000000,
        HOST_CTRL_CAP: 0x07F3B407,
        WTMK_LVL: 0x08100810,
        MIX_CTRL: 0x80000000,
        FORCE_EVENT: 0x00000000,
        ADMA_ERR_STATUS: 0x00000000,
        ADMA_SYS_ADDR: 0x00000000,
        DLL_CTRL: 0x00000000,
        DLL_STATUS: 0x00000200,
        CLK_TUNE_CTRL_STATUS: 0x00000000,
        VEND_SPEC: 0x20007809,
        MMC_BOOT: 0x00000000,
        VEND_SPEC2: 0x00001006,
        TUNING_CTRL: 0x00212800,
    };

    /// Safe access to USDHC1
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
        let taken = USDHC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USDHC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USDHC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USDHC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USDHC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with USDHC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::USDHC1];

    /// The interrupts associated with USDHC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USDHC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USDHC1: *const RegisterBlock = 0x402c0000 as *const _;

/// The USDHC2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type USDHC2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for USDHC2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for USDHC2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USDHC2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USDHC2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USDHC2 {
    const INSTANCE: Self = Self {
        addr: 0x402c4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::USDHC2],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in USDHC2
    pub const reset: ResetValues = ResetValues {
        DS_ADDR: 0x00000000,
        BLK_ATT: 0x00000000,
        CMD_ARG: 0x00000000,
        CMD_XFR_TYP: 0x00000000,
        CMD_RSP0: 0x00000000,
        CMD_RSP1: 0x00000000,
        CMD_RSP2: 0x00000000,
        CMD_RSP3: 0x00000000,
        DATA_BUFF_ACC_PORT: 0x00000000,
        PRES_STATE: 0x00008080,
        PROT_CTRL: 0x08800020,
        SYS_CTRL: 0x0080800F,
        INT_STATUS: 0x00000000,
        INT_STATUS_EN: 0x00000000,
        INT_SIGNAL_EN: 0x00000000,
        AUTOCMD12_ERR_STATUS: 0x00000000,
        HOST_CTRL_CAP: 0x07F3B407,
        WTMK_LVL: 0x08100810,
        MIX_CTRL: 0x80000000,
        FORCE_EVENT: 0x00000000,
        ADMA_ERR_STATUS: 0x00000000,
        ADMA_SYS_ADDR: 0x00000000,
        DLL_CTRL: 0x00000000,
        DLL_STATUS: 0x00000200,
        CLK_TUNE_CTRL_STATUS: 0x00000000,
        VEND_SPEC: 0x20007809,
        MMC_BOOT: 0x00000000,
        VEND_SPEC2: 0x00001006,
        TUNING_CTRL: 0x00212800,
    };

    /// Safe access to USDHC2
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
        let taken = USDHC2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USDHC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USDHC2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USDHC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USDHC2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with USDHC2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::USDHC2];

    /// The interrupts associated with USDHC2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USDHC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USDHC2: *const RegisterBlock = 0x402c4000 as *const _;
