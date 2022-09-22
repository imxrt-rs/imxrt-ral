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
    #[doc = "Master power-down for bandgap module"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    pub mod REFTOP_LINREGREF_PWD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    pub mod REFTOP_PWDVBGUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low-power control bit"]
    pub mod REFTOP_LOWPOWER {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bandgap self-bias control bit"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_SET {
    #[doc = "Master power-down for bandgap module"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    pub mod REFTOP_LINREGREF_PWD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    pub mod REFTOP_PWDVBGUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low-power control bit"]
    pub mod REFTOP_LOWPOWER {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bandgap self-bias control bit"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_CLR {
    #[doc = "Master power-down for bandgap module"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    pub mod REFTOP_LINREGREF_PWD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    pub mod REFTOP_PWDVBGUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low-power control bit"]
    pub mod REFTOP_LOWPOWER {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bandgap self-bias control bit"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_TOG {
    #[doc = "Master power-down for bandgap module"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-down for bandgap voltage-reference buffer"]
    pub mod REFTOP_LINREGREF_PWD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-down VBGUP detector in bandgap"]
    pub mod REFTOP_PWDVBGUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low-power control bit"]
    pub mod REFTOP_LOWPOWER {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bandgap self-bias control bit"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0 {
    #[doc = "Brief description here"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD1_PORB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD2_PORB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD3_PORB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_SET {
    #[doc = "Brief description here"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD1_PORB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD2_PORB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD3_PORB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_CLR {
    #[doc = "Brief description here"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD1_PORB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD2_PORB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD3_PORB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_TOG {
    #[doc = "Brief description here"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD1_PORB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD2_PORB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Brief description here"]
    pub mod VDD3_PORB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
