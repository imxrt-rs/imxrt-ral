#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Crossbar Switch

use crate::RWRegister;

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// Crossbar B Select Register 0
pub mod SEL0 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT0 (refer to Functional Description section for input/output assignment)
    pub mod SEL0 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT1 (refer to Functional Description section for input/output assignment)
    pub mod SEL1 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 1
pub mod SEL1 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)
    pub mod SEL2 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)
    pub mod SEL3 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 2
pub mod SEL2 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)
    pub mod SEL4 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)
    pub mod SEL5 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 3
pub mod SEL3 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)
    pub mod SEL6 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)
    pub mod SEL7 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 4
pub mod SEL4 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT8 (refer to Functional Description section for input/output assignment)
    pub mod SEL8 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT9 (refer to Functional Description section for input/output assignment)
    pub mod SEL9 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 5
pub mod SEL5 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)
    pub mod SEL10 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)
    pub mod SEL11 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 6
pub mod SEL6 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT12 (refer to Functional Description section for input/output assignment)
    pub mod SEL12 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT13 (refer to Functional Description section for input/output assignment)
    pub mod SEL13 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 7
pub mod SEL7 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)
    pub mod SEL14 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)
    pub mod SEL15 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Crossbar B Select Register 0
    pub SEL0: RWRegister<u16>,

    /// Crossbar B Select Register 1
    pub SEL1: RWRegister<u16>,

    /// Crossbar B Select Register 2
    pub SEL2: RWRegister<u16>,

    /// Crossbar B Select Register 3
    pub SEL3: RWRegister<u16>,

    /// Crossbar B Select Register 4
    pub SEL4: RWRegister<u16>,

    /// Crossbar B Select Register 5
    pub SEL5: RWRegister<u16>,

    /// Crossbar B Select Register 6
    pub SEL6: RWRegister<u16>,

    /// Crossbar B Select Register 7
    pub SEL7: RWRegister<u16>,
}
pub struct ResetValues {
    pub SEL0: u16,
    pub SEL1: u16,
    pub SEL2: u16,
    pub SEL3: u16,
    pub SEL4: u16,
    pub SEL5: u16,
    pub SEL6: u16,
    pub SEL7: u16,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance<const N: u8> {
    pub(crate) addr: u32,
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

pub(crate) mod private {
    pub trait Sealed {}
}

/// Describes a valid `Const<N>` for this peripheral instance.
pub trait Valid: private::Sealed {}

/// The XBARB peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type XBARB = Instance<0>;

/// The XBARB peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XBARB = Instance<0>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct XBARB {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for XBARB {}
#[cfg(not(feature = "nosync"))]
impl Valid for XBARB {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XBARB_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XBARB peripheral instance
#[cfg(not(feature = "nosync"))]
impl XBARB {
    const INSTANCE: Self = Self {
        addr: 0x403c0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in XBARB
    pub const reset: ResetValues = ResetValues {
        SEL0: 0x00000000,
        SEL1: 0x00000000,
        SEL2: 0x00000000,
        SEL3: 0x00000000,
        SEL4: 0x00000000,
        SEL5: 0x00000000,
        SEL6: 0x00000000,
        SEL7: 0x00000000,
    };

    /// Safe access to XBARB
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
        let taken = XBARB_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XBARB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XBARB_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XBARB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XBARB_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with XBARB
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with XBARB
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XBARB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XBARB: *const RegisterBlock = 0x403c0000 as *const _;
