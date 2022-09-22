#[doc = "CAU"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Status Register"]
    pub CASR: crate::RWRegister<u32>,
    #[doc = "Accumulator"]
    pub CAA: crate::RWRegister<u32>,
    #[doc = "General Purpose Register"]
    pub CA: [crate::RWRegister<u32>; 9usize],
}
#[doc = "Status Register"]
pub mod CASR {
    #[doc = "Illegal Command"]
    pub mod IC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No illegal commands issued."]
            pub const IC_0: u32 = 0;
            #[doc = "Illegal command issued."]
            pub const IC_1: u32 = 0x01;
        }
    }
    #[doc = "DES Parity Error"]
    pub mod DPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error detected."]
            pub const DPE_0: u32 = 0;
            #[doc = "DES key parity error detected."]
            pub const DPE_1: u32 = 0x01;
        }
    }
    #[doc = "CAU Version"]
    pub mod VER {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initial CAU version."]
            pub const VER_1: u32 = 0x01;
            #[doc = "Second version, added support for SHA-256 algorithm (This is the value on this device)."]
            pub const VER_2: u32 = 0x02;
        }
    }
}
#[doc = "Accumulator"]
pub mod CAA {
    #[doc = "Accumulator"]
    pub mod ACC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Register"]
pub mod CA {
    #[doc = "General Purpose Registers"]
    pub mod CAN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
