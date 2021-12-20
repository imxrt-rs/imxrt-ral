#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CSI
//!
//! Used by: imxrt1062, imxrt1064

use crate::imxrt106::peripherals::csi::private;
pub use crate::imxrt106::peripherals::csi::{Instance, Valid};
pub use crate::imxrt106::peripherals::csi::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::csi::{
    CSICR1, CSICR18, CSICR19, CSICR2, CSICR3, CSIDMASA_FB1, CSIDMASA_FB2, CSIDMASA_STATFIFO,
    CSIDMATS_STATFIFO, CSIFBUF_PARA, CSIIMAG_PARA, CSIRFIFO, CSIRXCNT, CSISR, CSISTATFIFO,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CSI peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CSI = Instance<0>;

/// The CSI peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CSI = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct CSI {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl private::Sealed for CSI {}
impl Valid for CSI {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CSI_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CSI peripheral instance
#[cfg(not(feature = "nosync"))]
impl CSI {
    const INSTANCE: Self = Self {
        addr: 0x402bc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CSI],
    };

    /// Reset values for each field in CSI
    pub const reset: ResetValues = ResetValues {
        CSICR1: 0x40000800,
        CSICR2: 0x00000000,
        CSICR3: 0x00000000,
        CSISTATFIFO: 0x00000000,
        CSIRFIFO: 0x00000000,
        CSIRXCNT: 0x00009600,
        CSISR: 0x80004000,
        CSIDMASA_STATFIFO: 0x00000000,
        CSIDMATS_STATFIFO: 0x00000000,
        CSIDMASA_FB1: 0x00000000,
        CSIDMASA_FB2: 0x00000000,
        CSIFBUF_PARA: 0x00000000,
        CSIIMAG_PARA: 0x00000000,
        CSICR18: 0x0002D000,
        CSICR19: 0x00000000,
    };

    /// Safe access to CSI
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
        let taken = CSI_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CSI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CSI_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CSI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CSI_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CSI {
    /// The interrupts associated with CSI
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::CSI];

    /// The interrupts associated with CSI
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CSI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CSI: *const RegisterBlock = 0x402bc000 as *const _;
