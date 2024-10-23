#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0: crate::RWRegister<u32>,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_SET: crate::RWRegister<u32>,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_CLR: crate::RWRegister<u32>,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_TOG: crate::RWRegister<u32>,
    _reserved0: [u8; 0x40],
    #[doc = "Analog Status Register STAT0"]
    pub STAT0: crate::RORegister<u32>,
    #[doc = "Analog Status Register STAT0"]
    pub STAT0_SET: crate::RORegister<u32>,
    #[doc = "Analog Status Register STAT0"]
    pub STAT0_CLR: crate::RORegister<u32>,
    #[doc = "Analog Status Register STAT0"]
    pub STAT0_TOG: crate::RORegister<u32>,
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0 {
    #[doc = "LinrReg master enable"]
    pub mod LINREG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg power-up load disable"]
    pub mod LINREG_PWRUPLOAD_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal pull-down enabled"]
            pub const LINREG_PWRUPLOAD_DIS_0: u32 = 0;
            #[doc = "Internal pull-down disabled"]
            pub const LINREG_PWRUPLOAD_DIS_1: u32 = 0x01;
        }
    }
    #[doc = "LinReg current-limit enable"]
    pub mod LINREG_ILIMIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg output voltage target setting"]
    pub mod LINREG_OUTPUT_TRG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Isolation control for attached PHY load"]
    pub mod LINREG_PHY_ISO_B {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_SET {
    #[doc = "LinrReg master enable"]
    pub mod LINREG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg power-up load disable"]
    pub mod LINREG_PWRUPLOAD_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg current-limit enable"]
    pub mod LINREG_ILIMIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg output voltage target setting"]
    pub mod LINREG_OUTPUT_TRG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Isolation control for attached PHY load"]
    pub mod LINREG_PHY_ISO_B {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_CLR {
    #[doc = "LinrReg master enable"]
    pub mod LINREG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg power-up load disable"]
    pub mod LINREG_PWRUPLOAD_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg current-limit enable"]
    pub mod LINREG_ILIMIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg output voltage target setting"]
    pub mod LINREG_OUTPUT_TRG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Isolation control for attached PHY load"]
    pub mod LINREG_PHY_ISO_B {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_TOG {
    #[doc = "LinrReg master enable"]
    pub mod LINREG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg power-up load disable"]
    pub mod LINREG_PWRUPLOAD_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg current-limit enable"]
    pub mod LINREG_ILIMIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LinReg output voltage target setting"]
    pub mod LINREG_OUTPUT_TRG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Isolation control for attached PHY load"]
    pub mod LINREG_PHY_ISO_B {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0 {
    #[doc = "LinReg Status Bits"]
    pub mod LINREG_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_SET {
    #[doc = "LinReg Status Bits"]
    pub mod LINREG_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_CLR {
    #[doc = "LinReg Status Bits"]
    pub mod LINREG_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_TOG {
    #[doc = "LinReg Status Bits"]
    pub mod LINREG_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
