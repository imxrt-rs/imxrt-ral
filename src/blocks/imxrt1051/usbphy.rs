#[doc = "USBPHY Register Reference Index"]
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
}
#[doc = "USB PHY Power-Down Register"]
pub mod PWD {
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x0f << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Power-Down Register"]
pub mod PWD_SET {
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x0f << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Power-Down Register"]
pub mod PWD_CLR {
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x0f << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Power-Down Register"]
pub mod PWD_TOG {
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x0f << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod TX {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    pub mod USBPHY_TX_EDGECTRL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod TX_SET {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    pub mod USBPHY_TX_EDGECTRL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod TX_CLR {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    pub mod USBPHY_TX_EDGECTRL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod TX_TOG {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    pub mod USBPHY_TX_EDGECTRL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD5 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY Receiver Control Register"]
pub mod RX {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x7fff << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x7fff << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01ff << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x7fff << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01ff << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x7fff << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB PHY General Control Register"]
pub mod CTRL {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device is connected"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    pub mod DATA_ON_LRADC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables UTMI+ Level3"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for the wakeup events."]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that there is a wakeup event"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    pub mod ENAUTO_PWRON_PLL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    pub mod ENDPDMCHG_WKUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    pub mod ENIDCHG_WKUP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    pub mod ENVBUSCHG_WKUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate UTMI Clocks"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
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
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device is connected"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    pub mod DATA_ON_LRADC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables UTMI+ Level3"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for the wakeup events."]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that there is a wakeup event"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    pub mod ENAUTO_PWRON_PLL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    pub mod ENDPDMCHG_WKUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    pub mod ENIDCHG_WKUP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    pub mod ENVBUSCHG_WKUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate UTMI Clocks"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
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
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device is connected"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    pub mod DATA_ON_LRADC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables UTMI+ Level3"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for the wakeup events."]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that there is a wakeup event"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    pub mod ENAUTO_PWRON_PLL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    pub mod ENDPDMCHG_WKUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    pub mod ENIDCHG_WKUP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    pub mod ENVBUSCHG_WKUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate UTMI Clocks"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
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
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device is connected"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    pub mod DATA_ON_LRADC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables UTMI+ Level3"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables interrupt for the wakeup events."]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that there is a wakeup event"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    pub mod ENAUTO_PWRON_PLL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    pub mod ENDPDMCHG_WKUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    pub mod ENIDCHG_WKUP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    pub mod ENVBUSCHG_WKUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    pub mod FSDLL_RST_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    pub mod HOST_FORCE_LS_SE0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate UTMI Clocks"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
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
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device has disconnected while in high-speed host mode."]
    pub mod HOSTDISCONDETECT_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the device has been connected on the USB_DP and USB_DM lines."]
    pub mod DEVPLUGIN_STATUS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the results of ID pin on MiniAB plug"]
    pub mod OTGID_STATUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD3 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend and has triggered an interrupt."]
    pub mod RESUME_STATUS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD4 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x001f_ffff << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 6;
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD3 {
        pub const offset: u32 = 31;
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
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 6;
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD3 {
        pub const offset: u32 = 31;
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
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 6;
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD3 {
        pub const offset: u32 = 31;
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
    #[doc = "Reserved."]
    pub mod RSVD0 {
        pub const offset: u32 = 6;
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
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Reserved."]
    pub mod RSVD3 {
        pub const offset: u32 = 31;
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
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    pub mod ENTAILADJVD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTMI Debug Status Register 1"]
pub mod DEBUG1_SET {
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    pub mod ENTAILADJVD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTMI Debug Status Register 1"]
pub mod DEBUG1_CLR {
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    pub mod ENTAILADJVD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTMI Debug Status Register 1"]
pub mod DEBUG1_TOG {
    #[doc = "Reserved. Note: This bit should remain clear."]
    pub mod RSVD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    pub mod ENTAILADJVD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RSVD1 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0001_ffff << offset;
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
