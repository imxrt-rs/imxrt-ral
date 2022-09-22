#[doc = "USB Analog"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01a0],
    #[doc = "USB VBUS Detect Register"]
    pub USB1_VBUS_DETECT: crate::RWRegister<u32>,
    #[doc = "USB VBUS Detect Register"]
    pub USB1_VBUS_DETECT_SET: crate::RWRegister<u32>,
    #[doc = "USB VBUS Detect Register"]
    pub USB1_VBUS_DETECT_CLR: crate::RWRegister<u32>,
    #[doc = "USB VBUS Detect Register"]
    pub USB1_VBUS_DETECT_TOG: crate::RWRegister<u32>,
    #[doc = "USB Charger Detect Register"]
    pub USB1_CHRG_DETECT: crate::RWRegister<u32>,
    #[doc = "USB Charger Detect Register"]
    pub USB1_CHRG_DETECT_SET: crate::RWRegister<u32>,
    #[doc = "USB Charger Detect Register"]
    pub USB1_CHRG_DETECT_CLR: crate::RWRegister<u32>,
    #[doc = "USB Charger Detect Register"]
    pub USB1_CHRG_DETECT_TOG: crate::RWRegister<u32>,
    #[doc = "USB VBUS Detect Status Register"]
    pub USB1_VBUS_DETECT_STAT: crate::RORegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "USB Charger Detect Status Register"]
    pub USB1_CHRG_DETECT_STAT: crate::RORegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "USB Loopback Test Register"]
    pub USB1_LOOPBACK: crate::RWRegister<u32>,
    #[doc = "USB Loopback Test Register"]
    pub USB1_LOOPBACK_SET: crate::RWRegister<u32>,
    #[doc = "USB Loopback Test Register"]
    pub USB1_LOOPBACK_CLR: crate::RWRegister<u32>,
    #[doc = "USB Loopback Test Register"]
    pub USB1_LOOPBACK_TOG: crate::RWRegister<u32>,
    #[doc = "USB Misc Register"]
    pub USB1_MISC: crate::RWRegister<u32>,
    #[doc = "USB Misc Register"]
    pub USB1_MISC_SET: crate::RWRegister<u32>,
    #[doc = "USB Misc Register"]
    pub USB1_MISC_CLR: crate::RWRegister<u32>,
    #[doc = "USB Misc Register"]
    pub USB1_MISC_TOG: crate::RWRegister<u32>,
    _reserved3: [u8; 0x60],
    #[doc = "Chip Silicon Version"]
    pub DIGPROG: crate::RORegister<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod USB1_VBUS_DETECT {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4.0V"]
            pub const _4V0: u32 = 0;
            #[doc = "4.1V"]
            pub const _4V1: u32 = 0x01;
            #[doc = "4.2V"]
            pub const _4V2: u32 = 0x02;
            #[doc = "4.3V"]
            pub const _4V3: u32 = 0x03;
            #[doc = "4.4V (default)"]
            pub const _4V4: u32 = 0x04;
            #[doc = "4.5V"]
            pub const _4V5: u32 = 0x05;
            #[doc = "4.6V"]
            pub const _4V6: u32 = 0x06;
            #[doc = "4.7V"]
            pub const _4V7: u32 = 0x07;
        }
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB OTG discharge VBUS."]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB OTG charge VBUS."]
    pub mod CHARGE_VBUS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_SET {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4.0V"]
            pub const _4V0: u32 = 0;
            #[doc = "4.1V"]
            pub const _4V1: u32 = 0x01;
            #[doc = "4.2V"]
            pub const _4V2: u32 = 0x02;
            #[doc = "4.3V"]
            pub const _4V3: u32 = 0x03;
            #[doc = "4.4V (default)"]
            pub const _4V4: u32 = 0x04;
            #[doc = "4.5V"]
            pub const _4V5: u32 = 0x05;
            #[doc = "4.6V"]
            pub const _4V6: u32 = 0x06;
            #[doc = "4.7V"]
            pub const _4V7: u32 = 0x07;
        }
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB OTG discharge VBUS."]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB OTG charge VBUS."]
    pub mod CHARGE_VBUS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_CLR {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4.0V"]
            pub const _4V0: u32 = 0;
            #[doc = "4.1V"]
            pub const _4V1: u32 = 0x01;
            #[doc = "4.2V"]
            pub const _4V2: u32 = 0x02;
            #[doc = "4.3V"]
            pub const _4V3: u32 = 0x03;
            #[doc = "4.4V (default)"]
            pub const _4V4: u32 = 0x04;
            #[doc = "4.5V"]
            pub const _4V5: u32 = 0x05;
            #[doc = "4.6V"]
            pub const _4V6: u32 = 0x06;
            #[doc = "4.7V"]
            pub const _4V7: u32 = 0x07;
        }
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB OTG discharge VBUS."]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB OTG charge VBUS."]
    pub mod CHARGE_VBUS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_TOG {
    #[doc = "Set the threshold for the VBUSVALID comparator"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4.0V"]
            pub const _4V0: u32 = 0;
            #[doc = "4.1V"]
            pub const _4V1: u32 = 0x01;
            #[doc = "4.2V"]
            pub const _4V2: u32 = 0x02;
            #[doc = "4.3V"]
            pub const _4V3: u32 = 0x03;
            #[doc = "4.4V (default)"]
            pub const _4V4: u32 = 0x04;
            #[doc = "4.5V"]
            pub const _4V5: u32 = 0x05;
            #[doc = "4.6V"]
            pub const _4V6: u32 = 0x06;
            #[doc = "4.7V"]
            pub const _4V7: u32 = 0x07;
        }
    }
    #[doc = "Powers up comparators for vbus_valid detector."]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB OTG discharge VBUS."]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB OTG charge VBUS."]
    pub mod CHARGE_VBUS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Charger Detect Register"]
pub mod USB1_CHRG_DETECT {
    #[doc = "Check the contact of USB plug"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not check the contact of USB plug."]
            pub const NO_CHECK: u32 = 0;
            #[doc = "Check whether the USB plug has been in contact with each other"]
            pub const CHECK: u32 = 0x01;
        }
    }
    #[doc = "Check the charger connection"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
            pub const CHECK: u32 = 0;
            #[doc = "Do not check whether a charger is connected to the USB port."]
            pub const NO_CHECK: u32 = 0x01;
        }
    }
    #[doc = "Control the charger detector."]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the charger detector."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the charger detector."]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "USB Charger Detect Register"]
pub mod USB1_CHRG_DETECT_SET {
    #[doc = "Check the contact of USB plug"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not check the contact of USB plug."]
            pub const NO_CHECK: u32 = 0;
            #[doc = "Check whether the USB plug has been in contact with each other"]
            pub const CHECK: u32 = 0x01;
        }
    }
    #[doc = "Check the charger connection"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
            pub const CHECK: u32 = 0;
            #[doc = "Do not check whether a charger is connected to the USB port."]
            pub const NO_CHECK: u32 = 0x01;
        }
    }
    #[doc = "Control the charger detector."]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the charger detector."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the charger detector."]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "USB Charger Detect Register"]
pub mod USB1_CHRG_DETECT_CLR {
    #[doc = "Check the contact of USB plug"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not check the contact of USB plug."]
            pub const NO_CHECK: u32 = 0;
            #[doc = "Check whether the USB plug has been in contact with each other"]
            pub const CHECK: u32 = 0x01;
        }
    }
    #[doc = "Check the charger connection"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
            pub const CHECK: u32 = 0;
            #[doc = "Do not check whether a charger is connected to the USB port."]
            pub const NO_CHECK: u32 = 0x01;
        }
    }
    #[doc = "Control the charger detector."]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the charger detector."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the charger detector."]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "USB Charger Detect Register"]
pub mod USB1_CHRG_DETECT_TOG {
    #[doc = "Check the contact of USB plug"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not check the contact of USB plug."]
            pub const NO_CHECK: u32 = 0;
            #[doc = "Check whether the USB plug has been in contact with each other"]
            pub const CHECK: u32 = 0x01;
        }
    }
    #[doc = "Check the charger connection"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
            pub const CHECK: u32 = 0;
            #[doc = "Do not check whether a charger is connected to the USB port."]
            pub const NO_CHECK: u32 = 0x01;
        }
    }
    #[doc = "Control the charger detector."]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the charger detector."]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable the charger detector."]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "USB VBUS Detect Status Register"]
pub mod USB1_VBUS_DETECT_STAT {
    #[doc = "Session End for USB OTG"]
    pub mod SESSEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates VBus is valid for a B-peripheral"]
    pub mod BVALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates VBus is valid for a A-peripheral"]
    pub mod AVALID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBus valid for USB OTG"]
    pub mod VBUS_VALID {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Charger Detect Status Register"]
pub mod USB1_CHRG_DETECT_STAT {
    #[doc = "State of the USB plug contact detector."]
    pub mod PLUG_CONTACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The USB plug has not made contact."]
            pub const NO_CONTACT: u32 = 0;
            #[doc = "The USB plug has made good contact."]
            pub const GOOD_CONTACT: u32 = 0x01;
        }
    }
    #[doc = "State of charger detection. This bit is a read only version of the state of the analog signal."]
    pub mod CHRG_DETECTED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The USB port is not connected to a charger."]
            pub const CHARGER_NOT_PRESENT: u32 = 0;
            #[doc = "A charger (either a dedicated charger or a host charger) is connected to the USB port."]
            pub const CHARGER_PRESENT: u32 = 0x01;
        }
    }
    #[doc = "DM line state output of the charger detector."]
    pub mod DM_STATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP line state output of the charger detector."]
    pub mod DP_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Loopback Test Register"]
pub mod USB1_LOOPBACK {
    #[doc = "Setting this bit can enable 1"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Loopback Test Register"]
pub mod USB1_LOOPBACK_SET {
    #[doc = "Setting this bit can enable 1"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Loopback Test Register"]
pub mod USB1_LOOPBACK_CLR {
    #[doc = "Setting this bit can enable 1"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Loopback Test Register"]
pub mod USB1_LOOPBACK_TOG {
    #[doc = "Setting this bit can enable 1"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Misc Register"]
pub mod USB1_MISC {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    pub mod HS_USE_EXTERNAL_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    pub mod EN_DEGLITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the clk to the UTMI block."]
    pub mod EN_CLK_UTMI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Misc Register"]
pub mod USB1_MISC_SET {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    pub mod HS_USE_EXTERNAL_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    pub mod EN_DEGLITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the clk to the UTMI block."]
    pub mod EN_CLK_UTMI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Misc Register"]
pub mod USB1_MISC_CLR {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    pub mod HS_USE_EXTERNAL_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    pub mod EN_DEGLITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the clk to the UTMI block."]
    pub mod EN_CLK_UTMI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB Misc Register"]
pub mod USB1_MISC_TOG {
    #[doc = "Use external resistor to generate the current bias for the high speed transmitter"]
    pub mod HS_USE_EXTERNAL_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the deglitching circuit of the USB PLL output."]
    pub mod EN_DEGLITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the clk to the UTMI block."]
    pub mod EN_CLK_UTMI {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Chip Silicon Version"]
pub mod DIGPROG {
    #[doc = "Chip silicon revision"]
    pub mod SILICON_REVISION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Silicon revision 1.0"]
            pub const SILICON_REVISION_7012352: u32 = 0x006b_0000;
        }
    }
}
