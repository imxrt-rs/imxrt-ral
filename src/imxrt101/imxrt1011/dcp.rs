#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DCP register reference index

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::dcp::Instance;
pub use crate::imxrt101::peripherals::dcp::{RegisterBlock, ResetValues};

pub use crate::imxrt101::peripherals::dcp::{
    CAPABILITY0, CAPABILITY1, CH0CMDPTR, CH0OPTS, CH0OPTS_CLR, CH0OPTS_SET, CH0OPTS_TOG, CH0SEMA,
    CH0STAT, CH0STAT_CLR, CH0STAT_SET, CH0STAT_TOG, CH1CMDPTR, CH1OPTS, CH1OPTS_CLR, CH1OPTS_SET,
    CH1OPTS_TOG, CH1SEMA, CH1STAT, CH1STAT_CLR, CH1STAT_SET, CH1STAT_TOG, CH2CMDPTR, CH2OPTS,
    CH2OPTS_CLR, CH2OPTS_SET, CH2OPTS_TOG, CH2SEMA, CH2STAT, CH2STAT_CLR, CH2STAT_SET, CH2STAT_TOG,
    CH3CMDPTR, CH3OPTS, CH3OPTS_CLR, CH3OPTS_SET, CH3OPTS_TOG, CH3SEMA, CH3STAT, CH3STAT_CLR,
    CH3STAT_SET, CH3STAT_TOG, CHANNELCTRL, CHANNELCTRL_CLR, CHANNELCTRL_SET, CHANNELCTRL_TOG,
    CONTEXT, CTRL, CTRL_CLR, CTRL_SET, CTRL_TOG, DBGDATA, DBGSELECT, KEY, KEYDATA, PACKET0,
    PACKET1, PACKET2, PACKET3, PACKET4, PACKET5, PACKET6, PAGETABLE, STAT, STAT_CLR, STAT_SET,
    STAT_TOG, VERSION,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DCP peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type DCP = Instance<0>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DCP_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DCP peripheral instance
#[cfg(not(feature = "nosync"))]
impl DCP {
    const INSTANCE: Self = Self {
        addr: 0x400f0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::DCP, crate::interrupt::DCP_VMI],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in DCP
    pub const reset: ResetValues = ResetValues {
        CTRL: 0xF0800000,
        CTRL_SET: 0xF0800000,
        CTRL_CLR: 0xF0800000,
        CTRL_TOG: 0xF0800000,
        STAT: 0x10000000,
        STAT_SET: 0x10000000,
        STAT_CLR: 0x10000000,
        STAT_TOG: 0x10000000,
        CHANNELCTRL: 0x00000000,
        CHANNELCTRL_SET: 0x00000000,
        CHANNELCTRL_CLR: 0x00000000,
        CHANNELCTRL_TOG: 0x00000000,
        CAPABILITY0: 0x00000404,
        CAPABILITY1: 0x00070001,
        CONTEXT: 0x00000000,
        KEY: 0x00000000,
        KEYDATA: 0x00000000,
        PACKET0: 0x00000000,
        PACKET1: 0x00000000,
        PACKET2: 0x00000000,
        PACKET3: 0x00000000,
        PACKET4: 0x00000000,
        PACKET5: 0x00000000,
        PACKET6: 0x00000000,
        CH0CMDPTR: 0x00000000,
        CH0SEMA: 0x00000000,
        CH0STAT: 0x00000000,
        CH0STAT_SET: 0x00000000,
        CH0STAT_CLR: 0x00000000,
        CH0STAT_TOG: 0x00000000,
        CH0OPTS: 0x00000000,
        CH0OPTS_SET: 0x00000000,
        CH0OPTS_CLR: 0x00000000,
        CH0OPTS_TOG: 0x00000000,
        CH1CMDPTR: 0x00000000,
        CH1SEMA: 0x00000000,
        CH1STAT: 0x00000000,
        CH1STAT_SET: 0x00000000,
        CH1STAT_CLR: 0x00000000,
        CH1STAT_TOG: 0x00000000,
        CH1OPTS: 0x00000000,
        CH1OPTS_SET: 0x00000000,
        CH1OPTS_CLR: 0x00000000,
        CH1OPTS_TOG: 0x00000000,
        CH2CMDPTR: 0x00000000,
        CH2SEMA: 0x00000000,
        CH2STAT: 0x00000000,
        CH2STAT_SET: 0x00000000,
        CH2STAT_CLR: 0x00000000,
        CH2STAT_TOG: 0x00000000,
        CH2OPTS: 0x00000000,
        CH2OPTS_SET: 0x00000000,
        CH2OPTS_CLR: 0x00000000,
        CH2OPTS_TOG: 0x00000000,
        CH3CMDPTR: 0x00000000,
        CH3SEMA: 0x00000000,
        CH3STAT: 0x00000000,
        CH3STAT_SET: 0x00000000,
        CH3STAT_CLR: 0x00000000,
        CH3STAT_TOG: 0x00000000,
        CH3OPTS: 0x00000000,
        CH3OPTS_SET: 0x00000000,
        CH3OPTS_CLR: 0x00000000,
        CH3OPTS_TOG: 0x00000000,
        DBGSELECT: 0x00000000,
        DBGDATA: 0x00000000,
        PAGETABLE: 0x00000000,
        VERSION: 0x02010000,
    };

    /// Safe access to DCP
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
        let taken = DCP_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DCP
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

        let taken = DCP_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DCP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DCP_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with DCP
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::DCP, crate::interrupt::DCP_VMI];

    /// The interrupts associated with DCP
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DCP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DCP: *const RegisterBlock = 0x400f0000 as *const _;
