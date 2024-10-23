#[doc = "NETC EMDIO base function"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c00],
    #[doc = "External MDIO configuration register"]
    pub EMDIO_CFG: crate::RWRegister<u32>,
    #[doc = "External MDIO interface control register"]
    pub EMDIO_CTL: crate::RWRegister<u32>,
    #[doc = "External MDIO interface data register"]
    pub EMDIO_DATA: crate::RWRegister<u32>,
    #[doc = "External MDIO register address register"]
    pub EMDIO_ADDR: crate::RWRegister<u32>,
    #[doc = "External MDIO status register"]
    pub EMDIO_STAT: crate::RORegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "PHY status configuration register"]
    pub PHY_STATUS_CFG: crate::RWRegister<u32>,
    #[doc = "PHY status control register"]
    pub PHY_STATUS_CTL: crate::RWRegister<u32>,
    #[doc = "PHY status data register"]
    pub PHY_STATUS_DATA: crate::RORegister<u32>,
    #[doc = "PHY status register address register"]
    pub PHY_STATUS_ADDR: crate::RWRegister<u32>,
    #[doc = "PHY status event register"]
    pub PHY_STATUS_EVENT: crate::RWRegister<u32>,
    #[doc = "PHY status mask register"]
    pub PHY_STATUS_MASK: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "MDIO configuration register"]
    pub MDIO_CFG: crate::RORegister<u32>,
}
#[doc = "External MDIO configuration register"]
pub mod EMDIO_CFG {
    #[doc = "Busy 2 (same as bit 31)"]
    pub mod BSY2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An MDIO transaction is not occurring; software may access other MDIO registers."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO transaction is occurring."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Read Error"]
    pub mod MDIO_RD_ER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const ZERO: u32 = 0;
            #[doc = "The last read transaction received no response from a PHY; any data read should be considered invalid (for example, the PHY address does not match any PHY available on the MDIO bus)."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Hold Time"]
    pub mod MDIO_HOLD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 NETC cycle"]
            pub const NETC1: u32 = 0;
            #[doc = "3 NETC cycles"]
            pub const NETC3: u32 = 0x01;
            #[doc = "5 NETC cycles (default - recommended value)"]
            pub const NETC5: u32 = 0x02;
            #[doc = "7 NETC cycles"]
            pub const NETC7: u32 = 0x03;
            #[doc = "9 NETC cycles"]
            pub const NETC9: u32 = 0x04;
            #[doc = "11 NETC cycles"]
            pub const NETC11: u32 = 0x05;
            #[doc = "13 NETC cycles"]
            pub const NETC13: u32 = 0x06;
            #[doc = "15 NETC cycles"]
            pub const NETC15: u32 = 0x07;
        }
    }
    #[doc = "MDIO Preamble Disable"]
    pub mod PRE_DIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generation of MDIO preamble is enabled (default operation)."]
            pub const ZERO: u32 = 0;
            #[doc = "Generation of MDIO preamble is disabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Enable Clause 45 Support"]
    pub mod ENC45 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clause 22 transactions are used."]
            pub const ZERO: u32 = 0;
            #[doc = "Clause 45 transactions are used."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Clock Divisor"]
    pub mod MDIO_CLK_DIV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Returns the virtual port ID."]
    pub mod WHOAMI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended HOLD"]
    pub mod EHOLD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation. MDIO hold time is specified in ."]
            pub const ZERO: u32 = 0;
            #[doc = "Extended Operation"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Negative Edge"]
    pub mod NEG {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation - positive edge"]
            pub const NORMAL: u32 = 0;
            #[doc = "MDIO is driven by master on MDC negative edge (default for external MDIOs)"]
            pub const NEGATIVE: u32 = 0x01;
        }
    }
    #[doc = "Address Error"]
    pub mod ADDR_ERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "Error. An access control violation has occurred. The request address used does not match the MDIO PHY's address (clause 22) or MDIO port address (clause 45) assigned."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "MDIO Command Completion Interrupt Enable Mask"]
    pub mod CIM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const MASKED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "MDIO Command Completion"]
    pub mod CMP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An MDIO command completion did not occur."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO command completion occurred."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Busy 1"]
    pub mod BSY1 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An MDIO transaction is not occurring; software may access other MDIO registers."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO transaction is occurring."]
            pub const ONE: u32 = 0x01;
        }
    }
}
#[doc = "External MDIO interface control register"]
pub mod EMDIO_CTL {
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    pub mod DEV_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    pub mod PORT_ADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO read with address post-increment initiation. Self-clearing once transaction is complete."]
    pub mod POST_INC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO read initiation."]
    pub mod READ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External MDIO interface data register"]
pub mod EMDIO_DATA {
    #[doc = "16-bit MDIO data."]
    pub mod MDIO_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External MDIO register address register"]
pub mod EMDIO_ADDR {
    #[doc = "MDIO PHY register address."]
    pub mod REGADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External MDIO status register"]
pub mod EMDIO_STAT {
    #[doc = "Global MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY white list"]
    pub mod WHT_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY white list enable"]
    pub mod WHT_LIST_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port ID"]
    pub mod PORT_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port ID"]
    pub mod REQ_TYPE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY status configuration register"]
pub mod PHY_STATUS_CFG {
    #[doc = "MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO read error"]
    pub mod MDIO_RD_ER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY status read interval"]
    pub mod STATUS_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY status control register"]
pub mod PHY_STATUS_CTL {
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    pub mod DEV_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    pub mod PORT_ADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY status data register"]
pub mod PHY_STATUS_DATA {
    #[doc = "16-bit MDIO data"]
    pub mod MDIO_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current count"]
    pub mod CURR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY status register address register"]
pub mod PHY_STATUS_ADDR {
    #[doc = "MDIO PHY register address. Address of the register within the Clause 45 PHY device from which data is to be read."]
    pub mod REGADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY status event register"]
pub mod PHY_STATUS_EVENT {
    #[doc = "Status event high-to-low. Set to 1 if a 1->0 transition on a corresponding data bit has occurred. Write 1 to clear."]
    pub mod STATUS_EVENT_HL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status event low-to-high. Set to 1 if a 0->1 transition on a corresponding data bit has occurred. Write 1 to clear."]
    pub mod STATUS_EVENT_LH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY status mask register"]
pub mod PHY_STATUS_MASK {
    #[doc = "Status high-to-low mask. If set to 1, assert an interrupt if the corresponding event bit is set."]
    pub mod STATUS_MASK_HL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status mask low-to-high. If set to 1, assert an interrupt if the corresponding event bit is set."]
    pub mod STATUS_MASK_LH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MDIO configuration register"]
pub mod MDIO_CFG {
    #[doc = "MDIO pin mode"]
    pub mod MDIO_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDC pin mode"]
    pub mod MDC_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
