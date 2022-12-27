#[doc = "ENET"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "Interrupt Event Register"]
    pub EIR: crate::RWRegister<u32>,
    #[doc = "Interrupt Mask Register"]
    pub EIMR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Receive Descriptor Active Register - Ring 0"]
    pub RDAR: crate::RWRegister<u32>,
    #[doc = "Transmit Descriptor Active Register - Ring 0"]
    pub TDAR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "Ethernet Control Register"]
    pub ECR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x18],
    #[doc = "MII Management Frame Register"]
    pub MMFR: crate::RWRegister<u32>,
    #[doc = "MII Speed Control Register"]
    pub MSCR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x1c],
    #[doc = "MIB Control Register"]
    pub MIBC: crate::RWRegister<u32>,
    _reserved5: [u8; 0x1c],
    #[doc = "Receive Control Register"]
    pub RCR: crate::RWRegister<u32>,
    _reserved6: [u8; 0x3c],
    #[doc = "Transmit Control Register"]
    pub TCR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x1c],
    #[doc = "Physical Address Lower Register"]
    pub PALR: crate::RWRegister<u32>,
    #[doc = "Physical Address Upper Register"]
    pub PAUR: crate::RWRegister<u32>,
    #[doc = "Opcode/Pause Duration Register"]
    pub OPD: crate::RWRegister<u32>,
    #[doc = "Transmit Interrupt Coalescing Register"]
    pub TXIC: [crate::RWRegister<u32>; 3usize],
    _reserved8: [u8; 0x04],
    #[doc = "Receive Interrupt Coalescing Register"]
    pub RXIC: [crate::RWRegister<u32>; 3usize],
    _reserved9: [u8; 0x0c],
    #[doc = "Descriptor Individual Upper Address Register"]
    pub IAUR: crate::RWRegister<u32>,
    #[doc = "Descriptor Individual Lower Address Register"]
    pub IALR: crate::RWRegister<u32>,
    #[doc = "Descriptor Group Upper Address Register"]
    pub GAUR: crate::RWRegister<u32>,
    #[doc = "Descriptor Group Lower Address Register"]
    pub GALR: crate::RWRegister<u32>,
    _reserved10: [u8; 0x1c],
    #[doc = "Transmit FIFO Watermark Register"]
    pub TFWR: crate::RWRegister<u32>,
    _reserved11: [u8; 0x38],
    #[doc = "Receive Descriptor Ring 0 Start Register"]
    pub RDSR: crate::RWRegister<u32>,
    #[doc = "Transmit Buffer Descriptor Ring 0 Start Register"]
    pub TDSR: crate::RWRegister<u32>,
    #[doc = "Maximum Receive Buffer Size Register - Ring 0"]
    pub MRBR: crate::RWRegister<u32>,
    _reserved12: [u8; 0x04],
    #[doc = "Receive FIFO Section Full Threshold"]
    pub RSFL: crate::RWRegister<u32>,
    #[doc = "Receive FIFO Section Empty Threshold"]
    pub RSEM: crate::RWRegister<u32>,
    #[doc = "Receive FIFO Almost Empty Threshold"]
    pub RAEM: crate::RWRegister<u32>,
    #[doc = "Receive FIFO Almost Full Threshold"]
    pub RAFL: crate::RWRegister<u32>,
    #[doc = "Transmit FIFO Section Empty Threshold"]
    pub TSEM: crate::RWRegister<u32>,
    #[doc = "Transmit FIFO Almost Empty Threshold"]
    pub TAEM: crate::RWRegister<u32>,
    #[doc = "Transmit FIFO Almost Full Threshold"]
    pub TAFL: crate::RWRegister<u32>,
    #[doc = "Transmit Inter-Packet Gap"]
    pub TIPG: crate::RWRegister<u32>,
    #[doc = "Frame Truncation Length"]
    pub FTRL: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "Transmit Accelerator Function Configuration"]
    pub TACC: crate::RWRegister<u32>,
    #[doc = "Receive Accelerator Function Configuration"]
    pub RACC: crate::RWRegister<u32>,
    _reserved14: [u8; 0x3c],
    #[doc = "Tx Packet Count Statistic Register"]
    pub RMON_T_PACKETS: crate::RORegister<u32>,
    #[doc = "Tx Broadcast Packets Statistic Register"]
    pub RMON_T_BC_PKT: crate::RORegister<u32>,
    #[doc = "Tx Multicast Packets Statistic Register"]
    pub RMON_T_MC_PKT: crate::RORegister<u32>,
    #[doc = "Tx Packets with CRC/Align Error Statistic Register"]
    pub RMON_T_CRC_ALIGN: crate::RORegister<u32>,
    #[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
    pub RMON_T_UNDERSIZE: crate::RORegister<u32>,
    #[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
    pub RMON_T_OVERSIZE: crate::RORegister<u32>,
    #[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub RMON_T_FRAG: crate::RORegister<u32>,
    #[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
    pub RMON_T_JAB: crate::RORegister<u32>,
    #[doc = "Tx Collision Count Statistic Register"]
    pub RMON_T_COL: crate::RORegister<u32>,
    #[doc = "Tx 64-Byte Packets Statistic Register"]
    pub RMON_T_P64: crate::RORegister<u32>,
    #[doc = "Tx 65- to 127-byte Packets Statistic Register"]
    pub RMON_T_P65TO127: crate::RORegister<u32>,
    #[doc = "Tx 128- to 255-byte Packets Statistic Register"]
    pub RMON_T_P128TO255: crate::RORegister<u32>,
    #[doc = "Tx 256- to 511-byte Packets Statistic Register"]
    pub RMON_T_P256TO511: crate::RORegister<u32>,
    #[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
    pub RMON_T_P512TO1023: crate::RORegister<u32>,
    #[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
    pub RMON_T_P1024TO2047: crate::RORegister<u32>,
    #[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
    pub RMON_T_P_GTE2048: crate::RORegister<u32>,
    #[doc = "Tx Octets Statistic Register"]
    pub RMON_T_OCTETS: crate::RORegister<u32>,
    #[doc = "Reserved Statistic Register"]
    pub IEEE_T_DROP: crate::RORegister<u32>,
    #[doc = "Frames Transmitted OK Statistic Register"]
    pub IEEE_T_FRAME_OK: crate::RORegister<u32>,
    #[doc = "Frames Transmitted with Single Collision Statistic Register"]
    pub IEEE_T_1COL: crate::RORegister<u32>,
    #[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
    pub IEEE_T_MCOL: crate::RORegister<u32>,
    #[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
    pub IEEE_T_DEF: crate::RORegister<u32>,
    #[doc = "Frames Transmitted with Late Collision Statistic Register"]
    pub IEEE_T_LCOL: crate::RORegister<u32>,
    #[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
    pub IEEE_T_EXCOL: crate::RORegister<u32>,
    #[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
    pub IEEE_T_MACERR: crate::RORegister<u32>,
    #[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
    pub IEEE_T_CSERR: crate::RORegister<u32>,
    #[doc = "Reserved Statistic Register"]
    pub IEEE_T_SQE: crate::RORegister<u32>,
    #[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
    pub IEEE_T_FDXFC: crate::RORegister<u32>,
    #[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
    pub IEEE_T_OCTETS_OK: crate::RORegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "Rx Packet Count Statistic Register"]
    pub RMON_R_PACKETS: crate::RORegister<u32>,
    #[doc = "Rx Broadcast Packets Statistic Register"]
    pub RMON_R_BC_PKT: crate::RORegister<u32>,
    #[doc = "Rx Multicast Packets Statistic Register"]
    pub RMON_R_MC_PKT: crate::RORegister<u32>,
    #[doc = "Rx Packets with CRC/Align Error Statistic Register"]
    pub RMON_R_CRC_ALIGN: crate::RORegister<u32>,
    #[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
    pub RMON_R_UNDERSIZE: crate::RORegister<u32>,
    #[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
    pub RMON_R_OVERSIZE: crate::RORegister<u32>,
    #[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub RMON_R_FRAG: crate::RORegister<u32>,
    #[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
    pub RMON_R_JAB: crate::RORegister<u32>,
    _reserved16: [u8; 0x04],
    #[doc = "Rx 64-Byte Packets Statistic Register"]
    pub RMON_R_P64: crate::RORegister<u32>,
    #[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
    pub RMON_R_P65TO127: crate::RORegister<u32>,
    #[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
    pub RMON_R_P128TO255: crate::RORegister<u32>,
    #[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
    pub RMON_R_P256TO511: crate::RORegister<u32>,
    #[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
    pub RMON_R_P512TO1023: crate::RORegister<u32>,
    #[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
    pub RMON_R_P1024TO2047: crate::RORegister<u32>,
    #[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
    pub RMON_R_P_GTE2048: crate::RORegister<u32>,
    #[doc = "Rx Octets Statistic Register"]
    pub RMON_R_OCTETS: crate::RORegister<u32>,
    #[doc = "Frames not Counted Correctly Statistic Register"]
    pub IEEE_R_DROP: crate::RORegister<u32>,
    #[doc = "Frames Received OK Statistic Register"]
    pub IEEE_R_FRAME_OK: crate::RORegister<u32>,
    #[doc = "Frames Received with CRC Error Statistic Register"]
    pub IEEE_R_CRC: crate::RORegister<u32>,
    #[doc = "Frames Received with Alignment Error Statistic Register"]
    pub IEEE_R_ALIGN: crate::RORegister<u32>,
    #[doc = "Receive FIFO Overflow Count Statistic Register"]
    pub IEEE_R_MACERR: crate::RORegister<u32>,
    #[doc = "Flow Control Pause Frames Received Statistic Register"]
    pub IEEE_R_FDXFC: crate::RORegister<u32>,
    #[doc = "Octet Count for Frames Received without Error Statistic Register"]
    pub IEEE_R_OCTETS_OK: crate::RORegister<u32>,
    _reserved17: [u8; 0x011c],
    #[doc = "Adjustable Timer Control Register"]
    pub ATCR: crate::RWRegister<u32>,
    #[doc = "Timer Value Register"]
    pub ATVR: crate::RWRegister<u32>,
    #[doc = "Timer Offset Register"]
    pub ATOFF: crate::RWRegister<u32>,
    #[doc = "Timer Period Register"]
    pub ATPER: crate::RWRegister<u32>,
    #[doc = "Timer Correction Register"]
    pub ATCOR: crate::RWRegister<u32>,
    #[doc = "Time-Stamping Clock Period Register"]
    pub ATINC: crate::RWRegister<u32>,
    #[doc = "Timestamp of Last Transmitted Frame"]
    pub ATSTMP: crate::RORegister<u32>,
    _reserved18: [u8; 0x01e8],
    #[doc = "Timer Global Status Register"]
    pub TGSR: crate::RWRegister<u32>,
    #[doc = "Timer Control Status Register"]
    pub TCSR0: crate::RWRegister<u32>,
    #[doc = "Timer Compare Capture Register"]
    pub TCCR0: crate::RWRegister<u32>,
    #[doc = "Timer Control Status Register"]
    pub TCSR1: crate::RWRegister<u32>,
    #[doc = "Timer Compare Capture Register"]
    pub TCCR1: crate::RWRegister<u32>,
    #[doc = "Timer Control Status Register"]
    pub TCSR2: crate::RWRegister<u32>,
    #[doc = "Timer Compare Capture Register"]
    pub TCCR2: crate::RWRegister<u32>,
    #[doc = "Timer Control Status Register"]
    pub TCSR3: crate::RWRegister<u32>,
    #[doc = "Timer Compare Capture Register"]
    pub TCCR3: crate::RWRegister<u32>,
}
#[doc = "Interrupt Event Register"]
pub mod EIR {
    #[doc = "Timestamp Timer"]
    pub mod TS_TIMER {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Timestamp Available"]
    pub mod TS_AVAIL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Node Wakeup Request Indication"]
    pub mod WAKEUP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Receive Error"]
    pub mod PLR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO Underrun"]
    pub mod UN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Collision Retry Limit"]
    pub mod RL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Late Collision"]
    pub mod LC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ethernet Bus Error"]
    pub mod EBERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MII Interrupt."]
    pub mod MII {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Buffer Interrupt"]
    pub mod RXB {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Frame Interrupt"]
    pub mod RXF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Buffer Interrupt"]
    pub mod TXB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Frame Interrupt"]
    pub mod TXF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Graceful Stop Complete"]
    pub mod GRA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Babbling Transmit Error"]
    pub mod BABT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Babbling Receive Error"]
    pub mod BABR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Mask Register"]
pub mod EIMR {
    #[doc = "TS_TIMER Interrupt Mask"]
    pub mod TS_TIMER {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "TS_AVAIL Interrupt Mask"]
    pub mod TS_AVAIL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "WAKEUP Interrupt Mask"]
    pub mod WAKEUP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "PLR Interrupt Mask"]
    pub mod PLR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "UN Interrupt Mask"]
    pub mod UN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "RL Interrupt Mask"]
    pub mod RL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "LC Interrupt Mask"]
    pub mod LC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "EBERR Interrupt Mask"]
    pub mod EBERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MII Interrupt Mask"]
    pub mod MII {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "RXB Interrupt Mask"]
    pub mod RXB {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "RXF Interrupt Mask"]
    pub mod RXF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const ZERO: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "TXB Interrupt Mask"]
    pub mod TXB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const TXB_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const TXB_1: u32 = 0x01;
        }
    }
    #[doc = "TXF Interrupt Mask"]
    pub mod TXF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const TXF_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const TXF_1: u32 = 0x01;
        }
    }
    #[doc = "GRA Interrupt Mask"]
    pub mod GRA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const GRA_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const GRA_1: u32 = 0x01;
        }
    }
    #[doc = "BABT Interrupt Mask"]
    pub mod BABT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const BABT_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const BABT_1: u32 = 0x01;
        }
    }
    #[doc = "BABR Interrupt Mask"]
    pub mod BABR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding interrupt source is masked."]
            pub const BABR_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const BABR_1: u32 = 0x01;
        }
    }
}
#[doc = "Receive Descriptor Active Register - Ring 0"]
pub mod RDAR {
    #[doc = "Receive Descriptor Active"]
    pub mod RDAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Descriptor Active Register - Ring 0"]
pub mod TDAR {
    #[doc = "Transmit Descriptor Active"]
    pub mod TDAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ethernet Control Register"]
pub mod ECR {
    #[doc = "Ethernet MAC Reset"]
    pub mod RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ethernet Enable"]
    pub mod ETHEREN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
            pub const ETHEREN_0: u32 = 0;
            #[doc = "MAC is enabled, and reception and transmission are possible."]
            pub const ETHEREN_1: u32 = 0x01;
        }
    }
    #[doc = "Magic Packet Detection Enable"]
    pub mod MAGICEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Magic detection logic disabled."]
            pub const MAGICEN_0: u32 = 0;
            #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\] when a frame is detected."]
            pub const MAGICEN_1: u32 = 0x01;
        }
    }
    #[doc = "Sleep Mode Enable"]
    pub mod SLEEP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operating mode."]
            pub const SLEEP_0: u32 = 0;
            #[doc = "Sleep mode."]
            pub const SLEEP_1: u32 = 0x01;
        }
    }
    #[doc = "EN1588 Enable"]
    pub mod EN1588 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Legacy FEC buffer descriptors and functions enabled."]
            pub const EN1588_0: u32 = 0;
            #[doc = "Enhanced frame time-stamping functions enabled."]
            pub const EN1588_1: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC continues operation in debug mode."]
            pub const DBGEN_0: u32 = 0;
            #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
            pub const DBGEN_1: u32 = 0x01;
        }
    }
    #[doc = "Descriptor Byte Swapping Enable"]
    pub mod DBSWP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
            pub const DBSWP_0: u32 = 0;
            #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
            pub const DBSWP_1: u32 = 0x01;
        }
    }
}
#[doc = "MII Management Frame Register"]
pub mod MMFR {
    #[doc = "Management Frame Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Turn Around"]
    pub mod TA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Register Address"]
    pub mod RA {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY Address"]
    pub mod PA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Operation Code"]
    pub mod OP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Of Frame Delimiter"]
    pub mod ST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MII Speed Control Register"]
pub mod MSCR {
    #[doc = "MII Speed"]
    pub mod MII_SPEED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Preamble"]
    pub mod DIS_PRE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Preamble enabled."]
            pub const DIS_PRE_0: u32 = 0;
            #[doc = "Preamble (32 ones) is not prepended to the MII management frame."]
            pub const DIS_PRE_1: u32 = 0x01;
        }
    }
    #[doc = "Hold time On MDIO Output"]
    pub mod HOLDTIME {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 internal module clock cycle"]
            pub const HOLDTIME_0: u32 = 0;
            #[doc = "2 internal module clock cycles"]
            pub const HOLDTIME_1: u32 = 0x01;
            #[doc = "3 internal module clock cycles"]
            pub const HOLDTIME_2: u32 = 0x02;
            #[doc = "8 internal module clock cycles"]
            pub const HOLDTIME_7: u32 = 0x07;
        }
    }
}
#[doc = "MIB Control Register"]
pub mod MIBC {
    #[doc = "MIB Clear"]
    pub mod MIB_CLEAR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "See note above."]
            pub const MIB_CLEAR_0: u32 = 0;
            #[doc = "All statistics counters are reset to 0."]
            pub const MIB_CLEAR_1: u32 = 0x01;
        }
    }
    #[doc = "MIB Idle"]
    pub mod MIB_IDLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The MIB block is updating MIB counters."]
            pub const MIB_IDLE_0: u32 = 0;
            #[doc = "The MIB block is not currently updating any MIB counters."]
            pub const MIB_IDLE_1: u32 = 0x01;
        }
    }
    #[doc = "Disable MIB Logic"]
    pub mod MIB_DIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MIB logic is enabled."]
            pub const MIB_DIS_0: u32 = 0;
            #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
            pub const MIB_DIS_1: u32 = 0x01;
        }
    }
}
#[doc = "Receive Control Register"]
pub mod RCR {
    #[doc = "Internal Loopback"]
    pub mod LOOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Loopback disabled."]
            pub const LOOP_0: u32 = 0;
            #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
            pub const LOOP_1: u32 = 0x01;
        }
    }
    #[doc = "Disable Receive On Transmit"]
    pub mod DRT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
            pub const DRT_0: u32 = 0;
            #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
            pub const DRT_1: u32 = 0x01;
        }
    }
    #[doc = "Media Independent Interface Mode"]
    pub mod MII_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
            pub const MII_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Promiscuous Mode"]
    pub mod PROM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const PROM_0: u32 = 0;
            #[doc = "Enabled."]
            pub const PROM_1: u32 = 0x01;
        }
    }
    #[doc = "Broadcast Frame Reject"]
    pub mod BC_REJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Will not reject frames as described above"]
            pub const ZERO: u32 = 0;
            #[doc = "Will reject frames as described above"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Flow Control Enable"]
    pub mod FCE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable flow control"]
            pub const ZERO: u32 = 0;
            #[doc = "Enable flow control"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "RMII Mode Enable"]
    pub mod RMII_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC configured for MII mode."]
            pub const RMII_MODE_0: u32 = 0;
            #[doc = "MAC configured for RMII operation."]
            pub const RMII_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "Enables 10-Mbit/s mode of the RMII ."]
    pub mod RMII_10T {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100-Mbit/s operation."]
            pub const RMII_10T_0: u32 = 0;
            #[doc = "10-Mbit/s operation."]
            pub const RMII_10T_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Frame Padding Remove On Receive"]
    pub mod PADEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No padding is removed on receive by the MAC."]
            pub const PADEN_0: u32 = 0;
            #[doc = "Padding is removed from received frames."]
            pub const PADEN_1: u32 = 0x01;
        }
    }
    #[doc = "Terminate/Forward Pause Frames"]
    pub mod PAUFWD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pause frames are terminated and discarded in the MAC."]
            pub const PAUFWD_0: u32 = 0;
            #[doc = "Pause frames are forwarded to the user application."]
            pub const PAUFWD_1: u32 = 0x01;
        }
    }
    #[doc = "Terminate/Forward Received CRC"]
    pub mod CRCFWD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRC field of received frames is transmitted to the user application."]
            pub const CRCFWD_0: u32 = 0;
            #[doc = "The CRC field is stripped from the frame."]
            pub const CRCFWD_1: u32 = 0x01;
        }
    }
    #[doc = "MAC Control Frame Enable"]
    pub mod CFEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
            pub const CFEN_0: u32 = 0;
            #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
            pub const CFEN_1: u32 = 0x01;
        }
    }
    #[doc = "Maximum Frame Length"]
    pub mod MAX_FL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Length Check Disable"]
    pub mod NLC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The payload length check is disabled."]
            pub const NLC_0: u32 = 0;
            #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\] field."]
            pub const NLC_1: u32 = 0x01;
        }
    }
    #[doc = "Graceful Receive Stopped"]
    pub mod GRS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive not stopped"]
            pub const ZERO: u32 = 0;
            #[doc = "Receive stopped"]
            pub const ONE: u32 = 0x01;
        }
    }
}
#[doc = "Transmit Control Register"]
pub mod TCR {
    #[doc = "Graceful Transmit Stop"]
    pub mod GTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable graceful transmit stop"]
            pub const ZERO: u32 = 0;
            #[doc = "Enable graceful transmit stop"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Full-Duplex Enable"]
    pub mod FDEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable full-duplex"]
            pub const ZERO: u32 = 0;
            #[doc = "Enable full-duplex"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Frame Control Pause"]
    pub mod TFC_PAUSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No PAUSE frame transmitted."]
            pub const TFC_PAUSE_0: u32 = 0;
            #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
            pub const TFC_PAUSE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Frame Control Pause"]
    pub mod RFC_PAUSE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source MAC Address Select On Transmit"]
    pub mod ADDSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Node MAC address programmed on PADDR1/2 registers."]
            pub const ADDSEL_0: u32 = 0;
        }
    }
    #[doc = "Set MAC Address On Transmit"]
    pub mod ADDINS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The source MAC address is not modified by the MAC."]
            pub const ADDINS_0: u32 = 0;
            #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
            pub const ADDINS_1: u32 = 0x01;
        }
    }
    #[doc = "Forward Frame From Application With CRC"]
    pub mod CRCFWD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TxBD\\[TC\\] controls whether the frame has a CRC from the application."]
            pub const CRCFWD_0: u32 = 0;
            #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
            pub const CRCFWD_1: u32 = 0x01;
        }
    }
}
#[doc = "Physical Address Lower Register"]
pub mod PALR {
    #[doc = "Pause Address"]
    pub mod PADDR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Physical Address Upper Register"]
pub mod PAUR {
    #[doc = "Type Field In PAUSE Frames"]
    pub mod TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    pub mod PADDR2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Opcode/Pause Duration Register"]
pub mod OPD {
    #[doc = "Pause Duration"]
    pub mod PAUSE_DUR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Opcode Field In PAUSE Frames"]
    pub mod OPCODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Interrupt Coalescing Register"]
pub mod TXIC {
    #[doc = "Interrupt coalescing timer threshold"]
    pub mod ICTT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    pub mod ICFT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    pub mod ICCS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MII/GMII TX clocks."]
            pub const ICCS_0: u32 = 0;
            #[doc = "Use ENET system clock."]
            pub const ICCS_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Coalescing Enable"]
    pub mod ICEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Interrupt coalescing."]
            pub const ICEN_0: u32 = 0;
            #[doc = "Enable Interrupt coalescing."]
            pub const ICEN_1: u32 = 0x01;
        }
    }
}
#[doc = "Receive Interrupt Coalescing Register"]
pub mod RXIC {
    #[doc = "Interrupt coalescing timer threshold"]
    pub mod ICTT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    pub mod ICFT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    pub mod ICCS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MII/GMII TX clocks."]
            pub const ICCS_0: u32 = 0;
            #[doc = "Use ENET system clock."]
            pub const ICCS_1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Coalescing Enable"]
    pub mod ICEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Interrupt coalescing."]
            pub const ICEN_0: u32 = 0;
            #[doc = "Enable Interrupt coalescing."]
            pub const ICEN_1: u32 = 0x01;
        }
    }
}
#[doc = "Descriptor Individual Upper Address Register"]
pub mod IAUR {
    #[doc = "Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    pub mod IADDR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Descriptor Individual Lower Address Register"]
pub mod IALR {
    #[doc = "Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    pub mod IADDR2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Descriptor Group Upper Address Register"]
pub mod GAUR {
    #[doc = "Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    pub mod GADDR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Descriptor Group Lower Address Register"]
pub mod GALR {
    #[doc = "Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    pub mod GADDR2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit FIFO Watermark Register"]
pub mod TFWR {
    #[doc = "Transmit FIFO Write"]
    pub mod TFWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64 bytes written."]
            pub const TFWR_0: u32 = 0;
            #[doc = "64 bytes written."]
            pub const TFWR_1: u32 = 0x01;
            #[doc = "128 bytes written."]
            pub const TFWR_2: u32 = 0x02;
            #[doc = "192 bytes written."]
            pub const TFWR_3: u32 = 0x03;
            #[doc = "1984 bytes written."]
            pub const TFWR_31: u32 = 0x1f;
        }
    }
    #[doc = "Store And Forward Enable"]
    pub mod STRFWD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
            pub const STRFWD_0: u32 = 0;
            #[doc = "Enabled."]
            pub const STRFWD_1: u32 = 0x01;
        }
    }
}
#[doc = "Receive Descriptor Ring 0 Start Register"]
pub mod RDSR {
    #[doc = "Pointer to the beginning of the receive buffer descriptor queue."]
    pub mod R_DES_START {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Buffer Descriptor Ring 0 Start Register"]
pub mod TDSR {
    #[doc = "Pointer to the beginning of the transmit buffer descriptor queue."]
    pub mod X_DES_START {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Maximum Receive Buffer Size Register - Ring 0"]
pub mod MRBR {
    #[doc = "Receive buffer size in bytes"]
    pub mod R_BUF_SIZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive FIFO Section Full Threshold"]
pub mod RSFL {
    #[doc = "Value Of Receive FIFO Section Full Threshold"]
    pub mod RX_SECTION_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive FIFO Section Empty Threshold"]
pub mod RSEM {
    #[doc = "Value Of The Receive FIFO Section Empty Threshold"]
    pub mod RX_SECTION_EMPTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Status FIFO Section Empty Threshold"]
    pub mod STAT_SECTION_EMPTY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive FIFO Almost Empty Threshold"]
pub mod RAEM {
    #[doc = "Value Of The Receive FIFO Almost Empty Threshold"]
    pub mod RX_ALMOST_EMPTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive FIFO Almost Full Threshold"]
pub mod RAFL {
    #[doc = "Value Of The Receive FIFO Almost Full Threshold"]
    pub mod RX_ALMOST_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit FIFO Section Empty Threshold"]
pub mod TSEM {
    #[doc = "Value Of The Transmit FIFO Section Empty Threshold"]
    pub mod TX_SECTION_EMPTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit FIFO Almost Empty Threshold"]
pub mod TAEM {
    #[doc = "Value of Transmit FIFO Almost Empty Threshold"]
    pub mod TX_ALMOST_EMPTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit FIFO Almost Full Threshold"]
pub mod TAFL {
    #[doc = "Value Of The Transmit FIFO Almost Full Threshold"]
    pub mod TX_ALMOST_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Inter-Packet Gap"]
pub mod TIPG {
    #[doc = "Transmit Inter-Packet Gap"]
    pub mod IPG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame Truncation Length"]
pub mod FTRL {
    #[doc = "Frame Truncation Length"]
    pub mod TRUNC_FL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Accelerator Function Configuration"]
pub mod TACC {
    #[doc = "TX FIFO Shift-16"]
    pub mod SHIFT16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const SHIFT16_0: u32 = 0;
            #[doc = "Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
            pub const SHIFT16_1: u32 = 0x01;
        }
    }
    #[doc = "Enables insertion of IP header checksum."]
    pub mod IPCHK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Checksum is not inserted."]
            pub const IPCHK_0: u32 = 0;
            #[doc = "If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
            pub const IPCHK_1: u32 = 0x01;
        }
    }
    #[doc = "Enables insertion of protocol checksum."]
    pub mod PROCHK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Checksum not inserted."]
            pub const PROCHK_0: u32 = 0;
            #[doc = "If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
            pub const PROCHK_1: u32 = 0x01;
        }
    }
}
#[doc = "Receive Accelerator Function Configuration"]
pub mod RACC {
    #[doc = "Enable Padding Removal For Short IP Frames"]
    pub mod PADREM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Padding not removed."]
            pub const PADREM_0: u32 = 0;
            #[doc = "Any bytes following the IP payload section of the frame are removed from the frame."]
            pub const PADREM_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    pub mod IPDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frames with wrong IPv4 header checksum are not discarded."]
            pub const IPDIS_0: u32 = 0;
            #[doc = "If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
            pub const IPDIS_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Discard Of Frames With Wrong Protocol Checksum"]
    pub mod PRODIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frames with wrong checksum are not discarded."]
            pub const PRODIS_0: u32 = 0;
            #[doc = "If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
            pub const PRODIS_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Discard Of Frames With MAC Layer Errors"]
    pub mod LINEDIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frames with errors are not discarded."]
            pub const LINEDIS_0: u32 = 0;
            #[doc = "Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
            pub const LINEDIS_1: u32 = 0x01;
        }
    }
    #[doc = "RX FIFO Shift-16"]
    pub mod SHIFT16 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const SHIFT16_0: u32 = 0;
            #[doc = "Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
            pub const SHIFT16_1: u32 = 0x01;
        }
    }
}
#[doc = "Tx Packet Count Statistic Register"]
pub mod RMON_T_PACKETS {
    #[doc = "Packet count"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Broadcast Packets Statistic Register"]
pub mod RMON_T_BC_PKT {
    #[doc = "Broadcast packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Multicast Packets Statistic Register"]
pub mod RMON_T_MC_PKT {
    #[doc = "Multicast packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Packets with CRC/Align Error Statistic Register"]
pub mod RMON_T_CRC_ALIGN {
    #[doc = "Packets with CRC/align error"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
pub mod RMON_T_UNDERSIZE {
    #[doc = "Number of transmit packets less than 64 bytes with good CRC"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
pub mod RMON_T_OVERSIZE {
    #[doc = "Number of transmit packets greater than MAX_FL bytes with good CRC"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod RMON_T_FRAG {
    #[doc = "Number of packets less than 64 bytes with bad CRC"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
pub mod RMON_T_JAB {
    #[doc = "Number of transmit packets greater than MAX_FL bytes and bad CRC"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Collision Count Statistic Register"]
pub mod RMON_T_COL {
    #[doc = "Number of transmit collisions"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx 64-Byte Packets Statistic Register"]
pub mod RMON_T_P64 {
    #[doc = "Number of 64-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx 65- to 127-byte Packets Statistic Register"]
pub mod RMON_T_P65TO127 {
    #[doc = "Number of 65- to 127-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx 128- to 255-byte Packets Statistic Register"]
pub mod RMON_T_P128TO255 {
    #[doc = "Number of 128- to 255-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx 256- to 511-byte Packets Statistic Register"]
pub mod RMON_T_P256TO511 {
    #[doc = "Number of 256- to 511-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
pub mod RMON_T_P512TO1023 {
    #[doc = "Number of 512- to 1023-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
pub mod RMON_T_P1024TO2047 {
    #[doc = "Number of 1024- to 2047-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
pub mod RMON_T_P_GTE2048 {
    #[doc = "Number of transmit packets greater than 2048 bytes"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Octets Statistic Register"]
pub mod RMON_T_OCTETS {
    #[doc = "Number of transmit octets"]
    pub mod TXOCTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Transmitted OK Statistic Register"]
pub mod IEEE_T_FRAME_OK {
    #[doc = "Number of frames transmitted OK"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Transmitted with Single Collision Statistic Register"]
pub mod IEEE_T_1COL {
    #[doc = "Number of frames transmitted with one collision"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
pub mod IEEE_T_MCOL {
    #[doc = "Number of frames transmitted with multiple collisions"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
pub mod IEEE_T_DEF {
    #[doc = "Number of frames transmitted with deferral delay"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Transmitted with Late Collision Statistic Register"]
pub mod IEEE_T_LCOL {
    #[doc = "Number of frames transmitted with late collision"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
pub mod IEEE_T_EXCOL {
    #[doc = "Number of frames transmitted with excessive collisions"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
pub mod IEEE_T_MACERR {
    #[doc = "Number of frames transmitted with transmit FIFO underrun"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
pub mod IEEE_T_CSERR {
    #[doc = "Number of frames transmitted with carrier sense error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reserved Statistic Register"]
pub mod IEEE_T_SQE {
    #[doc = "This read-only field is reserved and always has the value 0"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
pub mod IEEE_T_FDXFC {
    #[doc = "Number of flow-control pause frames transmitted"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
pub mod IEEE_T_OCTETS_OK {
    #[doc = "Octet count for frames transmitted without error Counts total octets (includes header and FCS fields)."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Packet Count Statistic Register"]
pub mod RMON_R_PACKETS {
    #[doc = "Number of packets received"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Broadcast Packets Statistic Register"]
pub mod RMON_R_BC_PKT {
    #[doc = "Number of receive broadcast packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Multicast Packets Statistic Register"]
pub mod RMON_R_MC_PKT {
    #[doc = "Number of receive multicast packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Packets with CRC/Align Error Statistic Register"]
pub mod RMON_R_CRC_ALIGN {
    #[doc = "Number of receive packets with CRC or align error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
pub mod RMON_R_UNDERSIZE {
    #[doc = "Number of receive packets with less than 64 bytes and good CRC"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
pub mod RMON_R_OVERSIZE {
    #[doc = "Number of receive packets greater than MAX_FL and good CRC"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod RMON_R_FRAG {
    #[doc = "Number of receive packets with less than 64 bytes and bad CRC"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
pub mod RMON_R_JAB {
    #[doc = "Number of receive packets greater than MAX_FL and bad CRC"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx 64-Byte Packets Statistic Register"]
pub mod RMON_R_P64 {
    #[doc = "Number of 64-byte receive packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
pub mod RMON_R_P65TO127 {
    #[doc = "Number of 65- to 127-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
pub mod RMON_R_P128TO255 {
    #[doc = "Number of 128- to 255-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
pub mod RMON_R_P256TO511 {
    #[doc = "Number of 256- to 511-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
pub mod RMON_R_P512TO1023 {
    #[doc = "Number of 512- to 1023-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
pub mod RMON_R_P1024TO2047 {
    #[doc = "Number of 1024- to 2047-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
pub mod RMON_R_P_GTE2048 {
    #[doc = "Number of greater-than-2048-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Octets Statistic Register"]
pub mod RMON_R_OCTETS {
    #[doc = "Number of receive octets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames not Counted Correctly Statistic Register"]
pub mod IEEE_R_DROP {
    #[doc = "Frame count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Received OK Statistic Register"]
pub mod IEEE_R_FRAME_OK {
    #[doc = "Number of frames received OK"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Received with CRC Error Statistic Register"]
pub mod IEEE_R_CRC {
    #[doc = "Number of frames received with CRC error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frames Received with Alignment Error Statistic Register"]
pub mod IEEE_R_ALIGN {
    #[doc = "Number of frames received with alignment error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive FIFO Overflow Count Statistic Register"]
pub mod IEEE_R_MACERR {
    #[doc = "Receive FIFO overflow count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flow Control Pause Frames Received Statistic Register"]
pub mod IEEE_R_FDXFC {
    #[doc = "Number of flow-control pause frames received"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Octet Count for Frames Received without Error Statistic Register"]
pub mod IEEE_R_OCTETS_OK {
    #[doc = "Number of octets for frames received without error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Adjustable Timer Control Register"]
pub mod ATCR {
    #[doc = "Enable Timer"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The timer stops at the current value."]
            pub const EN_0: u32 = 0;
            #[doc = "The timer starts incrementing."]
            pub const EN_1: u32 = 0x01;
        }
    }
    #[doc = "Enable One-Shot Offset Event"]
    pub mod OFFEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const OFFEN_0: u32 = 0;
            #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
            pub const OFFEN_1: u32 = 0x01;
        }
    }
    #[doc = "Reset Timer On Offset Event"]
    pub mod OFFRST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
            pub const OFFRST_0: u32 = 0;
            #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
            pub const OFFRST_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Periodical Event"]
    pub mod PEREN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const PEREN_0: u32 = 0;
            #[doc = "A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
            pub const PEREN_1: u32 = 0x01;
        }
    }
    #[doc = "Enables event signal output assertion on period event"]
    pub mod PINPER {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const PINPER_0: u32 = 0;
            #[doc = "Enable."]
            pub const PINPER_1: u32 = 0x01;
        }
    }
    #[doc = "Reset Timer"]
    pub mod RESTART {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capture Timer Value"]
    pub mod CAPTURE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const CAPTURE_0: u32 = 0;
            #[doc = "The current time is captured and can be read from the ATVR register."]
            pub const CAPTURE_1: u32 = 0x01;
        }
    }
    #[doc = "Enable Timer Slave Mode"]
    pub mod SLAVE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The timer is active and all configuration fields in this register are relevant."]
            pub const SLAVE_0: u32 = 0;
            #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
            pub const SLAVE_1: u32 = 0x01;
        }
    }
}
#[doc = "Timer Value Register"]
pub mod ATVR {
    #[doc = "A write sets the timer"]
    pub mod ATIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Offset Register"]
pub mod ATOFF {
    #[doc = "Offset value for one-shot event generation"]
    pub mod OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Period Register"]
pub mod ATPER {
    #[doc = "Value for generating periodic events"]
    pub mod PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Correction Register"]
pub mod ATCOR {
    #[doc = "Correction Counter Wrap-Around Value"]
    pub mod COR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time-Stamping Clock Period Register"]
pub mod ATINC {
    #[doc = "Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    pub mod INC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Correction Increment Value"]
    pub mod INC_CORR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp of Last Transmitted Frame"]
pub mod ATSTMP {
    #[doc = "Timestamp of the last frame transmitted by the core that had TxBD\\[TS\\] set"]
    pub mod TIMESTAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Global Status Register"]
pub mod TGSR {
    #[doc = "Copy Of Timer Flag For Channel 0"]
    pub mod TF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Flag for Channel 0 is clear"]
            pub const TF0_0: u32 = 0;
            #[doc = "Timer Flag for Channel 0 is set"]
            pub const TF0_1: u32 = 0x01;
        }
    }
    #[doc = "Copy Of Timer Flag For Channel 1"]
    pub mod TF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Flag for Channel 1 is clear"]
            pub const TF1_0: u32 = 0;
            #[doc = "Timer Flag for Channel 1 is set"]
            pub const TF1_1: u32 = 0x01;
        }
    }
    #[doc = "Copy Of Timer Flag For Channel 2"]
    pub mod TF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Flag for Channel 2 is clear"]
            pub const TF2_0: u32 = 0;
            #[doc = "Timer Flag for Channel 2 is set"]
            pub const TF2_1: u32 = 0x01;
        }
    }
    #[doc = "Copy Of Timer Flag For Channel 3"]
    pub mod TF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Flag for Channel 3 is clear"]
            pub const TF3_0: u32 = 0;
            #[doc = "Timer Flag for Channel 3 is set"]
            pub const TF3_1: u32 = 0x01;
        }
    }
}
#[doc = "Timer Control Status Register"]
pub mod TCSR0 {
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const ZERO: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer Mode"]
    pub mod TMODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Channel is disabled."]
            pub const TMR_DIS: u32 = 0;
            #[doc = "Timer Channel is configured for Input Capture on rising edge."]
            pub const TMR_RE: u32 = 0x01;
            #[doc = "Timer Channel is configured for Input Capture on falling edge."]
            pub const TMR_FE: u32 = 0x02;
            #[doc = "Timer Channel is configured for Input Capture on both edges."]
            pub const TMR_BE: u32 = 0x03;
            #[doc = "Timer Channel is configured for Output Compare - software only."]
            pub const TMR_OUT: u32 = 0x04;
            #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
            pub const TMR_TOGGLE: u32 = 0x05;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
            pub const TMR_CLR: u32 = 0x06;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
            pub const TMR_SET_OUT: u32 = 0x07;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
            pub const TMR_CLR_SET1: u32 = 0x09;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
            pub const TMR_CLR_SET: u32 = 0x0a;
            #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMR_OUT_CMP_LOW: u32 = 0x0e;
            #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMR_OUT_CMP_HIGH: u32 = 0x0f;
        }
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const ZERO: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer Flag"]
    pub mod TF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Capture or Output Compare has not occurred."]
            pub const ZERO: u32 = 0;
            #[doc = "Input Capture or Output Compare has occurred."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer PulseWidth Control"]
    pub mod TPWC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pulse width is one 1588-clock cycle."]
            pub const VALW1: u32 = 0;
            #[doc = "Pulse width is two 1588-clock cycles."]
            pub const VALW2: u32 = 0x01;
            #[doc = "Pulse width is three 1588-clock cycles."]
            pub const VALW3: u32 = 0x02;
            #[doc = "Pulse width is four 1588-clock cycles."]
            pub const VALW4: u32 = 0x03;
            #[doc = "Pulse width is 32 1588-clock cycles."]
            pub const VALW32: u32 = 0x1f;
        }
    }
}
#[doc = "Timer Compare Capture Register"]
pub mod TCCR0 {
    #[doc = "Timer Capture Compare"]
    pub mod TCC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Control Status Register"]
pub mod TCSR1 {
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const ZERO: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer Mode"]
    pub mod TMODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Channel is disabled."]
            pub const TMR_DIS: u32 = 0;
            #[doc = "Timer Channel is configured for Input Capture on rising edge."]
            pub const TMR_RE: u32 = 0x01;
            #[doc = "Timer Channel is configured for Input Capture on falling edge."]
            pub const TMR_FE: u32 = 0x02;
            #[doc = "Timer Channel is configured for Input Capture on both edges."]
            pub const TMR_BE: u32 = 0x03;
            #[doc = "Timer Channel is configured for Output Compare - software only."]
            pub const TMR_OUT: u32 = 0x04;
            #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
            pub const TMR_TOGGLE: u32 = 0x05;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
            pub const TMR_CLR: u32 = 0x06;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
            pub const TMR_SET_OUT: u32 = 0x07;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
            pub const TMR_CLR_SET1: u32 = 0x09;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
            pub const TMR_CLR_SET: u32 = 0x0a;
            #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMR_OUT_CMP_LOW: u32 = 0x0e;
            #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMR_OUT_CMP_HIGH: u32 = 0x0f;
        }
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const ZERO: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer Flag"]
    pub mod TF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Capture or Output Compare has not occurred."]
            pub const ZERO: u32 = 0;
            #[doc = "Input Capture or Output Compare has occurred."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer PulseWidth Control"]
    pub mod TPWC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pulse width is one 1588-clock cycle."]
            pub const VALW1: u32 = 0;
            #[doc = "Pulse width is two 1588-clock cycles."]
            pub const VALW2: u32 = 0x01;
            #[doc = "Pulse width is three 1588-clock cycles."]
            pub const VALW3: u32 = 0x02;
            #[doc = "Pulse width is four 1588-clock cycles."]
            pub const VALW4: u32 = 0x03;
            #[doc = "Pulse width is 32 1588-clock cycles."]
            pub const VALW32: u32 = 0x1f;
        }
    }
}
#[doc = "Timer Compare Capture Register"]
pub mod TCCR1 {
    #[doc = "Timer Capture Compare"]
    pub mod TCC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Control Status Register"]
pub mod TCSR2 {
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const ZERO: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer Mode"]
    pub mod TMODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Channel is disabled."]
            pub const TMR_DIS: u32 = 0;
            #[doc = "Timer Channel is configured for Input Capture on rising edge."]
            pub const TMR_RE: u32 = 0x01;
            #[doc = "Timer Channel is configured for Input Capture on falling edge."]
            pub const TMR_FE: u32 = 0x02;
            #[doc = "Timer Channel is configured for Input Capture on both edges."]
            pub const TMR_BE: u32 = 0x03;
            #[doc = "Timer Channel is configured for Output Compare - software only."]
            pub const TMR_OUT: u32 = 0x04;
            #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
            pub const TMR_TOGGLE: u32 = 0x05;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
            pub const TMR_CLR: u32 = 0x06;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
            pub const TMR_SET_OUT: u32 = 0x07;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
            pub const TMR_CLR_SET1: u32 = 0x09;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
            pub const TMR_CLR_SET: u32 = 0x0a;
            #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMR_OUT_CMP_LOW: u32 = 0x0e;
            #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMR_OUT_CMP_HIGH: u32 = 0x0f;
        }
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const ZERO: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer Flag"]
    pub mod TF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Capture or Output Compare has not occurred."]
            pub const ZERO: u32 = 0;
            #[doc = "Input Capture or Output Compare has occurred."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer PulseWidth Control"]
    pub mod TPWC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pulse width is one 1588-clock cycle."]
            pub const VALW1: u32 = 0;
            #[doc = "Pulse width is two 1588-clock cycles."]
            pub const VALW2: u32 = 0x01;
            #[doc = "Pulse width is three 1588-clock cycles."]
            pub const VALW3: u32 = 0x02;
            #[doc = "Pulse width is four 1588-clock cycles."]
            pub const VALW4: u32 = 0x03;
            #[doc = "Pulse width is 32 1588-clock cycles."]
            pub const VALW32: u32 = 0x1f;
        }
    }
}
#[doc = "Timer Compare Capture Register"]
pub mod TCCR2 {
    #[doc = "Timer Capture Compare"]
    pub mod TCC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Control Status Register"]
pub mod TCSR3 {
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const ZERO: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer Mode"]
    pub mod TMODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Channel is disabled."]
            pub const TMR_DIS: u32 = 0;
            #[doc = "Timer Channel is configured for Input Capture on rising edge."]
            pub const TMR_RE: u32 = 0x01;
            #[doc = "Timer Channel is configured for Input Capture on falling edge."]
            pub const TMR_FE: u32 = 0x02;
            #[doc = "Timer Channel is configured for Input Capture on both edges."]
            pub const TMR_BE: u32 = 0x03;
            #[doc = "Timer Channel is configured for Output Compare - software only."]
            pub const TMR_OUT: u32 = 0x04;
            #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
            pub const TMR_TOGGLE: u32 = 0x05;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
            pub const TMR_CLR: u32 = 0x06;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
            pub const TMR_SET_OUT: u32 = 0x07;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
            pub const TMR_CLR_SET1: u32 = 0x09;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
            pub const TMR_CLR_SET: u32 = 0x0a;
            #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMR_OUT_CMP_LOW: u32 = 0x0e;
            #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMR_OUT_CMP_HIGH: u32 = 0x0f;
        }
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const ZERO: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer Flag"]
    pub mod TF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input Capture or Output Compare has not occurred."]
            pub const ZERO: u32 = 0;
            #[doc = "Input Capture or Output Compare has occurred."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Timer PulseWidth Control"]
    pub mod TPWC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pulse width is one 1588-clock cycle."]
            pub const VALW1: u32 = 0;
            #[doc = "Pulse width is two 1588-clock cycles."]
            pub const VALW2: u32 = 0x01;
            #[doc = "Pulse width is three 1588-clock cycles."]
            pub const VALW3: u32 = 0x02;
            #[doc = "Pulse width is four 1588-clock cycles."]
            pub const VALW4: u32 = 0x03;
            #[doc = "Pulse width is 32 1588-clock cycles."]
            pub const VALW32: u32 = 0x1f;
        }
    }
}
#[doc = "Timer Compare Capture Register"]
pub mod TCCR3 {
    #[doc = "Timer Capture Compare"]
    pub mod TCC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
