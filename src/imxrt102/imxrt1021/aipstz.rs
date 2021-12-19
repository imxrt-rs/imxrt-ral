#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AIPSTZ Control Registers

use crate::RWRegister;

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// Master Priviledge Registers
pub mod MPR {

    /// Master 5 Priviledge, Buffer, Read, Write Control.
    pub mod MPROT5 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\[1\] access attribute.
            pub const MPL0: u32 = 0b0000;

            /// 0b0000: Accesses from this master are not forced to user-mode. The hprot\[1\] access attribute is used directly to determine ips_supervisor_access.
            pub const MPL1: u32 = 0b0000;
        }
    }

    /// Master 3 Priviledge, Buffer, Read, Write Control.
    pub mod MPROT3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MPROT5::RW;
    }

    /// Master 2 Priviledge, Buffer, Read, Write Control
    pub mod MPROT2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MPROT5::RW;
    }

    /// Master 1 Priviledge, Buffer, Read, Write Control
    pub mod MPROT1 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MPROT5::RW;
    }

    /// Master 0 Priviledge, Buffer, Read, Write Control
    pub mod MPROT0 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MPROT5::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR {

    /// Off-platform Peripheral Access Control 7
    pub mod OPAC7 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 6
    pub mod OPAC6 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 5
    pub mod OPAC5 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 4
    pub mod OPAC4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 3
    pub mod OPAC3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 2
    pub mod OPAC2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 1
    pub mod OPAC1 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 0
    pub mod OPAC0 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR1 {

    /// Off-platform Peripheral Access Control 15
    pub mod OPAC15 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 14
    pub mod OPAC14 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 13
    pub mod OPAC13 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 12
    pub mod OPAC12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 11
    pub mod OPAC11 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 10
    pub mod OPAC10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 9
    pub mod OPAC9 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 8
    pub mod OPAC8 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR2 {

    /// Off-platform Peripheral Access Control 23
    pub mod OPAC23 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 22
    pub mod OPAC22 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 21
    pub mod OPAC21 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 20
    pub mod OPAC20 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 19
    pub mod OPAC19 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 18
    pub mod OPAC18 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 17
    pub mod OPAC17 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 16
    pub mod OPAC16 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR3 {

    /// Off-platform Peripheral Access Control 31
    pub mod OPAC31 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 30
    pub mod OPAC30 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 29
    pub mod OPAC29 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 28
    pub mod OPAC28 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 27
    pub mod OPAC27 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 26
    pub mod OPAC26 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 25
    pub mod OPAC25 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 24
    pub mod OPAC24 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR4 {

    /// Off-platform Peripheral Access Control 33
    pub mod OPAC33 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 32
    pub mod OPAC32 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC33::RW;
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Master Priviledge Registers
    pub MPR: RWRegister<u32>,

    _reserved1: [u32; 15],

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR: RWRegister<u32>,

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR1: RWRegister<u32>,

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR2: RWRegister<u32>,

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR3: RWRegister<u32>,

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR4: RWRegister<u32>,
}
pub struct ResetValues {
    pub MPR: u32,
    pub OPACR: u32,
    pub OPACR1: u32,
    pub OPACR2: u32,
    pub OPACR3: u32,
    pub OPACR4: u32,
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

/// The AIPSTZ1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type AIPSTZ1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for AIPSTZ1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for AIPSTZ1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AIPSTZ1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AIPSTZ1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AIPSTZ1 {
    const INSTANCE: Self = Self {
        addr: 0x4007c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ1
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    /// Safe access to AIPSTZ1
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
        let taken = AIPSTZ1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AIPSTZ1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AIPSTZ1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with AIPSTZ1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ1: *const RegisterBlock = 0x4007c000 as *const _;

/// The AIPSTZ2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type AIPSTZ2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for AIPSTZ2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for AIPSTZ2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AIPSTZ2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AIPSTZ2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AIPSTZ2 {
    const INSTANCE: Self = Self {
        addr: 0x4017c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ2
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    /// Safe access to AIPSTZ2
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
        let taken = AIPSTZ2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AIPSTZ2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AIPSTZ2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with AIPSTZ2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ2: *const RegisterBlock = 0x4017c000 as *const _;

/// The AIPSTZ3 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type AIPSTZ3 = Instance<3>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for AIPSTZ3 {}
#[cfg(not(feature = "nosync"))]
impl Valid for AIPSTZ3 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AIPSTZ3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AIPSTZ3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AIPSTZ3 {
    const INSTANCE: Self = Self {
        addr: 0x4027c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ3
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    /// Safe access to AIPSTZ3
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
        let taken = AIPSTZ3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AIPSTZ3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AIPSTZ3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with AIPSTZ3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ3: *const RegisterBlock = 0x4027c000 as *const _;

/// The AIPSTZ4 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type AIPSTZ4 = Instance<4>;

#[cfg(not(feature = "nosync"))]
impl private::Sealed for AIPSTZ4 {}
#[cfg(not(feature = "nosync"))]
impl Valid for AIPSTZ4 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AIPSTZ4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AIPSTZ4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AIPSTZ4 {
    const INSTANCE: Self = Self {
        addr: 0x4037c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ4
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    /// Safe access to AIPSTZ4
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
        let taken = AIPSTZ4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AIPSTZ4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AIPSTZ4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with AIPSTZ4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ4: *const RegisterBlock = 0x4037c000 as *const _;
