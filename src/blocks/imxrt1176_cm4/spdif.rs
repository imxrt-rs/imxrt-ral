#[doc = "SPDIF"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SPDIF Configuration Register"]
    pub SCR: crate::RWRegister<u32>,
    #[doc = "CDText Control Register"]
    pub SRCD: crate::RWRegister<u32>,
    #[doc = "PhaseConfig Register"]
    pub SRPC: crate::RWRegister<u32>,
    #[doc = "InterruptEn Register"]
    pub SIE: crate::RWRegister<u32>,
    #[doc = "InterruptStat Register"]
    pub SIS: crate::RORegister<u32>,
    #[doc = "SPDIFRxLeft Register"]
    pub SRL: crate::RORegister<u32>,
    #[doc = "SPDIFRxRight Register"]
    pub SRR: crate::RORegister<u32>,
    #[doc = "SPDIFRxCChannel_h Register"]
    pub SRCSH: crate::RORegister<u32>,
    #[doc = "SPDIFRxCChannel_l Register"]
    pub SRCSL: crate::RORegister<u32>,
    #[doc = "UchannelRx Register"]
    pub SRU: crate::RORegister<u32>,
    #[doc = "QchannelRx Register"]
    pub SRQ: crate::RORegister<u32>,
    #[doc = "SPDIFTxLeft Register"]
    pub STL: crate::WORegister<u32>,
    #[doc = "SPDIFTxRight Register"]
    pub STR: crate::WORegister<u32>,
    #[doc = "SPDIFTxCChannelCons_h Register"]
    pub STCSCH: crate::RWRegister<u32>,
    #[doc = "SPDIFTxCChannelCons_l Register"]
    pub STCSCL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "FreqMeas Register"]
    pub SRFM: crate::RORegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "SPDIFTxClk Register"]
    pub STC: crate::RWRegister<u32>,
}
#[doc = "SPDIF Configuration Register"]
pub mod SCR {
    #[doc = "USrc_Sel"]
    pub mod USRC_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No embedded U channel"]
            pub const USRC_SEL_0: u32 = 0;
            #[doc = "U channel from SPDIF receive block (CD mode)"]
            pub const USRC_SEL_1: u32 = 0x01;
            #[doc = "U channel from on chip transmitter"]
            pub const USRC_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "TxSel"]
    pub mod TXSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Off and output 0"]
            pub const TXSEL_0: u32 = 0;
            #[doc = "Feed-through SPDIFIN"]
            pub const TXSEL_1: u32 = 0x01;
            #[doc = "Tx Normal operation"]
            pub const TXSEL_5: u32 = 0x05;
        }
    }
    #[doc = "ValCtrl"]
    pub mod VALCTRL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Outgoing Validity always set"]
            pub const VALCTRL_0: u32 = 0;
            #[doc = "Outgoing Validity always clear"]
            pub const VALCTRL_1: u32 = 0x01;
        }
    }
    #[doc = "InputSrcSel"]
    pub mod INPUTSRCSEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF_IN"]
            pub const SPDIF_IN: u32 = 0;
        }
    }
    #[doc = "DMA_TX_En"]
    pub mod DMA_TX_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA_Rx_En"]
    pub mod DMA_RX_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxFIFO_Ctrl"]
    pub mod TXFIFO_CTRL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Send out digital zero on SPDIF Tx"]
            pub const TXFIFO_CTRL_0: u32 = 0;
            #[doc = "Tx Normal operation"]
            pub const TXFIFO_CTRL_1: u32 = 0x01;
            #[doc = "Reset to 1 sample remaining"]
            pub const TXFIFO_CTRL_2: u32 = 0x02;
        }
    }
    #[doc = "soft_reset"]
    pub mod SOFT_RESET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LOW_POWER"]
    pub mod LOW_POWER {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxFIFOEmpty_Sel"]
    pub mod TXFIFOEMPTY_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs"]
            pub const TXFIFOEMPTY_SEL_0: u32 = 0;
            #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
            pub const TXFIFOEMPTY_SEL_1: u32 = 0x01;
            #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
            pub const TXFIFOEMPTY_SEL_2: u32 = 0x02;
            #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
            pub const TXFIFOEMPTY_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "TxAutoSync"]
    pub mod TXAUTOSYNC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tx FIFO auto sync off"]
            pub const TXAUTOSYNC_0: u32 = 0;
            #[doc = "Tx FIFO auto sync on"]
            pub const TXAUTOSYNC_1: u32 = 0x01;
        }
    }
    #[doc = "RxAutoSync"]
    pub mod RXAUTOSYNC {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx FIFO auto sync off"]
            pub const RXAUTOSYNC_0: u32 = 0;
            #[doc = "RxFIFO auto sync on"]
            pub const RXAUTOSYNC_1: u32 = 0x01;
        }
    }
    #[doc = "RxFIFOFull_Sel"]
    pub mod RXFIFOFULL_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
            pub const RXFIFOFULL_SEL_0: u32 = 0;
            #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
            pub const RXFIFOFULL_SEL_1: u32 = 0x01;
            #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
            pub const RXFIFOFULL_SEL_2: u32 = 0x02;
            #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
            pub const RXFIFOFULL_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "RxFIFO_Rst"]
    pub mod RXFIFO_RST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const RXFIFO_RST_0: u32 = 0;
            #[doc = "Reset register to 1 sample remaining"]
            pub const RXFIFO_RST_1: u32 = 0x01;
        }
    }
    #[doc = "RxFIFO_Off_On"]
    pub mod RXFIFO_OFF_ON {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF Rx FIFO is on"]
            pub const RXFIFO_OFF_ON_0: u32 = 0;
            #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface"]
            pub const RXFIFO_OFF_ON_1: u32 = 0x01;
        }
    }
    #[doc = "RxFIFO_Ctrl"]
    pub mod RXFIFO_CTRL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const RXFIFO_CTRL_0: u32 = 0;
            #[doc = "Always read zero from Rx data register"]
            pub const RXFIFO_CTRL_1: u32 = 0x01;
        }
    }
}
#[doc = "CDText Control Register"]
pub mod SRCD {
    #[doc = "no description available"]
    pub mod USYNCMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Non-CD data"]
            pub const USYNCMODE_0: u32 = 0;
            #[doc = "CD user channel subcode"]
            pub const USYNCMODE_1: u32 = 0x01;
        }
    }
}
#[doc = "PhaseConfig Register"]
pub mod SRPC {
    #[doc = "Gain selection:"]
    pub mod GAINSEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "24*(2**10)"]
            pub const GAINSEL_0: u32 = 0;
            #[doc = "16*(2**10)"]
            pub const GAINSEL_1: u32 = 0x01;
            #[doc = "12*(2**10)"]
            pub const GAINSEL_2: u32 = 0x02;
            #[doc = "8*(2**10)"]
            pub const GAINSEL_3: u32 = 0x03;
            #[doc = "6*(2**10)"]
            pub const GAINSEL_4: u32 = 0x04;
            #[doc = "4*(2**10)"]
            pub const GAINSEL_5: u32 = 0x05;
            #[doc = "3*(2**10)"]
            pub const GAINSEL_6: u32 = 0x06;
        }
    }
    #[doc = "LOCK bit to show that the internal DPLL is locked, read only"]
    pub mod LOCK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock source selection, all other settings not shown are reserved:"]
    pub mod CLKSRC_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
            pub const CLKSRC_SEL_0: u32 = 0;
            #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
            pub const CLKSRC_SEL_1: u32 = 0x01;
            #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
            pub const CLKSRC_SEL_3: u32 = 0x03;
            #[doc = "REF_CLK_32K (XTALOSC)"]
            pub const CLKSRC_SEL_5: u32 = 0x05;
            #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
            pub const CLKSRC_SEL_6: u32 = 0x06;
            #[doc = "SPDIF_EXT_CLK"]
            pub const CLKSRC_SEL_8: u32 = 0x08;
        }
    }
}
#[doc = "InterruptEn Register"]
pub mod SIE {
    #[doc = "SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    pub mod RXFIFOFUL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    pub mod TXEM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receiver loss of lock"]
    pub mod LOCKLOSS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx FIFO resync"]
    pub mod RXFIFORESYN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx FIFO underrun/overrun"]
    pub mod RXFIFOUNOV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "U/Q Channel framing error"]
    pub mod UQERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "U/Q Channel sync found"]
    pub mod UQSYNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Q Channel receive register overrun"]
    pub mod QRXOV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Q Channel receive register full, can't be cleared with reg"]
    pub mod QRXFUL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "U Channel receive register overrun"]
    pub mod URXOV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "U Channel receive register full, can't be cleared with reg"]
    pub mod URXFUL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receiver found parity bit error"]
    pub mod BITERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receiver found illegal symbol"]
    pub mod SYMERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF validity flag no good"]
    pub mod VALNOGOOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receive change in value of control channel"]
    pub mod CNEW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF Tx FIFO resync"]
    pub mod TXRESYN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF Tx FIFO under/overrun"]
    pub mod TXUNOV {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receiver's DPLL is locked"]
    pub mod LOCK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "InterruptStat Register"]
pub mod SIS {
    #[doc = "SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    pub mod RXFIFOFUL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    pub mod TXEM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receiver loss of lock"]
    pub mod LOCKLOSS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx FIFO resync"]
    pub mod RXFIFORESYN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx FIFO underrun/overrun"]
    pub mod RXFIFOUNOV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "U/Q Channel framing error"]
    pub mod UQERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "U/Q Channel sync found"]
    pub mod UQSYNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Q Channel receive register overrun"]
    pub mod QRXOV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Q Channel receive register full, can't be cleared with reg"]
    pub mod QRXFUL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "U Channel receive register overrun"]
    pub mod URXOV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "U Channel receive register full, can't be cleared with reg"]
    pub mod URXFUL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receiver found parity bit error"]
    pub mod BITERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receiver found illegal symbol"]
    pub mod SYMERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF validity flag no good"]
    pub mod VALNOGOOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receive change in value of control channel"]
    pub mod CNEW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF Tx FIFO resync"]
    pub mod TXRESYN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF Tx FIFO under/overrun"]
    pub mod TXUNOV {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPDIF receiver's DPLL is locked"]
    pub mod LOCK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFRxLeft Register"]
pub mod SRL {
    #[doc = "Processor receive SPDIF data left"]
    pub mod RXDATALEFT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFRxRight Register"]
pub mod SRR {
    #[doc = "Processor receive SPDIF data right"]
    pub mod RXDATARIGHT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFRxCChannel_h Register"]
pub mod SRCSH {
    #[doc = "SPDIF receive C channel register, contains first 24 bits of C channel without interpretation"]
    pub mod RXCCHANNEL_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFRxCChannel_l Register"]
pub mod SRCSL {
    #[doc = "SPDIF receive C channel register, contains next 24 bits of C channel without interpretation"]
    pub mod RXCCHANNEL_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UchannelRx Register"]
pub mod SRU {
    #[doc = "SPDIF receive U channel register, contains next 3 U channel bytes"]
    pub mod RXUCHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "QchannelRx Register"]
pub mod SRQ {
    #[doc = "SPDIF receive Q channel register, contains next 3 Q channel bytes"]
    pub mod RXQCHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxLeft Register"]
pub mod STL {
    #[doc = "SPDIF transmit left channel data. It is write-only, and always returns zeros when read"]
    pub mod TXDATALEFT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxRight Register"]
pub mod STR {
    #[doc = "SPDIF transmit right channel data. It is write-only, and always returns zeros when read"]
    pub mod TXDATARIGHT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxCChannelCons_h Register"]
pub mod STCSCH {
    #[doc = "SPDIF transmit Cons"]
    pub mod TXCCHANNELCONS_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxCChannelCons_l Register"]
pub mod STCSCL {
    #[doc = "SPDIF transmit Cons"]
    pub mod TXCCHANNELCONS_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FreqMeas Register"]
pub mod SRFM {
    #[doc = "Frequency measurement data"]
    pub mod FREQMEAS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxClk Register"]
pub mod STC {
    #[doc = "Divider factor (1-128)"]
    pub mod TXCLK_DF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divider factor is 1"]
            pub const TXCLK_DF_0: u32 = 0;
            #[doc = "divider factor is 2"]
            pub const TXCLK_DF_1: u32 = 0x01;
            #[doc = "divider factor is 128"]
            pub const TXCLK_DF_127: u32 = 0x7f;
        }
    }
    #[doc = "Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    pub mod TX_ALL_CLK_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable transfer clock."]
            pub const TX_ALL_CLK_EN_0: u32 = 0;
            #[doc = "enable transfer clock."]
            pub const TX_ALL_CLK_EN_1: u32 = 0x01;
        }
    }
    #[doc = "no description available"]
    pub mod TXCLK_SOURCE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "REF_CLK_32K input (XTALOSC 32 kHz clock)"]
            pub const TXCLK_SOURCE_0: u32 = 0;
            #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
            pub const TXCLK_SOURCE_1: u32 = 0x01;
            #[doc = "SPDIF_EXT_CLK, from pads"]
            pub const TXCLK_SOURCE_3: u32 = 0x03;
            #[doc = "ipg_clk input (frequency divided)"]
            pub const TXCLK_SOURCE_5: u32 = 0x05;
        }
    }
    #[doc = "system clock divider factor, 2~512."]
    pub mod SYSCLK_DF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no clock signal"]
            pub const SYSCLK_DF_0: u32 = 0;
            #[doc = "divider factor is 2"]
            pub const SYSCLK_DF_1: u32 = 0x01;
            #[doc = "divider factor is 512"]
            pub const SYSCLK_DF_511: u32 = 0x01ff;
        }
    }
}
