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
    #[doc = "ADMA Error Status"]
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
    _reserved2: [u8; 0x04],
    #[doc = "Strobe DLL control"]
    pub STROBE_DLL_CTRL: crate::RWRegister<u32>,
    #[doc = "Strobe DLL status"]
    pub STROBE_DLL_STATUS: crate::RORegister<u32>,
    _reserved3: [u8; 0x48],
    #[doc = "Vendor Specific Register"]
    pub VEND_SPEC: crate::RWRegister<u32>,
    #[doc = "eMMC Boot"]
    pub MMC_BOOT: crate::RWRegister<u32>,
    #[doc = "Vendor Specific 2 Register"]
    pub VEND_SPEC2: crate::RWRegister<u32>,
    #[doc = "Tuning Control"]
    pub TUNING_CTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x30],
    #[doc = "Command Queuing Version"]
    pub CQVER: crate::RORegister<u32>,
    #[doc = "Command Queuing Capabilities"]
    pub CQCAP: crate::RWRegister<u32>,
    #[doc = "Command Queuing Configuration"]
    pub CQCFG: crate::RWRegister<u32>,
    #[doc = "Command Queuing Control"]
    pub CQCTL: crate::RWRegister<u32>,
    #[doc = "Command Queuing Interrupt Status"]
    pub CQIS: crate::RWRegister<u32>,
    #[doc = "Command Queuing Interrupt Status Enable"]
    pub CQISTE: crate::RWRegister<u32>,
    #[doc = "Command Queuing Interrupt Signal Enable"]
    pub CQISGE: crate::RWRegister<u32>,
    #[doc = "Command Queuing Interrupt Coalescing"]
    pub CQIC: crate::RWRegister<u32>,
    #[doc = "Command Queuing Task Descriptor List Base Address"]
    pub CQTDLBA: crate::RWRegister<u32>,
    #[doc = "Command Queuing Task Descriptor List Base Address Upper 32 Bits"]
    pub CQTDLBAU: crate::RWRegister<u32>,
    #[doc = "Command Queuing Task Doorbell"]
    pub CQTDBR: crate::RWRegister<u32>,
    #[doc = "Command Queuing Task Completion Notification"]
    pub CQTCN: crate::RWRegister<u32>,
    #[doc = "Command Queuing Device Queue Status"]
    pub CQDQS: crate::RORegister<u32>,
    #[doc = "Command Queuing Device Pending Tasks"]
    pub CQDPT: crate::RORegister<u32>,
    #[doc = "Command Queuing Task Clear"]
    pub CQTCLR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x04],
    #[doc = "Command Queuing Send Status Configuration 1"]
    pub CQSSC1: crate::RWRegister<u32>,
    #[doc = "Command Queuing Send Status Configuration 2"]
    pub CQSSC2: crate::RWRegister<u32>,
    #[doc = "Command Queuing Command Response for Direct-Command Task"]
    pub CQCRDCT: crate::RORegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "Command Queuing Response Mode Error Mask"]
    pub CQRMEM: crate::RWRegister<u32>,
    #[doc = "Command Queuing Task Error Information"]
    pub CQTERRI: crate::RORegister<u32>,
    #[doc = "Command Queuing Command Response Index"]
    pub CQCRI: crate::RORegister<u32>,
    #[doc = "Command Queuing Command Response Argument"]
    pub CQCRA: crate::RORegister<u32>,
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
    #[doc = "DMAEN"]
    pub mod DMAEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP_0B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP_0A: u32 = 0x01;
        }
    }
    #[doc = "BCEN"]
    pub mod BCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP1_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP1_A: u32 = 0x01;
        }
    }
    #[doc = "AC12EN"]
    pub mod AC12EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP2_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP2_A: u32 = 0x01;
        }
    }
    #[doc = "DDR_EN"]
    pub mod DDR_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP3_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP3_A: u32 = 0x01;
        }
    }
    #[doc = "DTDSEL"]
    pub mod DTDSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP4_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP4_A: u32 = 0x01;
        }
    }
    #[doc = "MSBSEL"]
    pub mod MSBSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP5_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP5_A: u32 = 0x01;
        }
    }
    #[doc = "NIBBLE_POS"]
    pub mod NIBBLE_POS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP6_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP6_A: u32 = 0x01;
        }
    }
    #[doc = "AC23EN"]
    pub mod AC23EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP7_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP7_A: u32 = 0x01;
        }
    }
    #[doc = "Response type select"]
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
    #[doc = "Command CRC check enable"]
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
    #[doc = "Command index check enable"]
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
    #[doc = "Data present select"]
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
    #[doc = "Command type"]
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
    #[doc = "Command index"]
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
    #[doc = "Command inhibit (CMD)"]
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
    #[doc = "Command Inhibit Data (DATA)"]
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
    #[doc = "Data line active"]
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
    #[doc = "SD clock stable"]
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
    #[doc = "Write transfer active"]
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
    #[doc = "Read transfer active"]
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
    #[doc = "Buffer write enable"]
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
    #[doc = "Buffer read enable"]
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
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode, and eMMC HS200 mode)"]
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
    #[doc = "Tap select change done"]
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
    #[doc = "Card inserted"]
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
    #[doc = "Card detect pin level"]
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
    #[doc = "Write protect switch pin level"]
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
    #[doc = "CMD line signal level"]
    pub mod CLSL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DATA\\[7:0\\] line signal level"]
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
    #[doc = "Data transfer width"]
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
    #[doc = "DATA3 as card detection pin"]
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
    #[doc = "Endian mode"]
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
    #[doc = "DMA select"]
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
    #[doc = "Stop at block gap request"]
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
    #[doc = "Continue request"]
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
    #[doc = "Read wait control"]
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
    #[doc = "Interrupt at block gap"]
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
    #[doc = "Read performed number 8 clock"]
    pub mod RD_DONE_NO_8CLK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup event enable on card interrupt"]
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
    #[doc = "Wakeup event enable on SD card insertion"]
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
    #[doc = "Wakeup event enable on SD card removal"]
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
    #[doc = "Non-exact block read"]
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
    #[doc = "SDCLK frequency select"]
    pub mod SDCLKFS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data timeout counter value"]
    pub mod DTOCV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDCLK x 2 32"]
            pub const DTOCV_X: u32 = 0;
            #[doc = "SDCLK x 2 33"]
            pub const DTOCV_W: u32 = 0x01;
            #[doc = "SDCLK x 2 18"]
            pub const DTOCV_V: u32 = 0x02;
            #[doc = "SDCLK x 2 19"]
            pub const DTOCV_U: u32 = 0x03;
            #[doc = "SDCLK x 2 29, recommend to use for supported speed modes except HS200, HS400, SDR104 mode"]
            pub const DTOCV_T: u32 = 0x0d;
            #[doc = "SDCLK x 2 30, recommend to use for HS200 and SDR104 mode"]
            pub const DTOCV_S: u32 = 0x0e;
            #[doc = "SDCLK x 2 31, recommend to use for HS400 mode"]
            pub const DTOCV_R: u32 = 0x0f;
        }
    }
    #[doc = "Reset the async FIFO"]
    pub mod RST_FIFO {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware reset"]
    pub mod IPP_RST_N {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset for all"]
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
    #[doc = "Software reset for CMD line"]
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
    #[doc = "Software reset for data line"]
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
    #[doc = "Initialization active"]
    pub mod INITA {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset tuning"]
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
    #[doc = "Command complete"]
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
    #[doc = "Transfer complete"]
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
    #[doc = "Block gap event"]
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
    #[doc = "DMA interrupt"]
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
    #[doc = "Buffer write ready"]
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
    #[doc = "Buffer read ready"]
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
    #[doc = "Card insertion"]
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
    #[doc = "Card removal"]
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
    #[doc = "Card interrupt"]
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
    #[doc = "Re-tuning event: (only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
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
    #[doc = "Tuning pass:(only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
    pub mod TP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command queuing interrupt"]
    pub mod CQI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Interrupt Status"]
    pub mod ERR_INT_STATUS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command timeout error"]
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
    #[doc = "Command CRC error"]
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
    #[doc = "Command end bit error"]
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
    #[doc = "Command index error"]
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
    #[doc = "Data timeout error"]
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
    #[doc = "Data CRC error"]
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
    #[doc = "Data end bit error"]
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
    #[doc = "Auto CMD12 error"]
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
    #[doc = "Tuning error: (only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
    pub mod TNE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA error"]
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
    #[doc = "Command complete status enable"]
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
    #[doc = "Transfer complete status enable"]
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
    #[doc = "Block gap event status enable"]
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
    #[doc = "DMA interrupt status enable"]
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
    #[doc = "Buffer write ready status enable"]
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
    #[doc = "Buffer read ready status enable"]
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
    #[doc = "Card insertion status enable"]
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
    #[doc = "Card removal status enable"]
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
    #[doc = "Card interrupt status enable"]
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
    #[doc = "Re-tuning event status enable"]
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
    #[doc = "Tuning pass status enable"]
    pub mod TPSEN {
        pub const offset: u32 = 13;
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
    #[doc = "Command queuing status enable"]
    pub mod CQISEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CQISEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CQISEN_B: u32 = 0x01;
        }
    }
    #[doc = "Command timeout error status enable"]
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
    #[doc = "Command CRC error status enable"]
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
    #[doc = "Command end bit error status enable"]
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
    #[doc = "Command index error status enable"]
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
    #[doc = "Data timeout error status enable"]
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
    #[doc = "Data CRC error status enable"]
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
    #[doc = "Data end bit error status enable"]
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
    #[doc = "Auto CMD12 error status enable"]
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
    #[doc = "Tuning error status enable"]
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
    #[doc = "DMA error status enable"]
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
    #[doc = "Command complete interrupt enable"]
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
    #[doc = "Transfer complete interrupt enable"]
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
    #[doc = "Block gap event interrupt enable"]
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
    #[doc = "DMA interrupt enable"]
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
    #[doc = "Buffer write ready interrupt enable"]
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
    #[doc = "Buffer read ready interrupt enable"]
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
    #[doc = "Card insertion interrupt enable"]
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
    #[doc = "Card removal interrupt enable"]
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
    #[doc = "Card interrupt enable"]
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
    #[doc = "Re-tuning event interrupt enable"]
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
    #[doc = "Tuning pass interrupt enable"]
    pub mod TPIEN {
        pub const offset: u32 = 13;
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
    #[doc = "Command queuing signal enable"]
    pub mod CQIIEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CQIIEN_T: u32 = 0;
            #[doc = "Enabled"]
            pub const CQIIEN_S: u32 = 0x01;
        }
    }
    #[doc = "Command timeout error interrupt enable"]
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
    #[doc = "Command CRC error interrupt enable"]
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
    #[doc = "Command end bit error interrupt enable"]
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
    #[doc = "Command index error interrupt enable"]
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
    #[doc = "Data timeout error interrupt enable"]
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
    #[doc = "Data CRC error interrupt enable"]
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
    #[doc = "Data end bit error interrupt enable"]
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
    #[doc = "Auto CMD12 error interrupt enable"]
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
    #[doc = "Tuning error interrupt enable"]
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
    #[doc = "DMA error interrupt enable"]
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
    #[doc = "Auto CMD12 not executed"]
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
    #[doc = "Auto CMD12 / 23 timeout error"]
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
    #[doc = "Auto CMD12 / 23 CRC error"]
    pub mod AC12CE {
        pub const offset: u32 = 2;
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
    #[doc = "Auto CMD12 / 23 end bit error"]
    pub mod AC12EBE {
        pub const offset: u32 = 3;
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
    #[doc = "Auto CMD12 / 23 index error"]
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
    #[doc = "Command not issued by Auto CMD12 error"]
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
    #[doc = "Execute tuning"]
    pub mod EXECUTE_TUNING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tuning procedure is aborted"]
            pub const EXECUTE_TUNING_0: u32 = 0;
            #[doc = "Start tuning procedure"]
            pub const EXECUTE_TUNING_1: u32 = 0x01;
        }
    }
    #[doc = "Sample clock select"]
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
    #[doc = "Max block length"]
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
    #[doc = "ADMA support"]
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
    #[doc = "High speed support"]
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
    #[doc = "DMA support"]
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
    #[doc = "Suspend / resume support"]
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
    #[doc = "Voltage support 3.3 V"]
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
    #[doc = "Voltage support 3.0 V"]
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
    #[doc = "Voltage support 1.8 V"]
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
    #[doc = "Read watermark level"]
    pub mod RD_WML {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write watermark level"]
    pub mod WR_WML {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Mixer Control"]
pub mod MIX_CTRL {
    #[doc = "DMA enable"]
    pub mod DMAEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DMAIN_B: u32 = 0;
            #[doc = "Enable"]
            pub const DMAIN_A: u32 = 0x01;
        }
    }
    #[doc = "Block count enable"]
    pub mod BCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const BCEN_B: u32 = 0;
            #[doc = "Enable"]
            pub const BCEN_A: u32 = 0x01;
        }
    }
    #[doc = "Auto CMD12 enable"]
    pub mod AC12EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const AC12EN_B: u32 = 0;
            #[doc = "Enable"]
            pub const AC12EN_A: u32 = 0x01;
        }
    }
    #[doc = "Dual data rate mode selection"]
    pub mod DDR_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data transfer direction select"]
    pub mod DTDSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write (Host to card)"]
            pub const DTDSEL_B: u32 = 0;
            #[doc = "Read (Card to host)"]
            pub const DTDSEL_A: u32 = 0x01;
        }
    }
    #[doc = "Multi / Single block select"]
    pub mod MSBSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single block"]
            pub const MSBSEL_B: u32 = 0;
            #[doc = "Multiple blocks"]
            pub const MSBSEL_A: u32 = 0x01;
        }
    }
    #[doc = "Nibble position indication"]
    pub mod NIBBLE_POS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD23 enable"]
    pub mod AC23EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Execute tuning: (Only used for SD3.0, SDR104 mode and eMMC HS200 mode)"]
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
    #[doc = "Clock selection"]
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
    #[doc = "Auto tuning enable (Only used for SD3.0, SDR104 mode and eMMC HS200 mode)"]
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
    #[doc = "Feedback clock source selection (Only used for SD3.0, SDR104 mode and eMMC HS200 mode)"]
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
    #[doc = "Enable HS400 mode"]
    pub mod HS400_MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable enhance HS400 mode"]
    pub mod EN_HS400_MODE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
#[doc = "ADMA Error Status"]
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
#[doc = "Strobe DLL control"]
pub mod STROBE_DLL_CTRL {
    #[doc = "Strobe DLL control enable"]
    pub mod STROBE_DLL_CTRL_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL control reset"]
    pub mod STROBE_DLL_CTRL_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL control slave force updated"]
    pub mod STROBE_DLL_CTRL_SLV_FORCE_UPD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL Control Slave Delay Target"]
    pub mod STROBE_DLL_CTRL_SLV_DLY_TARGET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL control gate update"]
    pub mod STROBE_DLL_CTRL_GATE_UPDATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL control slave override"]
    pub mod STROBE_DLL_CTRL_SLV_OVERRIDE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL control slave Override value"]
    pub mod STROBE_DLL_CTRL_SLV_OVERRIDE_VAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL control slave update interval"]
    pub mod STROBE_DLL_CTRL_SLV_UPDATE_INT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL control reference update interval"]
    pub mod STROBE_DLL_CTRL_REF_UPDATE_INT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Strobe DLL status"]
pub mod STROBE_DLL_STATUS {
    #[doc = "Strobe DLL status slave lock"]
    pub mod STROBE_DLL_STS_SLV_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL status reference lock"]
    pub mod STROBE_DLL_STS_REF_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL status slave select"]
    pub mod STROBE_DLL_STS_SLV_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Strobe DLL status reference select"]
    pub mod STROBE_DLL_STS_REF_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Vendor Specific Register"]
pub mod VEND_SPEC {
    #[doc = "Voltage selection"]
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
    #[doc = "Check busy enable"]
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
    #[doc = "Force CLK"]
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
    #[doc = "Register byte access for CMD_XFR_TYP"]
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
#[doc = "eMMC Boot"]
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
    #[doc = "Card interrupt detection test"]
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
    #[doc = "Tuning bit enable"]
    pub mod TUNING_BIT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Tuning circuit for DATA\\[3:0\\]"]
            pub const TUNING_BIT_EN_0: u32 = 0;
            #[doc = "Enable Tuning circuit for DATA\\[7:0\\]"]
            pub const TUNING_BIT_EN_1: u32 = 0x01;
            #[doc = "Enable Tuning circuit for DATA\\[0\\]"]
            pub const TUNING_BIT_EN_2: u32 = 0x02;
            #[doc = "Invalid"]
            pub const TUNING_BIT_EN_3: u32 = 0x03;
        }
    }
    #[doc = "Tuning command enable"]
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
    #[doc = "HS400 write clock stop enable"]
    pub mod HS400_WR_CLK_STOP_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HS400 read clock stop enable"]
    pub mod HS400_RD_CLK_STOP_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
    #[doc = "Select the clock source for host card detection."]
    pub mod EN_32K_CLK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the peripheral clock (ipg_clk) for card detection."]
            pub const CD_CLK_SEL_A: u32 = 0;
            #[doc = "Use the low power clock (ipg_clk_lp) for card detection."]
            pub const CD_CLK_SEL_B: u32 = 0x01;
        }
    }
    #[doc = "Enable extra delay on internal feedback clock"]
    pub mod FBCLK_TAP_SEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tuning Control"]
pub mod TUNING_CTRL {
    #[doc = "Tuning start"]
    pub mod TUNING_START_TAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable command check for standard tuning"]
    pub mod DIS_CMD_CHK_FOR_STD_TUNING {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tuning counter"]
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
    #[doc = "Data window"]
    pub mod TUNING_WINDOW {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Standard tuning circuit and procedure enable"]
    pub mod STD_TUNING_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Version"]
pub mod CQVER {
    #[doc = "eMMC version suffix"]
    pub mod VERSION_SUFFIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "eMMC minor version number"]
    pub mod MINOR_VN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "eMMC major version number"]
    pub mod MAJOR_VN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Capabilities"]
pub mod CQCAP {
    #[doc = "Internal timer clock frequency value"]
    pub mod ITCFVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Internal timer clock frequency multiplier"]
    pub mod ITCFMUL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.001 MHz"]
            pub const ITCFMUL_A: u32 = 0x01;
            #[doc = "0.01 MHz"]
            pub const ITCFMUL_B: u32 = 0x02;
            #[doc = "0.1 MHz"]
            pub const ITCFMUL_C: u32 = 0x03;
            #[doc = "1 MHz"]
            pub const ITCFMUL_D: u32 = 0x04;
            #[doc = "10 MHz"]
            pub const ITCFMUL_E: u32 = 0x05;
        }
    }
}
#[doc = "Command Queuing Configuration"]
pub mod CQCFG {
    #[doc = "Command queuing enable"]
    pub mod CQUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task descriptor size"]
    pub mod TDS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Task descriptor size is 64 bits"]
            pub const TDS_A: u32 = 0;
            #[doc = "Task descriptor size is 128 bits"]
            pub const TDS_B: u32 = 0x01;
        }
    }
    #[doc = "Direct command (DCMD) enable"]
    pub mod DCMDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Task descriptor in slot #31 is a Data Transfer Task Descriptor"]
            pub const DCMDE_A: u32 = 0;
            #[doc = "Task descriptor in slot #31 is a DCMD Task Descriptor"]
            pub const DCMDE_B: u32 = 0x01;
        }
    }
}
#[doc = "Command Queuing Control"]
pub mod CQCTL {
    #[doc = "Halt"]
    pub mod HALT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear all tasks"]
    pub mod CLEAR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Interrupt Status"]
pub mod CQIS {
    #[doc = "Halt complete interrupt"]
    pub mod HAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task complete interrupt"]
    pub mod TCC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response error detected interrupt"]
    pub mod RED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task cleared"]
    pub mod TCL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Interrupt Status Enable"]
pub mod CQISTE {
    #[doc = "Halt complete status enable"]
    pub mod HAC_STE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CQIS\\[HAC\\] is disabled"]
            pub const HAC_STE_A: u32 = 0;
            #[doc = "CQIS\\[HAC\\] is set when its interrupt condition is active"]
            pub const HAC_STE_B: u32 = 0x01;
        }
    }
    #[doc = "Task complete status enable"]
    pub mod TCC_STE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CQIS\\[TCC\\] is disabled"]
            pub const TCC_STE_A: u32 = 0;
            #[doc = "CQIS\\[TCC\\] is set when its interrupt condition is active"]
            pub const TCC_STE_B: u32 = 0x01;
        }
    }
    #[doc = "Response error detected status enable"]
    pub mod RED_STE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CQIS\\[RED\\]is disabled"]
            pub const RED_STE_A: u32 = 0;
            #[doc = "CQIS\\[RED\\] is set when its interrupt condition is active"]
            pub const RED_STE_B: u32 = 0x01;
        }
    }
    #[doc = "Task cleared status enable"]
    pub mod TCL_STE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CQIS\\[TCL\\] is disabled"]
            pub const TCL_STE_A: u32 = 0;
            #[doc = "CQIS\\[TCL\\] is set when its interrupt condition is active"]
            pub const TCL_STE_B: u32 = 0x01;
        }
    }
}
#[doc = "Command Queuing Interrupt Signal Enable"]
pub mod CQISGE {
    #[doc = "Halt complete signal enable"]
    pub mod HAC_SGE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task complete signal enable"]
    pub mod TCC_SGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response error detected signal enable"]
    pub mod RED_SGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task cleared signal enable"]
    pub mod TCL_SGE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Interrupt Coalescing"]
pub mod CQIC {
    #[doc = "Interrupt coalescing timeout value"]
    pub mod ICTOVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt coalescing timeout value write enable"]
    pub mod ICTOVALWEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt coalescing counter threshold"]
    pub mod ICCTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt coalescing counter threshold write enable"]
    pub mod ICCTHWEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counter and timer reset"]
    pub mod ICCTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt coalescing status"]
    pub mod ICSB {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No task completions have occurred since last counter reset (IC counter =0)"]
            pub const ICSB_A: u32 = 0;
            #[doc = "At least one task completion has been counted (IC counter >0)"]
            pub const ICSB_B: u32 = 0x01;
        }
    }
    #[doc = "Interrupt coalescing enable/disable"]
    pub mod ICENDIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Task Descriptor List Base Address"]
pub mod CQTDLBA {
    #[doc = "Task descriptor list base address"]
    pub mod TDLBA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Task Descriptor List Base Address Upper 32 Bits"]
pub mod CQTDLBAU {
    #[doc = "Task descriptor list base address"]
    pub mod TDLBAU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Task Doorbell"]
pub mod CQTDBR {
    #[doc = "Task doorbell"]
    pub mod TDBR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Task Completion Notification"]
pub mod CQTCN {
    #[doc = "Task complete notification"]
    pub mod TCN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Device Queue Status"]
pub mod CQDQS {
    #[doc = "Device queue status"]
    pub mod DQS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Device Pending Tasks"]
pub mod CQDPT {
    #[doc = "Device pending tasks"]
    pub mod DPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Task Clear"]
pub mod CQTCLR {
    #[doc = "Task clear"]
    pub mod TCLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Send Status Configuration 1"]
pub mod CQSSC1 {
    #[doc = "Send status command idle timer"]
    pub mod CIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Send status command block counter"]
    pub mod CBC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Send Status Configuration 2"]
pub mod CQSSC2 {
    #[doc = "Send queue status RCA"]
    pub mod SSC2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Command Response for Direct-Command Task"]
pub mod CQCRDCT {
    #[doc = "Direct command last response"]
    pub mod CRDCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Response Mode Error Mask"]
pub mod CQRMEM {
    #[doc = "Response mode error mask"]
    pub mod RMEM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When a R1/R1b response is received, bit i in the device status is ignored"]
            pub const RMEM_A: u32 = 0;
            #[doc = "When a R1/R1b response is received, with bit i in the device status set, a RED interrupt is generated"]
            pub const RMEM_B: u32 = 0x01;
        }
    }
}
#[doc = "Command Queuing Task Error Information"]
pub mod CQTERRI {
    #[doc = "Response mode error command index"]
    pub mod RMECI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response mode error task ID"]
    pub mod RMETID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response mode error fields valid"]
    pub mod RMEFV {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data transfer error command index"]
    pub mod DTECI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data transfer error task ID"]
    pub mod DTETID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data transfer error fields valid"]
    pub mod DTEFV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Command Response Index"]
pub mod CQCRI {
    #[doc = "Last command response index"]
    pub mod LCMDRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Queuing Command Response Argument"]
pub mod CQCRA {
    #[doc = "Last command response argument"]
    pub mod LCMDRA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
