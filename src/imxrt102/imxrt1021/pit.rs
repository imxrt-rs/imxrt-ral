#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PIT

use crate::{RORegister, RWRegister};

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// PIT Module Control Register
pub mod MCR {

    /// Freeze
    pub mod FRZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timers continue to run in Debug mode.
            pub const FRZ_0: u32 = 0b0;

            /// 0b1: Timers are stopped in Debug mode.
            pub const FRZ_1: u32 = 0b1;
        }
    }

    /// Module Disable - (PIT section)
    pub mod MDIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock for standard PIT timers is enabled.
            pub const MDIS_0: u32 = 0b0;

            /// 0b1: Clock for standard PIT timers is disabled.
            pub const MDIS_1: u32 = 0b1;
        }
    }
}

/// PIT Upper Lifetime Timer Register
pub mod LTMR64H {

    /// Life Timer value
    pub mod LTH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PIT Lower Lifetime Timer Register
pub mod LTMR64L {

    /// Life Timer value
    pub mod LTL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Load Value Register
pub mod LDVAL0 {

    /// Timer Start Value
    pub mod TSV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Current Timer Value Register
pub mod CVAL0 {

    /// Current Timer Value
    pub mod TVL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Control Register
pub mod TCTRL0 {

    /// Timer Enable
    pub mod TEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timer n is disabled.
            pub const TEN_0: u32 = 0b0;

            /// 0b1: Timer n is enabled.
            pub const TEN_1: u32 = 0b1;
        }
    }

    /// Timer Interrupt Enable
    pub mod TIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt requests from Timer n are disabled.
            pub const TIE_0: u32 = 0b0;

            /// 0b1: Interrupt will be requested whenever TIF is set.
            pub const TIE_1: u32 = 0b1;
        }
    }

    /// Chain Mode
    pub mod CHN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timer is not chained.
            pub const CHN_0: u32 = 0b0;

            /// 0b1: Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1.
            pub const CHN_1: u32 = 0b1;
        }
    }
}

/// Timer Flag Register
pub mod TFLG0 {

    /// Timer Interrupt Flag
    pub mod TIF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timeout has not yet occurred.
            pub const TIF_0: u32 = 0b0;

            /// 0b1: Timeout has occurred.
            pub const TIF_1: u32 = 0b1;
        }
    }
}

/// Timer Load Value Register
pub mod LDVAL1 {
    pub use super::LDVAL0::TSV;
}

/// Current Timer Value Register
pub mod CVAL1 {
    pub use super::CVAL0::TVL;
}

/// Timer Control Register
pub mod TCTRL1 {
    pub use super::TCTRL0::CHN;
    pub use super::TCTRL0::TEN;
    pub use super::TCTRL0::TIE;
}

/// Timer Flag Register
pub mod TFLG1 {
    pub use super::TFLG0::TIF;
}

/// Timer Load Value Register
pub mod LDVAL2 {
    pub use super::LDVAL0::TSV;
}

/// Current Timer Value Register
pub mod CVAL2 {
    pub use super::CVAL0::TVL;
}

/// Timer Control Register
pub mod TCTRL2 {
    pub use super::TCTRL0::CHN;
    pub use super::TCTRL0::TEN;
    pub use super::TCTRL0::TIE;
}

/// Timer Flag Register
pub mod TFLG2 {
    pub use super::TFLG0::TIF;
}

/// Timer Load Value Register
pub mod LDVAL3 {
    pub use super::LDVAL0::TSV;
}

/// Current Timer Value Register
pub mod CVAL3 {
    pub use super::CVAL0::TVL;
}

/// Timer Control Register
pub mod TCTRL3 {
    pub use super::TCTRL0::CHN;
    pub use super::TCTRL0::TEN;
    pub use super::TCTRL0::TIE;
}

/// Timer Flag Register
pub mod TFLG3 {
    pub use super::TFLG0::TIF;
}
#[repr(C)]
pub struct RegisterBlock {
    /// PIT Module Control Register
    pub MCR: RWRegister<u32>,

    _reserved1: [u32; 55],

    /// PIT Upper Lifetime Timer Register
    pub LTMR64H: RORegister<u32>,

    /// PIT Lower Lifetime Timer Register
    pub LTMR64L: RORegister<u32>,

    _reserved2: [u32; 6],

    /// Timer Load Value Register
    pub LDVAL0: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL0: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL0: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG0: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL1: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL1: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL1: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG1: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL2: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL2: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL2: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG2: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL3: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL3: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL3: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG3: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub LTMR64H: u32,
    pub LTMR64L: u32,
    pub LDVAL0: u32,
    pub CVAL0: u32,
    pub TCTRL0: u32,
    pub TFLG0: u32,
    pub LDVAL1: u32,
    pub CVAL1: u32,
    pub TCTRL1: u32,
    pub TFLG1: u32,
    pub LDVAL2: u32,
    pub CVAL2: u32,
    pub TCTRL2: u32,
    pub TFLG2: u32,
    pub LDVAL3: u32,
    pub CVAL3: u32,
    pub TCTRL3: u32,
    pub TFLG3: u32,
}
pub struct Instance<const N: u8> {
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) addr: u32,
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) intrs: &'static [crate::Interrupt],
}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> ::core::ops::Deref for Instance<N> {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

#[cfg(not(feature = "nosync"))]
unsafe impl<const N: u8> Send for Instance<N> {}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> Instance<N> {
    /// Return the interrupt signals associated with this
    /// peripheral instance
    ///
    /// Collection may be empty if there is no interrupt signal
    /// associated with the peripheral. There's no guarantee for
    /// interrupt signal ordering in the collection.
    #[inline(always)]
    pub const fn interrupts<'a>(&'a self) -> &'a [crate::Interrupt] {
        self.intrs
    }
}

/// The PIT peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PIT = Instance<0>;

/// The PIT peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PIT = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct PIT {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PIT {}
impl crate::Valid for PIT {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PIT_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PIT peripheral instance
#[cfg(not(feature = "nosync"))]
impl PIT {
    const INSTANCE: Self = Self {
        addr: 0x40084000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::PIT],
    };

    /// Reset values for each field in PIT
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00000002,
        LTMR64H: 0x00000000,
        LTMR64L: 0x00000000,
        LDVAL0: 0x00000000,
        CVAL0: 0x00000000,
        TCTRL0: 0x00000000,
        TFLG0: 0x00000000,
        LDVAL1: 0x00000000,
        CVAL1: 0x00000000,
        TCTRL1: 0x00000000,
        TFLG1: 0x00000000,
        LDVAL2: 0x00000000,
        CVAL2: 0x00000000,
        TCTRL2: 0x00000000,
        TFLG2: 0x00000000,
        LDVAL3: 0x00000000,
        CVAL3: 0x00000000,
        TCTRL3: 0x00000000,
        TFLG3: 0x00000000,
    };

    /// Safe access to PIT
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
        let taken = PIT_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PIT
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PIT_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PIT
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PIT_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PIT {
    /// The interrupts associated with PIT
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::PIT];

    /// The interrupts associated with PIT
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PIT
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PIT: *const RegisterBlock = 0x40084000 as *const _;
