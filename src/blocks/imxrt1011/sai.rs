#[doc = "I2S"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "SAI Transmit Control Register"]
    pub TCSR: crate::RWRegister<u32>,
    #[doc = "SAI Transmit Configuration 1 Register"]
    pub TCR1: crate::RWRegister<u32>,
    #[doc = "SAI Transmit Configuration 2 Register"]
    pub TCR2: crate::RWRegister<u32>,
    #[doc = "SAI Transmit Configuration 3 Register"]
    pub TCR3: crate::RWRegister<u32>,
    #[doc = "SAI Transmit Configuration 4 Register"]
    pub TCR4: crate::RWRegister<u32>,
    #[doc = "SAI Transmit Configuration 5 Register"]
    pub TCR5: crate::RWRegister<u32>,
    #[doc = "SAI Transmit Data Register"]
    pub TDR: [crate::RWRegister<u32>; 2usize],
    _reserved0: [u8; 0x18],
    #[doc = "SAI Transmit FIFO Register"]
    pub TFR: [crate::RORegister<u32>; 2usize],
    _reserved1: [u8; 0x18],
    #[doc = "SAI Transmit Mask Register"]
    pub TMR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x24],
    #[doc = "SAI Receive Control Register"]
    pub RCSR: crate::RWRegister<u32>,
    #[doc = "SAI Receive Configuration 1 Register"]
    pub RCR1: crate::RWRegister<u32>,
    #[doc = "SAI Receive Configuration 2 Register"]
    pub RCR2: crate::RWRegister<u32>,
    #[doc = "SAI Receive Configuration 3 Register"]
    pub RCR3: crate::RWRegister<u32>,
    #[doc = "SAI Receive Configuration 4 Register"]
    pub RCR4: crate::RWRegister<u32>,
    #[doc = "SAI Receive Configuration 5 Register"]
    pub RCR5: crate::RWRegister<u32>,
    #[doc = "SAI Receive Data Register"]
    pub RDR: [crate::RORegister<u32>; 2usize],
    _reserved3: [u8; 0x18],
    #[doc = "SAI Receive FIFO Register"]
    pub RFR: [crate::RORegister<u32>; 2usize],
    _reserved4: [u8; 0x18],
    #[doc = "SAI Receive Mask Register"]
    pub RMR: crate::RWRegister<u32>,
}
#[doc = "Version ID Register"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard feature set."]
            pub const FEATURE_0: u32 = 0;
        }
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter Register"]
pub mod PARAM {
    #[doc = "Number of Datalines"]
    pub mod DATALINE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Size"]
    pub mod FIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Size"]
    pub mod FRAME {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Transmit Control Register"]
pub mod TCSR {
    #[doc = "FIFO Request DMA Enable"]
    pub mod FRDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the DMA request."]
            pub const FRDE_0: u32 = 0;
            #[doc = "Enables the DMA request."]
            pub const FRDE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Warning DMA Enable"]
    pub mod FWDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the DMA request."]
            pub const FWDE_0: u32 = 0;
            #[doc = "Enables the DMA request."]
            pub const FWDE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Request Interrupt Enable"]
    pub mod FRIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const FRIE_0: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const FRIE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    pub mod FWIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const FWIE_0: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const FWIE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const FEIE_0: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const FEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Sync Error Interrupt Enable"]
    pub mod SEIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt."]
            pub const SEIE_0: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const SEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Word Start Interrupt Enable"]
    pub mod WSIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt."]
            pub const WSIE_0: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const WSIE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Request Flag"]
    pub mod FRF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit FIFO watermark has not been reached."]
            pub const FRF_0: u32 = 0;
            #[doc = "Transmit FIFO watermark has been reached."]
            pub const FRF_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Warning Flag"]
    pub mod FWF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No enabled transmit FIFO is empty."]
            pub const FWF_0: u32 = 0;
            #[doc = "Enabled transmit FIFO is empty."]
            pub const FWF_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit underrun not detected."]
            pub const FEF_0: u32 = 0;
            #[doc = "Transmit underrun detected."]
            pub const FEF_1: u32 = 0x01;
        }
    }
    #[doc = "Sync Error Flag"]
    pub mod SEF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sync error not detected."]
            pub const SEF_0: u32 = 0;
            #[doc = "Frame sync error detected."]
            pub const SEF_1: u32 = 0x01;
        }
    }
    #[doc = "Word Start Flag"]
    pub mod WSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start of word not detected."]
            pub const WSF_0: u32 = 0;
            #[doc = "Start of word detected."]
            pub const WSF_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod SR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const SR_0: u32 = 0;
            #[doc = "Software reset."]
            pub const SR_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Reset"]
    pub mod FR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const FR_0: u32 = 0;
            #[doc = "FIFO reset."]
            pub const FR_1: u32 = 0x01;
        }
    }
    #[doc = "Bit Clock Enable"]
    pub mod BCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit bit clock is disabled."]
            pub const BCE_0: u32 = 0;
            #[doc = "Transmit bit clock is enabled."]
            pub const BCE_1: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter is disabled in Debug mode, after completing the current frame."]
            pub const DBGE_0: u32 = 0;
            #[doc = "Transmitter is enabled in Debug mode."]
            pub const DBGE_1: u32 = 0x01;
        }
    }
    #[doc = "Stop Enable"]
    pub mod STOPE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter disabled in Stop mode."]
            pub const STOPE_0: u32 = 0;
            #[doc = "Transmitter enabled in Stop mode."]
            pub const STOPE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmitter Enable"]
    pub mod TE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter is disabled."]
            pub const TE_0: u32 = 0;
            #[doc = "Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame."]
            pub const TE_1: u32 = 0x01;
        }
    }
}
#[doc = "SAI Transmit Configuration 1 Register"]
pub mod TCR1 {
    #[doc = "Transmit FIFO Watermark"]
    pub mod TFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Transmit Configuration 2 Register"]
pub mod TCR2 {
    #[doc = "Bit Clock Divide"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Clock Direction"]
    pub mod BCD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock is generated externally in Slave mode."]
            pub const BCD_0: u32 = 0;
            #[doc = "Bit clock is generated internally in Master mode."]
            pub const BCD_1: u32 = 0x01;
        }
    }
    #[doc = "Bit Clock Polarity"]
    pub mod BCP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
            pub const BCP_0: u32 = 0;
            #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
            pub const BCP_1: u32 = 0x01;
        }
    }
    #[doc = "MCLK Select"]
    pub mod MSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus Clock selected."]
            pub const MSEL_0: u32 = 0;
            #[doc = "Master Clock (MCLK) 1 option selected."]
            pub const MSEL_1: u32 = 0x01;
            #[doc = "Master Clock (MCLK) 2 option selected."]
            pub const MSEL_2: u32 = 0x02;
            #[doc = "Master Clock (MCLK) 3 option selected."]
            pub const MSEL_3: u32 = 0x03;
        }
    }
    #[doc = "Bit Clock Input"]
    pub mod BCI {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const BCI_0: u32 = 0;
            #[doc = "Internal logic is clocked as if bit clock was externally generated."]
            pub const BCI_1: u32 = 0x01;
        }
    }
    #[doc = "Bit Clock Swap"]
    pub mod BCS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the normal bit clock source."]
            pub const BCS_0: u32 = 0;
            #[doc = "Swap the bit clock source."]
            pub const BCS_1: u32 = 0x01;
        }
    }
    #[doc = "Synchronous Mode"]
    pub mod SYNC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode."]
            pub const SYNC_0: u32 = 0;
            #[doc = "Synchronous with receiver."]
            pub const SYNC_1: u32 = 0x01;
        }
    }
}
#[doc = "SAI Transmit Configuration 3 Register"]
pub mod TCR3 {
    #[doc = "Word Flag Configuration"]
    pub mod WDFL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Channel Enable"]
    pub mod TCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel FIFO Reset"]
    pub mod CFR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Transmit Configuration 4 Register"]
pub mod TCR4 {
    #[doc = "Frame Sync Direction"]
    pub mod FSD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync is generated externally in Slave mode."]
            pub const FSD_0: u32 = 0;
            #[doc = "Frame sync is generated internally in Master mode."]
            pub const FSD_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Sync Polarity"]
    pub mod FSP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync is active high."]
            pub const FSP_0: u32 = 0;
            #[doc = "Frame sync is active low."]
            pub const FSP_1: u32 = 0x01;
        }
    }
    #[doc = "On Demand Mode"]
    pub mod ONDEM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal frame sync is generated continuously."]
            pub const ONDEM_0: u32 = 0;
            #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
            pub const ONDEM_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Sync Early"]
    pub mod FSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync asserts with the first bit of the frame."]
            pub const FSE_0: u32 = 0;
            #[doc = "Frame sync asserts one bit before the first bit of the frame."]
            pub const FSE_1: u32 = 0x01;
        }
    }
    #[doc = "MSB First"]
    pub mod MF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB is transmitted first."]
            pub const MF_0: u32 = 0;
            #[doc = "MSB is transmitted first."]
            pub const MF_1: u32 = 0x01;
        }
    }
    #[doc = "Channel Mode"]
    pub mod CHMOD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled."]
            pub const CHMOD_0: u32 = 0;
            #[doc = "Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled."]
            pub const CHMOD_1: u32 = 0x01;
        }
    }
    #[doc = "Sync Width"]
    pub mod SYWD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame size"]
    pub mod FRSZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Packing Mode"]
    pub mod FPACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO packing is disabled"]
            pub const FPACK_0: u32 = 0;
            #[doc = "8-bit FIFO packing is enabled"]
            pub const FPACK_2: u32 = 0x02;
            #[doc = "16-bit FIFO packing is enabled"]
            pub const FPACK_3: u32 = 0x03;
        }
    }
    #[doc = "FIFO Combine Mode"]
    pub mod FCOMB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO combine mode disabled."]
            pub const FCOMB_0: u32 = 0;
            #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers)."]
            pub const FCOMB_1: u32 = 0x01;
            #[doc = "FIFO combine mode enabled on FIFO writes (by software)."]
            pub const FCOMB_2: u32 = 0x02;
            #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
            pub const FCOMB_3: u32 = 0x03;
        }
    }
    #[doc = "FIFO Continue on Error"]
    pub mod FCONT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
            pub const FCONT_0: u32 = 0;
            #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
            pub const FCONT_1: u32 = 0x01;
        }
    }
}
#[doc = "SAI Transmit Configuration 5 Register"]
pub mod TCR5 {
    #[doc = "First Bit Shifted"]
    pub mod FBT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word 0 Width"]
    pub mod W0W {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word N Width"]
    pub mod WNW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Transmit Data Register"]
pub mod TDR {
    #[doc = "Transmit Data Register"]
    pub mod TDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Transmit FIFO Register"]
pub mod TFR {
    #[doc = "Read FIFO Pointer"]
    pub mod RFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write FIFO Pointer"]
    pub mod WFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Channel Pointer"]
    pub mod WCP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const WCP_0: u32 = 0;
            #[doc = "FIFO combine is enabled for FIFO writes and this FIFO will be written on the next FIFO write."]
            pub const WCP_1: u32 = 0x01;
        }
    }
}
#[doc = "SAI Transmit Mask Register"]
pub mod TMR {
    #[doc = "Transmit Word Mask"]
    pub mod TWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Word N is enabled."]
            pub const TWM_0: u32 = 0;
            #[doc = "Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
            pub const TWM_1: u32 = 0x01;
        }
    }
}
#[doc = "SAI Receive Control Register"]
pub mod RCSR {
    #[doc = "FIFO Request DMA Enable"]
    pub mod FRDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the DMA request."]
            pub const FRDE_0: u32 = 0;
            #[doc = "Enables the DMA request."]
            pub const FRDE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Warning DMA Enable"]
    pub mod FWDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the DMA request."]
            pub const FWDE_0: u32 = 0;
            #[doc = "Enables the DMA request."]
            pub const FWDE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Request Interrupt Enable"]
    pub mod FRIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const FRIE_0: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const FRIE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    pub mod FWIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const FWIE_0: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const FWIE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const FEIE_0: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const FEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Sync Error Interrupt Enable"]
    pub mod SEIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt."]
            pub const SEIE_0: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const SEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Word Start Interrupt Enable"]
    pub mod WSIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt."]
            pub const WSIE_0: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const WSIE_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Request Flag"]
    pub mod FRF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive FIFO watermark not reached."]
            pub const FRF_0: u32 = 0;
            #[doc = "Receive FIFO watermark has been reached."]
            pub const FRF_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Warning Flag"]
    pub mod FWF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No enabled receive FIFO is full."]
            pub const FWF_0: u32 = 0;
            #[doc = "Enabled receive FIFO is full."]
            pub const FWF_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive overflow not detected."]
            pub const FEF_0: u32 = 0;
            #[doc = "Receive overflow detected."]
            pub const FEF_1: u32 = 0x01;
        }
    }
    #[doc = "Sync Error Flag"]
    pub mod SEF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sync error not detected."]
            pub const SEF_0: u32 = 0;
            #[doc = "Frame sync error detected."]
            pub const SEF_1: u32 = 0x01;
        }
    }
    #[doc = "Word Start Flag"]
    pub mod WSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start of word not detected."]
            pub const WSF_0: u32 = 0;
            #[doc = "Start of word detected."]
            pub const WSF_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod SR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const SR_0: u32 = 0;
            #[doc = "Software reset."]
            pub const SR_1: u32 = 0x01;
        }
    }
    #[doc = "FIFO Reset"]
    pub mod FR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const FR_0: u32 = 0;
            #[doc = "FIFO reset."]
            pub const FR_1: u32 = 0x01;
        }
    }
    #[doc = "Bit Clock Enable"]
    pub mod BCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive bit clock is disabled."]
            pub const BCE_0: u32 = 0;
            #[doc = "Receive bit clock is enabled."]
            pub const BCE_1: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
            pub const DBGE_0: u32 = 0;
            #[doc = "Receiver is enabled in Debug mode."]
            pub const DBGE_1: u32 = 0x01;
        }
    }
    #[doc = "Stop Enable"]
    pub mod STOPE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver disabled in Stop mode."]
            pub const STOPE_0: u32 = 0;
            #[doc = "Receiver enabled in Stop mode."]
            pub const STOPE_1: u32 = 0x01;
        }
    }
    #[doc = "Receiver Enable"]
    pub mod RE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver is disabled."]
            pub const RE_0: u32 = 0;
            #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
            pub const RE_1: u32 = 0x01;
        }
    }
}
#[doc = "SAI Receive Configuration 1 Register"]
pub mod RCR1 {
    #[doc = "Receive FIFO Watermark"]
    pub mod RFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Receive Configuration 2 Register"]
pub mod RCR2 {
    #[doc = "Bit Clock Divide"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Clock Direction"]
    pub mod BCD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock is generated externally in Slave mode."]
            pub const BCD_0: u32 = 0;
            #[doc = "Bit clock is generated internally in Master mode."]
            pub const BCD_1: u32 = 0x01;
        }
    }
    #[doc = "Bit Clock Polarity"]
    pub mod BCP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
            pub const BCP_0: u32 = 0;
            #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
            pub const BCP_1: u32 = 0x01;
        }
    }
    #[doc = "MCLK Select"]
    pub mod MSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus Clock selected."]
            pub const MSEL_0: u32 = 0;
            #[doc = "Master Clock (MCLK) 1 option selected."]
            pub const MSEL_1: u32 = 0x01;
            #[doc = "Master Clock (MCLK) 2 option selected."]
            pub const MSEL_2: u32 = 0x02;
            #[doc = "Master Clock (MCLK) 3 option selected."]
            pub const MSEL_3: u32 = 0x03;
        }
    }
    #[doc = "Bit Clock Input"]
    pub mod BCI {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const BCI_0: u32 = 0;
            #[doc = "Internal logic is clocked as if bit clock was externally generated."]
            pub const BCI_1: u32 = 0x01;
        }
    }
    #[doc = "Bit Clock Swap"]
    pub mod BCS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the normal bit clock source."]
            pub const BCS_0: u32 = 0;
            #[doc = "Swap the bit clock source."]
            pub const BCS_1: u32 = 0x01;
        }
    }
    #[doc = "Synchronous Mode"]
    pub mod SYNC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode."]
            pub const SYNC_0: u32 = 0;
            #[doc = "Synchronous with transmitter."]
            pub const SYNC_1: u32 = 0x01;
        }
    }
}
#[doc = "SAI Receive Configuration 3 Register"]
pub mod RCR3 {
    #[doc = "Word Flag Configuration"]
    pub mod WDFL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Channel Enable"]
    pub mod RCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel FIFO Reset"]
    pub mod CFR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Receive Configuration 4 Register"]
pub mod RCR4 {
    #[doc = "Frame Sync Direction"]
    pub mod FSD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Sync is generated externally in Slave mode."]
            pub const FSD_0: u32 = 0;
            #[doc = "Frame Sync is generated internally in Master mode."]
            pub const FSD_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Sync Polarity"]
    pub mod FSP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync is active high."]
            pub const FSP_0: u32 = 0;
            #[doc = "Frame sync is active low."]
            pub const FSP_1: u32 = 0x01;
        }
    }
    #[doc = "On Demand Mode"]
    pub mod ONDEM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal frame sync is generated continuously."]
            pub const ONDEM_0: u32 = 0;
            #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
            pub const ONDEM_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Sync Early"]
    pub mod FSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync asserts with the first bit of the frame."]
            pub const FSE_0: u32 = 0;
            #[doc = "Frame sync asserts one bit before the first bit of the frame."]
            pub const FSE_1: u32 = 0x01;
        }
    }
    #[doc = "MSB First"]
    pub mod MF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB is received first."]
            pub const MF_0: u32 = 0;
            #[doc = "MSB is received first."]
            pub const MF_1: u32 = 0x01;
        }
    }
    #[doc = "Sync Width"]
    pub mod SYWD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Size"]
    pub mod FRSZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Packing Mode"]
    pub mod FPACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO packing is disabled"]
            pub const FPACK_0: u32 = 0;
            #[doc = "8-bit FIFO packing is enabled"]
            pub const FPACK_2: u32 = 0x02;
            #[doc = "16-bit FIFO packing is enabled"]
            pub const FPACK_3: u32 = 0x03;
        }
    }
    #[doc = "FIFO Combine Mode"]
    pub mod FCOMB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO combine mode disabled."]
            pub const FCOMB_0: u32 = 0;
            #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers)."]
            pub const FCOMB_1: u32 = 0x01;
            #[doc = "FIFO combine mode enabled on FIFO reads (by software)."]
            pub const FCOMB_2: u32 = 0x02;
            #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
            pub const FCOMB_3: u32 = 0x03;
        }
    }
    #[doc = "FIFO Continue on Error"]
    pub mod FCONT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
            pub const FCONT_0: u32 = 0;
            #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
            pub const FCONT_1: u32 = 0x01;
        }
    }
}
#[doc = "SAI Receive Configuration 5 Register"]
pub mod RCR5 {
    #[doc = "First Bit Shifted"]
    pub mod FBT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word 0 Width"]
    pub mod W0W {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word N Width"]
    pub mod WNW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Receive Data Register"]
pub mod RDR {
    #[doc = "Receive Data Register"]
    pub mod RDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Receive FIFO Register"]
pub mod RFR {
    #[doc = "Read FIFO Pointer"]
    pub mod RFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Channel Pointer"]
    pub mod RCP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const RCP_0: u32 = 0;
            #[doc = "FIFO combine is enabled for FIFO reads and this FIFO will be read on the next FIFO read."]
            pub const RCP_1: u32 = 0x01;
        }
    }
    #[doc = "Write FIFO Pointer"]
    pub mod WFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI Receive Mask Register"]
pub mod RMR {
    #[doc = "Receive Word Mask"]
    pub mod RWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Word N is enabled."]
            pub const RWM_0: u32 = 0;
            #[doc = "Word N is masked."]
            pub const RWM_1: u32 = 0x01;
        }
    }
}
