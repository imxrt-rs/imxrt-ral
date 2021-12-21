#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! EWM
//!
//! Used by: imxrt1011, imxrt1015

use crate::RWRegister;

/// Control Register
pub mod CTRL {

    /// EWM enable.
    pub mod EWMEN {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EWM_in's Assertion State Select.
    pub mod ASSIN {
        /// Offset (1 bits)
        pub const offset: u8 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input Enable.
    pub mod INEN {
        /// Offset (2 bits)
        pub const offset: u8 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt Enable.
    pub mod INTEN {
        /// Offset (3 bits)
        pub const offset: u8 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Service Register
pub mod SERV {

    /// SERVICE
    pub mod SERVICE {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Compare Low Register
pub mod CMPL {

    /// COMPAREL
    pub mod COMPAREL {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Compare High Register
pub mod CMPH {

    /// COMPAREH
    pub mod COMPAREH {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock Control Register
pub mod CLKCTRL {

    /// CLKSEL
    pub mod CLKSEL {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock Prescaler Register
pub mod CLKPRESCALER {

    /// CLK_DIV
    pub mod CLK_DIV {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
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
    /// Control Register
    pub CTRL: RWRegister<u8>,

    /// Service Register
    pub SERV: RWRegister<u8>,

    /// Compare Low Register
    pub CMPL: RWRegister<u8>,

    /// Compare High Register
    pub CMPH: RWRegister<u8>,

    /// Clock Control Register
    pub CLKCTRL: RWRegister<u8>,

    /// Clock Prescaler Register
    pub CLKPRESCALER: RWRegister<u8>,
}
pub struct ResetValues {
    pub CTRL: u8,
    pub SERV: u8,
    pub CMPL: u8,
    pub CMPH: u8,
    pub CLKCTRL: u8,
    pub CLKPRESCALER: u8,
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
