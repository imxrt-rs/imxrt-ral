#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "Lane Configuration Register"]
    pub CFG_NUM_LANES: crate::RWRegister<u32>,
    #[doc = "Disable Data Lane Register"]
    pub CFG_DISABLE_DATA_LANES: crate::RWRegister<u32>,
    #[doc = "ECC and CRC Error Status Register"]
    pub BIT_ERR: crate::RORegister<u32>,
    #[doc = "IRQ Status Register"]
    pub IRQ_STATUS: crate::RORegister<u32>,
    #[doc = "IRQ Mask Setting Register"]
    pub IRQ_MASK: crate::RWRegister<u32>,
    #[doc = "Ultra Low Power State (ULPS) Status Register"]
    pub ULPS_STATUS: crate::RORegister<u32>,
    #[doc = "ERRSot HS Status Register"]
    pub PPI_ERRSOT_HS: crate::RORegister<u32>,
    #[doc = "ErrSotSync HS Status Register"]
    pub PPI_ERRSOTSYNC_HS: crate::RORegister<u32>,
    #[doc = "ErrEsc Status Register"]
    pub PPI_ERRESC: crate::RORegister<u32>,
    #[doc = "ErrSyncEsc Status Register"]
    pub PPI_ERRSYNCESC: crate::RORegister<u32>,
    #[doc = "ErrControl Status Register"]
    pub PPI_ERRCONTROL: crate::RORegister<u32>,
    #[doc = "Disable Payload 0 Register"]
    pub CFG_DISABLE_PAYLOAD_0: crate::RWRegister<u32>,
    #[doc = "Disable Payload 1 Register"]
    pub CFG_DISABLE_PAYLOAD_1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4c],
    #[doc = "Ignore Virtual Channel Register"]
    pub CFG_IGNORE_VC: crate::RWRegister<u32>,
    #[doc = "Virtual Channel value Register"]
    pub CFG_VID_VC: crate::RWRegister<u32>,
    #[doc = "FIFO Send Level Configuration Register"]
    pub CFG_VID_P_FIFO_SEND_LEVEL: crate::RWRegister<u32>,
    #[doc = "VSYNC Configuration Register"]
    pub CFG_VID_VSYNC: crate::RWRegister<u32>,
    #[doc = "Start of HSYNC Delay control Register"]
    pub CFG_VID_HSYNC_FP: crate::RWRegister<u32>,
    #[doc = "HSYNC Configuration Register"]
    pub CFG_VID_HSYNC: crate::RWRegister<u32>,
    #[doc = "End of HSYNC Delay Control Register"]
    pub CFG_VID_HSYNC_BP: crate::RWRegister<u32>,
}
#[doc = "Lane Configuration Register"]
pub mod CFG_NUM_LANES {
    #[doc = "This field is used to set the number of active lanes for receiving data."]
    pub mod CFG_NUM_LANES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 Lane"]
            pub const CFG_NUM_LANES_0: u32 = 0;
            #[doc = "2 Lane"]
            pub const CFG_NUM_LANES_1: u32 = 0x01;
        }
    }
}
#[doc = "Disable Data Lane Register"]
pub mod CFG_DISABLE_DATA_LANES {
    #[doc = "This field is used to disable data lanes."]
    pub mod CFG_DISABLE_DATA_LANES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC and CRC Error Status Register"]
pub mod BIT_ERR {
    #[doc = "This field shows the error status of ECC and CRC"]
    pub mod BIT_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IRQ Status Register"]
pub mod IRQ_STATUS {
    #[doc = "This field shows the IRQ status"]
    pub mod IRQ_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IRQ Mask Setting Register"]
pub mod IRQ_MASK {
    #[doc = "This field shows the IRQ Mask setting"]
    pub mod IRQ_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ultra Low Power State (ULPS) Status Register"]
pub mod ULPS_STATUS {
    #[doc = "This field shows the status of Rx D-PHY ULPS state"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERRSot HS Status Register"]
pub mod PPI_ERRSOT_HS {
    #[doc = "This field indicates PPI ErrSotHS captured status from D-PHY"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ErrSotSync HS Status Register"]
pub mod PPI_ERRSOTSYNC_HS {
    #[doc = "This field indicates PPI ErrSotSync_HS captured status from D-PHY"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ErrEsc Status Register"]
pub mod PPI_ERRESC {
    #[doc = "This field indicates PPI ErrEsc captured status from D-PHY"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ErrSyncEsc Status Register"]
pub mod PPI_ERRSYNCESC {
    #[doc = "This field indicates PPI ErrSyncEsc captured status from D-PHY"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ErrControl Status Register"]
pub mod PPI_ERRCONTROL {
    #[doc = "This field indicates PPI ErrControl captured status from D-PHY"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Disable Payload 0 Register"]
pub mod CFG_DISABLE_PAYLOAD_0 {
    #[doc = "Null"]
    pub mod DIS_PAYLOAD_NULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Blank"]
    pub mod DIS_PAYLOAD_BLANK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Embedded"]
    pub mod DIS_PAYLOAD_EMBEDDED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Legacy YUV 420 8 bit"]
    pub mod DIS_PAYLOAD_YUV420 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "YUV422 8 bit"]
    pub mod DIS_PAYLOAD_YUV422_8BIT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RGB444"]
    pub mod DIS_PAYLOAD_RGB444 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RGB555"]
    pub mod DIS_PAYLOAD_RGB555 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RGB565"]
    pub mod DIS_PAYLOAD_RGB565 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RGB666"]
    pub mod DIS_PAYLOAD_RGB666 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RGB888"]
    pub mod DIS_PAYLOAD_RGB888 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Disable Payload 1 Register"]
pub mod CFG_DISABLE_PAYLOAD_1 {
    #[doc = "User defined type 0x31"]
    pub mod DIS_PAYLOAD_UDEF_30 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User defined type 0x32"]
    pub mod DIS_PAYLOAD_UDEF_31 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User defined type 0x33"]
    pub mod DIS_PAYLOAD_UDEF_32 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User defined type 0x34"]
    pub mod DIS_PAYLOAD_UDEF_33 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User defined type 0x35"]
    pub mod DIS_PAYLOAD_UDEF_34 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User defined type 0x35"]
    pub mod DIS_PAYLOAD_UDEF_35 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User defined type 0x36"]
    pub mod DIS_PAYLOAD_UDEF_36 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User defined type 0x37"]
    pub mod DIS_PAYLOAD_UDEF_37 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unsupported Data Types"]
    pub mod DIS_PAYLOAD_UNSUPPORTED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ignore Virtual Channel Register"]
pub mod CFG_IGNORE_VC {
    #[doc = "When set, this input causes the interface to ignore the Virtual Channel (VC) field in received packets"]
    pub mod IGNORE_VC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Virtual Channel value Register"]
pub mod CFG_VID_VC {
    #[doc = "This bit field sets the Virtual Channel value the interface must match in an incoming packet for it to accept the packet"]
    pub mod VID_VC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO Send Level Configuration Register"]
pub mod CFG_VID_P_FIFO_SEND_LEVEL {
    #[doc = "FIFO Send Level field"]
    pub mod SEND_LEVEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VSYNC Configuration Register"]
pub mod CFG_VID_VSYNC {
    #[doc = "Width of VSYNC"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start of HSYNC Delay control Register"]
pub mod CFG_VID_HSYNC_FP {
    #[doc = "Delay control for beginning of HSYNC pulse"]
    pub mod DELAY_CTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HSYNC Configuration Register"]
pub mod CFG_VID_HSYNC {
    #[doc = "Width of HSYNC"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "End of HSYNC Delay Control Register"]
pub mod CFG_VID_HSYNC_BP {
    #[doc = "Delay Control for end of HSYNC pulse"]
    pub mod DELAY_CTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
