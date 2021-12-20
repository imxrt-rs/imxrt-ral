#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIO

use crate::{RORegister, RWRegister, WORegister};

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// GPIO data register
pub mod DR {

    /// DR
    pub mod DR {
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

/// GPIO direction register
pub mod GDIR {

    /// GDIR
    pub mod GDIR {
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

/// GPIO pad status register
pub mod PSR {

    /// PSR
    pub mod PSR {
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

/// GPIO interrupt configuration register1
pub mod ICR1 {

    /// ICR0
    pub mod ICR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt n is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt n is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt n is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt n is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// ICR1
    pub mod ICR1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR2
    pub mod ICR2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR3
    pub mod ICR3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR4
    pub mod ICR4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR5
    pub mod ICR5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR6
    pub mod ICR6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR7
    pub mod ICR7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR8
    pub mod ICR8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR9
    pub mod ICR9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR10
    pub mod ICR10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR11
    pub mod ICR11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR12
    pub mod ICR12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR13
    pub mod ICR13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR14
    pub mod ICR14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }

    /// ICR15
    pub mod ICR15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR0::RW;
    }
}

/// GPIO interrupt configuration register2
pub mod ICR2 {

    /// ICR16
    pub mod ICR16 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt n is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt n is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt n is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt n is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// ICR17
    pub mod ICR17 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR18
    pub mod ICR18 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR19
    pub mod ICR19 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR20
    pub mod ICR20 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR21
    pub mod ICR21 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR22
    pub mod ICR22 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR23
    pub mod ICR23 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR24
    pub mod ICR24 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR25
    pub mod ICR25 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR26
    pub mod ICR26 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR27
    pub mod ICR27 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR28
    pub mod ICR28 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR29
    pub mod ICR29 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR30
    pub mod ICR30 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }

    /// ICR31
    pub mod ICR31 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ICR16::RW;
    }
}

/// GPIO interrupt mask register
pub mod IMR {

    /// IMR
    pub mod IMR {
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

/// GPIO interrupt status register
pub mod ISR {

    /// ISR
    pub mod ISR {
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

/// GPIO edge select register
pub mod EDGE_SEL {

    /// GPIO_EDGE_SEL
    pub mod GPIO_EDGE_SEL {
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

/// GPIO data register SET
pub mod DR_SET {

    /// DR_SET
    pub mod DR_SET {
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

/// GPIO data register CLEAR
pub mod DR_CLEAR {

    /// DR_CLEAR
    pub mod DR_CLEAR {
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

/// GPIO data register TOGGLE
pub mod DR_TOGGLE {

    /// DR_TOGGLE
    pub mod DR_TOGGLE {
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
#[repr(C)]
pub struct RegisterBlock {
    /// GPIO data register
    pub DR: RWRegister<u32>,

    /// GPIO direction register
    pub GDIR: RWRegister<u32>,

    /// GPIO pad status register
    pub PSR: RORegister<u32>,

    /// GPIO interrupt configuration register1
    pub ICR1: RWRegister<u32>,

    /// GPIO interrupt configuration register2
    pub ICR2: RWRegister<u32>,

    /// GPIO interrupt mask register
    pub IMR: RWRegister<u32>,

    /// GPIO interrupt status register
    pub ISR: RWRegister<u32>,

    /// GPIO edge select register
    pub EDGE_SEL: RWRegister<u32>,

    _reserved1: [u32; 25],

    /// GPIO data register SET
    pub DR_SET: WORegister<u32>,

    /// GPIO data register CLEAR
    pub DR_CLEAR: WORegister<u32>,

    /// GPIO data register TOGGLE
    pub DR_TOGGLE: WORegister<u32>,
}
pub struct ResetValues {
    pub DR: u32,
    pub GDIR: u32,
    pub PSR: u32,
    pub ICR1: u32,
    pub ICR2: u32,
    pub IMR: u32,
    pub ISR: u32,
    pub EDGE_SEL: u32,
    pub DR_SET: u32,
    pub DR_CLEAR: u32,
    pub DR_TOGGLE: u32,
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

/// The GPIO1 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO1 = Instance<1>;

/// The GPIO1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO1 = Instance<1>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO1 {
    const INSTANCE: Self = Self {
        addr: 0x401b8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO1_INT0,
            crate::interrupt::GPIO1_INT1,
            crate::interrupt::GPIO1_INT2,
            crate::interrupt::GPIO1_INT3,
            crate::interrupt::GPIO1_INT4,
            crate::interrupt::GPIO1_INT5,
            crate::interrupt::GPIO1_INT6,
            crate::interrupt::GPIO1_INT7,
            crate::interrupt::GPIO1_Combined_0_15,
            crate::interrupt::GPIO1_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO1
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO1
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
        let taken = GPIO1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 10] = [
        crate::interrupt::GPIO1_INT0,
        crate::interrupt::GPIO1_INT1,
        crate::interrupt::GPIO1_INT2,
        crate::interrupt::GPIO1_INT3,
        crate::interrupt::GPIO1_INT4,
        crate::interrupt::GPIO1_INT5,
        crate::interrupt::GPIO1_INT6,
        crate::interrupt::GPIO1_INT7,
        crate::interrupt::GPIO1_Combined_0_15,
        crate::interrupt::GPIO1_Combined_16_31,
    ];

    /// The interrupts associated with GPIO1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO1: *const RegisterBlock = 0x401b8000 as *const _;

/// The GPIO2 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO2 = Instance<2>;

/// The GPIO2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO2 = Instance<2>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO2 {
    const INSTANCE: Self = Self {
        addr: 0x401bc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO2_Combined_0_15,
            crate::interrupt::GPIO2_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO2
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO2
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
        let taken = GPIO2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO2_Combined_0_15,
        crate::interrupt::GPIO2_Combined_16_31,
    ];

    /// The interrupts associated with GPIO2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO2: *const RegisterBlock = 0x401bc000 as *const _;

/// The GPIO3 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO3 = Instance<3>;

/// The GPIO3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO3 = Instance<3>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO3 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO3 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO3 {
    const INSTANCE: Self = Self {
        addr: 0x401c0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO3_Combined_0_15,
            crate::interrupt::GPIO3_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO3
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO3
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
        let taken = GPIO3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO3_Combined_0_15,
        crate::interrupt::GPIO3_Combined_16_31,
    ];

    /// The interrupts associated with GPIO3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO3: *const RegisterBlock = 0x401c0000 as *const _;

/// The GPIO5 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO5 = Instance<5>;

/// The GPIO5 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO5 = Instance<5>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO5 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO5 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO5 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO5_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO5 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO5 {
    const INSTANCE: Self = Self {
        addr: 0x400c0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO5_Combined_0_15,
            crate::interrupt::GPIO5_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO5
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO5
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
        let taken = GPIO5_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO5_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO5_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO5
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO5_Combined_0_15,
        crate::interrupt::GPIO5_Combined_16_31,
    ];

    /// The interrupts associated with GPIO5
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO5: *const RegisterBlock = 0x400c0000 as *const _;
