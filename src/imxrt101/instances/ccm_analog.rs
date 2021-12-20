#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CCM_ANALOG
//!
//! Used by: imxrt1011, imxrt1015

#[cfg(not(feature = "nosync"))]
use crate::imxrt101::peripherals::ccm_analog::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::ccm_analog::{Instance, Valid};
pub use crate::imxrt101::peripherals::ccm_analog::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::ccm_analog::{
    MISC0, MISC0_CLR, MISC0_SET, MISC0_TOG, MISC1, MISC1_CLR, MISC1_SET, MISC1_TOG, MISC2,
    MISC2_CLR, MISC2_SET, MISC2_TOG, PFD_480, PFD_480_CLR, PFD_480_SET, PFD_480_TOG, PFD_528,
    PFD_528_CLR, PFD_528_SET, PFD_528_TOG, PLL_AUDIO, PLL_AUDIO_CLR, PLL_AUDIO_DENOM,
    PLL_AUDIO_NUM, PLL_AUDIO_SET, PLL_AUDIO_TOG, PLL_ENET, PLL_ENET_CLR, PLL_ENET_SET,
    PLL_ENET_TOG, PLL_SYS, PLL_SYS_CLR, PLL_SYS_DENOM, PLL_SYS_NUM, PLL_SYS_SET, PLL_SYS_SS,
    PLL_SYS_TOG, PLL_USB1, PLL_USB1_CLR, PLL_USB1_SET, PLL_USB1_TOG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CCM_ANALOG peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type CCM_ANALOG = Instance<0>;

/// The CCM_ANALOG peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CCM_ANALOG = Instance<0>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct CCM_ANALOG {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for CCM_ANALOG {}
#[cfg(not(feature = "nosync"))]
impl Valid for CCM_ANALOG {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CCM_ANALOG_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CCM_ANALOG peripheral instance
#[cfg(not(feature = "nosync"))]
impl CCM_ANALOG {
    const INSTANCE: Self = Self {
        addr: 0x400d8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in CCM_ANALOG
    pub const reset: ResetValues = ResetValues {
        PLL_USB1: 0x00012000,
        PLL_USB1_SET: 0x00012000,
        PLL_USB1_CLR: 0x00012000,
        PLL_USB1_TOG: 0x00012000,
        PLL_SYS: 0x00013001,
        PLL_SYS_SET: 0x00013001,
        PLL_SYS_CLR: 0x00013001,
        PLL_SYS_TOG: 0x00013001,
        PLL_SYS_SS: 0x00000000,
        PLL_SYS_NUM: 0x00000000,
        PLL_SYS_DENOM: 0x00000012,
        PLL_AUDIO: 0x00011006,
        PLL_AUDIO_SET: 0x00011006,
        PLL_AUDIO_CLR: 0x00011006,
        PLL_AUDIO_TOG: 0x00011006,
        PLL_AUDIO_NUM: 0x05F5E100,
        PLL_AUDIO_DENOM: 0x2964619C,
        PLL_ENET: 0x00011001,
        PLL_ENET_SET: 0x00011001,
        PLL_ENET_CLR: 0x00011001,
        PLL_ENET_TOG: 0x00011001,
        PFD_480: 0x1311100C,
        PFD_480_SET: 0x1311100C,
        PFD_480_CLR: 0x1311100C,
        PFD_480_TOG: 0x1311100C,
        PFD_528: 0x1018101B,
        PFD_528_SET: 0x1018101B,
        PFD_528_CLR: 0x1018101B,
        PFD_528_TOG: 0x1018101B,
        MISC0: 0x04000000,
        MISC0_SET: 0x04000000,
        MISC0_CLR: 0x04000000,
        MISC0_TOG: 0x04000000,
        MISC1: 0x00000000,
        MISC1_SET: 0x00000000,
        MISC1_CLR: 0x00000000,
        MISC1_TOG: 0x00000000,
        MISC2: 0x00272727,
        MISC2_SET: 0x00272727,
        MISC2_CLR: 0x00272727,
        MISC2_TOG: 0x00272727,
    };

    /// Safe access to CCM_ANALOG
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
        let taken = CCM_ANALOG_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CCM_ANALOG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CCM_ANALOG_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CCM_ANALOG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CCM_ANALOG_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with CCM_ANALOG
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with CCM_ANALOG
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CCM_ANALOG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CCM_ANALOG: *const RegisterBlock = 0x400d8000 as *const _;
