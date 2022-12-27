#[doc = "LPSPI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RORegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "Control Register"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub IER: crate::RWRegister<u32>,
    #[doc = "DMA Enable Register"]
    pub DER: crate::RWRegister<u32>,
    #[doc = "Configuration Register 0"]
    pub CFGR0: crate::RWRegister<u32>,
    #[doc = "Configuration Register 1"]
    pub CFGR1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "Data Match Register 0"]
    pub DMR0: crate::RWRegister<u32>,
    #[doc = "Data Match Register 1"]
    pub DMR1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Clock Configuration Register"]
    pub CCR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x14],
    #[doc = "FIFO Control Register"]
    pub FCR: crate::RWRegister<u32>,
    #[doc = "FIFO Status Register"]
    pub FSR: crate::RORegister<u32>,
    #[doc = "Transmit Command Register"]
    pub TCR: crate::RWRegister<u32>,
    #[doc = "Transmit Data Register"]
    pub TDR: crate::WORegister<u32>,
    _reserved4: [u8; 0x08],
    #[doc = "Receive Status Register"]
    pub RSR: crate::RORegister<u32>,
    #[doc = "Receive Data Register"]
    pub RDR: crate::RORegister<u32>,
}
#[doc = "Version ID Register"]
pub mod VERID {
    #[doc = "Module Identification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard feature set supporting a 32-bit shift register."]
            pub const FEATURE_4: u32 = 0x04;
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
    #[doc = "Transmit FIFO Size"]
    pub mod TXFIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Size"]
    pub mod RXFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCS Number"]
    pub mod PCSNUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register"]
pub mod CR {
    #[doc = "Module Enable"]
    pub mod MEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Module is disabled"]
            pub const MEN_0: u32 = 0;
            #[doc = "Module is enabled"]
            pub const MEN_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Module is not reset"]
            pub const RST_0: u32 = 0;
            #[doc = "Module is reset"]
            pub const RST_1: u32 = 0x01;
        }
    }
    #[doc = "Doze Mode Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI module is enabled in Doze mode"]
            pub const DOZEN_0: u32 = 0;
            #[doc = "LPSPI module is disabled in Doze mode"]
            pub const DOZEN_1: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI module is disabled in debug mode"]
            pub const DBGEN_0: u32 = 0;
            #[doc = "LPSPI module is enabled in debug mode"]
            pub const DBGEN_1: u32 = 0x01;
        }
    }
    #[doc = "Reset Transmit FIFO"]
    pub mod RTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const RTF_0: u32 = 0;
            #[doc = "Transmit FIFO is reset"]
            pub const RTF_1: u32 = 0x01;
        }
    }
    #[doc = "Reset Receive FIFO"]
    pub mod RRF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const RRF_0: u32 = 0;
            #[doc = "Receive FIFO is reset"]
            pub const RRF_1: u32 = 0x01;
        }
    }
}
#[doc = "Status Register"]
pub mod SR {
    #[doc = "Transmit Data Flag"]
    pub mod TDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit data not requested"]
            pub const TDF_0: u32 = 0;
            #[doc = "Transmit data is requested"]
            pub const TDF_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Flag"]
    pub mod RDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Data is not ready"]
            pub const RDF_0: u32 = 0;
            #[doc = "Receive data is ready"]
            pub const RDF_1: u32 = 0x01;
        }
    }
    #[doc = "Word Complete Flag"]
    pub mod WCF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transfer of a received word has not yet completed"]
            pub const WCF_0: u32 = 0;
            #[doc = "Transfer of a received word has completed"]
            pub const WCF_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Complete Flag"]
    pub mod FCF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame transfer has not completed"]
            pub const FCF_0: u32 = 0;
            #[doc = "Frame transfer has completed"]
            pub const FCF_1: u32 = 0x01;
        }
    }
    #[doc = "Transfer Complete Flag"]
    pub mod TCF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All transfers have not completed"]
            pub const TCF_0: u32 = 0;
            #[doc = "All transfers have completed"]
            pub const TCF_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit Error Flag"]
    pub mod TEF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit FIFO underrun has not occurred"]
            pub const TEF_0: u32 = 0;
            #[doc = "Transmit FIFO underrun has occurred"]
            pub const TEF_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Error Flag"]
    pub mod REF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive FIFO has not overflowed"]
            pub const REF_0: u32 = 0;
            #[doc = "Receive FIFO has overflowed"]
            pub const REF_1: u32 = 0x01;
        }
    }
    #[doc = "Data Match Flag"]
    pub mod DMF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Have not received matching data"]
            pub const DMF_0: u32 = 0;
            #[doc = "Have received matching data"]
            pub const DMF_1: u32 = 0x01;
        }
    }
    #[doc = "Module Busy Flag"]
    pub mod MBF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPSPI is idle"]
            pub const MBF_0: u32 = 0;
            #[doc = "LPSPI is busy"]
            pub const MBF_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable Register"]
pub mod IER {
    #[doc = "Transmit Data Interrupt Enable"]
    pub mod TDIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TDIE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Interrupt Enable"]
    pub mod RDIE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const RDIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const RDIE_1: u32 = 0x01;
        }
    }
    #[doc = "Word Complete Interrupt Enable"]
    pub mod WCIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const WCIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const WCIE_1: u32 = 0x01;
        }
    }
    #[doc = "Frame Complete Interrupt Enable"]
    pub mod FCIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const FCIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const FCIE_1: u32 = 0x01;
        }
    }
    #[doc = "Transfer Complete Interrupt Enable"]
    pub mod TCIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TCIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TCIE_1: u32 = 0x01;
        }
    }
    #[doc = "Transmit Error Interrupt Enable"]
    pub mod TEIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TEIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const TEIE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Error Interrupt Enable"]
    pub mod REIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const REIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const REIE_1: u32 = 0x01;
        }
    }
    #[doc = "Data Match Interrupt Enable"]
    pub mod DMIE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DMIE_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DMIE_1: u32 = 0x01;
        }
    }
}
#[doc = "DMA Enable Register"]
pub mod DER {
    #[doc = "Transmit Data DMA Enable"]
    pub mod TDDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const TDDE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const TDDE_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data DMA Enable"]
    pub mod RDDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const RDDE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const RDDE_1: u32 = 0x01;
        }
    }
}
#[doc = "Configuration Register 0"]
pub mod CFGR0 {
    #[doc = "Host Request Enable"]
    pub mod HREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Request Polarity"]
    pub mod HRPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Request Select"]
    pub mod HRSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Circular FIFO Enable"]
    pub mod CIRFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Circular FIFO is disabled"]
            pub const CIRFIFO_0: u32 = 0;
            #[doc = "Circular FIFO is enabled"]
            pub const CIRFIFO_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Match Only"]
    pub mod RDMO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received data is stored in the receive FIFO as in normal operations"]
            pub const RDMO_0: u32 = 0;
            #[doc = "Received data is discarded unless the Data Match Flag (DMF) is set"]
            pub const RDMO_1: u32 = 0x01;
        }
    }
}
#[doc = "Configuration Register 1"]
pub mod CFGR1 {
    #[doc = "Master Mode"]
    pub mod MASTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave mode"]
            pub const MASTER_0: u32 = 0;
            #[doc = "Master mode"]
            pub const MASTER_1: u32 = 0x01;
        }
    }
    #[doc = "Sample Point"]
    pub mod SAMPLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input data is sampled on SCK edge"]
            pub const SAMPLE_0: u32 = 0;
            #[doc = "Input data is sampled on delayed SCK edge"]
            pub const SAMPLE_1: u32 = 0x01;
        }
    }
    #[doc = "Automatic PCS"]
    pub mod AUTOPCS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic PCS generation is disabled"]
            pub const AUTOPCS_0: u32 = 0;
            #[doc = "Automatic PCS generation is enabled"]
            pub const AUTOPCS_1: u32 = 0x01;
        }
    }
    #[doc = "No Stall"]
    pub mod NOSTALL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transfers will stall when the transmit FIFO is empty or the receive FIFO is full"]
            pub const NOSTALL_0: u32 = 0;
            #[doc = "Transfers will not stall, allowing transmit FIFO underruns or receive FIFO overruns to occur"]
            pub const NOSTALL_1: u32 = 0x01;
        }
    }
    #[doc = "Peripheral Chip Select Polarity"]
    pub mod PCSPOL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match Configuration"]
    pub mod MATCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match is disabled"]
            pub const MATCFG_0: u32 = 0;
            #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
            pub const MATCFG_2: u32 = 0x02;
            #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
            pub const MATCFG_3: u32 = 0x03;
            #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., \\[(1st data word = MATCH0) * (2nd data word = MATCH1)\\]"]
            pub const MATCFG_4: u32 = 0x04;
            #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., \\[(any data word = MATCH0) * (next data word = MATCH1)\\]"]
            pub const MATCFG_5: u32 = 0x05;
            #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(1st data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
            pub const MATCFG_6: u32 = 0x06;
            #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(any data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
            pub const MATCFG_7: u32 = 0x07;
        }
    }
    #[doc = "Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIN is used for input data and SOUT is used for output data"]
            pub const PINCFG_0: u32 = 0;
            #[doc = "SIN is used for both input and output data"]
            pub const PINCFG_1: u32 = 0x01;
            #[doc = "SOUT is used for both input and output data"]
            pub const PINCFG_2: u32 = 0x02;
            #[doc = "SOUT is used for input data and SIN is used for output data"]
            pub const PINCFG_3: u32 = 0x03;
        }
    }
    #[doc = "Output Config"]
    pub mod OUTCFG {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output data retains last value when chip select is negated"]
            pub const OUTCFG_0: u32 = 0;
            #[doc = "Output data is tristated when chip select is negated"]
            pub const OUTCFG_1: u32 = 0x01;
        }
    }
    #[doc = "Peripheral Chip Select Configuration"]
    pub mod PCSCFG {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PCS\\[3:2\\] are enabled"]
            pub const PCSCFG_0: u32 = 0;
            #[doc = "PCS\\[3:2\\] are disabled"]
            pub const PCSCFG_1: u32 = 0x01;
        }
    }
}
#[doc = "Data Match Register 0"]
pub mod DMR0 {
    #[doc = "Match 0 Value"]
    pub mod MATCH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Match Register 1"]
pub mod DMR1 {
    #[doc = "Match 1 Value"]
    pub mod MATCH1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Configuration Register"]
pub mod CCR {
    #[doc = "SCK Divider"]
    pub mod SCKDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay Between Transfers"]
    pub mod DBT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCS-to-SCK Delay"]
    pub mod PCSSCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCK-to-PCS Delay"]
    pub mod SCKPCS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO Control Register"]
pub mod FCR {
    #[doc = "Transmit FIFO Watermark"]
    pub mod TXWATER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Watermark"]
    pub mod RXWATER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO Status Register"]
pub mod FSR {
    #[doc = "Transmit FIFO Count"]
    pub mod TXCOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Count"]
    pub mod RXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Command Register"]
pub mod TCR {
    #[doc = "Frame Size"]
    pub mod FRAMESZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transfer Width"]
    pub mod WIDTH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bit transfer"]
            pub const WIDTH_0: u32 = 0;
            #[doc = "2 bit transfer"]
            pub const WIDTH_1: u32 = 0x01;
            #[doc = "4 bit transfer"]
            pub const WIDTH_2: u32 = 0x02;
        }
    }
    #[doc = "Transmit Data Mask"]
    pub mod TXMSK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transfer"]
            pub const TXMSK_0: u32 = 0;
            #[doc = "Mask transmit data"]
            pub const TXMSK_1: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Mask"]
    pub mod RXMSK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transfer"]
            pub const RXMSK_0: u32 = 0;
            #[doc = "Receive data is masked"]
            pub const RXMSK_1: u32 = 0x01;
        }
    }
    #[doc = "Continuing Command"]
    pub mod CONTC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command word for start of new transfer"]
            pub const CONTC_0: u32 = 0;
            #[doc = "Command word for continuing transfer"]
            pub const CONTC_1: u32 = 0x01;
        }
    }
    #[doc = "Continuous Transfer"]
    pub mod CONT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continuous transfer is disabled"]
            pub const CONT_0: u32 = 0;
            #[doc = "Continuous transfer is enabled"]
            pub const CONT_1: u32 = 0x01;
        }
    }
    #[doc = "Byte Swap"]
    pub mod BYSW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte swap is disabled"]
            pub const BYSW_0: u32 = 0;
            #[doc = "Byte swap is enabled"]
            pub const BYSW_1: u32 = 0x01;
        }
    }
    #[doc = "LSB First"]
    pub mod LSBF {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data is transferred MSB first"]
            pub const LSBF_0: u32 = 0;
            #[doc = "Data is transferred LSB first"]
            pub const LSBF_1: u32 = 0x01;
        }
    }
    #[doc = "Peripheral Chip Select"]
    pub mod PCS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transfer using LPSPI_PCS\\[0\\]"]
            pub const PCS_0: u32 = 0;
            #[doc = "Transfer using LPSPI_PCS\\[1\\]"]
            pub const PCS_1: u32 = 0x01;
            #[doc = "Transfer using LPSPI_PCS\\[2\\]"]
            pub const PCS_2: u32 = 0x02;
            #[doc = "Transfer using LPSPI_PCS\\[3\\]"]
            pub const PCS_3: u32 = 0x03;
        }
    }
    #[doc = "Prescaler Value"]
    pub mod PRESCALE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const PRESCALE_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const PRESCALE_1: u32 = 0x01;
            #[doc = "Divide by 4"]
            pub const PRESCALE_2: u32 = 0x02;
            #[doc = "Divide by 8"]
            pub const PRESCALE_3: u32 = 0x03;
            #[doc = "Divide by 16"]
            pub const PRESCALE_4: u32 = 0x04;
            #[doc = "Divide by 32"]
            pub const PRESCALE_5: u32 = 0x05;
            #[doc = "Divide by 64"]
            pub const PRESCALE_6: u32 = 0x06;
            #[doc = "Divide by 128"]
            pub const PRESCALE_7: u32 = 0x07;
        }
    }
    #[doc = "Clock Phase"]
    pub mod CPHA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data is captured on the leading edge of SCK and changed on the following edge of SCK"]
            pub const CPHA_0: u32 = 0;
            #[doc = "Data is changed on the leading edge of SCK and captured on the following edge of SCK"]
            pub const CPHA_1: u32 = 0x01;
        }
    }
    #[doc = "Clock Polarity"]
    pub mod CPOL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The inactive state value of SCK is low"]
            pub const CPOL_0: u32 = 0;
            #[doc = "The inactive state value of SCK is high"]
            pub const CPOL_1: u32 = 0x01;
        }
    }
}
#[doc = "Transmit Data Register"]
pub mod TDR {
    #[doc = "Transmit Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Status Register"]
pub mod RSR {
    #[doc = "Start Of Frame"]
    pub mod SOF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Subsequent data word received after LPSPI_PCS assertion"]
            pub const SOF_0: u32 = 0;
            #[doc = "First data word received after LPSPI_PCS assertion"]
            pub const SOF_1: u32 = 0x01;
        }
    }
    #[doc = "RX FIFO Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RX FIFO is not empty"]
            pub const RXEMPTY_0: u32 = 0;
            #[doc = "RX FIFO is empty"]
            pub const RXEMPTY_1: u32 = 0x01;
        }
    }
}
#[doc = "Receive Data Register"]
pub mod RDR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
