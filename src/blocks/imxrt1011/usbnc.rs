#[doc = "USB"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "USB OTG1 Control Register"]
    pub USB_OTG1_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x14],
    #[doc = "OTG1 UTMI PHY Control 0 Register"]
    pub USB_OTG1_PHY_CTRL_0: crate::RWRegister<u32>,
}
#[doc = "USB OTG1 Control Register"]
pub mod USB_OTG1_CTRL {
    #[doc = "Disable OTG1 Overcurrent Detection"]
    pub mod OVER_CUR_DIS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables overcurrent detection"]
            pub const OVER_CUR_DIS_0: u32 = 0;
            #[doc = "Disables overcurrent detection"]
            pub const OVER_CUR_DIS_1: u32 = 0x01;
        }
    }
    #[doc = "OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    pub mod OVER_CUR_POL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "High active (high on this signal represents an overcurrent condition)"]
            pub const OVER_CUR_POL_0: u32 = 0;
            #[doc = "Low active (low on this signal represents an overcurrent condition)"]
            pub const OVER_CUR_POL_1: u32 = 0x01;
        }
    }
    #[doc = "OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    pub mod PWR_POL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMIC Power Pin is Low active."]
            pub const PWR_POL_0: u32 = 0;
            #[doc = "PMIC Power Pin is High active."]
            pub const PWR_POL_1: u32 = 0x01;
        }
    }
    #[doc = "OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    pub mod WIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt Disabled"]
            pub const WIE_0: u32 = 0;
            #[doc = "Interrupt Enabled"]
            pub const WIE_1: u32 = 0x01;
        }
    }
    #[doc = "OTG1 Software Wake-up Enable"]
    pub mod WKUP_SW_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WKUP_SW_EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const WKUP_SW_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OTG1 Software Wake-up"]
    pub mod WKUP_SW {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inactive"]
            pub const WKUP_SW_0: u32 = 0;
            #[doc = "Force wake-up"]
            pub const WKUP_SW_1: u32 = 0x01;
        }
    }
    #[doc = "OTG1 Wake-up on ID change enable"]
    pub mod WKUP_ID_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WKUP_ID_EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const WKUP_ID_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OTG1 wake-up on VBUS change enable"]
    pub mod WKUP_VBUS_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WKUP_VBUS_EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const WKUP_VBUS_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Wake-up on DPDM change enable"]
    pub mod WKUP_DPDM_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0."]
            pub const WKUP_DPDM_EN_0: u32 = 0;
            #[doc = "(Default) DPDM changes wake-up to be enabled, it is for device only."]
            pub const WKUP_DPDM_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port"]
    pub mod WIR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No wake-up interrupt request received"]
            pub const WIR_0: u32 = 0;
            #[doc = "Wake-up Interrupt Request received"]
            pub const WIR_1: u32 = 0x01;
        }
    }
}
#[doc = "OTG1 UTMI PHY Control 0 Register"]
pub mod USB_OTG1_PHY_CTRL_0 {
    #[doc = "Indicating whether OTG1 UTMI PHY clock is valid"]
    pub mod UTMI_CLK_VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Invalid"]
            pub const UTMI_CLK_VLD_0: u32 = 0;
            #[doc = "Valid"]
            pub const UTMI_CLK_VLD_1: u32 = 0x01;
        }
    }
}
