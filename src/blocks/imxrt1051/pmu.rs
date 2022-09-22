#[doc = "PMU"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0110],
    #[doc = "Regulator 1P1 Register"]
    pub REG_1P1: crate::RWRegister<u32>,
    #[doc = "Regulator 1P1 Register"]
    pub REG_1P1_SET: crate::RWRegister<u32>,
    #[doc = "Regulator 1P1 Register"]
    pub REG_1P1_CLR: crate::RWRegister<u32>,
    #[doc = "Regulator 1P1 Register"]
    pub REG_1P1_TOG: crate::RWRegister<u32>,
    #[doc = "Regulator 3P0 Register"]
    pub REG_3P0: crate::RWRegister<u32>,
    #[doc = "Regulator 3P0 Register"]
    pub REG_3P0_SET: crate::RWRegister<u32>,
    #[doc = "Regulator 3P0 Register"]
    pub REG_3P0_CLR: crate::RWRegister<u32>,
    #[doc = "Regulator 3P0 Register"]
    pub REG_3P0_TOG: crate::RWRegister<u32>,
    #[doc = "Regulator 2P5 Register"]
    pub REG_2P5: crate::RWRegister<u32>,
    #[doc = "Regulator 2P5 Register"]
    pub REG_2P5_SET: crate::RWRegister<u32>,
    #[doc = "Regulator 2P5 Register"]
    pub REG_2P5_CLR: crate::RWRegister<u32>,
    #[doc = "Regulator 2P5 Register"]
    pub REG_2P5_TOG: crate::RWRegister<u32>,
    #[doc = "Digital Regulator Core Register"]
    pub REG_CORE: crate::RWRegister<u32>,
    #[doc = "Digital Regulator Core Register"]
    pub REG_CORE_SET: crate::RWRegister<u32>,
    #[doc = "Digital Regulator Core Register"]
    pub REG_CORE_CLR: crate::RWRegister<u32>,
    #[doc = "Digital Regulator Core Register"]
    pub REG_CORE_TOG: crate::RWRegister<u32>,
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
    #[doc = "Miscellaneous Control Register"]
    pub MISC2: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Control Register"]
    pub MISC2_SET: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Control Register"]
    pub MISC2_CLR: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Control Register"]
    pub MISC2_TOG: crate::RWRegister<u32>,
}
#[doc = "Regulator 1P1 Register"]
pub mod REG_1P1 {
    #[doc = "Control bit to enable the regulator output."]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    pub mod ENABLE_PULLDOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.8V"]
            pub const OUTPUT_TRG_4: u32 = 0x04;
            #[doc = "1.1V"]
            pub const OUTPUT_TRG_16: u32 = 0x10;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD1P1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD1P1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the weak 1p1 regulator"]
    pub mod ENABLE_WEAK_LINREG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    pub mod SELREF_WEAK_LINREG {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
            pub const SELREF_WEAK_LINREG_0: u32 = 0;
            #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
            pub const SELREF_WEAK_LINREG_1: u32 = 0x01;
        }
    }
}
#[doc = "Regulator 1P1 Register"]
pub mod REG_1P1_SET {
    #[doc = "Control bit to enable the regulator output."]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    pub mod ENABLE_PULLDOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.8V"]
            pub const OUTPUT_TRG_4: u32 = 0x04;
            #[doc = "1.1V"]
            pub const OUTPUT_TRG_16: u32 = 0x10;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD1P1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD1P1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the weak 1p1 regulator"]
    pub mod ENABLE_WEAK_LINREG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    pub mod SELREF_WEAK_LINREG {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
            pub const SELREF_WEAK_LINREG_0: u32 = 0;
            #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
            pub const SELREF_WEAK_LINREG_1: u32 = 0x01;
        }
    }
}
#[doc = "Regulator 1P1 Register"]
pub mod REG_1P1_CLR {
    #[doc = "Control bit to enable the regulator output."]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    pub mod ENABLE_PULLDOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.8V"]
            pub const OUTPUT_TRG_4: u32 = 0x04;
            #[doc = "1.1V"]
            pub const OUTPUT_TRG_16: u32 = 0x10;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD1P1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD1P1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the weak 1p1 regulator"]
    pub mod ENABLE_WEAK_LINREG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    pub mod SELREF_WEAK_LINREG {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
            pub const SELREF_WEAK_LINREG_0: u32 = 0;
            #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
            pub const SELREF_WEAK_LINREG_1: u32 = 0x01;
        }
    }
}
#[doc = "Regulator 1P1 Register"]
pub mod REG_1P1_TOG {
    #[doc = "Control bit to enable the regulator output."]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    pub mod ENABLE_PULLDOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.8V"]
            pub const OUTPUT_TRG_4: u32 = 0x04;
            #[doc = "1.1V"]
            pub const OUTPUT_TRG_16: u32 = 0x10;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD1P1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD1P1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the weak 1p1 regulator"]
    pub mod ENABLE_WEAK_LINREG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    pub mod SELREF_WEAK_LINREG {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
            pub const SELREF_WEAK_LINREG_0: u32 = 0;
            #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
            pub const SELREF_WEAK_LINREG_1: u32 = 0x01;
        }
    }
}
#[doc = "Regulator 3P0 Register"]
pub mod REG_3P0 {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    pub mod VBUS_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Utilize VBUS OTG2 power"]
            pub const USB_OTG2_VBUS: u32 = 0;
            #[doc = "Utilize VBUS OTG1 power"]
            pub const USB_OTG1_VBUS: u32 = 0x01;
        }
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.625V"]
            pub const OUTPUT_TRG_0: u32 = 0;
            #[doc = "3.000V"]
            pub const OUTPUT_TRG_15: u32 = 0x0f;
            #[doc = "3.400V"]
            pub const OUTPUT_TRG_31: u32 = 0x1f;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD3P0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD3P0 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Regulator 3P0 Register"]
pub mod REG_3P0_SET {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    pub mod VBUS_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Utilize VBUS OTG2 power"]
            pub const USB_OTG2_VBUS: u32 = 0;
            #[doc = "Utilize VBUS OTG1 power"]
            pub const USB_OTG1_VBUS: u32 = 0x01;
        }
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.625V"]
            pub const OUTPUT_TRG_0: u32 = 0;
            #[doc = "3.000V"]
            pub const OUTPUT_TRG_15: u32 = 0x0f;
            #[doc = "3.400V"]
            pub const OUTPUT_TRG_31: u32 = 0x1f;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD3P0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD3P0 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Regulator 3P0 Register"]
pub mod REG_3P0_CLR {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    pub mod VBUS_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Utilize VBUS OTG2 power"]
            pub const USB_OTG2_VBUS: u32 = 0;
            #[doc = "Utilize VBUS OTG1 power"]
            pub const USB_OTG1_VBUS: u32 = 0x01;
        }
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.625V"]
            pub const OUTPUT_TRG_0: u32 = 0;
            #[doc = "3.000V"]
            pub const OUTPUT_TRG_15: u32 = 0x0f;
            #[doc = "3.400V"]
            pub const OUTPUT_TRG_31: u32 = 0x1f;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD3P0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD3P0 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Regulator 3P0 Register"]
pub mod REG_3P0_TOG {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    pub mod VBUS_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Utilize VBUS OTG2 power"]
            pub const USB_OTG2_VBUS: u32 = 0;
            #[doc = "Utilize VBUS OTG1 power"]
            pub const USB_OTG1_VBUS: u32 = 0x01;
        }
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.625V"]
            pub const OUTPUT_TRG_0: u32 = 0;
            #[doc = "3.000V"]
            pub const OUTPUT_TRG_15: u32 = 0x0f;
            #[doc = "3.400V"]
            pub const OUTPUT_TRG_31: u32 = 0x1f;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD3P0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD3P0 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Regulator 2P5 Register"]
pub mod REG_2P5 {
    #[doc = "Control bit to enable the regulator output."]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    pub mod ENABLE_PULLDOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.10V"]
            pub const OUTPUT_TRG_0: u32 = 0;
            #[doc = "2.50V"]
            pub const OUTPUT_TRG_16: u32 = 0x10;
            #[doc = "2.875V"]
            pub const OUTPUT_TRG_31: u32 = 0x1f;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD2P5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD2P5 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the weak 2p5 regulator"]
    pub mod ENABLE_WEAK_LINREG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Regulator 2P5 Register"]
pub mod REG_2P5_SET {
    #[doc = "Control bit to enable the regulator output."]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    pub mod ENABLE_PULLDOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.10V"]
            pub const OUTPUT_TRG_0: u32 = 0;
            #[doc = "2.50V"]
            pub const OUTPUT_TRG_16: u32 = 0x10;
            #[doc = "2.875V"]
            pub const OUTPUT_TRG_31: u32 = 0x1f;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD2P5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD2P5 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the weak 2p5 regulator"]
    pub mod ENABLE_WEAK_LINREG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Regulator 2P5 Register"]
pub mod REG_2P5_CLR {
    #[doc = "Control bit to enable the regulator output."]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    pub mod ENABLE_PULLDOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.10V"]
            pub const OUTPUT_TRG_0: u32 = 0;
            #[doc = "2.50V"]
            pub const OUTPUT_TRG_16: u32 = 0x10;
            #[doc = "2.875V"]
            pub const OUTPUT_TRG_31: u32 = 0x1f;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD2P5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD2P5 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the weak 2p5 regulator"]
    pub mod ENABLE_WEAK_LINREG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Regulator 2P5 Register"]
pub mod REG_2P5_TOG {
    #[doc = "Control bit to enable the regulator output."]
    pub mod ENABLE_LINREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    pub mod ENABLE_BO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    pub mod ENABLE_ILIMIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    pub mod ENABLE_PULLDOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    pub mod BO_OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    pub mod OUTPUT_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.10V"]
            pub const OUTPUT_TRG_0: u32 = 0;
            #[doc = "2.50V"]
            pub const OUTPUT_TRG_16: u32 = 0x10;
            #[doc = "2.875V"]
            pub const OUTPUT_TRG_31: u32 = 0x1f;
        }
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    pub mod BO_VDD2P5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    pub mod OK_VDD2P5 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the weak 2p5 regulator"]
    pub mod ENABLE_WEAK_LINREG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Digital Regulator Core Register"]
pub mod REG_CORE {
    #[doc = "This field defines the target voltage for the ARM core power domain"]
    pub mod REG0_TARG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG0_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG0_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG0_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG0_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG0_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG0_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG0_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG0_ADJ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG0_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG0_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG0_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG0_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG0_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG0_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG0_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG0_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG0_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG0_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG0_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG0_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG0_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG0_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG0_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG0_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    pub mod REG1_TARG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG1_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG1_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG1_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG1_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG1_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG1_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG1_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG1_ADJ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG1_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG1_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG1_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG1_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG1_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG1_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG1_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG1_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG1_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG1_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG1_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG1_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG1_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG1_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG1_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG1_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    pub mod REG2_TARG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG2_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG2_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG2_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG2_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG2_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG2_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG2_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG2_ADJ {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG2_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG2_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG2_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG2_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG2_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG2_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG2_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG2_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG2_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG2_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG2_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG2_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG2_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG2_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG2_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG2_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "Regulator voltage ramp rate."]
    pub mod RAMP_RATE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fast"]
            pub const RAMP_RATE_0: u32 = 0;
            #[doc = "Medium Fast"]
            pub const RAMP_RATE_1: u32 = 0x01;
            #[doc = "Medium Slow"]
            pub const RAMP_RATE_2: u32 = 0x02;
            #[doc = "Slow"]
            pub const RAMP_RATE_3: u32 = 0x03;
        }
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    pub mod FET_ODRIVE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Digital Regulator Core Register"]
pub mod REG_CORE_SET {
    #[doc = "This field defines the target voltage for the ARM core power domain"]
    pub mod REG0_TARG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG0_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG0_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG0_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG0_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG0_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG0_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG0_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG0_ADJ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG0_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG0_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG0_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG0_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG0_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG0_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG0_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG0_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG0_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG0_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG0_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG0_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG0_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG0_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG0_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG0_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    pub mod REG1_TARG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG1_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG1_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG1_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG1_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG1_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG1_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG1_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG1_ADJ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG1_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG1_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG1_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG1_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG1_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG1_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG1_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG1_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG1_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG1_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG1_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG1_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG1_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG1_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG1_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG1_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    pub mod REG2_TARG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG2_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG2_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG2_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG2_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG2_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG2_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG2_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG2_ADJ {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG2_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG2_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG2_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG2_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG2_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG2_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG2_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG2_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG2_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG2_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG2_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG2_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG2_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG2_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG2_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG2_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "Regulator voltage ramp rate."]
    pub mod RAMP_RATE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fast"]
            pub const RAMP_RATE_0: u32 = 0;
            #[doc = "Medium Fast"]
            pub const RAMP_RATE_1: u32 = 0x01;
            #[doc = "Medium Slow"]
            pub const RAMP_RATE_2: u32 = 0x02;
            #[doc = "Slow"]
            pub const RAMP_RATE_3: u32 = 0x03;
        }
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    pub mod FET_ODRIVE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Digital Regulator Core Register"]
pub mod REG_CORE_CLR {
    #[doc = "This field defines the target voltage for the ARM core power domain"]
    pub mod REG0_TARG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG0_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG0_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG0_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG0_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG0_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG0_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG0_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG0_ADJ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG0_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG0_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG0_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG0_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG0_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG0_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG0_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG0_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG0_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG0_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG0_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG0_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG0_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG0_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG0_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG0_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    pub mod REG1_TARG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG1_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG1_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG1_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG1_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG1_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG1_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG1_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG1_ADJ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG1_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG1_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG1_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG1_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG1_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG1_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG1_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG1_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG1_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG1_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG1_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG1_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG1_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG1_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG1_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG1_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    pub mod REG2_TARG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG2_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG2_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG2_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG2_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG2_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG2_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG2_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG2_ADJ {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG2_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG2_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG2_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG2_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG2_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG2_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG2_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG2_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG2_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG2_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG2_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG2_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG2_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG2_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG2_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG2_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "Regulator voltage ramp rate."]
    pub mod RAMP_RATE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fast"]
            pub const RAMP_RATE_0: u32 = 0;
            #[doc = "Medium Fast"]
            pub const RAMP_RATE_1: u32 = 0x01;
            #[doc = "Medium Slow"]
            pub const RAMP_RATE_2: u32 = 0x02;
            #[doc = "Slow"]
            pub const RAMP_RATE_3: u32 = 0x03;
        }
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    pub mod FET_ODRIVE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Digital Regulator Core Register"]
pub mod REG_CORE_TOG {
    #[doc = "This field defines the target voltage for the ARM core power domain"]
    pub mod REG0_TARG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG0_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG0_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG0_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG0_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG0_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG0_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG0_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG0_ADJ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG0_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG0_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG0_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG0_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG0_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG0_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG0_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG0_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG0_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG0_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG0_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG0_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG0_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG0_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG0_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG0_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    pub mod REG1_TARG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG1_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG1_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG1_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG1_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG1_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG1_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG1_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG1_ADJ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG1_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG1_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG1_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG1_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG1_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG1_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG1_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG1_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG1_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG1_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG1_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG1_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG1_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG1_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG1_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG1_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    pub mod REG2_TARG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power gated off"]
            pub const REG2_TARG_0: u32 = 0;
            #[doc = "Target core voltage = 0.725V"]
            pub const REG2_TARG_1: u32 = 0x01;
            #[doc = "Target core voltage = 0.750V"]
            pub const REG2_TARG_2: u32 = 0x02;
            #[doc = "Target core voltage = 0.775V"]
            pub const REG2_TARG_3: u32 = 0x03;
            #[doc = "Target core voltage = 1.100V"]
            pub const REG2_TARG_16: u32 = 0x10;
            #[doc = "Target core voltage = 1.450V"]
            pub const REG2_TARG_30: u32 = 0x1e;
            #[doc = "Power FET switched full on. No regulation."]
            pub const REG2_TARG_31: u32 = 0x1f;
        }
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    pub mod REG2_ADJ {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No adjustment"]
            pub const REG2_ADJ_0: u32 = 0;
            #[doc = "+ 0.25%"]
            pub const REG2_ADJ_1: u32 = 0x01;
            #[doc = "+ 0.50%"]
            pub const REG2_ADJ_2: u32 = 0x02;
            #[doc = "+ 0.75%"]
            pub const REG2_ADJ_3: u32 = 0x03;
            #[doc = "+ 1.00%"]
            pub const REG2_ADJ_4: u32 = 0x04;
            #[doc = "+ 1.25%"]
            pub const REG2_ADJ_5: u32 = 0x05;
            #[doc = "+ 1.50%"]
            pub const REG2_ADJ_6: u32 = 0x06;
            #[doc = "+ 1.75%"]
            pub const REG2_ADJ_7: u32 = 0x07;
            #[doc = "- 0.25%"]
            pub const REG2_ADJ_8: u32 = 0x08;
            #[doc = "- 0.50%"]
            pub const REG2_ADJ_9: u32 = 0x09;
            #[doc = "- 0.75%"]
            pub const REG2_ADJ_10: u32 = 0x0a;
            #[doc = "- 1.00%"]
            pub const REG2_ADJ_11: u32 = 0x0b;
            #[doc = "- 1.25%"]
            pub const REG2_ADJ_12: u32 = 0x0c;
            #[doc = "- 1.50%"]
            pub const REG2_ADJ_13: u32 = 0x0d;
            #[doc = "- 1.75%"]
            pub const REG2_ADJ_14: u32 = 0x0e;
            #[doc = "- 2.00%"]
            pub const REG2_ADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "Regulator voltage ramp rate."]
    pub mod RAMP_RATE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fast"]
            pub const RAMP_RATE_0: u32 = 0;
            #[doc = "Medium Fast"]
            pub const RAMP_RATE_1: u32 = 0x01;
            #[doc = "Medium Slow"]
            pub const RAMP_RATE_2: u32 = 0x02;
            #[doc = "Slow"]
            pub const RAMP_RATE_3: u32 = 0x03;
        }
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    pub mod FET_ODRIVE {
        pub const offset: u32 = 29;
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
    #[doc = "no description available"]
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
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
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
            #[doc = "SUSPEND (DSM)"]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Analog regulators are ON."]
            pub const STANDBY: u32 = 0x01;
            #[doc = "STOP (lower power)"]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "STOP (very lower power)"]
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
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
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
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    pub mod VID_PLL_PREDIV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const VID_PLL_PREDIV_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const VID_PLL_PREDIV_1: u32 = 0x01;
        }
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
    #[doc = "no description available"]
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
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
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
            #[doc = "SUSPEND (DSM)"]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Analog regulators are ON."]
            pub const STANDBY: u32 = 0x01;
            #[doc = "STOP (lower power)"]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "STOP (very lower power)"]
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
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
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
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    pub mod VID_PLL_PREDIV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const VID_PLL_PREDIV_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const VID_PLL_PREDIV_1: u32 = 0x01;
        }
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
    #[doc = "no description available"]
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
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
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
            #[doc = "SUSPEND (DSM)"]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Analog regulators are ON."]
            pub const STANDBY: u32 = 0x01;
            #[doc = "STOP (lower power)"]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "STOP (very lower power)"]
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
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
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
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    pub mod VID_PLL_PREDIV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const VID_PLL_PREDIV_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const VID_PLL_PREDIV_1: u32 = 0x01;
        }
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
    #[doc = "no description available"]
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
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
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
            #[doc = "SUSPEND (DSM)"]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Analog regulators are ON."]
            pub const STANDBY: u32 = 0x01;
            #[doc = "STOP (lower power)"]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "STOP (very lower power)"]
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
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
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
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    pub mod VID_PLL_PREDIV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const VID_PLL_PREDIV_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const VID_PLL_PREDIV_1: u32 = 0x01;
        }
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1 {
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
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
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    pub mod LVDS2_CLK_SEL {
        pub const offset: u32 = 5;
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
            #[doc = "MLB PLL"]
            pub const MLB_PLL: u32 = 0x08;
            #[doc = "ethernet ref clock (ENET_PLL)"]
            pub const ETHERNET_REF: u32 = 0x09;
            #[doc = "PCIe ref clock (125M)"]
            pub const PCIE_REF: u32 = 0x0a;
            #[doc = "SATA ref clock (100M)"]
            pub const SATA_REF: u32 = 0x0b;
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
            #[doc = "LVDS1 (loopback)"]
            pub const LVDS1: u32 = 0x13;
            #[doc = "LVDS2 (not useful)"]
            pub const LVDS2: u32 = 0x14;
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
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    pub mod LVDSCLK2_OBEN {
        pub const offset: u32 = 11;
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
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    pub mod LVDSCLK2_IBEN {
        pub const offset: u32 = 13;
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
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
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
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    pub mod LVDS2_CLK_SEL {
        pub const offset: u32 = 5;
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
            #[doc = "MLB PLL"]
            pub const MLB_PLL: u32 = 0x08;
            #[doc = "ethernet ref clock (ENET_PLL)"]
            pub const ETHERNET_REF: u32 = 0x09;
            #[doc = "PCIe ref clock (125M)"]
            pub const PCIE_REF: u32 = 0x0a;
            #[doc = "SATA ref clock (100M)"]
            pub const SATA_REF: u32 = 0x0b;
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
            #[doc = "LVDS1 (loopback)"]
            pub const LVDS1: u32 = 0x13;
            #[doc = "LVDS2 (not useful)"]
            pub const LVDS2: u32 = 0x14;
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
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    pub mod LVDSCLK2_OBEN {
        pub const offset: u32 = 11;
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
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    pub mod LVDSCLK2_IBEN {
        pub const offset: u32 = 13;
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
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
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
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    pub mod LVDS2_CLK_SEL {
        pub const offset: u32 = 5;
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
            #[doc = "MLB PLL"]
            pub const MLB_PLL: u32 = 0x08;
            #[doc = "ethernet ref clock (ENET_PLL)"]
            pub const ETHERNET_REF: u32 = 0x09;
            #[doc = "PCIe ref clock (125M)"]
            pub const PCIE_REF: u32 = 0x0a;
            #[doc = "SATA ref clock (100M)"]
            pub const SATA_REF: u32 = 0x0b;
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
            #[doc = "LVDS1 (loopback)"]
            pub const LVDS1: u32 = 0x13;
            #[doc = "LVDS2 (not useful)"]
            pub const LVDS2: u32 = 0x14;
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
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    pub mod LVDSCLK2_OBEN {
        pub const offset: u32 = 11;
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
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    pub mod LVDSCLK2_IBEN {
        pub const offset: u32 = 13;
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
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
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
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    pub mod LVDS2_CLK_SEL {
        pub const offset: u32 = 5;
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
            #[doc = "MLB PLL"]
            pub const MLB_PLL: u32 = 0x08;
            #[doc = "ethernet ref clock (ENET_PLL)"]
            pub const ETHERNET_REF: u32 = 0x09;
            #[doc = "PCIe ref clock (125M)"]
            pub const PCIE_REF: u32 = 0x0a;
            #[doc = "SATA ref clock (100M)"]
            pub const SATA_REF: u32 = 0x0b;
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
            #[doc = "LVDS1 (loopback)"]
            pub const LVDS1: u32 = 0x13;
            #[doc = "LVDS2 (not useful)"]
            pub const LVDS2: u32 = 0x14;
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
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    pub mod LVDSCLK2_OBEN {
        pub const offset: u32 = 11;
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
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    pub mod LVDSCLK2_IBEN {
        pub const offset: u32 = 13;
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
#[doc = "Miscellaneous Control Register"]
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
    #[doc = "Reg0 brownout status bit."]
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
    #[doc = "Enables the brownout detection."]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default value of \"0\""]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
    #[doc = "Reg1 brownout status bit."]
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
    #[doc = "Enables the brownout detection."]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
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
    #[doc = "Reg2 brownout status bit."]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the brownout detection."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
#[doc = "Miscellaneous Control Register"]
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
    #[doc = "Reg0 brownout status bit."]
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
    #[doc = "Enables the brownout detection."]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default value of \"0\""]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
    #[doc = "Reg1 brownout status bit."]
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
    #[doc = "Enables the brownout detection."]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
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
    #[doc = "Reg2 brownout status bit."]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the brownout detection."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
#[doc = "Miscellaneous Control Register"]
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
    #[doc = "Reg0 brownout status bit."]
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
    #[doc = "Enables the brownout detection."]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default value of \"0\""]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
    #[doc = "Reg1 brownout status bit."]
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
    #[doc = "Enables the brownout detection."]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
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
    #[doc = "Reg2 brownout status bit."]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the brownout detection."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
#[doc = "Miscellaneous Control Register"]
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
    #[doc = "Reg0 brownout status bit."]
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
    #[doc = "Enables the brownout detection."]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default value of \"0\""]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
    #[doc = "Reg1 brownout status bit."]
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
    #[doc = "Enables the brownout detection."]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
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
    #[doc = "Reg2 brownout status bit."]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the brownout detection."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
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
