#[doc = "USBPHY"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "USBPHY Powerdown"]
    pub PWD: crate::RWRegister<u32>,
    #[doc = "USBPHY Powerdown"]
    pub PWD_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Powerdown"]
    pub PWD_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Powerdown"]
    pub PWD_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY Transmitter Control"]
    pub TX: crate::RWRegister<u32>,
    #[doc = "USBPHY Transmitter Control"]
    pub TX_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Transmitter Control"]
    pub TX_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Transmitter Control"]
    pub TX_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY Receiver Control"]
    pub RX: crate::RWRegister<u32>,
    #[doc = "USBPHY Receiver Control"]
    pub RX_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Receiver Control"]
    pub RX_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Receiver Control"]
    pub RX_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY General Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "USBPHY General Control"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY General Control"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY General Control"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY Status"]
    pub STATUS: crate::RORegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "USBPHY Debug"]
    pub DEBUG: crate::RWRegister<u32>,
    #[doc = "USBPHY Debug"]
    pub DEBUG_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Debug"]
    pub DEBUG_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Debug"]
    pub DEBUG_TOG: crate::RWRegister<u32>,
    #[doc = "UTMI Debug Status 0"]
    pub DEBUG0_STATUS: crate::RORegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "UTMI Debug Status 1"]
    pub DEBUG1: crate::RWRegister<u32>,
    #[doc = "UTMI Debug Status 1"]
    pub DEBUG1_SET: crate::RWRegister<u32>,
    #[doc = "UTMI Debug Status 1"]
    pub DEBUG1_CLR: crate::RWRegister<u32>,
    #[doc = "UTMI Debug Status 1"]
    pub DEBUG1_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY Version"]
    pub VERSION: crate::RORegister<u32>,
    _reserved2: [u8; 0x1c],
    #[doc = "USBPHY PLL Control and Status"]
    pub PLL_SIC: crate::RWRegister<u32>,
    #[doc = "USBPHY PLL Control and Status"]
    pub PLL_SIC_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY PLL Control and Status"]
    pub PLL_SIC_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY PLL Control and Status"]
    pub PLL_SIC_TOG: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "USBPHY VBUS Detect Control"]
    pub USB1_VBUS_DETECT: crate::RWRegister<u32>,
    #[doc = "USBPHY VBUS Detect Control"]
    pub USB1_VBUS_DETECT_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY VBUS Detect Control"]
    pub USB1_VBUS_DETECT_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY VBUS Detect Control"]
    pub USB1_VBUS_DETECT_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY VBUS Detector Status"]
    pub USB1_VBUS_DET_STAT: crate::RORegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "USBPHY Charger Detect Control"]
    pub USB1_CHRG_DETECT: crate::RWRegister<u32>,
    #[doc = "USBPHY Charger Detect Control"]
    pub USB1_CHRG_DETECT_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Charger Detect Control"]
    pub USB1_CHRG_DETECT_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Charger Detect Control"]
    pub USB1_CHRG_DETECT_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY Charger Detect Status"]
    pub USB1_CHRG_DET_STAT: crate::RORegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "USBPHY Analog Control"]
    pub ANACTRL: crate::RWRegister<u32>,
    #[doc = "USBPHY Analog Control"]
    pub ANACTRL_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Analog Control"]
    pub ANACTRL_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Analog Control"]
    pub ANACTRL_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY Loopback Control and Status"]
    pub USB1_LOOPBACK: crate::RWRegister<u32>,
    #[doc = "USBPHY Loopback Control and Status"]
    pub USB1_LOOPBACK_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Loopback Control and Status"]
    pub USB1_LOOPBACK_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Loopback Control and Status"]
    pub USB1_LOOPBACK_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY Loopback Packet Number Selection"]
    pub USB1_LOOPBACK_HSFSCNT: crate::RWRegister<u32>,
    #[doc = "USBPHY Loopback Packet Number Selection"]
    pub USB1_LOOPBACK_HSFSCNT_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Loopback Packet Number Selection"]
    pub USB1_LOOPBACK_HSFSCNT_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Loopback Packet Number Selection"]
    pub USB1_LOOPBACK_HSFSCNT_TOG: crate::RWRegister<u32>,
    #[doc = "USBPHY Trim Override Enable"]
    pub TRIM_OVERRIDE_EN: crate::RWRegister<u32>,
    #[doc = "USBPHY Trim Override Enable"]
    pub TRIM_OVERRIDE_EN_SET: crate::RWRegister<u32>,
    #[doc = "USBPHY Trim Override Enable"]
    pub TRIM_OVERRIDE_EN_CLR: crate::RWRegister<u32>,
    #[doc = "USBPHY Trim Override Enable"]
    pub TRIM_OVERRIDE_EN_TOG: crate::RWRegister<u32>,
}
#[doc = "USBPHY Powerdown"]
pub mod PWD {
    #[doc = "TXPWDFS"]
    pub mod TXPWDFS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const NORMAL: u32 = 0;
            #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the drivers into high-impedance output"]
            pub const PWR_DOWN: u32 = 0x01;
        }
    }
    #[doc = "TXPWDIBIAS"]
    pub mod TXPWDIBIAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the USB is in suspend mode. This effectively powers down the entire USB transmit path"]
            pub const PWR_DOWN: u32 = 0x01;
        }
    }
    #[doc = "TXPWDV2I"]
    pub mod TXPWDV2I {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const NORMAL: u32 = 0;
            #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
            pub const PWR_DOWN: u32 = 0x01;
        }
    }
    #[doc = "RXPWDENV"]
    pub mod RXPWDENV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const NORMAL: u32 = 0;
            #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
            pub const PWR_DOWN: u32 = 0x01;
        }
    }
    #[doc = "RXPWD1PT1"]
    pub mod RXPWD1PT1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Power-down the USB full-speed differential receiver."]
            pub const PWR_DOWN: u32 = 0x01;
        }
    }
    #[doc = "RXPWDDIFF"]
    pub mod RXPWDDIFF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const NORMAL: u32 = 0;
            #[doc = "Power-down the USB high-speed differential receiver"]
            pub const PWR_DOWN: u32 = 0x01;
        }
    }
    #[doc = "RXPWDRX"]
    pub mod RXPWDRX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
            pub const PWR_DOWN: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY Powerdown"]
pub mod PWD_SET {
    #[doc = "0 = Normal operation"]
    pub mod TXPWDFS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod TXPWDIBIAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod TXPWDV2I {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDENV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWD1PT1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDDIFF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDRX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Powerdown"]
pub mod PWD_CLR {
    #[doc = "0 = Normal operation"]
    pub mod TXPWDFS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod TXPWDIBIAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod TXPWDV2I {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDENV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWD1PT1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDDIFF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDRX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Powerdown"]
pub mod PWD_TOG {
    #[doc = "0 = Normal operation"]
    pub mod TXPWDFS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod TXPWDIBIAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod TXPWDV2I {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDENV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWD1PT1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDDIFF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXPWDRX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Transmitter Control"]
pub mod TX {
    #[doc = "HS Transmit Output Current Trim"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "+20.30%"]
            pub const DCAL20P: u32 = 0;
            #[doc = "+17.60%"]
            pub const DCAL17P: u32 = 0x01;
            #[doc = "+14.80%"]
            pub const DCAL14P: u32 = 0x02;
            #[doc = "+12.60%"]
            pub const DCAL12P: u32 = 0x03;
            #[doc = "+8.79%"]
            pub const DCAL8P: u32 = 0x04;
            #[doc = "+6.04%"]
            pub const DCAL6P: u32 = 0x05;
            #[doc = "+2.75%"]
            pub const DCAL2P: u32 = 0x06;
            #[doc = "0%"]
            pub const DCAL0: u32 = 0x07;
            #[doc = "-2.75%"]
            pub const DCAL2N: u32 = 0x08;
            #[doc = "-5.49%"]
            pub const DCAL5N: u32 = 0x09;
            #[doc = "-7.69%"]
            pub const DCAL7N: u32 = 0x0a;
            #[doc = "-10.40%"]
            pub const DCAL10N: u32 = 0x0b;
            #[doc = "-12.60%"]
            pub const DCAL12N: u32 = 0x0c;
            #[doc = "-14.30%"]
            pub const DCAL14N: u32 = 0x0d;
            #[doc = "-18.10%"]
            pub const DCAL18N: u32 = 0x0e;
            #[doc = "-22.00%"]
            pub const DCAL22N: u32 = 0x0f;
        }
    }
    #[doc = "Transmit Calculation 45 ohm DN"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "+19.95%"]
            pub const DN19P: u32 = 0;
            #[doc = "+17.35%"]
            pub const DN17P: u32 = 0x01;
            #[doc = "+14.85%"]
            pub const DN14P: u32 = 0x02;
            #[doc = "+12.46%"]
            pub const DN12P: u32 = 0x03;
            #[doc = "+9.07%"]
            pub const DN9P: u32 = 0x04;
            #[doc = "+5.87%"]
            pub const DN5P: u32 = 0x05;
            #[doc = "+2.85%"]
            pub const DN2P: u32 = 0x06;
            #[doc = "0%"]
            pub const DN0: u32 = 0x07;
            #[doc = "-2.70%"]
            pub const DN2N: u32 = 0x08;
            #[doc = "-5.25%"]
            pub const DN5N: u32 = 0x09;
            #[doc = "-7.67%"]
            pub const DN7N: u32 = 0x0a;
            #[doc = "-9.98%"]
            pub const DN9N: u32 = 0x0b;
            #[doc = "-12.17%"]
            pub const DN12N: u32 = 0x0c;
            #[doc = "-14.25%"]
            pub const DN14N: u32 = 0x0d;
            #[doc = "-18.14%"]
            pub const DN18N: u32 = 0x0e;
            #[doc = "-21.68%"]
            pub const DN21N: u32 = 0x0f;
        }
    }
    #[doc = "Transmit Calculation 45 ohm DP"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "+19.95%"]
            pub const DP19P: u32 = 0;
            #[doc = "+17.35%"]
            pub const DP17P: u32 = 0x01;
            #[doc = "+14.85%"]
            pub const DP14P: u32 = 0x02;
            #[doc = "+12.46%"]
            pub const DP12P: u32 = 0x03;
            #[doc = "+9.07%"]
            pub const DP9P: u32 = 0x04;
            #[doc = "+5.87%"]
            pub const DP5P: u32 = 0x05;
            #[doc = "+2.85%"]
            pub const DP2P: u32 = 0x06;
            #[doc = "0%"]
            pub const DP0: u32 = 0x07;
            #[doc = "-2.70%"]
            pub const DP2N: u32 = 0x08;
            #[doc = "-5.25%"]
            pub const DP5N: u32 = 0x09;
            #[doc = "-7.67%"]
            pub const DP7N: u32 = 0x0a;
            #[doc = "-9.98%"]
            pub const DP9N: u32 = 0x0b;
            #[doc = "-12.17%"]
            pub const DP12N: u32 = 0x0c;
            #[doc = "-14.25%"]
            pub const DP14N: u32 = 0x0d;
            #[doc = "-18.14%"]
            pub const DP18N: u32 = 0x0e;
            #[doc = "-21.68%"]
            pub const DP21N: u32 = 0x0f;
        }
    }
}
#[doc = "USBPHY Transmitter Control"]
pub mod TX_SET {
    #[doc = "D_CAL"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXCAL45DN"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXCAL45DP"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Transmitter Control"]
pub mod TX_CLR {
    #[doc = "D_CAL"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXCAL45DN"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXCAL45DP"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Transmitter Control"]
pub mod TX_TOG {
    #[doc = "D_CAL"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXCAL45DN"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXCAL45DP"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Receiver Control"]
pub mod RX {
    #[doc = "ENVADJ"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trip-Level Voltage is 0.1000 V"]
            pub const LVL_P1: u32 = 0;
            #[doc = "Trip-Level Voltage is 0.1125 V"]
            pub const LVL_P1125: u32 = 0x01;
            #[doc = "Trip-Level Voltage is 0.1250 V"]
            pub const LVL_P1250: u32 = 0x02;
            #[doc = "Trip-Level Voltage is 0.0875 V"]
            pub const LVL_P0875: u32 = 0x03;
        }
    }
    #[doc = "DISCONADJ"]
    pub mod DISCONADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trip-Level Voltage is 0.56875 V"]
            pub const LVL_P56875: u32 = 0;
            #[doc = "Trip-Level Voltage is 0.55000 V"]
            pub const LVL_P55: u32 = 0x01;
            #[doc = "Trip-Level Voltage is 0.58125 V"]
            pub const LVL_P58125: u32 = 0x02;
            #[doc = "Trip-Level Voltage is 0.60000 V"]
            pub const LVL_P6: u32 = 0x03;
        }
    }
    #[doc = "RXDBYPASS"]
    pub mod RXDBYPASS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const NORMAL: u32 = 0;
            #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
            pub const OUT_SINGLE_END: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY Receiver Control"]
pub mod RX_SET {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    pub mod DISCONADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXDBYPASS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Receiver Control"]
pub mod RX_CLR {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    pub mod DISCONADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXDBYPASS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Receiver Control"]
pub mod RX_TOG {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    pub mod DISCONADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 = Normal operation"]
    pub mod RXDBYPASS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY General Control"]
pub mod CTRL {
    #[doc = "ID Change Interrupt Enable"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Host Disconnect Detection Enable"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Host Disconnect Interrupt Enable"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Connected"]
            pub const CONNECTED: u32 = 0;
            #[doc = "Disconnected"]
            pub const DISCONNECTED: u32 = 0x01;
        }
    }
    #[doc = "Nonstandard Resistive Plugged-In Detection Enable"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables 200kohm pullup resistors on DP and DN pins"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables 200kohm pullup resistors on DP and DN pins"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Device Plug-In Polarity"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Plugged in"]
            pub const PLUGGEDIN: u32 = 0;
            #[doc = "Unplugged"]
            pub const UNPLUGGED: u32 = 0x01;
        }
    }
    #[doc = "OTG ID Change Interrupt"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No ID change interrupt"]
            pub const NO_ID_CHG_IRQ: u32 = 0;
            #[doc = "ID change interrupt"]
            pub const ID_CHG_IRQ: u32 = 0x01;
        }
    }
    #[doc = "Enable Internal OTG ID Detector"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "RESUME_IRQ Sticky"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remains 1 during the wake-up period"]
            pub const NONSTICKY: u32 = 0;
            #[doc = "Remains 1 until you write 0 to it"]
            pub const STICKY: u32 = 0x01;
        }
    }
    #[doc = "Resume Detection Interrupt Enable"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Resume"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No resume interrupt"]
            pub const NO_RESUME: u32 = 0;
            #[doc = "Resume interrupt"]
            pub const RESUME: u32 = 0x01;
        }
    }
    #[doc = "Device Plug-In Interrupt Enable"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Device Plug-In Interrupt"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not connected"]
            pub const DISCONNECTED: u32 = 0;
            #[doc = "Connected"]
            pub const CONNECTED: u32 = 0x01;
        }
    }
    #[doc = "UTMI Level 2 Enable"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "UTMI Level 3 Enable"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Wake-Up Interrupt Enable"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Wake-Up Interrupt"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No wake-up event exists"]
            pub const NOWAKEUP: u32 = 0;
            #[doc = "Wake-up event exists"]
            pub const WAKEUP: u32 = 0x01;
        }
    }
    #[doc = "Auto Resume Enable"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Clock Gating Auto Clear Enable"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PHY PWD Auto Clear Enable"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FSDLL Reset Enable"]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ID Value"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lesser ID resistance than Ra_Plug_ID"]
            pub const FALSE: u32 = 0;
            #[doc = "Greater ID resistance than Rb_Plug_ID"]
            pub const TRUE: u32 = 0x01;
        }
    }
    #[doc = "FS EOP LS Timing"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not force the next FS packet"]
            pub const NO_FORCE: u32 = 0;
            #[doc = "Force the next FS packet"]
            pub const FORCE: u32 = 0x01;
        }
    }
    #[doc = "UTMI Suspend Mode"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suspended"]
            pub const NOSUSPEND: u32 = 0;
            #[doc = "Not suspended"]
            pub const SUSPEND: u32 = 0x01;
        }
    }
    #[doc = "Clock Gating"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Run clocks"]
            pub const RUN: u32 = 0;
            #[doc = "Gate clocks"]
            pub const GATE: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Release from reset"]
            pub const RST: u32 = 0;
            #[doc = "Reset"]
            pub const RLS: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY General Control"]
pub mod CTRL_SET {
    #[doc = "ID Change Interrupt Enable"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Detection Enable"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Interrupt Enable"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Nonstandard Resistive Plugged-In Detection Enable"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Polarity"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID Change Interrupt"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Internal OTG ID Detector"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUME_IRQ Sticky"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume Detection Interrupt Enable"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Resume"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Interrupt Enable"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Interrupt"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Level 2 Enable"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Level 3 Enable"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake-Up Interrupt Enable"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake-Up Interrupt"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto Resume Enable"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating Auto Clear Enable"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY PWD Auto Clear Enable"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FSDLL Reset Enable"]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Value"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FS EOP LS Timing"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Suspend Mode"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY General Control"]
pub mod CTRL_CLR {
    #[doc = "ID Change Interrupt Enable"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Detection Enable"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Interrupt Enable"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Nonstandard Resistive Plugged-In Detection Enable"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Polarity"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID Change Interrupt"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Internal OTG ID Detector"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUME_IRQ Sticky"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume Detection Interrupt Enable"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Resume"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Interrupt Enable"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Interrupt"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Level 2 Enable"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Level 3 Enable"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake-Up Interrupt Enable"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake-Up Interrupt"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto Resume Enable"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating Auto Clear Enable"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY PWD Auto Clear Enable"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FSDLL Reset Enable"]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Value"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FS EOP LS Timing"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Suspend Mode"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY General Control"]
pub mod CTRL_TOG {
    #[doc = "ID Change Interrupt Enable"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Detection Enable"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Interrupt Enable"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Nonstandard Resistive Plugged-In Detection Enable"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Polarity"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID Change Interrupt"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Internal OTG ID Detector"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUME_IRQ Sticky"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume Detection Interrupt Enable"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Resume"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Interrupt Enable"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Plug-In Interrupt"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Level 2 Enable"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Level 3 Enable"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake-Up Interrupt Enable"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake-Up Interrupt"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto Resume Enable"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating Auto Clear Enable"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY PWD Auto Clear Enable"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FSDLL Reset Enable"]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Value"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FS EOP LS Timing"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Suspend Mode"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Status"]
pub mod STATUS {
    #[doc = "Host Disconnect Detection Status"]
    pub mod HOSTDISCONDETECT_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "USB cable disconnect has not been detected at the local host"]
            pub const NOT_DET: u32 = 0;
            #[doc = "USB cable disconnect has been detected at the local host"]
            pub const DET: u32 = 0x01;
        }
    }
    #[doc = "Status Indicator for Nonstandard Resistive Plugged-In Detection"]
    pub mod DEVPLUGIN_STATUS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No attachment to a USB host is detected"]
            pub const NO_ATTACH: u32 = 0;
            #[doc = "Cable attachment to a USB host is detected"]
            pub const ATTACH: u32 = 0x01;
        }
    }
    #[doc = "OTG ID Status"]
    pub mod OTGID_STATUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lesser ID resistance"]
            pub const FALSE: u32 = 0;
            #[doc = "Greater ID resistance"]
            pub const TRUE: u32 = 0x01;
        }
    }
    #[doc = "Resume Status"]
    pub mod RESUME_STATUS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Is in J state"]
            pub const NO_RESUME: u32 = 0;
            #[doc = "Is not in J state"]
            pub const RESUME: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY Debug"]
pub mod DEBUG {
    #[doc = "Hold OTG_ID"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select DP and DN Pulldown Resistors in Host Pulldown Overdrive Mode"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disconnect the resistors"]
            pub const DISCONNECT: u32 = 0;
            #[doc = "Connect 15 kohm pulldown on DN"]
            pub const DN: u32 = 0x01;
            #[doc = "Connect 15 kohm pulldown on DP"]
            pub const DP: u32 = 0x02;
            #[doc = "Connect 15 kohm pulldown on DP and DN"]
            pub const DP_DN: u32 = 0x03;
        }
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Host Pulldown Overdrive mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "Override the control of DN 15 kohm pulldown"]
            pub const DN: u32 = 0x01;
            #[doc = "Override the control of DP 15 kohm pulldown"]
            pub const DP: u32 = 0x02;
            #[doc = "Override the control of DP and DN 15 kohm pulldown"]
            pub const DP_DN: u32 = 0x03;
        }
    }
    #[doc = "Set Countdown Delay Value from TX to RX Packets for Debug"]
    pub mod TX2RXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Countdown from TX to RX Packets for Debug"]
    pub mod ENTX2RXCOUNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Squelch Reset Count"]
    pub mod SQUELCHRESETCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Squelch Reset"]
    pub mod ENSQUELCHRESET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Squelch Reset Length"]
    pub mod SQUELCHRESETLENGTH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Resume"]
    pub mod HOST_RESUME_DEBUG {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Based on CTRL\\[HOST_FORCE_LS_SE0\\]"]
            pub const HOST_FORCE: u32 = 0;
            #[doc = "Based on CTRL\\[UTMI_SUSPENDM\\]"]
            pub const UTMI_SUSPENDM: u32 = 0x01;
        }
    }
    #[doc = "Clock Gating"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Run clocks"]
            pub const RUN: u32 = 0;
            #[doc = "Gate clocks"]
            pub const GATE: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY Debug"]
pub mod DEBUG_SET {
    #[doc = "Hold OTG_ID"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select DP and DN Pulldown Resistors in Host Pulldown Overdrive Mode"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set Countdown Delay Value from TX to RX Packets for Debug"]
    pub mod TX2RXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Countdown from TX to RX Packets for Debug"]
    pub mod ENTX2RXCOUNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Squelch Reset Count"]
    pub mod SQUELCHRESETCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Squelch Reset"]
    pub mod ENSQUELCHRESET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Squelch Reset Length"]
    pub mod SQUELCHRESETLENGTH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Resume"]
    pub mod HOST_RESUME_DEBUG {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Debug"]
pub mod DEBUG_CLR {
    #[doc = "Hold OTG_ID"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select DP and DN Pulldown Resistors in Host Pulldown Overdrive Mode"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set Countdown Delay Value from TX to RX Packets for Debug"]
    pub mod TX2RXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Countdown from TX to RX Packets for Debug"]
    pub mod ENTX2RXCOUNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Squelch Reset Count"]
    pub mod SQUELCHRESETCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Squelch Reset"]
    pub mod ENSQUELCHRESET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Squelch Reset Length"]
    pub mod SQUELCHRESETLENGTH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Resume"]
    pub mod HOST_RESUME_DEBUG {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Debug"]
pub mod DEBUG_TOG {
    #[doc = "Hold OTG_ID"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select DP and DN Pulldown Resistors in Host Pulldown Overdrive Mode"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set Countdown Delay Value from TX to RX Packets for Debug"]
    pub mod TX2RXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Countdown from TX to RX Packets for Debug"]
    pub mod ENTX2RXCOUNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Squelch Reset Count"]
    pub mod SQUELCHRESETCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Squelch Reset"]
    pub mod ENSQUELCHRESET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Squelch Reset Length"]
    pub mod SQUELCHRESETLENGTH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Resume"]
    pub mod HOST_RESUME_DEBUG {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gating"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTMI Debug Status 0"]
pub mod DEBUG0_STATUS {
    #[doc = "Running count of the failed pseudo-random generator loopback"]
    pub mod LOOP_BACK_FAIL_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Running count of the UTMI_RXERROR."]
    pub mod UTMI_RXERROR_FAIL_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Running count of the squelch reset instead of normal end for HS RX."]
    pub mod SQUELCH_COUNT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTMI Debug Status 1"]
pub mod DEBUG1 {
    #[doc = "HS RX Squelch Rise Time Delay Trim"]
    pub mod ENTAILADJVD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Delay is nominal"]
            pub const NOM_DELAY: u32 = 0;
            #[doc = "Delay is +20%"]
            pub const DELAY_20_P: u32 = 0x01;
            #[doc = "Delay is -20%"]
            pub const DELAY_20_N: u32 = 0x02;
            #[doc = "Delay is -40%"]
            pub const DELAY_40_N: u32 = 0x03;
        }
    }
    #[doc = "Self-Bias Off for Reference Bias Amplifiers and Comparators"]
    pub mod USB2_REFBIAS_SELFBIASOFF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Self-bias"]
            pub const ENABLE: u32 = 0;
            #[doc = "Current reference bias"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Bandgap Voltage Status Comparator Powerdown"]
    pub mod USB2_REFBIAS_PWDVBGUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables"]
            pub const VBGUP_CMP_ENABLE: u32 = 0;
            #[doc = "Disables"]
            pub const VBGUP_CMP_DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Reference Bias Low Power Configuration"]
    pub mod USB2_REFBIAS_LOWPWR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal bias current"]
            pub const BIAS_NOM: u32 = 0;
            #[doc = "50% of nominal bias current"]
            pub const BIAS_50: u32 = 0x01;
        }
    }
    #[doc = "Bandgap Voltage Adjustment"]
    pub mod USB2_REFBIAS_VBGADJ {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal bandgap voltage; flattest temperature coefficient"]
            pub const V_NOM: u32 = 0;
            #[doc = "+10 mV compared to nominal"]
            pub const V_P10: u32 = 0x01;
            #[doc = "+20 mV compared to nominal"]
            pub const V_P20: u32 = 0x02;
            #[doc = "+30 mV compared to nominal; most-positive temperature coefficient"]
            pub const V_P30: u32 = 0x03;
            #[doc = "-10 mV compared to nominal"]
            pub const V_M10: u32 = 0x04;
            #[doc = "-20 mV compared to nominal"]
            pub const V_M20: u32 = 0x05;
            #[doc = "-30 mV compared to nominal"]
            pub const V_M30: u32 = 0x06;
            #[doc = "-40 mV compared to nominal; most-negative temperature coefficient"]
            pub const V_M40: u32 = 0x07;
        }
    }
    #[doc = "Bias Current Control Adjustment"]
    pub mod USB2_REFBIAS_TST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "10 uA reference current; nominal"]
            pub const IREF_NOMINAL: u32 = 0;
            #[doc = "0.9x compared to nominal"]
            pub const IREF_0P9X: u32 = 0x01;
            #[doc = "0.8x compared to nominal"]
            pub const IREF_0P8X: u32 = 0x02;
            #[doc = "1.1x compared to nominal"]
            pub const IREF_1P1X: u32 = 0x03;
        }
    }
}
#[doc = "UTMI Debug Status 1"]
pub mod DEBUG1_SET {
    #[doc = "ENTAILADJVD"]
    pub mod ENTAILADJVD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to 1 to disable self bias, 100 us after power up refbias(usb2_refbias_pwd).This can reduce noise on power."]
    pub mod USB2_REFBIAS_SELFBIASOFF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the bandgap detect logic, will affect vbgup on misc1 register."]
    pub mod USB2_REFBIAS_PWDVBGUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "to be added"]
    pub mod USB2_REFBIAS_LOWPWR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Adjustment bits on bandgap"]
    pub mod USB2_REFBIAS_VBGADJ {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bias current control for usb2_phy"]
    pub mod USB2_REFBIAS_TST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTMI Debug Status 1"]
pub mod DEBUG1_CLR {
    #[doc = "ENTAILADJVD"]
    pub mod ENTAILADJVD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to 1 to disable self bias, 100 us after power up refbias(usb2_refbias_pwd).This can reduce noise on power."]
    pub mod USB2_REFBIAS_SELFBIASOFF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the bandgap detect logic, will affect vbgup on misc1 register."]
    pub mod USB2_REFBIAS_PWDVBGUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "to be added"]
    pub mod USB2_REFBIAS_LOWPWR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Adjustment bits on bandgap"]
    pub mod USB2_REFBIAS_VBGADJ {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bias current control for usb2_phy"]
    pub mod USB2_REFBIAS_TST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTMI Debug Status 1"]
pub mod DEBUG1_TOG {
    #[doc = "ENTAILADJVD"]
    pub mod ENTAILADJVD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to 1 to disable self bias, 100 us after power up refbias(usb2_refbias_pwd).This can reduce noise on power."]
    pub mod USB2_REFBIAS_SELFBIASOFF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Powers down the bandgap detect logic, will affect vbgup on misc1 register."]
    pub mod USB2_REFBIAS_PWDVBGUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "to be added"]
    pub mod USB2_REFBIAS_LOWPWR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Adjustment bits on bandgap"]
    pub mod USB2_REFBIAS_VBGADJ {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bias current control for usb2_phy"]
    pub mod USB2_REFBIAS_TST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Version"]
pub mod VERSION {
    #[doc = "Fixed read-only value reflecting the stepping of the RTL version."]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fixed read-only value reflecting the MINOR field of the RTL version."]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fixed read-only value reflecting the MAJOR field of the RTL version."]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY PLL Control and Status"]
pub mod PLL_SIC {
    #[doc = "PLL Post-Divider Output Value Configuration"]
    pub mod PLL_POSTDIV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the output of PLL post divider"]
            pub const POSTDIV0_DISABLE: u32 = 0;
            #[doc = "Divide value is 1"]
            pub const POSTDIV0_DIV1: u32 = 0x01;
            #[doc = "Divide value is 2"]
            pub const POSTDIV0_DIV2: u32 = 0x02;
            #[doc = "Divide value is 3"]
            pub const POSTDIV0_DIV3: u32 = 0x03;
            #[doc = "Divide value is 4"]
            pub const POSTDIV0_DIV4: u32 = 0x04;
            #[doc = "Divide value is 5"]
            pub const POSTDIV0_DIV5: u32 = 0x05;
            #[doc = "Divide value is 6"]
            pub const POSTDIV0_DIV6: u32 = 0x06;
        }
    }
    #[doc = "PLL USB Clocks Enable"]
    pub mod PLL_EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PLL Power"]
    pub mod PLL_POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PLL Enable"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PLL Bypass"]
    pub mod PLL_BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not bypass"]
            pub const DISABLE: u32 = 0;
            #[doc = "Bypass"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Reference Bias Powerdown Selection"]
    pub mod REFBIAS_PWD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selects PLL_POWER to control the reference bias"]
            pub const PLL_PWR: u32 = 0;
            #[doc = "Selects REFBIAS_PWD to control the reference bias."]
            pub const REFBIAS_PWD: u32 = 0x01;
        }
    }
    #[doc = "Reference Bias Powerdown"]
    pub mod REFBIAS_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PLL Regulator Enable"]
    pub mod PLL_REG_ENABLE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PLL Divider Selection"]
    pub mod PLL_DIV_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 13"]
            pub const DIV_BY_13: u32 = 0;
            #[doc = "Divide by 15"]
            pub const DIV_BY_15: u32 = 0x01;
            #[doc = "Divide by 16"]
            pub const DIV_BY_16: u32 = 0x02;
            #[doc = "Divide by 20"]
            pub const DIV_BY_20: u32 = 0x03;
            #[doc = "Divide by 22"]
            pub const DIV_BY_22: u32 = 0x04;
            #[doc = "Divide by 25"]
            pub const DIV_BY_25: u32 = 0x05;
            #[doc = "Divide by 30"]
            pub const DIV_BY_30: u32 = 0x06;
            #[doc = "Divide by 240"]
            pub const DIV_BY_240: u32 = 0x07;
        }
    }
    #[doc = "PLL Lock"]
    pub mod PLL_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL is not currently locked"]
            pub const NOT_LOCKED: u32 = 0;
            #[doc = "PLL is currently locked"]
            pub const LOCKED: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY PLL Control and Status"]
pub mod PLL_SIC_SET {
    #[doc = "PLL_POSTDIV"]
    pub mod PLL_POSTDIV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_EN_USB_CLKS"]
    pub mod PLL_EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_POWER"]
    pub mod PLL_POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_ENABLE"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_BYPASS"]
    pub mod PLL_BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REFBIAS_PWD_SEL"]
    pub mod REFBIAS_PWD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down the reference bias"]
    pub mod REFBIAS_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_REG_ENABLE"]
    pub mod PLL_REG_ENABLE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_DIV_SEL"]
    pub mod PLL_DIV_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_LOCK"]
    pub mod PLL_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY PLL Control and Status"]
pub mod PLL_SIC_CLR {
    #[doc = "PLL_POSTDIV"]
    pub mod PLL_POSTDIV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_EN_USB_CLKS"]
    pub mod PLL_EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_POWER"]
    pub mod PLL_POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_ENABLE"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_BYPASS"]
    pub mod PLL_BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REFBIAS_PWD_SEL"]
    pub mod REFBIAS_PWD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down the reference bias"]
    pub mod REFBIAS_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_REG_ENABLE"]
    pub mod PLL_REG_ENABLE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_DIV_SEL"]
    pub mod PLL_DIV_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_LOCK"]
    pub mod PLL_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY PLL Control and Status"]
pub mod PLL_SIC_TOG {
    #[doc = "PLL_POSTDIV"]
    pub mod PLL_POSTDIV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_EN_USB_CLKS"]
    pub mod PLL_EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_POWER"]
    pub mod PLL_POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_ENABLE"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_BYPASS"]
    pub mod PLL_BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REFBIAS_PWD_SEL"]
    pub mod REFBIAS_PWD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down the reference bias"]
    pub mod REFBIAS_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_REG_ENABLE"]
    pub mod PLL_REG_ENABLE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_DIV_SEL"]
    pub mod PLL_DIV_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL_LOCK"]
    pub mod PLL_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY VBUS Detect Control"]
pub mod USB1_VBUS_DETECT {
    #[doc = "VBUSVALID Threshold"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4.0 V"]
            pub const VOLT_4: u32 = 0;
            #[doc = "4.1 V"]
            pub const VOLT_4P1: u32 = 0x01;
            #[doc = "4.2 V"]
            pub const VOLT_4P2: u32 = 0x02;
            #[doc = "4.3 V"]
            pub const VOLT_4P3: u32 = 0x03;
            #[doc = "4.4 V (Default)"]
            pub const VOLT_4P4: u32 = 0x04;
            #[doc = "4.5 V"]
            pub const VOLT_4P5: u32 = 0x05;
            #[doc = "4.6 V"]
            pub const VOLT_4P6: u32 = 0x06;
            #[doc = "4.7 V"]
            pub const VOLT_4P7: u32 = 0x07;
        }
    }
    #[doc = "VBUS Detect Signal Override Enable"]
    pub mod VBUS_OVERRIDE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
            pub const INTERNAL: u32 = 0;
            #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
            pub const OVERRIDE: u32 = 0x01;
        }
    }
    #[doc = "Override Value for SESSEND"]
    pub mod SESSEND_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overridden to 0"]
            pub const OVERRIDE_0: u32 = 0;
            #[doc = "Overridden to 1"]
            pub const OVERRIDE_1: u32 = 0x01;
        }
    }
    #[doc = "Override Value for B-Device Session Valid"]
    pub mod BVALID_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overridden to 0"]
            pub const OVERRIDE_0: u32 = 0;
            #[doc = "Overridden to 1"]
            pub const OVERRIDE_1: u32 = 0x01;
        }
    }
    #[doc = "Override Value for A-Device Session Valid"]
    pub mod AVALID_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overridden to 0"]
            pub const OVERRIDE_0: u32 = 0;
            #[doc = "Overridden to 1"]
            pub const OVERRIDE_1: u32 = 0x01;
        }
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    pub mod VBUSVALID_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overridden to 0"]
            pub const OVERRIDE_0: u32 = 0;
            #[doc = "Overridden to 1"]
            pub const OVERRIDE_1: u32 = 0x01;
        }
    }
    #[doc = "VBUS_VALID Source Selection"]
    pub mod VBUSVALID_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
            pub const COMP: u32 = 0;
            #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
            pub const DET_3V: u32 = 0x01;
        }
    }
    #[doc = "VBUS_VALID Source Selection"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
            pub const VBUS_VALID_COMP: u32 = 0;
            #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
            pub const SESSION_VALID_COMP: u32 = 0x01;
            #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
            pub const SESSION_VALID_COMP_1: u32 = 0x02;
        }
    }
    #[doc = "Enable Local ID Pin Status Override"]
    pub mod ID_OVERRIDE_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not allow override"]
            pub const NO_PHY_ID_OVERRIDE: u32 = 0;
            #[doc = "Allow override"]
            pub const USE_PHY_ID_OVERRIDE: u32 = 0x01;
        }
    }
    #[doc = "ID Pin Status Local Override Value"]
    pub mod ID_OVERRIDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    pub mod VBUSVALID_TO_SESSVALID {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
            pub const VBUS_VALID: u32 = 0;
            #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
            pub const SESSION_VALID: u32 = 0x01;
        }
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    pub mod PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Powers down the VBUS_VALID comparator"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the SESS_VALID comparator (default)"]
            pub const ENABLE: u32 = 0x01;
            #[doc = "Enables the 3Vdetect (default)"]
            pub const VDETECT: u32 = 0x02;
        }
    }
    #[doc = "VBUS Discharge Resistor Controller"]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VBUS discharge resistor is disabled (Default)"]
            pub const DISABLE: u32 = 0;
            #[doc = "VBUS discharge resistor is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Charger Resistor Enable"]
    pub mod EN_CHARGER_RESISTOR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable resistive charger detection resistors on DP and DP"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable resistive charger detection resistors on DP and DP"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY VBUS Detect Control"]
pub mod USB1_VBUS_DETECT_SET {
    #[doc = "VBUSVALID_THRESH"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS detect signal override enable"]
    pub mod VBUS_OVERRIDE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for SESSEND"]
    pub mod SESSEND_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for B-Device Session Valid"]
    pub mod BVALID_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for A-Device Session Valid"]
    pub mod AVALID_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller"]
    pub mod VBUSVALID_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller"]
    pub mod VBUSVALID_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TBA"]
    pub mod ID_OVERRIDE_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TBA"]
    pub mod ID_OVERRIDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the comparator used for VBUS_VALID"]
    pub mod VBUSVALID_TO_SESSVALID {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the VBUS_VALID comparator"]
    pub mod PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls VBUS discharge resistor"]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    pub mod EN_CHARGER_RESISTOR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY VBUS Detect Control"]
pub mod USB1_VBUS_DETECT_CLR {
    #[doc = "VBUSVALID_THRESH"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS detect signal override enable"]
    pub mod VBUS_OVERRIDE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for SESSEND"]
    pub mod SESSEND_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for B-Device Session Valid"]
    pub mod BVALID_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for A-Device Session Valid"]
    pub mod AVALID_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller"]
    pub mod VBUSVALID_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller"]
    pub mod VBUSVALID_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TBA"]
    pub mod ID_OVERRIDE_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TBA"]
    pub mod ID_OVERRIDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the comparator used for VBUS_VALID"]
    pub mod VBUSVALID_TO_SESSVALID {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the VBUS_VALID comparator"]
    pub mod PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls VBUS discharge resistor"]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    pub mod EN_CHARGER_RESISTOR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY VBUS Detect Control"]
pub mod USB1_VBUS_DETECT_TOG {
    #[doc = "VBUSVALID_THRESH"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS detect signal override enable"]
    pub mod VBUS_OVERRIDE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for SESSEND"]
    pub mod SESSEND_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for B-Device Session Valid"]
    pub mod BVALID_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for A-Device Session Valid"]
    pub mod AVALID_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller"]
    pub mod VBUSVALID_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller"]
    pub mod VBUSVALID_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TBA"]
    pub mod ID_OVERRIDE_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TBA"]
    pub mod ID_OVERRIDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the comparator used for VBUS_VALID"]
    pub mod VBUSVALID_TO_SESSVALID {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the VBUS_VALID comparator"]
    pub mod PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls VBUS discharge resistor"]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    pub mod EN_CHARGER_RESISTOR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY VBUS Detector Status"]
pub mod USB1_VBUS_DET_STAT {
    #[doc = "Session End indicator"]
    pub mod SESSEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The VBUS voltage is above the Session Valid threshold"]
            pub const ABOVE: u32 = 0;
            #[doc = "The VBUS voltage is below the Session Valid threshold"]
            pub const BELOW: u32 = 0x01;
        }
    }
    #[doc = "B-Device Session Valid status"]
    pub mod BVALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The VBUS voltage is below the Session Valid threshold"]
            pub const BELOW: u32 = 0;
            #[doc = "The VBUS voltage is above the Session Valid threshold"]
            pub const ABOVE: u32 = 0x01;
        }
    }
    #[doc = "A-Device Session Valid status"]
    pub mod AVALID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The VBUS voltage is below the Session Valid threshold"]
            pub const BELOW: u32 = 0;
            #[doc = "The VBUS voltage is above the Session Valid threshold"]
            pub const ABOVE: u32 = 0x01;
        }
    }
    #[doc = "VBUS voltage status"]
    pub mod VBUS_VALID {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VBUS is below the comparator threshold"]
            pub const BELOW: u32 = 0;
            #[doc = "VBUS is above the comparator threshold"]
            pub const ABOVE: u32 = 0x01;
        }
    }
    #[doc = "VBUS_VALID_3V detector status"]
    pub mod VBUS_VALID_3V {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VBUS voltage is below VBUS_VALID_3V threshold"]
            pub const BELOW: u32 = 0;
            #[doc = "VBUS voltage is above VBUS_VALID_3V threshold"]
            pub const ABOVE: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY Charger Detect Control"]
pub mod USB1_CHRG_DETECT {
    #[doc = "DP Pullup Resistor Enable Override Control"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "BGR Bias"]
    pub mod BGR_BIAS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use local bias powered from USB1_VBUS for 10uA reference (Default)"]
            pub const LOCAL_BIAS: u32 = 0;
            #[doc = "Use bandgap bias powered from VREGIN0/VREGIN1 for 10uA reference"]
            pub const BANDGAP: u32 = 0x01;
        }
    }
    #[doc = "DCD Selection"]
    pub mod DCDSEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fields in the USB1_CHRG_DETECT register"]
            pub const USB1_CHRG_DETECT_FUNCTION: u32 = 0;
            #[doc = "Fields and state machines in the USBHSDCD module"]
            pub const USBHSDCD_FUNCTION: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY Charger Detect Control"]
pub mod USB1_CHRG_DETECT_SET {
    #[doc = "DP Pullup Resistor Enable Override Control"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BGR Bias"]
    pub mod BGR_BIAS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCD Selection"]
    pub mod DCDSEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Charger Detect Control"]
pub mod USB1_CHRG_DETECT_CLR {
    #[doc = "DP Pullup Resistor Enable Override Control"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BGR Bias"]
    pub mod BGR_BIAS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCD Selection"]
    pub mod DCDSEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Charger Detect Control"]
pub mod USB1_CHRG_DETECT_TOG {
    #[doc = "DP Pullup Resistor Enable Override Control"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BGR Bias"]
    pub mod BGR_BIAS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCD Selection"]
    pub mod DCDSEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Charger Detect Status"]
pub mod USB1_CHRG_DET_STAT {
    #[doc = "Battery Charging Data Contact Detection phase output"]
    pub mod PLUG_CONTACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No USB cable attachment has been detected"]
            pub const NO_ATTACH: u32 = 0;
            #[doc = "A USB cable attachment between the device and host has been detected"]
            pub const ATTACH: u32 = 0x01;
        }
    }
    #[doc = "Battery Charging Primary Detection phase output"]
    pub mod CHRG_DETECTED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard Downstream Port (SDP) has been detected"]
            pub const SDP: u32 = 0;
            #[doc = "Charging Port has been detected"]
            pub const CHRG_PORT: u32 = 0x01;
        }
    }
    #[doc = "DN_STATE"]
    pub mod DN_STATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DN pin voltage is < 0.8V"]
            pub const BELOW_P8: u32 = 0;
            #[doc = "DN pin voltage is > 2.0V"]
            pub const ABOVE_2: u32 = 0x01;
        }
    }
    #[doc = "DP_STATE"]
    pub mod DP_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DP pin voltage is < 0.8V"]
            pub const BELOW_P8: u32 = 0;
            #[doc = "DP pin voltage is > 2.0V"]
            pub const ABOVE_2: u32 = 0x01;
        }
    }
    #[doc = "Battery Charging Secondary Detection phase output"]
    pub mod SECDET_DCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Charging Downstream Port (CDP) has been detected"]
            pub const CDP: u32 = 0;
            #[doc = "Downstream Charging Port (DCP) has been detected"]
            pub const DCP: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY Analog Control"]
pub mod ANACTRL {
    #[doc = "DEV_PULLDOWN"]
    pub mod DEV_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The 15kohm nominal pulldowns on the DP and DN pinsare disabled in device mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "The 15kohm nominal pulldowns on the DP and DN pinsare enabled in device mode."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "USBPHY Analog Control"]
pub mod ANACTRL_SET {
    #[doc = "DEV_PULLDOWN"]
    pub mod DEV_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Analog Control"]
pub mod ANACTRL_CLR {
    #[doc = "DEV_PULLDOWN"]
    pub mod DEV_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Analog Control"]
pub mod ANACTRL_TOG {
    #[doc = "DEV_PULLDOWN"]
    pub mod DEV_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Loopback Control and Status"]
pub mod USB1_LOOPBACK {
    #[doc = "UTMI Test Start"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "UTMI Digital Test 0"]
    pub mod UTMI_DIG_TST0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pseudorandom mode"]
            pub const PSEUDORANDOM_MODE: u32 = 0;
            #[doc = "Pulse mode"]
            pub const PULSE_MODE: u32 = 0x01;
        }
    }
    #[doc = "UTMI Digital Test 1"]
    pub mod UTMI_DIG_TST1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pulse mode"]
            pub const PULSE_MODE: u32 = 0;
            #[doc = "Pseudorandom mode"]
            pub const PSEUDORANDOM_MODE: u32 = 0x01;
        }
    }
    #[doc = "Loopback Test HS Mode"]
    pub mod TSTI_TX_HS_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FS"]
            pub const FS_MODE: u32 = 0;
            #[doc = "HS"]
            pub const HS_MODE: u32 = 0x01;
        }
    }
    #[doc = "Loopback Test LS Mode"]
    pub mod TSTI_TX_LS_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HS or FS (defined by TSTI1_TX_HS)"]
            pub const FS_MODE: u32 = 0;
            #[doc = "LS"]
            pub const LS_MODE: u32 = 0x01;
        }
    }
    #[doc = "Loopback Test Transmit Enable"]
    pub mod TSTI_TX_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Loopback Test Transmit Hi-Z"]
    pub mod TSTI_TX_HIZ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "UTMO Digital Test 0"]
    pub mod UTMO_DIG_TST0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Passing"]
            pub const PASSING: u32 = 0;
            #[doc = "Not passing"]
            pub const NO_PASSING: u32 = 0x01;
        }
    }
    #[doc = "UTMO Digital Test 1"]
    pub mod UTMO_DIG_TST1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not passing"]
            pub const NO_PASSING: u32 = 0;
            #[doc = "Passing"]
            pub const PASSING: u32 = 0x01;
        }
    }
    #[doc = "Loopback Test HS-FS Mode Enable"]
    pub mod TSTI_HSFS_MODE_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Testing Packet"]
    pub mod TSTPKT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Loopback Control and Status"]
pub mod USB1_LOOPBACK_SET {
    #[doc = "UTMI_TESTSTART"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_DIG_TST0"]
    pub mod UTMI_DIG_TST0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_DIG_TST1"]
    pub mod UTMI_DIG_TST1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_HS_MODE"]
    pub mod TSTI_TX_HS_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_LS_MODE"]
    pub mod TSTI_TX_LS_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_EN"]
    pub mod TSTI_TX_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_HIZ"]
    pub mod TSTI_TX_HIZ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMO_DIG_TST0"]
    pub mod UTMO_DIG_TST0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMO_DIG_TST1"]
    pub mod UTMO_DIG_TST1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_HSFS_MODE_EN"]
    pub mod TSTI_HSFS_MODE_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTPKT"]
    pub mod TSTPKT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Loopback Control and Status"]
pub mod USB1_LOOPBACK_CLR {
    #[doc = "UTMI_TESTSTART"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_DIG_TST0"]
    pub mod UTMI_DIG_TST0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_DIG_TST1"]
    pub mod UTMI_DIG_TST1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_HS_MODE"]
    pub mod TSTI_TX_HS_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_LS_MODE"]
    pub mod TSTI_TX_LS_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_EN"]
    pub mod TSTI_TX_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_HIZ"]
    pub mod TSTI_TX_HIZ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMO_DIG_TST0"]
    pub mod UTMO_DIG_TST0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMO_DIG_TST1"]
    pub mod UTMO_DIG_TST1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_HSFS_MODE_EN"]
    pub mod TSTI_HSFS_MODE_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTPKT"]
    pub mod TSTPKT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Loopback Control and Status"]
pub mod USB1_LOOPBACK_TOG {
    #[doc = "UTMI_TESTSTART"]
    pub mod UTMI_TESTSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_DIG_TST0"]
    pub mod UTMI_DIG_TST0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_DIG_TST1"]
    pub mod UTMI_DIG_TST1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_HS_MODE"]
    pub mod TSTI_TX_HS_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_LS_MODE"]
    pub mod TSTI_TX_LS_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_EN"]
    pub mod TSTI_TX_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_TX_HIZ"]
    pub mod TSTI_TX_HIZ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMO_DIG_TST0"]
    pub mod UTMO_DIG_TST0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMO_DIG_TST1"]
    pub mod UTMO_DIG_TST1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_HSFS_MODE_EN"]
    pub mod TSTI_HSFS_MODE_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTPKT"]
    pub mod TSTPKT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Loopback Packet Number Selection"]
pub mod USB1_LOOPBACK_HSFSCNT {
    #[doc = "TSTI_HS_NUMBER"]
    pub mod TSTI_HS_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_FS_NUMBER"]
    pub mod TSTI_FS_NUMBER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Loopback Packet Number Selection"]
pub mod USB1_LOOPBACK_HSFSCNT_SET {
    #[doc = "TSTI_HS_NUMBER"]
    pub mod TSTI_HS_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_FS_NUMBER"]
    pub mod TSTI_FS_NUMBER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Loopback Packet Number Selection"]
pub mod USB1_LOOPBACK_HSFSCNT_CLR {
    #[doc = "TSTI_HS_NUMBER"]
    pub mod TSTI_HS_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_FS_NUMBER"]
    pub mod TSTI_FS_NUMBER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Loopback Packet Number Selection"]
pub mod USB1_LOOPBACK_HSFSCNT_TOG {
    #[doc = "TSTI_HS_NUMBER"]
    pub mod TSTI_HS_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSTI_FS_NUMBER"]
    pub mod TSTI_FS_NUMBER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Trim Override Enable"]
pub mod TRIM_OVERRIDE_EN {
    #[doc = "Override Enable for PLL Divider Value"]
    pub mod TRIM_DIV_SEL_OVERRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Override Enable for HS RX Squelch Rise Time Delay Trim"]
    pub mod TRIM_ENV_TAIL_ADJ_VD_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Override Enable for the HS TX Output Current Trim"]
    pub mod TRIM_TX_D_CAL_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Override Enable for DP Series Termination Trim"]
    pub mod TRIM_TX_CAL45DP_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Override Enable for DN Series Termination Trim"]
    pub mod TRIM_TX_CAL45DN_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Override Enable for Bandgap Voltage Adjustment"]
    pub mod TRIM_REFBIAS_VBGADJ_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Override Enable for Bias Current Control"]
    pub mod TRIM_REFBIAS_TST_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Bandgap Voltage Adjustment Bits from Outside USBPHY"]
    pub mod TRIM_USB2_REFBIAS_VBGADJ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bias Current Control Adjustment Bits from Outside USBPHY"]
    pub mod TRIM_USB2_REFBIAS_TST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    pub mod TRIM_PLL_CTRL0_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HS RX Squelch Rise Time Delay Trim Bits from Outside USBPHY"]
    pub mod TRIM_USB_REG_ENV_TAIL_ADJ_VD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    pub mod TRIM_USBPHY_TX_D_CAL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Maximum current; approximately 19% above nominal"]
            pub const MAX: u32 = 0;
            #[doc = "Nominal"]
            pub const NOM: u32 = 0x07;
            #[doc = "Minimum current; approximately 19% below nominal"]
            pub const MIN: u32 = 0x0f;
        }
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    pub mod TRIM_USBPHY_TX_CAL45DP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DN Series Termination Resistance Trim Bits from Outside USBPHY"]
    pub mod TRIM_USBPHY_TX_CAL45DN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Trim Override Enable"]
pub mod TRIM_OVERRIDE_EN_SET {
    #[doc = "TRIM_DIV_SEL_OVERRIDE"]
    pub mod TRIM_DIV_SEL_OVERRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_ENV_TAIL_ADJ_VD_OVERRIDE"]
    pub mod TRIM_ENV_TAIL_ADJ_VD_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_D_CAL_OVERRIDE"]
    pub mod TRIM_TX_D_CAL_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_CAL45DP_OVERRIDE"]
    pub mod TRIM_TX_CAL45DP_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_CAL45DN_OVERRIDE"]
    pub mod TRIM_TX_CAL45DN_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for bandgap adjustment."]
    pub mod TRIM_REFBIAS_VBGADJ_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for bias current control"]
    pub mod TRIM_REFBIAS_TST_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB2_REFBIAS_VBGADJ"]
    pub mod TRIM_USB2_REFBIAS_VBGADJ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB2_REFBIAS_TST"]
    pub mod TRIM_USB2_REFBIAS_TST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_PLL_CTRL0_DIV_SEL"]
    pub mod TRIM_PLL_CTRL0_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB_REG_ENV_TAIL_ADJ_VD"]
    pub mod TRIM_USB_REG_ENV_TAIL_ADJ_VD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_D_CAL"]
    pub mod TRIM_USBPHY_TX_D_CAL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_CAL45DP"]
    pub mod TRIM_USBPHY_TX_CAL45DP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_CAL45DN"]
    pub mod TRIM_USBPHY_TX_CAL45DN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Trim Override Enable"]
pub mod TRIM_OVERRIDE_EN_CLR {
    #[doc = "TRIM_DIV_SEL_OVERRIDE"]
    pub mod TRIM_DIV_SEL_OVERRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_ENV_TAIL_ADJ_VD_OVERRIDE"]
    pub mod TRIM_ENV_TAIL_ADJ_VD_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_D_CAL_OVERRIDE"]
    pub mod TRIM_TX_D_CAL_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_CAL45DP_OVERRIDE"]
    pub mod TRIM_TX_CAL45DP_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_CAL45DN_OVERRIDE"]
    pub mod TRIM_TX_CAL45DN_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for bandgap adjustment."]
    pub mod TRIM_REFBIAS_VBGADJ_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for bias current control"]
    pub mod TRIM_REFBIAS_TST_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB2_REFBIAS_VBGADJ"]
    pub mod TRIM_USB2_REFBIAS_VBGADJ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB2_REFBIAS_TST"]
    pub mod TRIM_USB2_REFBIAS_TST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_PLL_CTRL0_DIV_SEL"]
    pub mod TRIM_PLL_CTRL0_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB_REG_ENV_TAIL_ADJ_VD"]
    pub mod TRIM_USB_REG_ENV_TAIL_ADJ_VD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_D_CAL"]
    pub mod TRIM_USBPHY_TX_D_CAL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_CAL45DP"]
    pub mod TRIM_USBPHY_TX_CAL45DP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_CAL45DN"]
    pub mod TRIM_USBPHY_TX_CAL45DN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USBPHY Trim Override Enable"]
pub mod TRIM_OVERRIDE_EN_TOG {
    #[doc = "TRIM_DIV_SEL_OVERRIDE"]
    pub mod TRIM_DIV_SEL_OVERRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_ENV_TAIL_ADJ_VD_OVERRIDE"]
    pub mod TRIM_ENV_TAIL_ADJ_VD_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_D_CAL_OVERRIDE"]
    pub mod TRIM_TX_D_CAL_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_CAL45DP_OVERRIDE"]
    pub mod TRIM_TX_CAL45DP_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_TX_CAL45DN_OVERRIDE"]
    pub mod TRIM_TX_CAL45DN_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for bandgap adjustment."]
    pub mod TRIM_REFBIAS_VBGADJ_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for bias current control"]
    pub mod TRIM_REFBIAS_TST_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB2_REFBIAS_VBGADJ"]
    pub mod TRIM_USB2_REFBIAS_VBGADJ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB2_REFBIAS_TST"]
    pub mod TRIM_USB2_REFBIAS_TST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_PLL_CTRL0_DIV_SEL"]
    pub mod TRIM_PLL_CTRL0_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USB_REG_ENV_TAIL_ADJ_VD"]
    pub mod TRIM_USB_REG_ENV_TAIL_ADJ_VD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_D_CAL"]
    pub mod TRIM_USBPHY_TX_D_CAL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_CAL45DP"]
    pub mod TRIM_USBPHY_TX_CAL45DP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRIM_USBPHY_TX_CAL45DN"]
    pub mod TRIM_USBPHY_TX_CAL45DN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
