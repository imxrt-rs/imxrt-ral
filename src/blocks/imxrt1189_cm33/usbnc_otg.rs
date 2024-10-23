#[doc = "USBNC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "USB OTG Control 1 Register"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "USB OTG Control 2 Register"]
    pub CTRL2: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "USB Host HSIC Control Register"]
    pub HSIC_CTRL: crate::RWRegister<u32>,
}
#[doc = "USB OTG Control 1 Register"]
pub mod CTRL1 {
    #[doc = "OVER_CUR_DIS"]
    pub mod OVER_CUR_DIS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables overcurrent detection"]
            pub const OVRCRNT_DETCT_EN: u32 = 0;
            #[doc = "Disables overcurrent detection"]
            pub const OVRCRNT_DETCT_DIS: u32 = 0x01;
        }
    }
    #[doc = "OVER_CUR_POL"]
    pub mod OVER_CUR_POL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "High active (high on this signal represents an overcurrent condition)"]
            pub const ACTIVE_HI_OVRCRNT: u32 = 0;
            #[doc = "Low active (low on this signal represents an overcurrent condition)"]
            pub const ACTIVE_LOW_OVRCRNT: u32 = 0x01;
        }
    }
    #[doc = "PWR_POL"]
    pub mod PWR_POL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMIC Power Pin is Low active."]
            pub const ACTIVE_LO_PMIC: u32 = 0;
            #[doc = "PMIC Power Pin is High active."]
            pub const ACTIVE_HI_PMIC: u32 = 0x01;
        }
    }
    #[doc = "WIE"]
    pub mod WIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt Disabled"]
            pub const INT_DIS: u32 = 0;
            #[doc = "Interrupt Enabled"]
            pub const INT_EN: u32 = 0x01;
        }
    }
    #[doc = "WKUP_SW_EN"]
    pub mod WKUP_SW_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const SW_WKUP_DIS: u32 = 0;
            #[doc = "Enable"]
            pub const SW_WKUP_EN: u32 = 0x01;
        }
    }
    #[doc = "WKUP_SW"]
    pub mod WKUP_SW {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inactive"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Force wake-up"]
            pub const FORCE_WKUP: u32 = 0x01;
        }
    }
    #[doc = "WKUP_ID_EN"]
    pub mod WKUP_ID_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WKUP_ID_DIS: u32 = 0;
            #[doc = "Enable"]
            pub const WKUP_ID_EN: u32 = 0x01;
        }
    }
    #[doc = "WKUP_VBUS_EN"]
    pub mod WKUP_VBUS_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WKUP_VBUS_DIS: u32 = 0;
            #[doc = "Enable"]
            pub const WKUP_VBUS_EN: u32 = 0x01;
        }
    }
    #[doc = "Wake-up on DP/DM change enable"]
    pub mod WKUP_DPDM_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DP/DM changes wake-up to be disabled only when VBUS is 0."]
            pub const DPDM_WKUP_DIS: u32 = 0;
            #[doc = "(Default) DP/DM changes wake-up to be enabled, it is for device only."]
            pub const DPDM_WKUP_EN: u32 = 0x01;
        }
    }
    #[doc = "WIR"]
    pub mod WIR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No wake-up interrupt request received"]
            pub const NO_WKUP_REQ: u32 = 0;
            #[doc = "Wake-up Interrupt Request received"]
            pub const WKUP_REQ: u32 = 0x01;
        }
    }
}
#[doc = "USB OTG Control 2 Register"]
pub mod CTRL2 {
    #[doc = "VBUS_SOURCE_SEL"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "vbus_valid"]
            pub const VBUS_VALID: u32 = 0;
            #[doc = "sess_valid"]
            pub const SESS_VALID_1: u32 = 0x01;
            #[doc = "sess_valid"]
            pub const SESS_VALID_2: u32 = 0x02;
            #[doc = "sess_valid"]
            pub const SESS_VALID_3: u32 = 0x03;
        }
    }
    #[doc = "Auto Resume Enable"]
    pub mod AUTURESUME_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default"]
            pub const DEFAULT: u32 = 0;
        }
    }
    #[doc = "LOWSPEED_EN"]
    pub mod LOWSPEED_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default"]
            pub const DEFAULT: u32 = 0;
        }
    }
    #[doc = "Short Packet Interrupt"]
    pub mod SHORT_PKT_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default"]
            pub const DEFAULT: u32 = 0;
        }
    }
    #[doc = "UTMI_CLK_VLD"]
    pub mod UTMI_CLK_VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default"]
            pub const DEFAULT: u32 = 0;
        }
    }
}
#[doc = "USB Host HSIC Control Register"]
pub mod HSIC_CTRL {
    #[doc = "HSIC_CLK_ON"]
    pub mod HSIC_CLK_ON {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inactive"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "HSIC_EN"]
    pub mod HSIC_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CLK_VLD"]
    pub mod CLK_VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Invalid"]
            pub const INVALID: u32 = 0;
            #[doc = "Valid"]
            pub const VALID: u32 = 0x01;
        }
    }
}
