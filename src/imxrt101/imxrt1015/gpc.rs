#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC

use crate::{RORegister, RWRegister};

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// GPC Interface control register
pub mod CNTR {

    /// MEGA domain power down request
    pub mod MEGA_PDN_REQ {
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

            /// 0b0: No Request
            pub const MEGA_PDN_REQ_0: u32 = 0b0;

            /// 0b1: Request power down sequence
            pub const MEGA_PDN_REQ_1: u32 = 0b1;
        }
    }

    /// MEGA domain power up request
    pub mod MEGA_PUP_REQ {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Request
            pub const MEGA_PUP_REQ_0: u32 = 0b0;

            /// 0b1: Request power up sequence
            pub const MEGA_PUP_REQ_1: u32 = 0b1;
        }
    }

    /// FlexRAM PDRAM0 Power Gate Enable
    pub mod PDRAM0_PGE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FlexRAM PDRAM0 domain will keep power on even if CPU core is power down.
            pub const PDRAM0_PGE_0: u32 = 0b0;

            /// 0b1: FlexRAM PDRAM0 domain will be power down once when CPU core is power down.
            pub const PDRAM0_PGE_1: u32 = 0b1;
        }
    }
}

/// IRQ masking register 1
pub mod IMR1 {

    /// IRQ\[31:0\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR1 {
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

/// IRQ masking register 2
pub mod IMR2 {

    /// IRQ\[63:32\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR2 {
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

/// IRQ masking register 3
pub mod IMR3 {

    /// IRQ\[95:64\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR3 {
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

/// IRQ masking register 4
pub mod IMR4 {

    /// IRQ\[127:96\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR4 {
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

/// IRQ status resister 1
pub mod ISR1 {

    /// IRQ\[31:0\] status, read only
    pub mod ISR1 {
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

/// IRQ status resister 2
pub mod ISR2 {

    /// IRQ\[63:32\] status, read only
    pub mod ISR2 {
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

/// IRQ status resister 3
pub mod ISR3 {

    /// IRQ\[95:64\] status, read only
    pub mod ISR3 {
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

/// IRQ status resister 4
pub mod ISR4 {

    /// IRQ\[127:96\] status, read only
    pub mod ISR4 {
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

/// IRQ masking register 5
pub mod IMR5 {

    /// IRQ\[159:128\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR5 {
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

/// IRQ status resister 5
pub mod ISR5 {
    pub use super::ISR4::ISR4;
}
#[repr(C)]
pub struct RegisterBlock {
    /// GPC Interface control register
    pub CNTR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// IRQ masking register 1
    pub IMR1: RWRegister<u32>,

    /// IRQ masking register 2
    pub IMR2: RWRegister<u32>,

    /// IRQ masking register 3
    pub IMR3: RWRegister<u32>,

    /// IRQ masking register 4
    pub IMR4: RWRegister<u32>,

    /// IRQ status resister 1
    pub ISR1: RORegister<u32>,

    /// IRQ status resister 2
    pub ISR2: RORegister<u32>,

    /// IRQ status resister 3
    pub ISR3: RORegister<u32>,

    /// IRQ status resister 4
    pub ISR4: RORegister<u32>,

    _reserved2: [u32; 3],

    /// IRQ masking register 5
    pub IMR5: RWRegister<u32>,

    /// IRQ status resister 5
    pub ISR5: RORegister<u32>,
}
pub struct ResetValues {
    pub CNTR: u32,
    pub IMR1: u32,
    pub IMR2: u32,
    pub IMR3: u32,
    pub IMR4: u32,
    pub ISR1: u32,
    pub ISR2: u32,
    pub ISR3: u32,
    pub ISR4: u32,
    pub IMR5: u32,
    pub ISR5: u32,
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

/// The GPC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPC = Instance<0>;

/// The GPC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct GPC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPC {}
impl crate::Valid for GPC {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPC peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPC {
    const INSTANCE: Self = Self {
        addr: 0x400f4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPC],
    };

    /// Reset values for each field in GPC
    pub const reset: ResetValues = ResetValues {
        CNTR: 0x00520000,
        IMR1: 0x00000000,
        IMR2: 0x00000000,
        IMR3: 0x00000000,
        IMR4: 0x00000000,
        ISR1: 0x00000000,
        ISR2: 0x00000000,
        ISR3: 0x00000000,
        ISR4: 0x00000000,
        IMR5: 0x00000000,
        ISR5: 0x00000000,
    };

    /// Safe access to GPC
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
        let taken = GPC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPC {
    /// The interrupts associated with GPC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPC];

    /// The interrupts associated with GPC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPC: *const RegisterBlock = 0x400f4000 as *const _;
