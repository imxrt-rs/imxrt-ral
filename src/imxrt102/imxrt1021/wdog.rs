#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WDOG

use crate::{RORegister, RWRegister};

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// Watchdog Control Register
pub mod WCR {

    /// WDZST
    pub mod WDZST {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Continue timer operation (Default).
            pub const WDZST_0: u16 = 0b0;

            /// 0b1: Suspend the watchdog timer.
            pub const WDZST_1: u16 = 0b1;
        }
    }

    /// WDBG
    pub mod WDBG {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Continue WDOG timer operation (Default).
            pub const WDBG_0: u16 = 0b0;

            /// 0b1: Suspend the watchdog timer.
            pub const WDBG_1: u16 = 0b1;
        }
    }

    /// WDE
    pub mod WDE {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable the Watchdog (Default).
            pub const WDE_0: u16 = 0b0;

            /// 0b1: Enable the Watchdog.
            pub const WDE_1: u16 = 0b1;
        }
    }

    /// WDT
    pub mod WDT {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect on WDOG_B (Default).
            pub const WDT_0: u16 = 0b0;

            /// 0b1: Assert WDOG_B upon a Watchdog Time-out event.
            pub const WDT_1: u16 = 0b1;
        }
    }

    /// SRS
    pub mod SRS {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Assert system reset signal.
            pub const SRS_0: u16 = 0b0;

            /// 0b1: No effect on the system (Default).
            pub const SRS_1: u16 = 0b1;
        }
    }

    /// WDA
    pub mod WDA {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Assert WDOG_B output.
            pub const WDA_0: u16 = 0b0;

            /// 0b1: No effect on system (Default).
            pub const WDA_1: u16 = 0b1;
        }
    }

    /// software reset extension, an option way to generate software reset
    pub mod SRE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: using original way to generate software reset (default)
            pub const SRE_0: u16 = 0b0;

            /// 0b1: using new way to generate software reset.
            pub const SRE_1: u16 = 0b1;
        }
    }

    /// WDW
    pub mod WDW {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Continue WDOG timer operation (Default).
            pub const WDW_0: u16 = 0b0;

            /// 0b1: Suspend WDOG timer operation.
            pub const WDW_1: u16 = 0b1;
        }
    }

    /// WT
    pub mod WT {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: - 0.5 Seconds (Default).
            pub const WT_0: u16 = 0b00000000;

            /// 0b00000001: - 1.0 Seconds.
            pub const WT_1: u16 = 0b00000001;

            /// 0b00000010: - 1.5 Seconds.
            pub const WT_2: u16 = 0b00000010;

            /// 0b00000011: - 2.0 Seconds.
            pub const WT_3: u16 = 0b00000011;

            /// 0b11111111: - 128 Seconds.
            pub const WT_255: u16 = 0b11111111;
        }
    }
}

/// Watchdog Service Register
pub mod WSR {

    /// WSR
    pub mod WSR {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0101010101010101: Write to the Watchdog Service Register (WDOG_WSR).
            pub const WSR_21845: u16 = 0b0101010101010101;

            /// 0b1010101010101010: Write to the Watchdog Service Register (WDOG_WSR).
            pub const WSR_43690: u16 = 0b1010101010101010;
        }
    }
}

/// Watchdog Reset Status Register
pub mod WRSR {

    /// SFTW
    pub mod SFTW {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not the result of a software reset.
            pub const SFTW_0: u16 = 0b0;

            /// 0b1: Reset is the result of a software reset.
            pub const SFTW_1: u16 = 0b1;
        }
    }

    /// TOUT
    pub mod TOUT {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not the result of a WDOG timeout.
            pub const TOUT_0: u16 = 0b0;

            /// 0b1: Reset is the result of a WDOG timeout.
            pub const TOUT_1: u16 = 0b1;
        }
    }

    /// POR
    pub mod POR {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not the result of a power on reset.
            pub const POR_0: u16 = 0b0;

            /// 0b1: Reset is the result of a power on reset.
            pub const POR_1: u16 = 0b1;
        }
    }
}

/// Watchdog Interrupt Control Register
pub mod WICR {

    /// WICT
    pub mod WICT {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: WICT\[7:0\] = Time duration between interrupt and time-out is 0 seconds.
            pub const WICT_0: u16 = 0b00000000;

            /// 0b00000001: WICT\[7:0\] = Time duration between interrupt and time-out is 0.5 seconds.
            pub const WICT_1: u16 = 0b00000001;

            /// 0b00000100: WICT\[7:0\] = Time duration between interrupt and time-out is 2 seconds (Default).
            pub const WICT_4: u16 = 0b00000100;

            /// 0b11111111: WICT\[7:0\] = Time duration between interrupt and time-out is 127.5 seconds.
            pub const WICT_255: u16 = 0b11111111;
        }
    }

    /// WTIS
    pub mod WTIS {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No interrupt has occurred (Default).
            pub const WTIS_0: u16 = 0b0;

            /// 0b1: Interrupt has occurred
            pub const WTIS_1: u16 = 0b1;
        }
    }

    /// WIE
    pub mod WIE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable Interrupt (Default).
            pub const WIE_0: u16 = 0b0;

            /// 0b1: Enable Interrupt.
            pub const WIE_1: u16 = 0b1;
        }
    }
}

/// Watchdog Miscellaneous Control Register
pub mod WMCR {

    /// PDE
    pub mod PDE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Power Down Counter of WDOG is disabled.
            pub const PDE_0: u16 = 0b0;

            /// 0b1: Power Down Counter of WDOG is enabled (Default).
            pub const PDE_1: u16 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Watchdog Control Register
    pub WCR: RWRegister<u16>,

    /// Watchdog Service Register
    pub WSR: RWRegister<u16>,

    /// Watchdog Reset Status Register
    pub WRSR: RORegister<u16>,

    /// Watchdog Interrupt Control Register
    pub WICR: RWRegister<u16>,

    /// Watchdog Miscellaneous Control Register
    pub WMCR: RWRegister<u16>,
}
pub struct ResetValues {
    pub WCR: u16,
    pub WSR: u16,
    pub WRSR: u16,
    pub WICR: u16,
    pub WMCR: u16,
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

/// The WDOG1 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type WDOG1 = Instance<1>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static WDOG1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the WDOG1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl WDOG1 {
    const INSTANCE: Self = Self {
        addr: 0x400b8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::WDOG1],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in WDOG1
    pub const reset: ResetValues = ResetValues {
        WCR: 0x00000030,
        WSR: 0x00000000,
        WRSR: 0x00000000,
        WICR: 0x00000004,
        WMCR: 0x00000001,
    };

    /// Safe access to WDOG1
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
        let taken = WDOG1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to WDOG1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = WDOG1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal WDOG1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        WDOG1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with WDOG1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::WDOG1];

    /// The interrupts associated with WDOG1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to WDOG1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WDOG1: *const RegisterBlock = 0x400b8000 as *const _;

/// The WDOG2 peripheral instance.
#[cfg(not(feature = "nosync"))]
pub type WDOG2 = Instance<2>;

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static WDOG2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the WDOG2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl WDOG2 {
    const INSTANCE: Self = Self {
        addr: 0x400d0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::WDOG2],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in WDOG2
    pub const reset: ResetValues = ResetValues {
        WCR: 0x00000030,
        WSR: 0x00000000,
        WRSR: 0x00000000,
        WICR: 0x00000004,
        WMCR: 0x00000001,
    };

    /// Safe access to WDOG2
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
        let taken = WDOG2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to WDOG2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = WDOG2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal WDOG2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        WDOG2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with WDOG2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::WDOG2];

    /// The interrupts associated with WDOG2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to WDOG2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WDOG2: *const RegisterBlock = 0x400d0000 as *const _;
