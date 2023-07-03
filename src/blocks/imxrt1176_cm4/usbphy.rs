#[doc = "USBPHY"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "USB PHY Power-Down Register"]
    pub PWD: crate::RWRegister<u32>,
    #[doc = "USB PHY Power-Down Register"]
    pub PWD_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Power-Down Register"]
    pub PWD_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Power-Down Register"]
    pub PWD_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY Transmitter Control Register"]
    pub TX: crate::RWRegister<u32>,
    #[doc = "USB PHY Transmitter Control Register"]
    pub TX_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Transmitter Control Register"]
    pub TX_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Transmitter Control Register"]
    pub TX_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY Receiver Control Register"]
    pub RX: crate::RWRegister<u32>,
    #[doc = "USB PHY Receiver Control Register"]
    pub RX_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Receiver Control Register"]
    pub RX_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Receiver Control Register"]
    pub RX_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY General Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "USB PHY General Control Register"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY General Control Register"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY General Control Register"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY Status Register"]
    pub STATUS: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "USB PHY Debug Register"]
    pub DEBUG: crate::RWRegister<u32>,
    #[doc = "USB PHY Debug Register"]
    pub DEBUG_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Debug Register"]
    pub DEBUG_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Debug Register"]
    pub DEBUG_TOG: crate::RWRegister<u32>,
    #[doc = "UTMI Debug Status Register 0"]
    pub DEBUG0_STATUS: crate::RORegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "UTMI Debug Status Register 1"]
    pub DEBUG1: crate::RWRegister<u32>,
    #[doc = "UTMI Debug Status Register 1"]
    pub DEBUG1_SET: crate::RWRegister<u32>,
    #[doc = "UTMI Debug Status Register 1"]
    pub DEBUG1_CLR: crate::RWRegister<u32>,
    #[doc = "UTMI Debug Status Register 1"]
    pub DEBUG1_TOG: crate::RWRegister<u32>,
    #[doc = "UTMI RTL Version"]
    pub VERSION: crate::RORegister<u32>,
    _reserved2: [u8; 0x1c],
    #[doc = "USB PHY PLL Control/Status Register"]
    pub PLL_SIC: crate::RWRegister<u32>,
    #[doc = "USB PHY PLL Control/Status Register"]
    pub PLL_SIC_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY PLL Control/Status Register"]
    pub PLL_SIC_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY PLL Control/Status Register"]
    pub PLL_SIC_TOG: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "USB PHY VBUS Detect Control Register"]
    pub USB1_VBUS_DETECT: crate::RWRegister<u32>,
    #[doc = "USB PHY VBUS Detect Control Register"]
    pub USB1_VBUS_DETECT_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY VBUS Detect Control Register"]
    pub USB1_VBUS_DETECT_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY VBUS Detect Control Register"]
    pub USB1_VBUS_DETECT_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY VBUS Detector Status Register"]
    pub USB1_VBUS_DET_STAT: crate::RORegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "USB PHY Charger Detect Control Register"]
    pub USB1_CHRG_DETECT: crate::RWRegister<u32>,
    #[doc = "USB PHY Charger Detect Control Register"]
    pub USB1_CHRG_DETECT_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Charger Detect Control Register"]
    pub USB1_CHRG_DETECT_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Charger Detect Control Register"]
    pub USB1_CHRG_DETECT_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY Charger Detect Status Register"]
    pub USB1_CHRG_DET_STAT: crate::RORegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "USB PHY Analog Control Register"]
    pub ANACTRL: crate::RWRegister<u32>,
    #[doc = "USB PHY Analog Control Register"]
    pub ANACTRL_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Analog Control Register"]
    pub ANACTRL_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Analog Control Register"]
    pub ANACTRL_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY Loopback Control/Status Register"]
    pub USB1_LOOPBACK: crate::RWRegister<u32>,
    #[doc = "USB PHY Loopback Control/Status Register"]
    pub USB1_LOOPBACK_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Loopback Control/Status Register"]
    pub USB1_LOOPBACK_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Loopback Control/Status Register"]
    pub USB1_LOOPBACK_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY Loopback Packet Number Select Register"]
    pub USB1_LOOPBACK_HSFSCNT: crate::RWRegister<u32>,
    #[doc = "USB PHY Loopback Packet Number Select Register"]
    pub USB1_LOOPBACK_HSFSCNT_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Loopback Packet Number Select Register"]
    pub USB1_LOOPBACK_HSFSCNT_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Loopback Packet Number Select Register"]
    pub USB1_LOOPBACK_HSFSCNT_TOG: crate::RWRegister<u32>,
    #[doc = "USB PHY Trim Override Enable Register"]
    pub TRIM_OVERRIDE_EN: crate::RWRegister<u32>,
    #[doc = "USB PHY Trim Override Enable Register"]
    pub TRIM_OVERRIDE_EN_SET: crate::RWRegister<u32>,
    #[doc = "USB PHY Trim Override Enable Register"]
    pub TRIM_OVERRIDE_EN_CLR: crate::RWRegister<u32>,
    #[doc = "USB PHY Trim Override Enable Register"]
    pub TRIM_OVERRIDE_EN_TOG: crate::RWRegister<u32>,
}
#[doc = "USB PHY Power-Down Register"]
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
#[doc = "USB PHY Power-Down Register"]
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
#[doc = "USB PHY Power-Down Register"]
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
#[doc = "USB PHY Power-Down Register"]
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
#[doc = "USB PHY Transmitter Control Register"]
pub mod TX {
    #[doc = "D_CAL"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Maximum current, approximately 19% above nominal."]
            pub const MAX: u32 = 0;
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0x07;
            #[doc = "Minimum current, approximately 19% below nominal."]
            pub const MIN: u32 = 0x0f;
        }
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
#[doc = "USB PHY Transmitter Control Register"]
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
#[doc = "USB PHY Transmitter Control Register"]
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
#[doc = "USB PHY Transmitter Control Register"]
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
#[doc = "USB PHY Receiver Control Register"]
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
#[doc = "USB PHY Receiver Control Register"]
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
#[doc = "USB PHY Receiver Control Register"]
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
#[doc = "USB PHY Receiver Control Register"]
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
#[doc = "USB PHY General Control Register"]
pub mod CTRL {
    #[doc = "ENOTG_ID_CHG_IRQ"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENHOSTDISCONDETECT"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQHOSTDISCON"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HOSTDISCONDETECT_IRQ"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables non-standard resistive plugged-in detection"]
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
    #[doc = "DEVPLUGIN_POLARITY"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG_ID_CHG_IRQ"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENOTGIDDETECT"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUMEIRQSTICKY"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQRESUMEDETECT"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUME_IRQ"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQDEVPLUGIN"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DEVPLUGIN_IRQ"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENUTMILEVEL2"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENUTMILEVEL3"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQWAKEUP"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WAKEUP_IRQ"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AUTORESUME_EN"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENAUTOCLR_CLKGATE"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENAUTOCLR_PHY_PWD"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENDPDMCHG_WKUP"]
    pub mod ENDPDMCHG_WKUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIDCHG_WKUP"]
    pub mod ENIDCHG_WKUP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENVBUSCHG_WKUP"]
    pub mod ENVBUSCHG_WKUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FSDLL_RST_EN"]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG_ID_VALUE"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HOST_FORCE_LS_SE0"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_SUSPENDM"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CLKGATE"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SFTRST"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY General Control Register"]
pub mod CTRL_SET {
    #[doc = "ENOTG_ID_CHG_IRQ"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENHOSTDISCONDETECT"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQHOSTDISCON"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HOSTDISCONDETECT_IRQ"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables non-standard resistive plugged-in detection"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DEVPLUGIN_POLARITY"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG_ID_CHG_IRQ"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENOTGIDDETECT"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUMEIRQSTICKY"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQRESUMEDETECT"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUME_IRQ"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQDEVPLUGIN"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DEVPLUGIN_IRQ"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENUTMILEVEL2"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENUTMILEVEL3"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQWAKEUP"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WAKEUP_IRQ"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AUTORESUME_EN"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENAUTOCLR_CLKGATE"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENAUTOCLR_PHY_PWD"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENDPDMCHG_WKUP"]
    pub mod ENDPDMCHG_WKUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIDCHG_WKUP"]
    pub mod ENIDCHG_WKUP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENVBUSCHG_WKUP"]
    pub mod ENVBUSCHG_WKUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FSDLL_RST_EN"]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG_ID_VALUE"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HOST_FORCE_LS_SE0"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_SUSPENDM"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CLKGATE"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SFTRST"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY General Control Register"]
pub mod CTRL_CLR {
    #[doc = "ENOTG_ID_CHG_IRQ"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENHOSTDISCONDETECT"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQHOSTDISCON"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HOSTDISCONDETECT_IRQ"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables non-standard resistive plugged-in detection"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DEVPLUGIN_POLARITY"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG_ID_CHG_IRQ"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENOTGIDDETECT"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUMEIRQSTICKY"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQRESUMEDETECT"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUME_IRQ"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQDEVPLUGIN"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DEVPLUGIN_IRQ"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENUTMILEVEL2"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENUTMILEVEL3"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQWAKEUP"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WAKEUP_IRQ"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AUTORESUME_EN"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENAUTOCLR_CLKGATE"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENAUTOCLR_PHY_PWD"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENDPDMCHG_WKUP"]
    pub mod ENDPDMCHG_WKUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIDCHG_WKUP"]
    pub mod ENIDCHG_WKUP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENVBUSCHG_WKUP"]
    pub mod ENVBUSCHG_WKUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FSDLL_RST_EN"]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG_ID_VALUE"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HOST_FORCE_LS_SE0"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_SUSPENDM"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CLKGATE"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SFTRST"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY General Control Register"]
pub mod CTRL_TOG {
    #[doc = "ENOTG_ID_CHG_IRQ"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENHOSTDISCONDETECT"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQHOSTDISCON"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HOSTDISCONDETECT_IRQ"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables non-standard resistive plugged-in detection"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DEVPLUGIN_POLARITY"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG_ID_CHG_IRQ"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENOTGIDDETECT"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUMEIRQSTICKY"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQRESUMEDETECT"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUME_IRQ"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQDEVPLUGIN"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DEVPLUGIN_IRQ"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENUTMILEVEL2"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENUTMILEVEL3"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIRQWAKEUP"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WAKEUP_IRQ"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AUTORESUME_EN"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENAUTOCLR_CLKGATE"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENAUTOCLR_PHY_PWD"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENDPDMCHG_WKUP"]
    pub mod ENDPDMCHG_WKUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENIDCHG_WKUP"]
    pub mod ENIDCHG_WKUP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENVBUSCHG_WKUP"]
    pub mod ENVBUSCHG_WKUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FSDLL_RST_EN"]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG_ID_VALUE"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HOST_FORCE_LS_SE0"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI_SUSPENDM"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CLKGATE"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SFTRST"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Status Register"]
pub mod STATUS {
    #[doc = "HOSTDISCONDETECT_STATUS"]
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
    #[doc = "Status indicator for non-standard resistive plugged-in detection"]
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
    #[doc = "OTGID_STATUS"]
    pub mod OTGID_STATUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESUME_STATUS"]
    pub mod RESUME_STATUS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Debug Register"]
pub mod DEBUG {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    pub mod DEBUG_INTERFACE_HOLD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    pub mod TX2RXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    pub mod ENTX2RXCOUNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    pub mod SQUELCHRESETCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    pub mod ENSQUELCHRESET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    pub mod SQUELCHRESETLENGTH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    pub mod HOST_RESUME_DEBUG {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate Test Clocks"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Debug Register"]
pub mod DEBUG_SET {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    pub mod DEBUG_INTERFACE_HOLD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    pub mod TX2RXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    pub mod ENTX2RXCOUNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    pub mod SQUELCHRESETCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    pub mod ENSQUELCHRESET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    pub mod SQUELCHRESETLENGTH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    pub mod HOST_RESUME_DEBUG {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate Test Clocks"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Debug Register"]
pub mod DEBUG_CLR {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    pub mod DEBUG_INTERFACE_HOLD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    pub mod TX2RXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    pub mod ENTX2RXCOUNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    pub mod SQUELCHRESETCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    pub mod ENSQUELCHRESET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    pub mod SQUELCHRESETLENGTH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    pub mod HOST_RESUME_DEBUG {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate Test Clocks"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Debug Register"]
pub mod DEBUG_TOG {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    pub mod DEBUG_INTERFACE_HOLD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    pub mod TX2RXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    pub mod ENTX2RXCOUNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    pub mod SQUELCHRESETCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    pub mod ENSQUELCHRESET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    pub mod SQUELCHRESETLENGTH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    pub mod HOST_RESUME_DEBUG {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate Test Clocks"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTMI Debug Status Register 0"]
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
#[doc = "UTMI Debug Status Register 1"]
pub mod DEBUG1 {
    #[doc = "ENTAILADJVD"]
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
#[doc = "UTMI Debug Status Register 1"]
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
#[doc = "UTMI Debug Status Register 1"]
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
#[doc = "UTMI Debug Status Register 1"]
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
#[doc = "UTMI RTL Version"]
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
#[doc = "USB PHY PLL Control/Status Register"]
pub mod PLL_SIC {
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
        pub mod RW {
            #[doc = "Selects PLL_POWER to control the reference bias"]
            pub const PLL_PWR: u32 = 0;
            #[doc = "Selects REFBIAS_PWD to control the reference bias."]
            pub const REFBIAS_PWD: u32 = 0x01;
        }
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
    #[doc = "PLL_LOCK"]
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
#[doc = "USB PHY PLL Control/Status Register"]
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
#[doc = "USB PHY PLL Control/Status Register"]
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
#[doc = "USB PHY PLL Control/Status Register"]
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
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod USB1_VBUS_DETECT {
    #[doc = "VBUSVALID_THRESH"]
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
    #[doc = "VBUS detect signal override enable"]
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
        pub mod RW {
            #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
            pub const COMP: u32 = 0;
            #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
            pub const DET_3V: u32 = 0x01;
        }
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller"]
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
        pub mod RW {
            #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
            pub const VBUS_VALID: u32 = 0;
            #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
            pub const SESSION_VALID: u32 = 0x01;
        }
    }
    #[doc = "Enables the VBUS_VALID comparator"]
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
    #[doc = "Controls VBUS discharge resistor"]
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
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
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
#[doc = "USB PHY VBUS Detect Control Register"]
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
#[doc = "USB PHY VBUS Detect Control Register"]
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
#[doc = "USB PHY VBUS Detect Control Register"]
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
#[doc = "USB PHY VBUS Detector Status Register"]
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
#[doc = "USB PHY Charger Detect Control Register"]
pub mod USB1_CHRG_DETECT {
    #[doc = "PULLUP_DP"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BGR_BIAS"]
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
}
#[doc = "USB PHY Charger Detect Control Register"]
pub mod USB1_CHRG_DETECT_SET {
    #[doc = "PULLUP_DP"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BGR_BIAS"]
    pub mod BGR_BIAS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Charger Detect Control Register"]
pub mod USB1_CHRG_DETECT_CLR {
    #[doc = "PULLUP_DP"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BGR_BIAS"]
    pub mod BGR_BIAS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Charger Detect Control Register"]
pub mod USB1_CHRG_DETECT_TOG {
    #[doc = "PULLUP_DP"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BGR_BIAS"]
    pub mod BGR_BIAS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Charger Detect Status Register"]
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
#[doc = "USB PHY Analog Control Register"]
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
#[doc = "USB PHY Analog Control Register"]
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
#[doc = "USB PHY Analog Control Register"]
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
#[doc = "USB PHY Analog Control Register"]
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
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod USB1_LOOPBACK {
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
#[doc = "USB PHY Loopback Control/Status Register"]
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
#[doc = "USB PHY Loopback Control/Status Register"]
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
#[doc = "USB PHY Loopback Control/Status Register"]
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
#[doc = "USB PHY Loopback Packet Number Select Register"]
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
#[doc = "USB PHY Loopback Packet Number Select Register"]
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
#[doc = "USB PHY Loopback Packet Number Select Register"]
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
#[doc = "USB PHY Loopback Packet Number Select Register"]
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
#[doc = "USB PHY Trim Override Enable Register"]
pub mod TRIM_OVERRIDE_EN {
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
#[doc = "USB PHY Trim Override Enable Register"]
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
#[doc = "USB PHY Trim Override Enable Register"]
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
#[doc = "USB PHY Trim Override Enable Register"]
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
