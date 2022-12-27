#[doc = "EMVSIM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VER_ID: crate::RORegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "Clock Configuration Register"]
    pub CLKCFG: crate::RWRegister<u32>,
    #[doc = "Baud Rate Divisor Register"]
    pub DIVISOR: crate::RWRegister<u32>,
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Interrupt Mask Register"]
    pub INT_MASK: crate::RWRegister<u32>,
    #[doc = "Receiver Threshold Register"]
    pub RX_THD: crate::RWRegister<u32>,
    #[doc = "Transmitter Threshold Register"]
    pub TX_THD: crate::RWRegister<u32>,
    #[doc = "Receive Status Register"]
    pub RX_STATUS: crate::RWRegister<u32>,
    #[doc = "Transmitter Status Register"]
    pub TX_STATUS: crate::RWRegister<u32>,
    #[doc = "Port Control and Status Register"]
    pub PCSR: crate::RWRegister<u32>,
    #[doc = "Receive Data Read Buffer"]
    pub RX_BUF: crate::RORegister<u32>,
    #[doc = "Transmit Data Buffer"]
    pub TX_BUF: crate::RWRegister<u32>,
    #[doc = "Transmitter Guard ETU Value Register"]
    pub TX_GETU: crate::RWRegister<u32>,
    #[doc = "Character Wait Time Value Register"]
    pub CWT_VAL: crate::RWRegister<u32>,
    #[doc = "Block Wait Time Value Register"]
    pub BWT_VAL: crate::RWRegister<u32>,
    #[doc = "Block Guard Time Value Register"]
    pub BGT_VAL: crate::RWRegister<u32>,
    #[doc = "General Purpose Counter 0 Timeout Value Register"]
    pub GPCNT0_VAL: crate::RWRegister<u32>,
    #[doc = "General Purpose Counter 1 Timeout Value"]
    pub GPCNT1_VAL: crate::RWRegister<u32>,
}
#[doc = "Version ID Register"]
pub mod VER_ID {
    #[doc = "Version ID of the module"]
    pub mod VER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter Register"]
pub mod PARAM {
    #[doc = "Receive FIFO Depth"]
    pub mod RX_FIFO_DEPTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO Depth"]
    pub mod TX_FIFO_DEPTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Configuration Register"]
pub mod CLKCFG {
    #[doc = "Clock Prescaler Value"]
    pub mod CLK_PRSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Counter 1 Clock Select"]
    pub mod GPCNT1_CLK_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled / Reset"]
            pub const DISABLED: u32 = 0;
            #[doc = "Card Clock"]
            pub const CARDCLK: u32 = 0x01;
            #[doc = "Receive Clock"]
            pub const RXCLK: u32 = 0x02;
            #[doc = "ETU Clock (transmit clock)"]
            pub const TXCLK: u32 = 0x03;
        }
    }
    #[doc = "General Purpose Counter 0 Clock Select"]
    pub mod GPCNT0_CLK_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled / Reset"]
            pub const DISABLED: u32 = 0;
            #[doc = "Card Clock"]
            pub const CARDCLK: u32 = 0x01;
            #[doc = "Receive Clock"]
            pub const RXCLK: u32 = 0x02;
            #[doc = "ETU Clock (transmit clock)"]
            pub const TXCLK: u32 = 0x03;
        }
    }
}
#[doc = "Baud Rate Divisor Register"]
pub mod DIVISOR {
    #[doc = "Divisor (F/D) Value"]
    pub mod DIVISOR_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "Inverse Convention"]
    pub mod IC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Direction convention transfers enabled"]
            pub const DIR_CONVENTION: u32 = 0;
            #[doc = "Inverse convention transfers enabled"]
            pub const INV_CONVENTION: u32 = 0x01;
        }
    }
    #[doc = "Initial Character Mode"]
    pub mod ICM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initial Character Mode disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Initial Character Mode enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Auto NACK Enable"]
    pub mod ANACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NACK generation on errors disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "NACK generation on errors enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Overrun NACK Enable"]
    pub mod ONACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NACK generation on overrun is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "NACK generation on overrun is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Flush Receiver Bit"]
    pub mod FLSH_RX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EMVSIM Receiver normal operation"]
            pub const NORMALOP: u32 = 0;
            #[doc = "EMVSIM Receiver held in Reset"]
            pub const RESETHOLD: u32 = 0x01;
        }
    }
    #[doc = "Flush Transmitter Bit"]
    pub mod FLSH_TX {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EMVSIM Transmitter normal operation"]
            pub const NORMALOP: u32 = 0;
            #[doc = "EMVSIM Transmitter held in Reset"]
            pub const RESETHOLD: u32 = 0x01;
        }
    }
    #[doc = "Software Reset Bit"]
    pub mod SW_RST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EMVSIM Normal operation"]
            pub const NORMALOP: u32 = 0;
            #[doc = "EMVSIM held in Reset"]
            pub const RESETHOLD: u32 = 0x01;
        }
    }
    #[doc = "Kill all internal clocks"]
    pub mod KILL_CLOCKS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EMVSIM input clock enabled"]
            pub const INCLK_ENABLED: u32 = 0;
            #[doc = "EMVSIM input clock is disabled"]
            pub const INCLK_DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Doze Enable"]
    pub mod DOZE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DOZE instruction gates all internal EMVSIM clocks as well as the Smart Card clock when the transmit FIFO is empty"]
            pub const DOZE_GATE: u32 = 0;
            #[doc = "DOZE instruction has no effect on EMVSIM module"]
            pub const DOZE_NOGATE: u32 = 0x01;
        }
    }
    #[doc = "STOP Enable"]
    pub mod STOP_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STOP instruction shuts down all EMVSIM clocks"]
            pub const STOP_ALL_CLKS: u32 = 0;
            #[doc = "STOP instruction shuts down all clocks except for the Smart Card Clock (SCK) (clock provided to Smart Card)"]
            pub const ONLY_SCK_ON: u32 = 0x01;
        }
    }
    #[doc = "Receiver Enable"]
    pub mod RCV_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EMVSIM Receiver disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "EMVSIM Receiver enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Transmitter Enable"]
    pub mod XMT_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EMVSIM Transmitter disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "EMVSIM Transmitter enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Receiver 11 ETU Mode Enable"]
    pub mod RCVR_11 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver configured for 12 ETU operation mode"]
            pub const RCVR_12: u32 = 0;
            #[doc = "Receiver configured for 11 ETU operation mode"]
            pub const RCVR_11: u32 = 0x01;
        }
    }
    #[doc = "Receive DMA Enable"]
    pub mod RX_DMA_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No DMA Read Request asserted for Receiver"]
            pub const NO_DMAREAD_REQ: u32 = 0;
            #[doc = "DMA Read Request asserted for Receiver"]
            pub const DMAREAD_REQ: u32 = 0x01;
        }
    }
    #[doc = "Transmit DMA Enable"]
    pub mod TX_DMA_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No DMA Write Request asserted for Transmitter"]
            pub const NO_DMAWRITE_REQ: u32 = 0;
            #[doc = "DMA Write Request asserted for Transmitter"]
            pub const DMAWRITE_REQ: u32 = 0x01;
        }
    }
    #[doc = "Invert bits in the CRC Output Value"]
    pub mod INV_CRC_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bits in CRC Output value are not inverted."]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Bits in CRC Output value are inverted."]
            pub const INVERT: u32 = 0x01;
        }
    }
    #[doc = "CRC Output Value Bit Reversal or Flip"]
    pub mod CRC_OUT_FLIP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bits within the CRC output bytes are not reversed i.e. 15:0 remains 15:0"]
            pub const NOT_REVERSED: u32 = 0;
            #[doc = "Bits within the CRC output bytes are reversed i.e. 15:0 becomes {8:15,0:7}"]
            pub const REVERSED: u32 = 0x01;
        }
    }
    #[doc = "CRC Input Byte's Bit Reversal or Flip Control"]
    pub mod CRC_IN_FLIP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bits in the input byte are not reversed (i.e. 7:0 remain 7:0) before the CRC calculation"]
            pub const NOT_REVERSED: u32 = 0;
            #[doc = "Bits in the input byte are reversed (i.e. 7:0 becomes 0:7) before CRC calculation"]
            pub const REVERSED: u32 = 0x01;
        }
    }
    #[doc = "Character Wait Time Counter Enable"]
    pub mod CWT_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Character Wait time Counter is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Character Wait time counter is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "LRC Enable"]
    pub mod LRC_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8-bit Linear Redundancy Checking disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "8-bit Linear Redundancy Checking enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "CRC Enable"]
    pub mod CRC_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16-bit Cyclic Redundancy Checking disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "16-bit Cyclic Redundancy Checking enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Transmit CRC or LRC Enable"]
    pub mod XMT_CRC_LRC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No CRC or LRC value is transmitted"]
            pub const NO_CRC_LRC_TX: u32 = 0;
            #[doc = "Transmit LRC or CRC info when FIFO empties (whichever is enabled)"]
            pub const CRC_LRC_TX: u32 = 0x01;
        }
    }
    #[doc = "Block Wait Time Counter Enable"]
    pub mod BWT_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable BWT, BGT Counters"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable BWT, BGT Counters"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Mask Register"]
pub mod INT_MASK {
    #[doc = "Receive Data Threshold Interrupt Mask"]
    pub mod RDT_IM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RDTF interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "RDTF interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Transmit Complete Interrupt Mask"]
    pub mod TC_IM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TCF interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "TCF interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Receive FIFO Overflow Interrupt Mask"]
    pub mod RFO_IM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RFO interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "RFO interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Complete Interrupt Mask"]
    pub mod ETC_IM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ETC interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "ETC interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Empty Interrupt Mask"]
    pub mod TFE_IM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TFE interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "TFE interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Transmit NACK Threshold Interrupt Mask"]
    pub mod TNACK_IM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TNTE interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "TNTE interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Full Interrupt Mask"]
    pub mod TFF_IM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TFF interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "TFF interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Transmit Data Threshold Interrupt Mask"]
    pub mod TDT_IM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TDTF interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "TDTF interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Timer 0 Timeout Interrupt Mask"]
    pub mod GPCNT0_IM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPCNT0_TO interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "GPCNT0_TO interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Character Wait Time Error Interrupt Mask"]
    pub mod CWT_ERR_IM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CWT_ERR interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "CWT_ERR interrupt masked"]
            pub const INT_DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Receiver NACK Threshold Interrupt Mask"]
    pub mod RNACK_IM {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RTE interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "RTE interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Block Wait Time Error Interrupt Mask"]
    pub mod BWT_ERR_IM {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "BWT_ERR interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "BWT_ERR interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Block Guard Time Error Interrupt"]
    pub mod BGT_ERR_IM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "BGT_ERR interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "BGT_ERR interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Counter 1 Timeout Interrupt Mask"]
    pub mod GPCNT1_IM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPCNT1_TO interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "GPCNT1_TO interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Interrupt Mask"]
    pub mod RX_DATA_IM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RX_DATA interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "RX_DATA interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Parity Error Interrupt Mask"]
    pub mod PEF_IM {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PEF interrupt enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "PEF interrupt masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
}
#[doc = "Receiver Threshold Register"]
pub mod RX_THD {
    #[doc = "Receiver Data Threshold Value"]
    pub mod RDT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receiver NACK Threshold Value"]
    pub mod RNCK_THD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Threshold Register"]
pub mod TX_THD {
    #[doc = "Transmitter Data Threshold Value"]
    pub mod TDT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter NACK Threshold Value"]
    pub mod TNCK_THD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Status Register"]
pub mod RX_STATUS {
    #[doc = "Receive FIFO Overflow Flag"]
    pub mod RFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overrun error has occurred"]
            pub const NO_OVERRUN: u32 = 0;
            #[doc = "A byte was received when the received FIFO was already full"]
            pub const OVERFLOW: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Interrupt Flag"]
    pub mod RX_DATA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No new byte is received"]
            pub const NO_BYTE_RX: u32 = 0;
            #[doc = "New byte is received ans stored in Receive FIFO"]
            pub const BYTE_RX: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Threshold Interrupt Flag"]
    pub mod RDTF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of unread bytes in receive FIFO less than the value set by RDT"]
            pub const LESSTHAN_RXTHRESH: u32 = 0;
            #[doc = "Number of unread bytes in receive FIFO greater or than equal to value set by RDT."]
            pub const GREATER_EQ_RXTHRESH: u32 = 0x01;
        }
    }
    #[doc = "LRC Check OK Flag"]
    pub mod LRC_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Current LRC value does not match remainder."]
            pub const LRC_NOTOK: u32 = 0;
            #[doc = "Current calculated LRC value matches the expected result (i.e. zero)."]
            pub const LRC_OK: u32 = 0x01;
        }
    }
    #[doc = "CRC Check OK Flag"]
    pub mod CRC_OK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Current CRC value does not match remainder."]
            pub const CRC_NOTOK: u32 = 0;
            #[doc = "Current calculated CRC value matches the expected result."]
            pub const CRC_OK: u32 = 0x01;
        }
    }
    #[doc = "Character Wait Time Error Flag"]
    pub mod CWT_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No CWT violation has occurred"]
            pub const NO_CWT_ERR: u32 = 0;
            #[doc = "Time between two consecutive characters has exceeded the value in CWT_VAL."]
            pub const CWT_ERR: u32 = 0x01;
        }
    }
    #[doc = "Received NACK Threshold Error Flag"]
    pub mod RTE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of NACKs generated by the receiver is less than the value programmed in RNCK_THD"]
            pub const LESSTHAN_NACKTHRESH: u32 = 0;
            #[doc = "Number of NACKs generated by the receiver is equal to the value programmed in RNCK_THD"]
            pub const GREATER_EQ_NACKTHRESH: u32 = 0x01;
        }
    }
    #[doc = "Block Wait Time Error Flag"]
    pub mod BWT_ERR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block wait time not exceeded"]
            pub const BWT_ERR_NO: u32 = 0;
            #[doc = "Block wait time was exceeded"]
            pub const BWT_ERR_YES: u32 = 0x01;
        }
    }
    #[doc = "Block Guard Time Error Flag"]
    pub mod BGT_ERR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block guard time was sufficient"]
            pub const BGT_ERR_SUFFICIENT: u32 = 0;
            #[doc = "Block guard time was too small"]
            pub const BGT_ERR_TOOSMALL: u32 = 0x01;
        }
    }
    #[doc = "Parity Error Flag"]
    pub mod PEF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No parity error detected"]
            pub const NO_PARITY_DETECT: u32 = 0;
            #[doc = "Parity error detected"]
            pub const PARITY_DETECT: u32 = 0x01;
        }
    }
    #[doc = "Frame Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No frame error detected"]
            pub const NO_FEF_DETECT: u32 = 0;
            #[doc = "Frame error detected"]
            pub const FEF_DETECT: u32 = 0x01;
        }
    }
    #[doc = "Receive FIFO Write Pointer Value"]
    pub mod RX_WPTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Byte Count"]
    pub mod RX_CNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO is emtpy"]
            pub const FIFO_EMPTY: u32 = 0;
        }
    }
}
#[doc = "Transmitter Status Register"]
pub mod TX_STATUS {
    #[doc = "Transmit NACK Threshold Error Flag"]
    pub mod TNTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit NACK threshold has not been reached"]
            pub const LESSTHAN_NACKTHRESH: u32 = 0;
            #[doc = "Transmit NACK threshold reached; transmitter frozen"]
            pub const GREATER_EQ_NACKTHRESH: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Empty Flag"]
    pub mod TFE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit FIFO is not empty"]
            pub const FIFO_EMPTY: u32 = 0;
            #[doc = "Transmit FIFO is empty"]
            pub const FIFO_NOTEMPTY: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Complete Flag"]
    pub mod ETCF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit pending or in progress"]
            pub const ETX_PENDING: u32 = 0;
            #[doc = "Transmit complete"]
            pub const ETX_COMPLETE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Complete Flag"]
    pub mod TCF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit pending or in progress"]
            pub const TX_PENDING: u32 = 0;
            #[doc = "Transmit complete"]
            pub const TX_COMPLETE: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Full Flag"]
    pub mod TFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit FIFO Full condition has not occurred"]
            pub const TX_FIFO_NOTFULL: u32 = 0;
            #[doc = "A Transmit FIFO Full condition has occurred"]
            pub const TX_FIFO_FULL: u32 = 0x01;
        }
    }
    #[doc = "Transmit Data Threshold Flag"]
    pub mod TDTF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of bytes in FIFO is greater than TDT, or bit has been cleared"]
            pub const LESSTHAN_TXTHRESH: u32 = 0;
            #[doc = "Number of bytes in FIFO is less than or equal to TDT"]
            pub const GREATER_EQ_TXTHRESH: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Counter 0 Timeout Flag"]
    pub mod GPCNT0_TO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPCNT0 time not reached, or bit has been cleared."]
            pub const GPCNT0_TO_NOTREACHED: u32 = 0;
            #[doc = "General Purpose counter has reached the GPCNT0 value"]
            pub const GPCNT0_TO_REACHED: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Counter 1 Timeout Flag"]
    pub mod GPCNT1_TO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPCNT1 time not reached, or bit has been cleared."]
            pub const GPCNT1_TO_NOTREACHED: u32 = 0;
            #[doc = "General Purpose counter has reached the GPCNT1 value"]
            pub const GPCNT1_TO_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Read Pointer"]
    pub mod TX_RPTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO Byte Count"]
    pub mod TX_CNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO is emtpy"]
            pub const FIFO_EMPTY: u32 = 0;
        }
    }
}
#[doc = "Port Control and Status Register"]
pub mod PCSR {
    #[doc = "Auto Power Down Enable"]
    pub mod SAPD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auto power down disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto power down enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Vcc Enable for Smart Card"]
    pub mod SVCC_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Smart Card Voltage disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Smart Card Voltage enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "VCC Enable Polarity Control"]
    pub mod VCCENP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SVCC_EN is active high. Polarity of SVCC_EN is unchanged."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "SVCC_EN is active low. Polarity of SVCC_EN is inverted."]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Reset to Smart Card"]
    pub mod SRST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Smart Card Reset is asserted"]
            pub const ASSERTED: u32 = 0;
            #[doc = "Smart Card Reset is de-asserted"]
            pub const DE_ASSERTED: u32 = 0x01;
        }
    }
    #[doc = "Clock Enable for Smart Card"]
    pub mod SCEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Smart Card Clock Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Smart Card Clock Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Smart Card Clock Stop Polarity"]
    pub mod SCSP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is logic 0 when stopped by SCEN"]
            pub const SCSP_LOGIC0: u32 = 0;
            #[doc = "Clock is logic 1 when stopped by SCEN"]
            pub const SCSP_LOGIC1: u32 = 0x01;
        }
    }
    #[doc = "Auto Power Down Control"]
    pub mod SPD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Start Auto Powerdown or Power Down is in progress"]
            pub const POWERDOWN: u32 = 0x01;
        }
    }
    #[doc = "Smart Card Presence Detect Interrupt Mask"]
    pub mod SPDIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIM presence detect interrupt is enabled"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "SIM presence detect interrupt is masked"]
            pub const INT_MASKED: u32 = 0x01;
        }
    }
    #[doc = "Smart Card Presence Detect Interrupt Flag"]
    pub mod SPDIF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No insertion or removal of Smart Card detected on Port"]
            pub const NO_INSERT_REMOVE_DETECT: u32 = 0;
            #[doc = "Insertion or removal of Smart Card detected on Port"]
            pub const INSERT_REMOVE_DETECT: u32 = 0x01;
        }
    }
    #[doc = "Smart Card Presence Detect Pin Status"]
    pub mod SPDP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIM Presence Detect pin is logic low"]
            pub const LOGIC_LOW: u32 = 0;
            #[doc = "SIM Presence Detectpin is logic high"]
            pub const LOGIC_HIGH: u32 = 0x01;
        }
    }
    #[doc = "SIM Presence Detect Edge Select"]
    pub mod SPDES {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Falling edge on the pin"]
            pub const FALLING_EDGE: u32 = 0;
            #[doc = "Rising edge on the pin"]
            pub const RISING_EDGE: u32 = 0x01;
        }
    }
}
#[doc = "Receive Data Read Buffer"]
pub mod RX_BUF {
    #[doc = "Receive Data Byte Read"]
    pub mod RX_BYTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Data Buffer"]
pub mod TX_BUF {
    #[doc = "Transmit Data Byte"]
    pub mod TX_BYTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Guard ETU Value Register"]
pub mod TX_GETU {
    #[doc = "Transmitter Guard Time Value in ETU"]
    pub mod GETU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Character Wait Time Value Register"]
pub mod CWT_VAL {
    #[doc = "Character Wait Time Value"]
    pub mod CWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Block Wait Time Value Register"]
pub mod BWT_VAL {
    #[doc = "Block Wait Time Value"]
    pub mod BWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Block Guard Time Value Register"]
pub mod BGT_VAL {
    #[doc = "Block Guard Time Value"]
    pub mod BGT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Counter 0 Timeout Value Register"]
pub mod GPCNT0_VAL {
    #[doc = "General Purpose Counter 0 Timeout Value"]
    pub mod GPCNT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Counter 1 Timeout Value"]
pub mod GPCNT1_VAL {
    #[doc = "General Purpose Counter 1 Timeout Value"]
    pub mod GPCNT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
