#[doc = "CCM_ANALOG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Analog ARM PLL control Register"]
    pub PLL_ARM: crate::RWRegister<u32>,
    #[doc = "Analog ARM PLL control Register"]
    pub PLL_ARM_SET: crate::RWRegister<u32>,
    #[doc = "Analog ARM PLL control Register"]
    pub PLL_ARM_CLR: crate::RWRegister<u32>,
    #[doc = "Analog ARM PLL control Register"]
    pub PLL_ARM_TOG: crate::RWRegister<u32>,
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    pub PLL_USB1: crate::RWRegister<u32>,
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    pub PLL_USB1_SET: crate::RWRegister<u32>,
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    pub PLL_USB1_CLR: crate::RWRegister<u32>,
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    pub PLL_USB1_TOG: crate::RWRegister<u32>,
    #[doc = "Analog USB2 480MHz PLL Control Register"]
    pub PLL_USB2: crate::RWRegister<u32>,
    #[doc = "Analog USB2 480MHz PLL Control Register"]
    pub PLL_USB2_SET: crate::RWRegister<u32>,
    #[doc = "Analog USB2 480MHz PLL Control Register"]
    pub PLL_USB2_CLR: crate::RWRegister<u32>,
    #[doc = "Analog USB2 480MHz PLL Control Register"]
    pub PLL_USB2_TOG: crate::RWRegister<u32>,
    #[doc = "Analog System PLL Control Register"]
    pub PLL_SYS: crate::RWRegister<u32>,
    #[doc = "Analog System PLL Control Register"]
    pub PLL_SYS_SET: crate::RWRegister<u32>,
    #[doc = "Analog System PLL Control Register"]
    pub PLL_SYS_CLR: crate::RWRegister<u32>,
    #[doc = "Analog System PLL Control Register"]
    pub PLL_SYS_TOG: crate::RWRegister<u32>,
    #[doc = "528MHz System PLL Spread Spectrum Register"]
    pub PLL_SYS_SS: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
    pub PLL_SYS_NUM: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
    pub PLL_SYS_DENOM: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "Analog Audio PLL control Register"]
    pub PLL_AUDIO: crate::RWRegister<u32>,
    #[doc = "Analog Audio PLL control Register"]
    pub PLL_AUDIO_SET: crate::RWRegister<u32>,
    #[doc = "Analog Audio PLL control Register"]
    pub PLL_AUDIO_CLR: crate::RWRegister<u32>,
    #[doc = "Analog Audio PLL control Register"]
    pub PLL_AUDIO_TOG: crate::RWRegister<u32>,
    #[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
    pub PLL_AUDIO_NUM: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
    pub PLL_AUDIO_DENOM: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Analog Video PLL control Register"]
    pub PLL_VIDEO: crate::RWRegister<u32>,
    #[doc = "Analog Video PLL control Register"]
    pub PLL_VIDEO_SET: crate::RWRegister<u32>,
    #[doc = "Analog Video PLL control Register"]
    pub PLL_VIDEO_CLR: crate::RWRegister<u32>,
    #[doc = "Analog Video PLL control Register"]
    pub PLL_VIDEO_TOG: crate::RWRegister<u32>,
    #[doc = "Numerator of Video PLL Fractional Loop Divider Register"]
    pub PLL_VIDEO_NUM: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Denominator of Video PLL Fractional Loop Divider Register"]
    pub PLL_VIDEO_DENOM: crate::RWRegister<u32>,
    _reserved6: [u8; 0x1c],
    #[doc = "Analog ENET PLL Control Register"]
    pub PLL_ENET: crate::RWRegister<u32>,
    #[doc = "Analog ENET PLL Control Register"]
    pub PLL_ENET_SET: crate::RWRegister<u32>,
    #[doc = "Analog ENET PLL Control Register"]
    pub PLL_ENET_CLR: crate::RWRegister<u32>,
    #[doc = "Analog ENET PLL Control Register"]
    pub PLL_ENET_TOG: crate::RWRegister<u32>,
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub PFD_480: crate::RWRegister<u32>,
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub PFD_480_SET: crate::RWRegister<u32>,
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub PFD_480_CLR: crate::RWRegister<u32>,
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub PFD_480_TOG: crate::RWRegister<u32>,
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub PFD_528: crate::RWRegister<u32>,
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub PFD_528_SET: crate::RWRegister<u32>,
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub PFD_528_CLR: crate::RWRegister<u32>,
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub PFD_528_TOG: crate::RWRegister<u32>,
    _reserved7: [u8; 0x40],
    #[doc = "Miscellaneous Register 0"]
    pub MISC0: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_SET: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_CLR: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_TOG: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 1"]
    pub MISC1: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 1"]
    pub MISC1_SET: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 1"]
    pub MISC1_CLR: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 1"]
    pub MISC1_TOG: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 2"]
    pub MISC2: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 2"]
    pub MISC2_SET: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 2"]
    pub MISC2_CLR: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 2"]
    pub MISC2_TOG: crate::RWRegister<u32>,
}
#[doc = "Analog ARM PLL control Register"]
pub mod PLL_ARM {
    #[doc = "This field controls the PLL loop divider"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source"]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved"]
    pub mod PLL_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog ARM PLL control Register"]
pub mod PLL_ARM_SET {
    #[doc = "This field controls the PLL loop divider"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source"]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved"]
    pub mod PLL_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog ARM PLL control Register"]
pub mod PLL_ARM_CLR {
    #[doc = "This field controls the PLL loop divider"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source"]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved"]
    pub mod PLL_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog ARM PLL control Register"]
pub mod PLL_ARM_TOG {
    #[doc = "This field controls the PLL loop divider"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source"]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved"]
    pub mod PLL_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod PLL_USB1 {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL outputs for USBPHYn off."]
            pub const EN_USB_CLKS_0: u32 = 0;
            #[doc = "PLL outputs for USBPHYn on."]
            pub const EN_USB_CLKS_1: u32 = 0x01;
        }
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod PLL_USB1_SET {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL outputs for USBPHYn off."]
            pub const EN_USB_CLKS_0: u32 = 0;
            #[doc = "PLL outputs for USBPHYn on."]
            pub const EN_USB_CLKS_1: u32 = 0x01;
        }
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod PLL_USB1_CLR {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL outputs for USBPHYn off."]
            pub const EN_USB_CLKS_0: u32 = 0;
            #[doc = "PLL outputs for USBPHYn on."]
            pub const EN_USB_CLKS_1: u32 = 0x01;
        }
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod PLL_USB1_TOG {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL outputs for USBPHYn off."]
            pub const EN_USB_CLKS_0: u32 = 0;
            #[doc = "PLL outputs for USBPHYn on."]
            pub const EN_USB_CLKS_1: u32 = 0x01;
        }
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod PLL_USB2 {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod PLL_USB2_SET {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod PLL_USB2_CLR {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod PLL_USB2_TOG {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog System PLL Control Register"]
pub mod PLL_SYS {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog System PLL Control Register"]
pub mod PLL_SYS_SET {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog System PLL Control Register"]
pub mod PLL_SYS_CLR {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog System PLL Control Register"]
pub mod PLL_SYS_TOG {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "528MHz System PLL Spread Spectrum Register"]
pub mod PLL_SYS_SS {
    #[doc = "Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable bit"]
    pub mod ENABLE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Spread spectrum modulation disabled"]
            pub const ENABLE_0: u32 = 0;
            #[doc = "Soread spectrum modulation enabled"]
            pub const ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    pub mod STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod PLL_SYS_NUM {
    #[doc = "30 bit numerator (A) of fractional loop divider (signed integer)."]
    pub mod A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod PLL_SYS_DENOM {
    #[doc = "30 bit denominator (B) of fractional loop divider (unsigned integer)."]
    pub mod B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Audio PLL control Register"]
pub mod PLL_AUDIO {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Audio PLL control Register"]
pub mod PLL_AUDIO_SET {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Audio PLL control Register"]
pub mod PLL_AUDIO_CLR {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Audio PLL control Register"]
pub mod PLL_AUDIO_TOG {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
pub mod PLL_AUDIO_NUM {
    #[doc = "30 bit numerator of fractional loop divider."]
    pub mod A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
pub mod PLL_AUDIO_DENOM {
    #[doc = "30 bit denominator of fractional loop divider."]
    pub mod B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Video PLL control Register"]
pub mod PLL_VIDEO {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enalbe PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Video PLL control Register"]
pub mod PLL_VIDEO_SET {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enalbe PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Video PLL control Register"]
pub mod PLL_VIDEO_CLR {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enalbe PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Video PLL control Register"]
pub mod PLL_VIDEO_TOG {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enalbe PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Numerator of Video PLL Fractional Loop Divider Register"]
pub mod PLL_VIDEO_NUM {
    #[doc = "30 bit numerator of fractional loop divider(Signed number), absolute value should be less than denominator"]
    pub mod A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Denominator of Video PLL Fractional Loop Divider Register"]
pub mod PLL_VIDEO_DENOM {
    #[doc = "30 bit Denominator of fractional loop divider."]
    pub mod B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog ENET PLL Control Register"]
pub mod PLL_ENET {
    #[doc = "Controls the frequency of the ethernet reference clock"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the ethernet clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    pub mod ENET_25M_REF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog ENET PLL Control Register"]
pub mod PLL_ENET_SET {
    #[doc = "Controls the frequency of the ethernet reference clock"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the ethernet clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    pub mod ENET_25M_REF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog ENET PLL Control Register"]
pub mod PLL_ENET_CLR {
    #[doc = "Controls the frequency of the ethernet reference clock"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the ethernet clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    pub mod ENET_25M_REF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog ENET PLL Control Register"]
pub mod PLL_ENET_TOG {
    #[doc = "Controls the frequency of the ethernet reference clock"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the ethernet clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
            #[doc = "Select the CLK1_N / CLK1_P as source."]
            pub const CLK1: u32 = 0x01;
        }
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables an offset in the phase frequency detector."]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    pub mod ENET_25M_REF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod PFD_480 {
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod PFD_480_SET {
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod PFD_480_CLR {
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod PFD_480_TOG {
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod PFD_528 {
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod PFD_528_SET {
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod PFD_528_CLR {
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod PFD_528_TOG {
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0 {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure the analog behavior in stop mode."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All analog except RTC powered down on stop mode assertion."]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_SET {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure the analog behavior in stop mode."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All analog except RTC powered down on stop mode assertion."]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_CLR {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure the analog behavior in stop mode."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All analog except RTC powered down on stop mode assertion."]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_TOG {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure the analog behavior in stop mode."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All analog except RTC powered down on stop mode assertion."]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1 {
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    pub mod LVDS1_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Arm PLL"]
            pub const ARM_PLL: u32 = 0;
            #[doc = "System PLL"]
            pub const SYS_PLL: u32 = 0x01;
            #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
            pub const PFD4: u32 = 0x02;
            #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
            pub const PFD5: u32 = 0x03;
            #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
            pub const PFD6: u32 = 0x04;
            #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
            pub const PFD7: u32 = 0x05;
            #[doc = "Audio PLL"]
            pub const AUDIO_PLL: u32 = 0x06;
            #[doc = "Video PLL"]
            pub const VIDEO_PLL: u32 = 0x07;
            #[doc = "ethernet ref clock (ENET_PLL)"]
            pub const ETHERNET_REF: u32 = 0x09;
            #[doc = "USB1 PLL clock"]
            pub const USB1_PLL: u32 = 0x0c;
            #[doc = "USB2 PLL clock"]
            pub const USB2_PLL: u32 = 0x0d;
            #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
            pub const PFD0: u32 = 0x0e;
            #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
            pub const PFD1: u32 = 0x0f;
            #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
            pub const PFD2: u32 = 0x10;
            #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
            pub const PFD3: u32 = 0x11;
            #[doc = "xtal (24M)"]
            pub const XTAL: u32 = 0x12;
        }
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    pub mod LVDSCLK1_OBEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    pub mod LVDSCLK1_IBEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    pub mod PFD_480_AUTOGATE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    pub mod PFD_528_AUTOGATE_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    pub mod IRQ_TEMPPANIC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    pub mod IRQ_TEMPLOW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    pub mod IRQ_TEMPHIGH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    pub mod IRQ_ANA_BO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    pub mod IRQ_DIG_BO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1_SET {
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    pub mod LVDS1_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Arm PLL"]
            pub const ARM_PLL: u32 = 0;
            #[doc = "System PLL"]
            pub const SYS_PLL: u32 = 0x01;
            #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
            pub const PFD4: u32 = 0x02;
            #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
            pub const PFD5: u32 = 0x03;
            #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
            pub const PFD6: u32 = 0x04;
            #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
            pub const PFD7: u32 = 0x05;
            #[doc = "Audio PLL"]
            pub const AUDIO_PLL: u32 = 0x06;
            #[doc = "Video PLL"]
            pub const VIDEO_PLL: u32 = 0x07;
            #[doc = "ethernet ref clock (ENET_PLL)"]
            pub const ETHERNET_REF: u32 = 0x09;
            #[doc = "USB1 PLL clock"]
            pub const USB1_PLL: u32 = 0x0c;
            #[doc = "USB2 PLL clock"]
            pub const USB2_PLL: u32 = 0x0d;
            #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
            pub const PFD0: u32 = 0x0e;
            #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
            pub const PFD1: u32 = 0x0f;
            #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
            pub const PFD2: u32 = 0x10;
            #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
            pub const PFD3: u32 = 0x11;
            #[doc = "xtal (24M)"]
            pub const XTAL: u32 = 0x12;
        }
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    pub mod LVDSCLK1_OBEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    pub mod LVDSCLK1_IBEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    pub mod PFD_480_AUTOGATE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    pub mod PFD_528_AUTOGATE_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    pub mod IRQ_TEMPPANIC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    pub mod IRQ_TEMPLOW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    pub mod IRQ_TEMPHIGH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    pub mod IRQ_ANA_BO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    pub mod IRQ_DIG_BO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1_CLR {
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    pub mod LVDS1_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Arm PLL"]
            pub const ARM_PLL: u32 = 0;
            #[doc = "System PLL"]
            pub const SYS_PLL: u32 = 0x01;
            #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
            pub const PFD4: u32 = 0x02;
            #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
            pub const PFD5: u32 = 0x03;
            #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
            pub const PFD6: u32 = 0x04;
            #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
            pub const PFD7: u32 = 0x05;
            #[doc = "Audio PLL"]
            pub const AUDIO_PLL: u32 = 0x06;
            #[doc = "Video PLL"]
            pub const VIDEO_PLL: u32 = 0x07;
            #[doc = "ethernet ref clock (ENET_PLL)"]
            pub const ETHERNET_REF: u32 = 0x09;
            #[doc = "USB1 PLL clock"]
            pub const USB1_PLL: u32 = 0x0c;
            #[doc = "USB2 PLL clock"]
            pub const USB2_PLL: u32 = 0x0d;
            #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
            pub const PFD0: u32 = 0x0e;
            #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
            pub const PFD1: u32 = 0x0f;
            #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
            pub const PFD2: u32 = 0x10;
            #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
            pub const PFD3: u32 = 0x11;
            #[doc = "xtal (24M)"]
            pub const XTAL: u32 = 0x12;
        }
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    pub mod LVDSCLK1_OBEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    pub mod LVDSCLK1_IBEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    pub mod PFD_480_AUTOGATE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    pub mod PFD_528_AUTOGATE_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    pub mod IRQ_TEMPPANIC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    pub mod IRQ_TEMPLOW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    pub mod IRQ_TEMPHIGH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    pub mod IRQ_ANA_BO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    pub mod IRQ_DIG_BO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1_TOG {
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    pub mod LVDS1_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Arm PLL"]
            pub const ARM_PLL: u32 = 0;
            #[doc = "System PLL"]
            pub const SYS_PLL: u32 = 0x01;
            #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
            pub const PFD4: u32 = 0x02;
            #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
            pub const PFD5: u32 = 0x03;
            #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
            pub const PFD6: u32 = 0x04;
            #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
            pub const PFD7: u32 = 0x05;
            #[doc = "Audio PLL"]
            pub const AUDIO_PLL: u32 = 0x06;
            #[doc = "Video PLL"]
            pub const VIDEO_PLL: u32 = 0x07;
            #[doc = "ethernet ref clock (ENET_PLL)"]
            pub const ETHERNET_REF: u32 = 0x09;
            #[doc = "USB1 PLL clock"]
            pub const USB1_PLL: u32 = 0x0c;
            #[doc = "USB2 PLL clock"]
            pub const USB2_PLL: u32 = 0x0d;
            #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
            pub const PFD0: u32 = 0x0e;
            #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
            pub const PFD1: u32 = 0x0f;
            #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
            pub const PFD2: u32 = 0x10;
            #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
            pub const PFD3: u32 = 0x11;
            #[doc = "xtal (24M)"]
            pub const XTAL: u32 = 0x12;
        }
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    pub mod LVDSCLK1_OBEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    pub mod LVDSCLK1_IBEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    pub mod PFD_480_AUTOGATE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    pub mod PFD_528_AUTOGATE_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    pub mod IRQ_TEMPPANIC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    pub mod IRQ_TEMPLOW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    pub mod IRQ_TEMPHIGH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    pub mod IRQ_ANA_BO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    pub mod IRQ_DIG_BO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Register 2"]
pub mod MISC2 {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    pub mod REG0_BO_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG0_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG0_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_BO_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG0_BO_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
            pub const PLL3_DISABLE_0: u32 = 0;
            #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
            pub const PLL3_DISABLE_1: u32 = 0x01;
        }
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG1_BO_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG1_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG1_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_BO_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG1_BO_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_OK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_LSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_LSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_LSB_1: u32 = 0x01;
        }
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG2_BO_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG2_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG2_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_ENABLE_BO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    pub mod REG2_OK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_MSB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_MSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_MSB_1: u32 = 0x01;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_STEP_TIME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_STEP_TIME {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_STEP_TIME {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Post-divider for video"]
    pub mod VIDEO_DIV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const VIDEO_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const VIDEO_DIV_1: u32 = 0x01;
            #[doc = "divide by 1"]
            pub const VIDEO_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const VIDEO_DIV_3: u32 = 0x03;
        }
    }
}
#[doc = "Miscellaneous Register 2"]
pub mod MISC2_SET {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    pub mod REG0_BO_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG0_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG0_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_BO_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG0_BO_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
            pub const PLL3_DISABLE_0: u32 = 0;
            #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
            pub const PLL3_DISABLE_1: u32 = 0x01;
        }
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG1_BO_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG1_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG1_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_BO_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG1_BO_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_OK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_LSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_LSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_LSB_1: u32 = 0x01;
        }
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG2_BO_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG2_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG2_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_ENABLE_BO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    pub mod REG2_OK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_MSB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_MSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_MSB_1: u32 = 0x01;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_STEP_TIME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_STEP_TIME {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_STEP_TIME {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Post-divider for video"]
    pub mod VIDEO_DIV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const VIDEO_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const VIDEO_DIV_1: u32 = 0x01;
            #[doc = "divide by 1"]
            pub const VIDEO_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const VIDEO_DIV_3: u32 = 0x03;
        }
    }
}
#[doc = "Miscellaneous Register 2"]
pub mod MISC2_CLR {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    pub mod REG0_BO_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG0_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG0_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_BO_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG0_BO_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
            pub const PLL3_DISABLE_0: u32 = 0;
            #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
            pub const PLL3_DISABLE_1: u32 = 0x01;
        }
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG1_BO_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG1_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG1_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_BO_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG1_BO_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_OK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_LSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_LSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_LSB_1: u32 = 0x01;
        }
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG2_BO_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG2_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG2_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_ENABLE_BO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    pub mod REG2_OK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_MSB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_MSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_MSB_1: u32 = 0x01;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_STEP_TIME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_STEP_TIME {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_STEP_TIME {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Post-divider for video"]
    pub mod VIDEO_DIV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const VIDEO_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const VIDEO_DIV_1: u32 = 0x01;
            #[doc = "divide by 1"]
            pub const VIDEO_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const VIDEO_DIV_3: u32 = 0x03;
        }
    }
}
#[doc = "Miscellaneous Register 2"]
pub mod MISC2_TOG {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    pub mod REG0_BO_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG0_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG0_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_BO_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG0_BO_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
            pub const PLL3_DISABLE_0: u32 = 0;
            #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
            pub const PLL3_DISABLE_1: u32 = 0x01;
        }
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG1_BO_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG1_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG1_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_BO_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG1_BO_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_OK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_LSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_LSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_LSB_1: u32 = 0x01;
        }
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG2_BO_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG2_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG2_BO_OFFSET_7: u32 = 0x07;
        }
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_ENABLE_BO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    pub mod REG2_OK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_MSB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_MSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_MSB_1: u32 = 0x01;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_STEP_TIME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_STEP_TIME {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_STEP_TIME {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
    }
    #[doc = "Post-divider for video"]
    pub mod VIDEO_DIV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divide by 1 (Default)"]
            pub const VIDEO_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const VIDEO_DIV_1: u32 = 0x01;
            #[doc = "divide by 1"]
            pub const VIDEO_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const VIDEO_DIV_3: u32 = 0x03;
        }
    }
}
