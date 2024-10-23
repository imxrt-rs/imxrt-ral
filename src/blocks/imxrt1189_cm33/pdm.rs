#[doc = "MICFIL"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "MICFIL Control 1"]
    pub CTRL_1: crate::RWRegister<u32>,
    #[doc = "MICFIL Control 2"]
    pub CTRL_2: crate::RWRegister<u32>,
    #[doc = "MICFIL Status"]
    pub STAT: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "MICFIL FIFO Control"]
    pub FIFO_CTRL: crate::RWRegister<u32>,
    #[doc = "MICFIL FIFO Status"]
    pub FIFO_STAT: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "MICFIL Output Result"]
    pub DATACH: [crate::RORegister<u32>; 8usize],
    _reserved2: [u8; 0x20],
    #[doc = "MICFIL DC Remover Control"]
    pub DC_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "MICFIL Range Control"]
    pub RANGE_CTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "MICFIL Range Status"]
    pub RANGE_STAT: crate::RWRegister<u32>,
    _reserved5: [u8; 0x10],
    #[doc = "Voice Activity Detector 0 Control"]
    pub VAD0_CTRL_1: crate::RWRegister<u32>,
    #[doc = "Voice Activity Detector 0 Control"]
    pub VAD0_CTRL_2: crate::RWRegister<u32>,
    #[doc = "Voice Activity Detector 0 Status"]
    pub VAD0_STAT: crate::RWRegister<u32>,
    #[doc = "Voice Activity Detector 0 Signal Configuration"]
    pub VAD0_SCONFIG: crate::RWRegister<u32>,
    #[doc = "Voice Activity Detector 0 Noise Configuration"]
    pub VAD0_NCONFIG: crate::RWRegister<u32>,
    #[doc = "Voice Activity Detector 0 Noise Data"]
    pub VAD0_NDATA: crate::RORegister<u32>,
    #[doc = "Voice Activity Detector 0 Zero-Crossing Detector"]
    pub VAD0_ZCD: crate::RWRegister<u32>,
}
#[doc = "MICFIL Control 1"]
pub mod CTRL_1 {
    #[doc = "Channel 0 Enable"]
    pub mod CH0EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 1 Enable"]
    pub mod CH1EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 2 Enable"]
    pub mod CH2EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 3 Enable"]
    pub mod CH3EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 4 Enable"]
    pub mod CH4EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 5 Enable"]
    pub mod CH5EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 6 Enable"]
    pub mod CH6EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 7 Enable"]
    pub mod CH7EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Interruption Enable"]
    pub mod ERREN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error Interrupts disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Error Interrupts enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "DMA Interrupt Selection"]
    pub mod DISEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA and interrupt requests disabled"]
            pub const ALL_DISABLED: u32 = 0;
            #[doc = "DMA requests enabled"]
            pub const DMAREQ_ENABLED: u32 = 0x01;
            #[doc = "Interrupt requests enabled"]
            pub const INTREQ_ENABLED: u32 = 0x02;
        }
    }
    #[doc = "Module Enable in Debug"]
    pub mod DBGE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled after completing the current frame"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod SRES {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Software reset"]
            pub const SW_RESET: u32 = 0x01;
        }
    }
    #[doc = "Debug Mode"]
    pub mod DBG {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Mode"]
            pub const NORMAL: u32 = 0;
            #[doc = "Debug Mode"]
            pub const DEBUG: u32 = 0x01;
        }
    }
    #[doc = "MICFIL Enable"]
    pub mod PDMIEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PDM stopped"]
            pub const STOPPED: u32 = 0;
            #[doc = "PDM operation started"]
            pub const STARTED: u32 = 0x01;
        }
    }
    #[doc = "Doze Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Module Disable"]
    pub mod MDIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Mode"]
            pub const NORMAL: u32 = 0;
            #[doc = "Disable/Low Leakage Mode"]
            pub const LOW_LEAKAGE: u32 = 0x01;
        }
    }
}
#[doc = "MICFIL Control 2"]
pub mod CTRL_2 {
    #[doc = "Clock Divider"]
    pub mod CLKDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal clock divider value = 0"]
            pub const CLKDIV_0: u32 = 0;
            #[doc = "Internal clock divider value = 1"]
            pub const CLKDIV_1: u32 = 0x01;
            #[doc = "..."]
            pub const CLKDIV_2_2: u32 = 0x02;
            #[doc = "..."]
            pub const CLKDIV_2_3: u32 = 0x03;
            #[doc = "..."]
            pub const CLKDIV_2_4: u32 = 0x04;
            #[doc = "..."]
            pub const CLKDIV_2_5: u32 = 0x05;
            #[doc = "..."]
            pub const CLKDIV_2_6: u32 = 0x06;
            #[doc = "..."]
            pub const CLKDIV_2_7: u32 = 0x07;
            #[doc = "..."]
            pub const CLKDIV_2_8: u32 = 0x08;
            #[doc = "..."]
            pub const CLKDIV_2_9: u32 = 0x09;
            #[doc = "Internal clock divider value = 255"]
            pub const CLKDIV_255: u32 = 0xff;
        }
    }
    #[doc = "CIC Decimation Rate"]
    pub mod CICOSR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CIC oversampling rate = 0"]
            pub const CICOSR_0: u32 = 0;
            #[doc = "CIC oversampling rate = 1"]
            pub const CICOSR_1: u32 = 0x01;
            #[doc = "..."]
            pub const CICOSR_2_2: u32 = 0x02;
            #[doc = "..."]
            pub const CICOSR_2_3: u32 = 0x03;
            #[doc = "..."]
            pub const CICOSR_2_4: u32 = 0x04;
            #[doc = "..."]
            pub const CICOSR_2_5: u32 = 0x05;
            #[doc = "..."]
            pub const CICOSR_2_6: u32 = 0x06;
            #[doc = "..."]
            pub const CICOSR_2_7: u32 = 0x07;
            #[doc = "..."]
            pub const CICOSR_2_8: u32 = 0x08;
            #[doc = "..."]
            pub const CICOSR_2_9: u32 = 0x09;
            #[doc = "CIC oversampling rate = 15"]
            pub const CICOSR_15: u32 = 0x0f;
        }
    }
    #[doc = "Quality Mode"]
    pub mod QSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Medium quality mode"]
            pub const MQ_MODE: u32 = 0;
            #[doc = "High quality mode"]
            pub const HQ_MODE: u32 = 0x01;
            #[doc = "Very low quality 2 mode"]
            pub const VLQ2_MODE: u32 = 0x04;
            #[doc = "Very low quality 1 mode"]
            pub const VLQ1_MODE: u32 = 0x05;
            #[doc = "Very low quality 0 mode"]
            pub const VLQ0_MODE: u32 = 0x06;
            #[doc = "Low quality mode"]
            pub const LQ_MODE: u32 = 0x07;
        }
    }
}
#[doc = "MICFIL Status"]
pub mod STAT {
    #[doc = "Channel 0 Output Data Flag"]
    pub mod CH0F {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel's FIFO did not reach the number of elements configured in watermark bit-field"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Channel's FIFO reached the number of elements configured in watermark bit-field"]
            pub const WM_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Channel 1 Output Data Flag"]
    pub mod CH1F {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel's FIFO did not reach the number of elements configured in watermark bit-field"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Channel's FIFO reached the number of elements configured in watermark bit-field"]
            pub const WM_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Channel 2 Output Data Flag"]
    pub mod CH2F {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel's FIFO did not reach the number of elements configured in watermark bit-field"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Channel's FIFO reached the number of elements configured in watermark bit-field"]
            pub const WM_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Channel 3 Output Data Flag"]
    pub mod CH3F {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel's FIFO did not reach the number of elements configured in watermark bit-field"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Channel's FIFO reached the number of elements configured in watermark bit-field"]
            pub const WM_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Channel 4 Output Data Flag"]
    pub mod CH4F {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel's FIFO did not reach the number of elements configured in watermark bit-field"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Channel's FIFO reached the number of elements configured in watermark bit-field"]
            pub const WM_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Channel 5 Output Data Flag"]
    pub mod CH5F {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel's FIFO did not reach the number of elements configured in watermark bit-field"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Channel's FIFO reached the number of elements configured in watermark bit-field"]
            pub const WM_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Channel 6 Output Data Flag"]
    pub mod CH6F {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel's FIFO did not reach the number of elements configured in watermark bit-field"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Channel's FIFO reached the number of elements configured in watermark bit-field"]
            pub const WM_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Channel 7 Output Data Flag"]
    pub mod CH7F {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel's FIFO did not reach the number of elements configured in watermark bit-field"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Channel's FIFO reached the number of elements configured in watermark bit-field"]
            pub const WM_REACHED: u32 = 0x01;
        }
    }
    #[doc = "Low Frequency Flag"]
    pub mod LOWFREQF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CLKDIV value is OK"]
            pub const CLKDIV_OK: u32 = 0;
            #[doc = "CLKDIV value is too low"]
            pub const CLKDIV_LOW: u32 = 0x01;
        }
    }
    #[doc = "Filter Data Ready"]
    pub mod FIR_RDY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Filter data is not reliable"]
            pub const NOT_RELIABLE: u32 = 0;
            #[doc = "Filter data is reliable"]
            pub const RELIABLE: u32 = 0x01;
        }
    }
    #[doc = "Busy Flag"]
    pub mod BSY_FIL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PDM is stopped"]
            pub const STOPPED: u32 = 0;
            #[doc = "PDM is running"]
            pub const RUNNING: u32 = 0x01;
        }
    }
}
#[doc = "MICFIL FIFO Control"]
pub mod FIFO_CTRL {
    #[doc = "FIFO Watermark Control"]
    pub mod FIFOWMK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MICFIL FIFO Status"]
pub mod FIFO_STAT {
    #[doc = "FIFO Overflow Exception flag for Channel 0"]
    pub mod FIFOOVF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Overflow Exception flag for Channel 1"]
    pub mod FIFOOVF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Overflow Exception flag for Channel 2"]
    pub mod FIFOOVF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Overflow Exception flag for Channel 3"]
    pub mod FIFOOVF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Overflow Exception flag for Channel 4"]
    pub mod FIFOOVF4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Overflow Exception flag for Channel 5"]
    pub mod FIFOOVF5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Overflow Exception flag for Channel 6"]
    pub mod FIFOOVF6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Overflow Exception flag for Channel 7"]
    pub mod FIFOOVF7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Underflow Exception flag for Channel 0"]
    pub mod FIFOUND0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO Underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Underflow Exception flag for Channel 1"]
    pub mod FIFOUND1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO Underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Underflow Exception flag for Channel 2"]
    pub mod FIFOUND2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO Underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Underflow Exception flag for Channel 3"]
    pub mod FIFOUND3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO Underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Underflow Exception flag for Channel 4"]
    pub mod FIFOUND4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO Underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Underflow Exception flag for Channel 5"]
    pub mod FIFOUND5 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO Underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Underflow Exception flag for Channel 6"]
    pub mod FIFOUND6 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO Underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "FIFO Underflow Exception flag for Channel 7"]
    pub mod FIFOUND7 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO Underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
}
#[doc = "MICFIL Output Result"]
pub mod DATACH {
    #[doc = "Channel n Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MICFIL DC Remover Control"]
pub mod DC_CTRL {
    #[doc = "Channel 0 DC Remover Configuration"]
    pub mod DCCONFIG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DC Remover cut-off at 21Hz"]
            pub const DC_REM_21HZ: u32 = 0;
            #[doc = "DC Remover cut-off at 83Hz"]
            pub const DC_REM_83HZ: u32 = 0x01;
            #[doc = "DC Remover cut-off at 152Hz"]
            pub const DC_REM_152HZ: u32 = 0x02;
            #[doc = "DC Remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 0x03;
        }
    }
    #[doc = "Channel 1 DC Remover Configuration"]
    pub mod DCCONFIG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DC Remover cut-off at 21Hz"]
            pub const DC_REM_21HZ: u32 = 0;
            #[doc = "DC Remover cut-off at 83Hz"]
            pub const DC_REM_83HZ: u32 = 0x01;
            #[doc = "DC Remover cut-off at 152Hz"]
            pub const DC_REM_152HZ: u32 = 0x02;
            #[doc = "DC Remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 0x03;
        }
    }
    #[doc = "Channel 2 DC Remover Configuration"]
    pub mod DCCONFIG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DC Remover cut-off at 21Hz"]
            pub const DC_REM_21HZ: u32 = 0;
            #[doc = "DC Remover cut-off at 83Hz"]
            pub const DC_REM_83HZ: u32 = 0x01;
            #[doc = "DC Remover cut-off at 152Hz"]
            pub const DC_REM_152HZ: u32 = 0x02;
            #[doc = "DC Remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 0x03;
        }
    }
    #[doc = "Channel 3 DC Remover Configuration"]
    pub mod DCCONFIG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DC Remover cut-off at 21Hz"]
            pub const DC_REM_21HZ: u32 = 0;
            #[doc = "DC Remover cut-off at 83Hz"]
            pub const DC_REM_83HZ: u32 = 0x01;
            #[doc = "DC Remover cut-off at 152Hz"]
            pub const DC_REM_152HZ: u32 = 0x02;
            #[doc = "DC Remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 0x03;
        }
    }
    #[doc = "Channel 4 DC Remover Configuration"]
    pub mod DCCONFIG4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DC Remover cut-off at 21Hz"]
            pub const DC_REM_21HZ: u32 = 0;
            #[doc = "DC Remover cut-off at 83Hz"]
            pub const DC_REM_83HZ: u32 = 0x01;
            #[doc = "DC Remover cut-off at 152Hz"]
            pub const DC_REM_152HZ: u32 = 0x02;
            #[doc = "DC Remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 0x03;
        }
    }
    #[doc = "Channel 5 DC Remover Configuration"]
    pub mod DCCONFIG5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DC Remover cut-off at 21Hz"]
            pub const DC_REM_21HZ: u32 = 0;
            #[doc = "DC Remover cut-off at 83Hz"]
            pub const DC_REM_83HZ: u32 = 0x01;
            #[doc = "DC Remover cut-off at 152Hz"]
            pub const DC_REM_152HZ: u32 = 0x02;
            #[doc = "DC Remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 0x03;
        }
    }
    #[doc = "Channel 6 DC Remover Configuration"]
    pub mod DCCONFIG6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DC Remover cut-off at 21Hz"]
            pub const DC_REM_21HZ: u32 = 0;
            #[doc = "DC Remover cut-off at 83Hz"]
            pub const DC_REM_83HZ: u32 = 0x01;
            #[doc = "DC Remover cut-off at 152Hz"]
            pub const DC_REM_152HZ: u32 = 0x02;
            #[doc = "DC Remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 0x03;
        }
    }
    #[doc = "Channel 7 DC Remover Configuration"]
    pub mod DCCONFIG7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DC Remover cut-off at 21Hz"]
            pub const DC_REM_21HZ: u32 = 0;
            #[doc = "DC Remover cut-off at 83Hz"]
            pub const DC_REM_83HZ: u32 = 0x01;
            #[doc = "DC Remover cut-off at 152Hz"]
            pub const DC_REM_152HZ: u32 = 0x02;
            #[doc = "DC Remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 0x03;
        }
    }
}
#[doc = "MICFIL Range Control"]
pub mod RANGE_CTRL {
    #[doc = "Channel 0 Range Adjustment"]
    pub mod RANGEADJ0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 1 Range Adjustment"]
    pub mod RANGEADJ1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 2 Range Adjustment"]
    pub mod RANGEADJ2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 3 Range Adjustment"]
    pub mod RANGEADJ3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 4 Range Adjustment"]
    pub mod RANGEADJ4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 5 Range Adjustment"]
    pub mod RANGEADJ5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 6 Range Adjustment"]
    pub mod RANGEADJ6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 7 Range Adjustment"]
    pub mod RANGEADJ7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MICFIL Range Status"]
pub mod RANGE_STAT {
    #[doc = "Channel 0 Range Overflow Error Flag"]
    pub mod RANGEOVF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 1 Range Overflow Error Flag"]
    pub mod RANGEOVF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 2 Range Overflow Error Flag"]
    pub mod RANGEOVF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 3 Range Overflow Error Flag"]
    pub mod RANGEOVF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 4 Range Overflow Error Flag"]
    pub mod RANGEOVF4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 5 Range Overflow Error Flag"]
    pub mod RANGEOVF5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 6 Range Overflow Error Flag"]
    pub mod RANGEOVF6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 7 Range Overflow Error Flag"]
    pub mod RANGEOVF7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 0 Range Underflow Error Flag"]
    pub mod RANGEUNF0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 1 Range Underflow Error Flag"]
    pub mod RANGEUNF1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 2 Range Underflow Error Flag"]
    pub mod RANGEUNF2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 3 Range Underflow Error Flag"]
    pub mod RANGEUNF3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 4 Range Underflow Error Flag"]
    pub mod RANGEUNF4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 5 Range Underflow Error Flag"]
    pub mod RANGEUNF5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 6 Range Underflow Error Flag"]
    pub mod RANGEUNF6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Channel 7 Range Underflow Error Flag"]
    pub mod RANGEUNF7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
}
#[doc = "Voice Activity Detector 0 Control"]
pub mod VAD0_CTRL_1 {
    #[doc = "Voice Activity Detector Enable"]
    pub mod VADEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The HWVAD is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "The HWVAD is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Reset"]
    pub mod VADRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voice Activity Detector Interruption Enable"]
    pub mod VADIE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HWVAD Interrupts disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "HWVAD Interrupts enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Error Interruption Enable"]
    pub mod VADERIE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HWVAD Error Interrupts disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "HWVAD Error Interrupts enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Internal Filters Initialization"]
    pub mod VADST10 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation."]
            pub const NORMAL_OP: u32 = 0;
            #[doc = "Filters are initialized."]
            pub const FILT_INIT: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Initialization Time"]
    pub mod VADINITT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VADINITT = 0"]
            pub const VADINITT_0: u32 = 0;
            #[doc = "VADINITT = 1"]
            pub const VADINITT_1: u32 = 0x01;
            #[doc = "..."]
            pub const VADINITT_2_2: u32 = 0x02;
            #[doc = "..."]
            pub const VADINITT_2_3: u32 = 0x03;
            #[doc = "..."]
            pub const VADINITT_2_4: u32 = 0x04;
            #[doc = "..."]
            pub const VADINITT_2_5: u32 = 0x05;
            #[doc = "..."]
            pub const VADINITT_2_6: u32 = 0x06;
            #[doc = "..."]
            pub const VADINITT_2_7: u32 = 0x07;
            #[doc = "..."]
            pub const VADINITT_2_8: u32 = 0x08;
            #[doc = "..."]
            pub const VADINITT_2_9: u32 = 0x09;
            #[doc = "VADINITT = 31"]
            pub const VADINITT_31: u32 = 0x1f;
        }
    }
    #[doc = "Voice Activity Detector CIC Oversampling Rate"]
    pub mod VADCICOSR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voice Activity Detector Channel Selector"]
    pub mod VADCHSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PDM Microphone 0 Left"]
            pub const VADCHSEL_0: u32 = 0;
            #[doc = "PDM Microphone 0 Right"]
            pub const VADCHSEL_1: u32 = 0x01;
            #[doc = "PDM Microphone 1 Left"]
            pub const VADCHSEL_2: u32 = 0x02;
            #[doc = "..."]
            pub const VADCHSEL_3_3: u32 = 0x03;
            #[doc = "..."]
            pub const VADCHSEL_3_4: u32 = 0x04;
            #[doc = "..."]
            pub const VADCHSEL_3_5: u32 = 0x05;
            #[doc = "PDM Microphone 3 Left"]
            pub const VADCHSEL_6: u32 = 0x06;
            #[doc = "PDM Microphone 3 Right"]
            pub const VADCHSEL_7: u32 = 0x07;
        }
    }
}
#[doc = "Voice Activity Detector 0 Control"]
pub mod VAD0_CTRL_2 {
    #[doc = "Voice Activity Detector High-Pass Filter"]
    pub mod VADHPF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Filter bypassed."]
            pub const FILT_BYPASS: u32 = 0;
            #[doc = "Cut-off frequency at 1750Hz."]
            pub const CUTOFF_1750HZ: u32 = 0x01;
            #[doc = "Cut-off frequency at 215Hz."]
            pub const CUTOFF_215HZ: u32 = 0x02;
            #[doc = "Cut-off frequency at 102Hz."]
            pub const CUTOFF_102HZ: u32 = 0x03;
        }
    }
    #[doc = "Voice Activity Detector Input Gain"]
    pub mod VADINPGAIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No shift"]
            pub const VADINGAIN_0000: u32 = 0;
            #[doc = "Shift 1 bit to the left"]
            pub const VADINGAIN_0001: u32 = 0x01;
            #[doc = "Shift 2 bits to the left"]
            pub const VADINGAIN_0010: u32 = 0x02;
            #[doc = "..."]
            pub const VADINGAIN_0011_3: u32 = 0x03;
            #[doc = "..."]
            pub const VADINGAIN_0011_4: u32 = 0x04;
            #[doc = "..."]
            pub const VADINGAIN_0011_5: u32 = 0x05;
            #[doc = "..."]
            pub const VADINGAIN_0011_6: u32 = 0x06;
            #[doc = "Shift 7 bits to the left"]
            pub const VADINGAIN_0111: u32 = 0x07;
            #[doc = "Shift 8 bits to the right"]
            pub const VADINGAIN_1000: u32 = 0x08;
            #[doc = "Shift 7 bits to the right"]
            pub const VADINGAIN_1001: u32 = 0x09;
            #[doc = "..."]
            pub const VADINGAIN_1010_10: u32 = 0x0a;
            #[doc = "..."]
            pub const VADINGAIN_1010_11: u32 = 0x0b;
            #[doc = "..."]
            pub const VADINGAIN_1010_12: u32 = 0x0c;
            #[doc = "..."]
            pub const VADINGAIN_1010_13: u32 = 0x0d;
            #[doc = "..."]
            pub const VADINGAIN_1010_14: u32 = 0x0e;
            #[doc = "Shift 1 bits to the right"]
            pub const VADINGAIN_1111: u32 = 0x0f;
        }
    }
    #[doc = "Voice Activity Detector Frame Time"]
    pub mod VADFRAMET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VADFRAMET = 1"]
            pub const VADFRAMET_0: u32 = 0;
            #[doc = "VADFRAMET = 2"]
            pub const VADFRAMET_1: u32 = 0x01;
            #[doc = "..."]
            pub const VADFRAMET_2_2: u32 = 0x02;
            #[doc = "..."]
            pub const VADFRAMET_2_3: u32 = 0x03;
            #[doc = "..."]
            pub const VADFRAMET_2_4: u32 = 0x04;
            #[doc = "..."]
            pub const VADFRAMET_2_5: u32 = 0x05;
            #[doc = "..."]
            pub const VADFRAMET_2_6: u32 = 0x06;
            #[doc = "..."]
            pub const VADFRAMET_2_7: u32 = 0x07;
            #[doc = "..."]
            pub const VADFRAMET_2_8: u32 = 0x08;
            #[doc = "..."]
            pub const VADFRAMET_2_9: u32 = 0x09;
            #[doc = "VADFRAMET = 63"]
            pub const VADFRAMET_63: u32 = 0x3f;
        }
    }
    #[doc = "Voice Activity Detector Force Output Disable"]
    pub mod VADFOUTDIS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output is enabled."]
            pub const OUT_ENABLED: u32 = 0;
            #[doc = "Output is disabled."]
            pub const OUT_DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Pre Filter Enable"]
    pub mod VADPREFEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pre-filter is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "Pre-filter is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Frame Energy Disable"]
    pub mod VADFRENDIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame energy calculus enabled."]
            pub const ENABLED: u32 = 0;
            #[doc = "Frame energy calculus disabled."]
            pub const DISABLED: u32 = 0x01;
        }
    }
}
#[doc = "Voice Activity Detector 0 Status"]
pub mod VAD0_STAT {
    #[doc = "Voice Activity Detector Interrupt Flag"]
    pub mod VADIF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Voice activity not detected"]
            pub const NO_DETECT: u32 = 0;
            #[doc = "Voice activity detected"]
            pub const DETECT: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Input Saturation Flag"]
    pub mod VADINSATF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception"]
            pub const EXCEPTION: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Initialization Flag"]
    pub mod VADINITF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HWVAD is not being initialized."]
            pub const NOT_INIT: u32 = 0;
            #[doc = "HWVAD is being initialized."]
            pub const INIT: u32 = 0x01;
        }
    }
}
#[doc = "Voice Activity Detector 0 Signal Configuration"]
pub mod VAD0_SCONFIG {
    #[doc = "Voice Activity Detector Signal Gain"]
    pub mod VADSGAIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multiplier = 1"]
            pub const VADSGAIN_0_0: u32 = 0;
            #[doc = "Multiplier = 1"]
            pub const VADSGAIN_0_1: u32 = 0x01;
            #[doc = "Multiplier = 2"]
            pub const VADSGAIN_2: u32 = 0x02;
            #[doc = "..."]
            pub const VADSGAIN_3_3: u32 = 0x03;
            #[doc = "..."]
            pub const VADSGAIN_3_4: u32 = 0x04;
            #[doc = "..."]
            pub const VADSGAIN_3_5: u32 = 0x05;
            #[doc = "..."]
            pub const VADSGAIN_3_6: u32 = 0x06;
            #[doc = "..."]
            pub const VADSGAIN_3_7: u32 = 0x07;
            #[doc = "..."]
            pub const VADSGAIN_3_8: u32 = 0x08;
            #[doc = "..."]
            pub const VADSGAIN_3_9: u32 = 0x09;
            #[doc = "Multiplier = 15"]
            pub const VADSGAIN_15: u32 = 0x0f;
        }
    }
    #[doc = "Voice Activity Detector Signal Maximum Enable"]
    pub mod VADSMAXEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Maximum block is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "Maximum block is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Signal Filter Enable"]
    pub mod VADSFILEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Signal filter is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Signal filter is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "Voice Activity Detector 0 Noise Configuration"]
pub mod VAD0_NCONFIG {
    #[doc = "Voice Activity Detector Noise Gain"]
    pub mod VADNGAIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multiplier = 1"]
            pub const VADNGAIN_0_0: u32 = 0;
            #[doc = "Multiplier = 1"]
            pub const VADNGAIN_0_1: u32 = 0x01;
            #[doc = "Multiplier = 2"]
            pub const VADNGAIN_2: u32 = 0x02;
            #[doc = "..."]
            pub const VADNGAIN_3_3: u32 = 0x03;
            #[doc = "..."]
            pub const VADNGAIN_3_4: u32 = 0x04;
            #[doc = "..."]
            pub const VADNGAIN_3_5: u32 = 0x05;
            #[doc = "..."]
            pub const VADNGAIN_3_6: u32 = 0x06;
            #[doc = "..."]
            pub const VADNGAIN_3_7: u32 = 0x07;
            #[doc = "..."]
            pub const VADNGAIN_3_8: u32 = 0x08;
            #[doc = "..."]
            pub const VADNGAIN_3_9: u32 = 0x09;
            #[doc = "Multiplier = 15"]
            pub const VADNGAIN_15: u32 = 0x0f;
        }
    }
    #[doc = "Voice Activity Detector Noise Filter Adjustment"]
    pub mod VADNFILADJ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Adjustment value = 0"]
            pub const VADNFILADJ_0: u32 = 0;
            #[doc = "Adjustment value = 1"]
            pub const VADNFILADJ_1: u32 = 0x01;
            #[doc = "..."]
            pub const VADNFILADJ_2_2: u32 = 0x02;
            #[doc = "..."]
            pub const VADNFILADJ_2_3: u32 = 0x03;
            #[doc = "..."]
            pub const VADNFILADJ_2_4: u32 = 0x04;
            #[doc = "..."]
            pub const VADNFILADJ_2_5: u32 = 0x05;
            #[doc = "..."]
            pub const VADNFILADJ_2_6: u32 = 0x06;
            #[doc = "..."]
            pub const VADNFILADJ_2_7: u32 = 0x07;
            #[doc = "..."]
            pub const VADNFILADJ_2_8: u32 = 0x08;
            #[doc = "..."]
            pub const VADNFILADJ_2_9: u32 = 0x09;
            #[doc = "Adjustment value = 31"]
            pub const VADNFILADJ_31: u32 = 0x1f;
        }
    }
    #[doc = "Voice Activity Detector Noise OR Enable"]
    pub mod VADNOREN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Noise input is not decimated."]
            pub const NOT_DECIMATED: u32 = 0;
            #[doc = "Noise input is decimated."]
            pub const DECIMATED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Noise Decimation Enable"]
    pub mod VADNDECEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Noise input is not decimated."]
            pub const NOT_DECIMATED: u32 = 0;
            #[doc = "Noise input is decimated."]
            pub const DECIMATED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Noise Minimum Enable"]
    pub mod VADNMINEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum block is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "Minimum block is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Voice Activity Detector Noise Filter Auto"]
    pub mod VADNFILAUTO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Noise filter is always enabled."]
            pub const NF_ALWAYS_EN: u32 = 0;
            #[doc = "Noise filter is enabled/disabled based on voice activity information."]
            pub const NF_COND_EN: u32 = 0x01;
        }
    }
}
#[doc = "Voice Activity Detector 0 Noise Data"]
pub mod VAD0_NDATA {
    #[doc = "Voice Activity Detector Noise Data"]
    pub mod VADNDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Voice Activity Detector 0 Zero-Crossing Detector"]
pub mod VAD0_ZCD {
    #[doc = "Zero-Crossing Detector Enable"]
    pub mod VADZCDEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The ZCD is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "The ZCD is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Zero-Crossing Detector Automatic Threshold"]
    pub mod VADZCDAUTO {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The ZCD threshold is not estimated automatically"]
            pub const NOT_ESTIMATED: u32 = 0;
            #[doc = "The ZCD threshold is estimated automatically"]
            pub const ESTIMATED: u32 = 0x01;
        }
    }
    #[doc = "Zero-Crossing Detector AND Behavior"]
    pub mod VADZCDAND {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The ZCD result is OR'ed with the energy-based detection."]
            pub const ORED: u32 = 0;
            #[doc = "The ZCD result is AND'ed with the energy-based detection."]
            pub const ANDED: u32 = 0x01;
        }
    }
    #[doc = "Zero-Crossing Detector Adjustment"]
    pub mod VADZCDADJ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Adjustment value = 0"]
            pub const VADZCDADJ_0: u32 = 0;
            #[doc = "Adjustment value = 1"]
            pub const VADZCDADJ_1: u32 = 0x01;
            #[doc = "..."]
            pub const VADZCDADJ_2_2: u32 = 0x02;
            #[doc = "..."]
            pub const VADZCDADJ_2_3: u32 = 0x03;
            #[doc = "..."]
            pub const VADZCDADJ_2_4: u32 = 0x04;
            #[doc = "..."]
            pub const VADZCDADJ_2_5: u32 = 0x05;
            #[doc = "..."]
            pub const VADZCDADJ_2_6: u32 = 0x06;
            #[doc = "..."]
            pub const VADZCDADJ_2_7: u32 = 0x07;
            #[doc = "..."]
            pub const VADZCDADJ_2_8: u32 = 0x08;
            #[doc = "..."]
            pub const VADZCDADJ_2_9: u32 = 0x09;
            #[doc = "Adjustment value = 15"]
            pub const VADZCDADJ_15: u32 = 0x0f;
        }
    }
    #[doc = "Zero-Crossing Detector Threshold"]
    pub mod VADZCDTH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
