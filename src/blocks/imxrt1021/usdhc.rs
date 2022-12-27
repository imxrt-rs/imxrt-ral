#[doc = "uSDHC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DMA System Address"]
    pub DS_ADDR: crate::RWRegister<u32>,
    #[doc = "Block Attributes"]
    pub BLK_ATT: crate::RWRegister<u32>,
    #[doc = "Command Argument"]
    pub CMD_ARG: crate::RWRegister<u32>,
    #[doc = "Command Transfer Type"]
    pub CMD_XFR_TYP: crate::RWRegister<u32>,
    #[doc = "Command Response0"]
    pub CMD_RSP0: crate::RORegister<u32>,
    #[doc = "Command Response1"]
    pub CMD_RSP1: crate::RORegister<u32>,
    #[doc = "Command Response2"]
    pub CMD_RSP2: crate::RORegister<u32>,
    #[doc = "Command Response3"]
    pub CMD_RSP3: crate::RORegister<u32>,
    #[doc = "Data Buffer Access Port"]
    pub DATA_BUFF_ACC_PORT: crate::RWRegister<u32>,
    #[doc = "Present State"]
    pub PRES_STATE: crate::RORegister<u32>,
    #[doc = "Protocol Control"]
    pub PROT_CTRL: crate::RWRegister<u32>,
    #[doc = "System Control"]
    pub SYS_CTRL: crate::RWRegister<u32>,
    #[doc = "Interrupt Status"]
    pub INT_STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Enable"]
    pub INT_STATUS_EN: crate::RWRegister<u32>,
    #[doc = "Interrupt Signal Enable"]
    pub INT_SIGNAL_EN: crate::RWRegister<u32>,
    #[doc = "Auto CMD12 Error Status"]
    pub AUTOCMD12_ERR_STATUS: crate::RWRegister<u32>,
    #[doc = "Host Controller Capabilities"]
    pub HOST_CTRL_CAP: crate::RWRegister<u32>,
    #[doc = "Watermark Level"]
    pub WTMK_LVL: crate::RWRegister<u32>,
    #[doc = "Mixer Control"]
    pub MIX_CTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Force Event"]
    pub FORCE_EVENT: crate::RWRegister<u32>,
    #[doc = "ADMA Error Status Register"]
    pub ADMA_ERR_STATUS: crate::RORegister<u32>,
    #[doc = "ADMA System Address"]
    pub ADMA_SYS_ADDR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "DLL (Delay Line) Control"]
    pub DLL_CTRL: crate::RWRegister<u32>,
    #[doc = "DLL Status"]
    pub DLL_STATUS: crate::RORegister<u32>,
    #[doc = "CLK Tuning Control and Status"]
    pub CLK_TUNE_CTRL_STATUS: crate::RWRegister<u32>,
    _reserved2: [u8; 0x54],
    #[doc = "Vendor Specific Register"]
    pub VEND_SPEC: crate::RWRegister<u32>,
    #[doc = "MMC Boot Register"]
    pub MMC_BOOT: crate::RWRegister<u32>,
    #[doc = "Vendor Specific 2 Register"]
    pub VEND_SPEC2: crate::RWRegister<u32>,
    #[doc = "Tuning Control Register"]
    pub TUNING_CTRL: crate::RWRegister<u32>,
}
#[doc = "DMA System Address"]
pub mod DS_ADDR {
    #[doc = "DS_ADDR"]
    pub mod DS_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Block Attributes"]
pub mod BLK_ATT {
    #[doc = "Block Size"]
    pub mod BLKSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No data transfer"]
            pub const BLKSIZE_0: u32 = 0;
            #[doc = "1 Byte"]
            pub const BLKSIZE_1: u32 = 0x01;
            #[doc = "2 Bytes"]
            pub const BLKSIZE_2: u32 = 0x02;
            #[doc = "3 Bytes"]
            pub const BLKSIZE_3: u32 = 0x03;
            #[doc = "4 Bytes"]
            pub const BLKSIZE_4: u32 = 0x04;
            #[doc = "511 Bytes"]
            pub const BLKSIZE_511: u32 = 0x01ff;
            #[doc = "512 Bytes"]
            pub const BLKSIZE_512: u32 = 0x0200;
            #[doc = "2048 Bytes"]
            pub const BLKSIZE_2048: u32 = 0x0800;
            #[doc = "4096 Bytes"]
            pub const BLKSIZE_4096: u32 = 0x1000;
        }
    }
    #[doc = "Block Count"]
    pub mod BLKCNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Count"]
            pub const BLKCNT_0: u32 = 0;
            #[doc = "1 block"]
            pub const BLKCNT_1: u32 = 0x01;
            #[doc = "2 blocks"]
            pub const BLKCNT_2: u32 = 0x02;
            #[doc = "65535 blocks"]
            pub const BLKCNT_65535: u32 = 0xffff;
        }
    }
}
#[doc = "Command Argument"]
pub mod CMD_ARG {
    #[doc = "Command Argument"]
    pub mod CMDARG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Transfer Type"]
pub mod CMD_XFR_TYP {
    #[doc = "Response Type Select"]
    pub mod RSPTYP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Response"]
            pub const RSPTYP_0: u32 = 0;
            #[doc = "Response Length 136"]
            pub const RSPTYP_1: u32 = 0x01;
            #[doc = "Response Length 48"]
            pub const RSPTYP_2: u32 = 0x02;
            #[doc = "Response Length 48, check Busy after response"]
            pub const RSPTYP_3: u32 = 0x03;
        }
    }
    #[doc = "Command CRC Check Enable"]
    pub mod CCCEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CCCEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const CCCEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command Index Check Enable"]
    pub mod CICEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CICEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const CICEN_1: u32 = 0x01;
        }
    }
    #[doc = "Data Present Select"]
    pub mod DPSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Data Present"]
            pub const DPSEL_0: u32 = 0;
            #[doc = "Data Present"]
            pub const DPSEL_1: u32 = 0x01;
        }
    }
    #[doc = "Command Type"]
    pub mod CMDTYP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Other commands"]
            pub const CMDTYP_0: u32 = 0;
            #[doc = "Suspend CMD52 for writing Bus Suspend in CCCR"]
            pub const CMDTYP_1: u32 = 0x01;
            #[doc = "Resume CMD52 for writing Function Select in CCCR"]
            pub const CMDTYP_2: u32 = 0x02;
            #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
            pub const CMDTYP_3: u32 = 0x03;
        }
    }
    #[doc = "Command Index"]
    pub mod CMDINX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Response0"]
pub mod CMD_RSP0 {
    #[doc = "Command Response 0"]
    pub mod CMDRSP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Response1"]
pub mod CMD_RSP1 {
    #[doc = "Command Response 1"]
    pub mod CMDRSP1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Response2"]
pub mod CMD_RSP2 {
    #[doc = "Command Response 2"]
    pub mod CMDRSP2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Response3"]
pub mod CMD_RSP3 {
    #[doc = "Command Response 3"]
    pub mod CMDRSP3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Buffer Access Port"]
pub mod DATA_BUFF_ACC_PORT {
    #[doc = "Data Content"]
    pub mod DATCONT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Present State"]
pub mod PRES_STATE {
    #[doc = "Command Inhibit (CMD)"]
    pub mod CIHB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Can issue command using only CMD line"]
            pub const CIHB_0: u32 = 0;
            #[doc = "Cannot issue command"]
            pub const CIHB_1: u32 = 0x01;
        }
    }
    #[doc = "Command Inhibit (DATA)"]
    pub mod CDIHB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Can issue command which uses the DATA line"]
            pub const CDIHB_0: u32 = 0;
            #[doc = "Cannot issue command which uses the DATA line"]
            pub const CDIHB_1: u32 = 0x01;
        }
    }
    #[doc = "Data Line Active"]
    pub mod DLA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DATA Line Inactive"]
            pub const DLA_0: u32 = 0;
            #[doc = "DATA Line Active"]
            pub const DLA_1: u32 = 0x01;
        }
    }
    #[doc = "SD Clock Stable"]
    pub mod SDSTB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is changing frequency and not stable."]
            pub const SDSTB_0: u32 = 0;
            #[doc = "Clock is stable."]
            pub const SDSTB_1: u32 = 0x01;
        }
    }
    #[doc = "IPG_CLK Gated Off Internally"]
    pub mod IPGOFF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IPG_CLK is active."]
            pub const IPGOFF_0: u32 = 0;
            #[doc = "IPG_CLK is gated off."]
            pub const IPGOFF_1: u32 = 0x01;
        }
    }
    #[doc = "HCLK Gated Off Internally"]
    pub mod HCKOFF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HCLK is active."]
            pub const HCKOFF_0: u32 = 0;
            #[doc = "HCLK is gated off."]
            pub const HCKOFF_1: u32 = 0x01;
        }
    }
    #[doc = "IPG_PERCLK Gated Off Internally"]
    pub mod PEROFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IPG_PERCLK is active."]
            pub const PEROFF_0: u32 = 0;
            #[doc = "IPG_PERCLK is gated off."]
            pub const PEROFF_1: u32 = 0x01;
        }
    }
    #[doc = "SD Clock Gated Off Internally"]
    pub mod SDOFF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SD Clock is active."]
            pub const SDOFF_0: u32 = 0;
            #[doc = "SD Clock is gated off."]
            pub const SDOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Write Transfer Active"]
    pub mod WTA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No valid data"]
            pub const WTA_0: u32 = 0;
            #[doc = "Transferring data"]
            pub const WTA_1: u32 = 0x01;
        }
    }
    #[doc = "Read Transfer Active"]
    pub mod RTA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No valid data"]
            pub const RTA_0: u32 = 0;
            #[doc = "Transferring data"]
            pub const RTA_1: u32 = 0x01;
        }
    }
    #[doc = "Buffer Write Enable"]
    pub mod BWEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write disable"]
            pub const BWEN_0: u32 = 0;
            #[doc = "Write enable"]
            pub const BWEN_1: u32 = 0x01;
        }
    }
    #[doc = "Buffer Read Enable"]
    pub mod BREN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read disable"]
            pub const BREN_0: u32 = 0;
            #[doc = "Read enable"]
            pub const BREN_1: u32 = 0x01;
        }
    }
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    pub mod RTR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed or well tuned sampling clock"]
            pub const RTR_0: u32 = 0;
            #[doc = "Sampling clock needs re-tuning"]
            pub const RTR_1: u32 = 0x01;
        }
    }
    #[doc = "Tape Select Change Done"]
    pub mod TSCD {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Delay cell select change is not finished."]
            pub const TSCD_0: u32 = 0;
            #[doc = "Delay cell select change is finished."]
            pub const TSCD_1: u32 = 0x01;
        }
    }
    #[doc = "Card Inserted"]
    pub mod CINST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power on Reset or No Card"]
            pub const CINST_0: u32 = 0;
            #[doc = "Card Inserted"]
            pub const CINST_1: u32 = 0x01;
        }
    }
    #[doc = "Card Detect Pin Level"]
    pub mod CDPL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No card present (CD_B = 1)"]
            pub const CDPL_0: u32 = 0;
            #[doc = "Card present (CD_B = 0)"]
            pub const CDPL_1: u32 = 0x01;
        }
    }
    #[doc = "Write Protect Switch Pin Level"]
    pub mod WPSPL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write protected (WP = 1)"]
            pub const WPSPL_0: u32 = 0;
            #[doc = "Write enabled (WP = 0)"]
            pub const WPSPL_1: u32 = 0x01;
        }
    }
    #[doc = "CMD Line Signal Level"]
    pub mod CLSL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DATA\\[7:0\\] Line Signal Level"]
    pub mod DLSL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data 0 line signal level"]
            pub const DATA0: u32 = 0;
            #[doc = "Data 1 line signal level"]
            pub const DATA1: u32 = 0x01;
            #[doc = "Data 2 line signal level"]
            pub const DATA2: u32 = 0x02;
            #[doc = "Data 3 line signal level"]
            pub const DATA3: u32 = 0x03;
            #[doc = "Data 4 line signal level"]
            pub const DATA4: u32 = 0x04;
            #[doc = "Data 5 line signal level"]
            pub const DATA5: u32 = 0x05;
            #[doc = "Data 6 line signal level"]
            pub const DATA6: u32 = 0x06;
            #[doc = "Data 7 line signal level"]
            pub const DATA7: u32 = 0x07;
        }
    }
}
#[doc = "Protocol Control"]
pub mod PROT_CTRL {
    #[doc = "LED Control"]
    pub mod LCTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LED off"]
            pub const LCTL_0: u32 = 0;
            #[doc = "LED on"]
            pub const LCTL_1: u32 = 0x01;
        }
    }
    #[doc = "Data Transfer Width"]
    pub mod DTW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1-bit mode"]
            pub const DTW_0: u32 = 0;
            #[doc = "4-bit mode"]
            pub const DTW_1: u32 = 0x01;
            #[doc = "8-bit mode"]
            pub const DTW_2: u32 = 0x02;
        }
    }
    #[doc = "DATA3 as Card Detection Pin"]
    pub mod D3CD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DATA3 does not monitor Card Insertion"]
            pub const D3CD_0: u32 = 0;
            #[doc = "DATA3 as Card Detection Pin"]
            pub const D3CD_1: u32 = 0x01;
        }
    }
    #[doc = "Endian Mode"]
    pub mod EMODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Big Endian Mode"]
            pub const EMODE_0: u32 = 0;
            #[doc = "Half Word Big Endian Mode"]
            pub const EMODE_1: u32 = 0x01;
            #[doc = "Little Endian Mode"]
            pub const EMODE_2: u32 = 0x02;
        }
    }
    #[doc = "Card Detect Test Level"]
    pub mod CDTL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Card Detect Test Level is 0, no card inserted"]
            pub const CDTL_0: u32 = 0;
            #[doc = "Card Detect Test Level is 1, card inserted"]
            pub const CDTL_1: u32 = 0x01;
        }
    }
    #[doc = "Card Detect Signal Selection"]
    pub mod CDSS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Card Detection Level is selected (for normal purpose)."]
            pub const CDSS_0: u32 = 0;
            #[doc = "Card Detection Test Level is selected (for test purpose)."]
            pub const CDSS_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Select"]
    pub mod DMASEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No DMA or Simple DMA is selected"]
            pub const DMASEL_0: u32 = 0;
            #[doc = "ADMA1 is selected"]
            pub const DMASEL_1: u32 = 0x01;
            #[doc = "ADMA2 is selected"]
            pub const DMASEL_2: u32 = 0x02;
        }
    }
    #[doc = "Stop At Block Gap Request"]
    pub mod SABGREQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transfer"]
            pub const SABGREQ_0: u32 = 0;
            #[doc = "Stop"]
            pub const SABGREQ_1: u32 = 0x01;
        }
    }
    #[doc = "Continue Request"]
    pub mod CREQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const CREQ_0: u32 = 0;
            #[doc = "Restart"]
            pub const CREQ_1: u32 = 0x01;
        }
    }
    #[doc = "Read Wait Control"]
    pub mod RWCTL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
            pub const RWCTL_0: u32 = 0;
            #[doc = "Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
            pub const RWCTL_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt At Block Gap"]
    pub mod IABG {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const IABG_0: u32 = 0;
            #[doc = "Enabled"]
            pub const IABG_1: u32 = 0x01;
        }
    }
    #[doc = "RD_DONE_NO_8CLK"]
    pub mod RD_DONE_NO_8CLK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup Event Enable On Card Interrupt"]
    pub mod WECINT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WECINT_0: u32 = 0;
            #[doc = "Enable"]
            pub const WECINT_1: u32 = 0x01;
        }
    }
    #[doc = "Wakeup Event Enable On SD Card Insertion"]
    pub mod WECINS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WECINS_0: u32 = 0;
            #[doc = "Enable"]
            pub const WECINS_1: u32 = 0x01;
        }
    }
    #[doc = "Wakeup Event Enable On SD Card Removal"]
    pub mod WECRM {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WECRM_0: u32 = 0;
            #[doc = "Enable"]
            pub const WECRM_1: u32 = 0x01;
        }
    }
    #[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    pub mod BURST_LEN_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Burst length is enabled for INCR"]
            pub const BURST_LEN_EN_1: u32 = 0x01;
        }
    }
    #[doc = "NON_EXACT_BLK_RD"]
    pub mod NON_EXACT_BLK_RD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
            pub const NON_EXACT_BLK_RD_0: u32 = 0;
            #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
            pub const NON_EXACT_BLK_RD_1: u32 = 0x01;
        }
    }
}
#[doc = "System Control"]
pub mod SYS_CTRL {
    #[doc = "Divisor"]
    pub mod DVS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide-by-1"]
            pub const DVS_0: u32 = 0;
            #[doc = "Divide-by-2"]
            pub const DVS_1: u32 = 0x01;
            #[doc = "Divide-by-15"]
            pub const DVS_14: u32 = 0x0e;
            #[doc = "Divide-by-16"]
            pub const DVS_15: u32 = 0x0f;
        }
    }
    #[doc = "SDCLK Frequency Select"]
    pub mod SDCLKFS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Timeout Counter Value"]
    pub mod DTOCV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDCLK x 2 14"]
            pub const DTOCV_0: u32 = 0;
            #[doc = "SDCLK x 2 15"]
            pub const DTOCV_1: u32 = 0x01;
            #[doc = "SDCLK x 2 27"]
            pub const DTOCV_13: u32 = 0x0d;
            #[doc = "SDCLK x 2 28"]
            pub const DTOCV_14: u32 = 0x0e;
            #[doc = "SDCLK x 2 29"]
            pub const DTOCV_15: u32 = 0x0f;
        }
    }
    #[doc = "IPP_RST_N"]
    pub mod IPP_RST_N {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset For ALL"]
    pub mod RSTA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Reset"]
            pub const RSTA_0: u32 = 0;
            #[doc = "Reset"]
            pub const RSTA_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset For CMD Line"]
    pub mod RSTC {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Reset"]
            pub const RSTC_0: u32 = 0;
            #[doc = "Reset"]
            pub const RSTC_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset For DATA Line"]
    pub mod RSTD {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Reset"]
            pub const RSTD_0: u32 = 0;
            #[doc = "Reset"]
            pub const RSTD_1: u32 = 0x01;
        }
    }
    #[doc = "Initialization Active"]
    pub mod INITA {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Tuning"]
    pub mod RSTT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status"]
pub mod INT_STATUS {
    #[doc = "Command Complete"]
    pub mod CC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command not complete"]
            pub const CC_0: u32 = 0;
            #[doc = "Command complete"]
            pub const CC_1: u32 = 0x01;
        }
    }
    #[doc = "Transfer Complete"]
    pub mod TC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transfer not complete"]
            pub const TC_0: u32 = 0;
            #[doc = "Transfer complete"]
            pub const TC_1: u32 = 0x01;
        }
    }
    #[doc = "Block Gap Event"]
    pub mod BGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No block gap event"]
            pub const BGE_0: u32 = 0;
            #[doc = "Transaction stopped at block gap"]
            pub const BGE_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Interrupt"]
    pub mod DINT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No DMA Interrupt"]
            pub const DINT_0: u32 = 0;
            #[doc = "DMA Interrupt is generated"]
            pub const DINT_1: u32 = 0x01;
        }
    }
    #[doc = "Buffer Write Ready"]
    pub mod BWR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not ready to write buffer"]
            pub const BWR_0: u32 = 0;
            #[doc = "Ready to write buffer:"]
            pub const BWR_1: u32 = 0x01;
        }
    }
    #[doc = "Buffer Read Ready"]
    pub mod BRR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not ready to read buffer"]
            pub const BRR_0: u32 = 0;
            #[doc = "Ready to read buffer"]
            pub const BRR_1: u32 = 0x01;
        }
    }
    #[doc = "Card Insertion"]
    pub mod CINS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Card state unstable or removed"]
            pub const CINS_0: u32 = 0;
            #[doc = "Card inserted"]
            pub const CINS_1: u32 = 0x01;
        }
    }
    #[doc = "Card Removal"]
    pub mod CRM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Card state unstable or inserted"]
            pub const CRM_0: u32 = 0;
            #[doc = "Card removed"]
            pub const CRM_1: u32 = 0x01;
        }
    }
    #[doc = "Card Interrupt"]
    pub mod CINT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Card Interrupt"]
            pub const CINT_0: u32 = 0;
            #[doc = "Generate Card Interrupt"]
            pub const CINT_1: u32 = 0x01;
        }
    }
    #[doc = "Re-Tuning Event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    pub mod RTE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Re-Tuning is not required"]
            pub const RTE_0: u32 = 0;
            #[doc = "Re-Tuning should be performed"]
            pub const RTE_1: u32 = 0x01;
        }
    }
    #[doc = "Tuning Pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    pub mod TP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Timeout Error"]
    pub mod CTOE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const CTOE_0: u32 = 0;
            #[doc = "Time out"]
            pub const CTOE_1: u32 = 0x01;
        }
    }
    #[doc = "Command CRC Error"]
    pub mod CCE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const CCE_0: u32 = 0;
            #[doc = "CRC Error Generated."]
            pub const CCE_1: u32 = 0x01;
        }
    }
    #[doc = "Command End Bit Error"]
    pub mod CEBE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const CEBE_0: u32 = 0;
            #[doc = "End Bit Error Generated"]
            pub const CEBE_1: u32 = 0x01;
        }
    }
    #[doc = "Command Index Error"]
    pub mod CIE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const CIE_0: u32 = 0;
            #[doc = "Error"]
            pub const CIE_1: u32 = 0x01;
        }
    }
    #[doc = "Data Timeout Error"]
    pub mod DTOE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const DTOE_0: u32 = 0;
            #[doc = "Time out"]
            pub const DTOE_1: u32 = 0x01;
        }
    }
    #[doc = "Data CRC Error"]
    pub mod DCE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const DCE_0: u32 = 0;
            #[doc = "Error"]
            pub const DCE_1: u32 = 0x01;
        }
    }
    #[doc = "Data End Bit Error"]
    pub mod DEBE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const DEBE_0: u32 = 0;
            #[doc = "Error"]
            pub const DEBE_1: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 Error"]
    pub mod AC12E {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const AC12E_0: u32 = 0;
            #[doc = "Error"]
            pub const AC12E_1: u32 = 0x01;
        }
    }
    #[doc = "Tuning Error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    pub mod TNE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Error"]
    pub mod DMAE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const DMAE_0: u32 = 0;
            #[doc = "Error"]
            pub const DMAE_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Status Enable"]
pub mod INT_STATUS_EN {
    #[doc = "Command Complete Status Enable"]
    pub mod CCSEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CCSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CCSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Transfer Complete Status Enable"]
    pub mod TCSEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TCSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TCSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Block Gap Event Status Enable"]
    pub mod BGESEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BGESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const BGESEN_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Interrupt Status Enable"]
    pub mod DINTSEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DINTSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DINTSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Buffer Write Ready Status Enable"]
    pub mod BWRSEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BWRSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const BWRSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Buffer Read Ready Status Enable"]
    pub mod BRRSEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BRRSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const BRRSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Card Insertion Status Enable"]
    pub mod CINSSEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CINSSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CINSSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Card Removal Status Enable"]
    pub mod CRMSEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CRMSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CRMSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Card Interrupt Status Enable"]
    pub mod CINTSEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CINTSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CINTSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Re-Tuning Event Status Enable"]
    pub mod RTESEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const RTESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const RTESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Tuning Pass Status Enable"]
    pub mod TPSEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TPSEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TPSEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command Timeout Error Status Enable"]
    pub mod CTOESEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CTOESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CTOESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command CRC Error Status Enable"]
    pub mod CCESEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CCESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CCESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command End Bit Error Status Enable"]
    pub mod CEBESEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CEBESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CEBESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command Index Error Status Enable"]
    pub mod CIESEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CIESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CIESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Data Timeout Error Status Enable"]
    pub mod DTOESEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DTOESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DTOESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Data CRC Error Status Enable"]
    pub mod DCESEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DCESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DCESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Data End Bit Error Status Enable"]
    pub mod DEBESEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DEBESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DEBESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 Error Status Enable"]
    pub mod AC12ESEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const AC12ESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const AC12ESEN_1: u32 = 0x01;
        }
    }
    #[doc = "Tuning Error Status Enable"]
    pub mod TNESEN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TNESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TNESEN_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Error Status Enable"]
    pub mod DMAESEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DMAESEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DMAESEN_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Signal Enable"]
pub mod INT_SIGNAL_EN {
    #[doc = "Command Complete Interrupt Enable"]
    pub mod CCIEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CCIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CCIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Transfer Complete Interrupt Enable"]
    pub mod TCIEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TCIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TCIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Block Gap Event Interrupt Enable"]
    pub mod BGEIEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BGEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const BGEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Interrupt Enable"]
    pub mod DINTIEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DINTIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DINTIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Buffer Write Ready Interrupt Enable"]
    pub mod BWRIEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BWRIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const BWRIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Buffer Read Ready Interrupt Enable"]
    pub mod BRRIEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BRRIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const BRRIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Card Insertion Interrupt Enable"]
    pub mod CINSIEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CINSIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CINSIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Card Removal Interrupt Enable"]
    pub mod CRMIEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CRMIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CRMIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Card Interrupt Interrupt Enable"]
    pub mod CINTIEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CINTIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CINTIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Re-Tuning Event Interrupt Enable"]
    pub mod RTEIEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const RTEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const RTEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Tuning Pass Interrupt Enable"]
    pub mod TPIEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TPIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TPIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command Timeout Error Interrupt Enable"]
    pub mod CTOEIEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CTOEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CTOEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command CRC Error Interrupt Enable"]
    pub mod CCEIEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CCEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CCEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command End Bit Error Interrupt Enable"]
    pub mod CEBEIEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CEBEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CEBEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Command Index Error Interrupt Enable"]
    pub mod CIEIEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CIEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const CIEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Data Timeout Error Interrupt Enable"]
    pub mod DTOEIEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DTOEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DTOEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Data CRC Error Interrupt Enable"]
    pub mod DCEIEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DCEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DCEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Data End Bit Error Interrupt Enable"]
    pub mod DEBEIEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DEBEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DEBEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 Error Interrupt Enable"]
    pub mod AC12EIEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const AC12EIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const AC12EIEN_1: u32 = 0x01;
        }
    }
    #[doc = "Tuning Error Interrupt Enable"]
    pub mod TNEIEN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TNEIEN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TNEIEN_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Error Interrupt Enable"]
    pub mod DMAEIEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DMAEIEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const DMAEIEN_1: u32 = 0x01;
        }
    }
}
#[doc = "Auto CMD12 Error Status"]
pub mod AUTOCMD12_ERR_STATUS {
    #[doc = "Auto CMD12 Not Executed"]
    pub mod AC12NE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Executed"]
            pub const AC12NE_0: u32 = 0;
            #[doc = "Not executed"]
            pub const AC12NE_1: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 / 23 Timeout Error"]
    pub mod AC12TOE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const AC12TOE_0: u32 = 0;
            #[doc = "Time out"]
            pub const AC12TOE_1: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 / 23 End Bit Error"]
    pub mod AC12EBE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const AC12EBE_0: u32 = 0;
            #[doc = "End Bit Error Generated"]
            pub const AC12EBE_1: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 / 23 CRC Error"]
    pub mod AC12CE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No CRC error"]
            pub const AC12CE_0: u32 = 0;
            #[doc = "CRC Error Met in Auto CMD12/23 Response"]
            pub const AC12CE_1: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 / 23 Index Error"]
    pub mod AC12IE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const AC12IE_0: u32 = 0;
            #[doc = "Error, the CMD index in response is not CMD12/23"]
            pub const AC12IE_1: u32 = 0x01;
        }
    }
    #[doc = "Command Not Issued By Auto CMD12 Error"]
    pub mod CNIBAC12E {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const CNIBAC12E_0: u32 = 0;
            #[doc = "Not Issued"]
            pub const CNIBAC12E_1: u32 = 0x01;
        }
    }
    #[doc = "Execute Tuning"]
    pub mod EXECUTE_TUNING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sample Clock Select"]
    pub mod SMP_CLK_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed clock is used to sample data"]
            pub const SMP_CLK_SEL_0: u32 = 0;
            #[doc = "Tuned clock is used to sample data"]
            pub const SMP_CLK_SEL_1: u32 = 0x01;
        }
    }
}
#[doc = "Host Controller Capabilities"]
pub mod HOST_CTRL_CAP {
    #[doc = "SDR50 support"]
    pub mod SDR50_SUPPORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDR104 support"]
    pub mod SDR104_SUPPORT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DDR50 support"]
    pub mod DDR50_SUPPORT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Counter for Retuning"]
    pub mod TIME_COUNT_RETUNING {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use Tuning for SDR50"]
    pub mod USE_TUNING_SDR50 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDR does not require tuning"]
            pub const USE_TUNING_SDR50_0: u32 = 0;
            #[doc = "SDR50 requires tuning"]
            pub const USE_TUNING_SDR50_1: u32 = 0x01;
        }
    }
    #[doc = "Retuning Mode"]
    pub mod RETUNING_MODE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mode 1"]
            pub const RETUNING_MODE_0: u32 = 0;
            #[doc = "Mode 2"]
            pub const RETUNING_MODE_1: u32 = 0x01;
            #[doc = "Mode 3"]
            pub const RETUNING_MODE_2: u32 = 0x02;
        }
    }
    #[doc = "Max Block Length"]
    pub mod MBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "512 bytes"]
            pub const MBL_0: u32 = 0;
            #[doc = "1024 bytes"]
            pub const MBL_1: u32 = 0x01;
            #[doc = "2048 bytes"]
            pub const MBL_2: u32 = 0x02;
            #[doc = "4096 bytes"]
            pub const MBL_3: u32 = 0x03;
        }
    }
    #[doc = "ADMA Support"]
    pub mod ADMAS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Advanced DMA Not supported"]
            pub const ADMAS_0: u32 = 0;
            #[doc = "Advanced DMA Supported"]
            pub const ADMAS_1: u32 = 0x01;
        }
    }
    #[doc = "High Speed Support"]
    pub mod HSS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "High Speed Not Supported"]
            pub const HSS_0: u32 = 0;
            #[doc = "High Speed Supported"]
            pub const HSS_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Support"]
    pub mod DMAS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA not supported"]
            pub const DMAS_0: u32 = 0;
            #[doc = "DMA Supported"]
            pub const DMAS_1: u32 = 0x01;
        }
    }
    #[doc = "Suspend / Resume Support"]
    pub mod SRS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const SRS_0: u32 = 0;
            #[doc = "Supported"]
            pub const SRS_1: u32 = 0x01;
        }
    }
    #[doc = "Voltage Support 3.3V"]
    pub mod VS33 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "3.3V not supported"]
            pub const VS33_0: u32 = 0;
            #[doc = "3.3V supported"]
            pub const VS33_1: u32 = 0x01;
        }
    }
    #[doc = "Voltage Support 3.0 V"]
    pub mod VS30 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "3.0V not supported"]
            pub const VS30_0: u32 = 0;
            #[doc = "3.0V supported"]
            pub const VS30_1: u32 = 0x01;
        }
    }
    #[doc = "Voltage Support 1.8 V"]
    pub mod VS18 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1.8V not supported"]
            pub const VS18_0: u32 = 0;
            #[doc = "1.8V supported"]
            pub const VS18_1: u32 = 0x01;
        }
    }
}
#[doc = "Watermark Level"]
pub mod WTMK_LVL {
    #[doc = "Read Watermark Level"]
    pub mod RD_WML {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    pub mod RD_BRST_LEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Watermark Level"]
    pub mod WR_WML {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    pub mod WR_BRST_LEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Mixer Control"]
pub mod MIX_CTRL {
    #[doc = "DMA Enable"]
    pub mod DMAEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DMAEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const DMAEN_1: u32 = 0x01;
        }
    }
    #[doc = "Block Count Enable"]
    pub mod BCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const BCEN_0: u32 = 0;
            #[doc = "Enable"]
            pub const BCEN_1: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 Enable"]
    pub mod AC12EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const AC12EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const AC12EN_1: u32 = 0x01;
        }
    }
    #[doc = "Dual Data Rate mode selection"]
    pub mod DDR_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Transfer Direction Select"]
    pub mod DTDSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write (Host to Card)"]
            pub const DTDSEL_0: u32 = 0;
            #[doc = "Read (Card to Host)"]
            pub const DTDSEL_1: u32 = 0x01;
        }
    }
    #[doc = "Multi / Single Block Select"]
    pub mod MSBSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single Block"]
            pub const MSBSEL_0: u32 = 0;
            #[doc = "Multiple Blocks"]
            pub const MSBSEL_1: u32 = 0x01;
        }
    }
    #[doc = "NIBBLE_POS"]
    pub mod NIBBLE_POS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD23 Enable"]
    pub mod AC23EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    pub mod EXE_TUNE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not Tuned or Tuning Completed"]
            pub const EXE_TUNE_0: u32 = 0;
            #[doc = "Execute Tuning"]
            pub const EXE_TUNE_1: u32 = 0x01;
        }
    }
    #[doc = "SMP_CLK_SEL"]
    pub mod SMP_CLK_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed clock is used to sample data / cmd"]
            pub const SMP_CLK_SEL_0: u32 = 0;
            #[doc = "Tuned clock is used to sample data / cmd"]
            pub const SMP_CLK_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    pub mod AUTO_TUNE_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable auto tuning"]
            pub const AUTO_TUNE_EN_0: u32 = 0;
            #[doc = "Enable auto tuning"]
            pub const AUTO_TUNE_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    pub mod FBCLK_SEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feedback clock comes from the loopback CLK"]
            pub const FBCLK_SEL_0: u32 = 0;
            #[doc = "Feedback clock comes from the ipp_card_clk_out"]
            pub const FBCLK_SEL_1: u32 = 0x01;
        }
    }
}
#[doc = "Force Event"]
pub mod FORCE_EVENT {
    #[doc = "Force Event Auto Command 12 Not Executed"]
    pub mod FEVTAC12NE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Auto Command 12 Time Out Error"]
    pub mod FEVTAC12TOE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Auto Command 12 CRC Error"]
    pub mod FEVTAC12CE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Auto Command 12 End Bit Error"]
    pub mod FEVTAC12EBE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Auto Command 12 Index Error"]
    pub mod FEVTAC12IE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Command Not Executed By Auto Command 12 Error"]
    pub mod FEVTCNIBAC12E {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Command Time Out Error"]
    pub mod FEVTCTOE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Command CRC Error"]
    pub mod FEVTCCE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Command End Bit Error"]
    pub mod FEVTCEBE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Command Index Error"]
    pub mod FEVTCIE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Data Time Out Error"]
    pub mod FEVTDTOE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Data CRC Error"]
    pub mod FEVTDCE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Data End Bit Error"]
    pub mod FEVTDEBE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Auto Command 12 Error"]
    pub mod FEVTAC12E {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Tuning Error"]
    pub mod FEVTTNE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event DMA Error"]
    pub mod FEVTDMAE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event Card Interrupt"]
    pub mod FEVTCINT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADMA Error Status Register"]
pub mod ADMA_ERR_STATUS {
    #[doc = "ADMA Error State (when ADMA Error is occurred)"]
    pub mod ADMAES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA Length Mismatch Error"]
    pub mod ADMALME {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const ADMALME_0: u32 = 0;
            #[doc = "Error"]
            pub const ADMALME_1: u32 = 0x01;
        }
    }
    #[doc = "ADMA Descriptor Error"]
    pub mod ADMADCE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Error"]
            pub const ADMADCE_0: u32 = 0;
            #[doc = "Error"]
            pub const ADMADCE_1: u32 = 0x01;
        }
    }
}
#[doc = "ADMA System Address"]
pub mod ADMA_SYS_ADDR {
    #[doc = "ADMA System Address"]
    pub mod ADS_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL (Delay Line) Control"]
pub mod DLL_CTRL {
    #[doc = "DLL_CTRL_ENABLE"]
    pub mod DLL_CTRL_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_RESET"]
    pub mod DLL_CTRL_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_SLV_FORCE_UPD"]
    pub mod DLL_CTRL_SLV_FORCE_UPD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_SLV_DLY_TARGET0"]
    pub mod DLL_CTRL_SLV_DLY_TARGET0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_GATE_UPDATE"]
    pub mod DLL_CTRL_GATE_UPDATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_SLV_OVERRIDE"]
    pub mod DLL_CTRL_SLV_OVERRIDE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_SLV_OVERRIDE_VAL"]
    pub mod DLL_CTRL_SLV_OVERRIDE_VAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_SLV_DLY_TARGET1"]
    pub mod DLL_CTRL_SLV_DLY_TARGET1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_SLV_UPDATE_INT"]
    pub mod DLL_CTRL_SLV_UPDATE_INT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_CTRL_REF_UPDATE_INT"]
    pub mod DLL_CTRL_REF_UPDATE_INT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL Status"]
pub mod DLL_STATUS {
    #[doc = "DLL_STS_SLV_LOCK"]
    pub mod DLL_STS_SLV_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_STS_REF_LOCK"]
    pub mod DLL_STS_REF_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_STS_SLV_SEL"]
    pub mod DLL_STS_SLV_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL_STS_REF_SEL"]
    pub mod DLL_STS_REF_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CLK Tuning Control and Status"]
pub mod CLK_TUNE_CTRL_STATUS {
    #[doc = "DLY_CELL_SET_POST"]
    pub mod DLY_CELL_SET_POST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLY_CELL_SET_OUT"]
    pub mod DLY_CELL_SET_OUT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLY_CELL_SET_PRE"]
    pub mod DLY_CELL_SET_PRE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NXT_ERR"]
    pub mod NXT_ERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TAP_SEL_POST"]
    pub mod TAP_SEL_POST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TAP_SEL_OUT"]
    pub mod TAP_SEL_OUT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TAP_SEL_PRE"]
    pub mod TAP_SEL_PRE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRE_ERR"]
    pub mod PRE_ERR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Vendor Specific Register"]
pub mod VEND_SPEC {
    #[doc = "Voltage Selection"]
    pub mod VSELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Change the voltage to high voltage range, around 3.0 V"]
            pub const VSELECT_0: u32 = 0;
            #[doc = "Change the voltage to low voltage range, around 1.8 V"]
            pub const VSELECT_1: u32 = 0x01;
        }
    }
    #[doc = "Conflict check enable."]
    pub mod CONFLICT_CHK_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Conflict check disable"]
            pub const CONFLICT_CHK_EN_0: u32 = 0;
            #[doc = "Conflict check enable"]
            pub const CONFLICT_CHK_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AC12_WR_CHKBUSY_EN"]
    pub mod AC12_WR_CHKBUSY_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not check busy after auto CMD12 for write data packet"]
            pub const AC12_WR_CHKBUSY_EN_0: u32 = 0;
            #[doc = "Check busy after auto CMD12 for write data packet"]
            pub const AC12_WR_CHKBUSY_EN_1: u32 = 0x01;
        }
    }
    #[doc = "FRC_SDCLK_ON"]
    pub mod FRC_SDCLK_ON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CLK active or inactive is fully controlled by the hardware."]
            pub const FRC_SDCLK_ON_0: u32 = 0;
            #[doc = "Force CLK active."]
            pub const FRC_SDCLK_ON_1: u32 = 0x01;
        }
    }
    #[doc = "CRC Check Disable"]
    pub mod CRC_CHK_DIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
            pub const CRC_CHK_DIS_0: u32 = 0;
            #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
            pub const CRC_CHK_DIS_1: u32 = 0x01;
        }
    }
    #[doc = "CMD_BYTE_EN"]
    pub mod CMD_BYTE_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_BYTE_EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_BYTE_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "MMC Boot Register"]
pub mod MMC_BOOT {
    #[doc = "DTOCV_ACK"]
    pub mod DTOCV_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDCLK x 2^14"]
            pub const DTOCV_ACK_0: u32 = 0;
            #[doc = "SDCLK x 2^15"]
            pub const DTOCV_ACK_1: u32 = 0x01;
            #[doc = "SDCLK x 2^16"]
            pub const DTOCV_ACK_2: u32 = 0x02;
            #[doc = "SDCLK x 2^17"]
            pub const DTOCV_ACK_3: u32 = 0x03;
            #[doc = "SDCLK x 2^18"]
            pub const DTOCV_ACK_4: u32 = 0x04;
            #[doc = "SDCLK x 2^19"]
            pub const DTOCV_ACK_5: u32 = 0x05;
            #[doc = "SDCLK x 2^20"]
            pub const DTOCV_ACK_6: u32 = 0x06;
            #[doc = "SDCLK x 2^21"]
            pub const DTOCV_ACK_7: u32 = 0x07;
            #[doc = "SDCLK x 2^28"]
            pub const DTOCV_ACK_14: u32 = 0x0e;
            #[doc = "SDCLK x 2^29"]
            pub const DTOCV_ACK_15: u32 = 0x0f;
        }
    }
    #[doc = "BOOT_ACK"]
    pub mod BOOT_ACK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No ack"]
            pub const BOOT_ACK_0: u32 = 0;
            #[doc = "Ack"]
            pub const BOOT_ACK_1: u32 = 0x01;
        }
    }
    #[doc = "BOOT_MODE"]
    pub mod BOOT_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal boot"]
            pub const BOOT_MODE_0: u32 = 0;
            #[doc = "Alternative boot"]
            pub const BOOT_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "BOOT_EN"]
    pub mod BOOT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fast boot disable"]
            pub const BOOT_EN_0: u32 = 0;
            #[doc = "Fast boot enable"]
            pub const BOOT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "AUTO_SABG_EN"]
    pub mod AUTO_SABG_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Time Out"]
    pub mod DISABLE_TIME_OUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable time out"]
            pub const DISABLE_TIME_OUT_0: u32 = 0;
            #[doc = "Disable time out"]
            pub const DISABLE_TIME_OUT_1: u32 = 0x01;
        }
    }
    #[doc = "BOOT_BLK_CNT"]
    pub mod BOOT_BLK_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Vendor Specific 2 Register"]
pub mod VEND_SPEC2 {
    #[doc = "Card Interrupt Detection Test"]
    pub mod CARD_INT_D3_TEST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Check the card interrupt only when DATA3 is high."]
            pub const CARD_INT_D3_TEST_0: u32 = 0;
            #[doc = "Check the card interrupt by ignoring the status of DATA3."]
            pub const CARD_INT_D3_TEST_1: u32 = 0x01;
        }
    }
    #[doc = "TUNING_8bit_EN"]
    pub mod TUNING_8BIT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TUNING_1bit_EN"]
    pub mod TUNING_1BIT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TUNING_CMD_EN"]
    pub mod TUNING_CMD_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auto tuning circuit does not check the CMD line."]
            pub const TUNING_CMD_EN_0: u32 = 0;
            #[doc = "Auto tuning circuit checks the CMD line."]
            pub const TUNING_CMD_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Argument2 register enable for ACMD23"]
    pub mod ACMD23_ARGU2_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const ACMD23_ARGU2_EN_0: u32 = 0;
            #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
            pub const ACMD23_ARGU2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "debug for part dll"]
    pub mod PART_DLL_DEBUG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BUS reset"]
    pub mod BUS_RST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tuning Control Register"]
pub mod TUNING_CTRL {
    #[doc = "TUNING_START_TAP"]
    pub mod TUNING_START_TAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TUNING_COUNTER"]
    pub mod TUNING_COUNTER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TUNING_STEP"]
    pub mod TUNING_STEP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TUNING_WINDOW"]
    pub mod TUNING_WINDOW {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "STD_TUNING_EN"]
    pub mod STD_TUNING_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
