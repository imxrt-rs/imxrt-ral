#[doc = "NETC privileged"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "NETC reset register"]
    pub NETCRR: crate::RWRegister<u32>,
    #[doc = "NETC status register"]
    pub NETCSR: crate::RORegister<u32>,
    _reserved1: [u8; 0x0100],
    #[doc = "Memory Error Injection Config Register"]
    pub MEICR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0bf4],
    #[doc = "Correctable memory error configuration register"]
    pub CMECR: crate::RWRegister<u32>,
    #[doc = "Correctable memory error status register"]
    pub CMESR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Correctable memory error count register"]
    pub CMECTR: crate::RORegister<u32>,
    _reserved4: [u8; 0x20],
    #[doc = "Uncorrectable non-fatal memory error configuration register"]
    pub UNMECR: crate::RWRegister<u32>,
    #[doc = "Uncorrectable non-fatal memory error status register 0"]
    pub UNMESR0: crate::RWRegister<u32>,
    #[doc = "Uncorrectable non-fatal memory error status register 1"]
    pub UNMESR1: crate::RORegister<u32>,
    #[doc = "Uncorrectable non-fatal memory error count register"]
    pub UNMECTR: crate::RORegister<u32>,
    #[doc = "Uncorrectable fatal memory error configuration register"]
    pub UFMECR: crate::RWRegister<u32>,
    #[doc = "Uncorrectable fatal memory error status register 0"]
    pub UFMESR0: crate::RWRegister<u32>,
    #[doc = "Uncorrectable fatal memory error status register 1"]
    pub UFMESR1: crate::RORegister<u32>,
}
#[doc = "NETC reset register"]
pub mod NETCRR {
    #[doc = "Soft reset"]
    pub mod SR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NETC status register"]
pub mod NETCSR {
    #[doc = "Error"]
    pub mod ERROR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates NETC's global operational state"]
    pub mod STATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Error Injection Config Register"]
pub mod MEICR {
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Armed"]
    pub mod ARM {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable"]
    pub mod EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Correctable memory error configuration register"]
pub mod CMECR {
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Correctable memory error status register"]
pub mod CMESR {
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Single-bit ECC error"]
    pub mod SBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Correctable memory error count register"]
pub mod CMECTR {
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal memory error configuration register"]
pub mod UNMECR {
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal memory error status register 0"]
pub mod UNMESR0 {
    #[doc = "Syndrome"]
    pub mod SYNDROME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multi-bit ECC error"]
    pub mod MBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal memory error status register 1"]
pub mod UNMESR1 {
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal memory error count register"]
pub mod UNMECTR {
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable fatal memory error configuration register"]
pub mod UFMECR {
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable fatal memory error status register 0"]
pub mod UFMESR0 {
    #[doc = "Syndrome"]
    pub mod SYNDROME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple"]
    pub mod M {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multi-bit ECC error"]
    pub mod MBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable fatal memory error status register 1"]
pub mod UFMESR1 {
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
