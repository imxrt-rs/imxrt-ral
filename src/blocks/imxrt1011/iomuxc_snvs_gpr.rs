#[doc = "IOMUXC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPR0 General Purpose Register"]
    pub GPR0: crate::RORegister<u32>,
    #[doc = "GPR1 General Purpose Register"]
    pub GPR1: crate::RORegister<u32>,
    #[doc = "GPR2 General Purpose Register"]
    pub GPR2: crate::RORegister<u32>,
    #[doc = "GPR3 General Purpose Register"]
    pub GPR3: crate::RWRegister<u32>,
}
#[doc = "GPR3 General Purpose Register"]
pub mod GPR3 {
    #[doc = "Set to enable LPSR mode."]
    pub mod LPSR_MODE_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC captured status clear"]
    pub mod DCDC_STATUS_CAPT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "POR_B pad control"]
    pub mod POR_PULL_TYPE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC_IN low voltage detect."]
    pub mod DCDC_IN_LOW_VOL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC output over current alert"]
    pub mod DCDC_OVER_CUR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC output over voltage alert"]
    pub mod DCDC_OVER_VOL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC status OK"]
    pub mod DCDC_STS_DC_OK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
