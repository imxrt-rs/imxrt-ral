#[doc = "TPM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "TPM Global"]
    pub GLOBAL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Status and Control"]
    pub SC: crate::RWRegister<u32>,
    #[doc = "Counter"]
    pub CNT: crate::RWRegister<u32>,
    #[doc = "Modulo"]
    pub MOD: crate::RWRegister<u32>,
    #[doc = "Capture and Compare Status"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "Array of registers: CSC, CV"]
    pub CHANNEL: [channel::RegisterBlock; 4usize],
    _reserved1: [u8; 0x24],
    #[doc = "Combine Channel"]
    pub COMBINE: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Channel Trigger"]
    pub TRIG: crate::RWRegister<u32>,
    #[doc = "Channel Polarity"]
    pub POL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Filter Control"]
    pub FILTER: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Quadrature Decoder Control and Status"]
    pub QDCTRL: crate::RWRegister<u32>,
    #[doc = "Configuration"]
    pub CONF: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Identification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard feature set"]
            pub const STANDARD: u32 = 0x01;
            #[doc = "Standard feature set with the filter and combine registers implemented"]
            pub const FILT_COMBINE: u32 = 0x03;
            #[doc = "Standard feature set with the quadrature register implemented"]
            pub const QUAD: u32 = 0x05;
            #[doc = "Standard feature set with the filter, combine, and quadrature registers implemented"]
            pub const FILT_COMBINE_QUAD: u32 = 0x07;
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
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "Channel Count"]
    pub mod CHAN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Count"]
    pub mod TRIG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counter Width"]
    pub mod WIDTH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TPM Global"]
pub mod GLOBAL {
    #[doc = "No Update"]
    pub mod NOUPDATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal double-buffered registers update as normal"]
            pub const UPDATE: u32 = 0;
            #[doc = "Internal double-buffered registers do not update"]
            pub const NOUPDATE: u32 = 0x01;
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
            pub const NOT_RESET: u32 = 0;
            #[doc = "Module is reset"]
            pub const RESET: u32 = 0x01;
        }
    }
}
#[doc = "Status and Control"]
pub mod SC {
    #[doc = "Prescale Factor Selection"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIV_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIV_2: u32 = 0x01;
            #[doc = "Divide by 4"]
            pub const DIV_4: u32 = 0x02;
            #[doc = "Divide by 8"]
            pub const DIV_8: u32 = 0x03;
            #[doc = "Divide by 16"]
            pub const DIV_16: u32 = 0x04;
            #[doc = "Divide by 32"]
            pub const DIV_32: u32 = 0x05;
            #[doc = "Divide by 64"]
            pub const DIV_64: u32 = 0x06;
            #[doc = "Divide by 128"]
            pub const DIV_128: u32 = 0x07;
        }
    }
    #[doc = "Clock Mode Selection"]
    pub mod CMOD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM counter is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "TPM counter increments on every TPM counter clock"]
            pub const COUNTER: u32 = 0x01;
            #[doc = "TPM counter increments on the rising edge of EXTCLK synchronized to the TPM counter clock"]
            pub const EXTCLK: u32 = 0x02;
            #[doc = "TPM counter increments on the rising edge of the selected external input trigger"]
            pub const TRIG: u32 = 0x03;
        }
    }
    #[doc = "Center-Aligned PWM Select"]
    pub mod CPWMS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Up counting mode"]
            pub const UP: u32 = 0;
            #[doc = "Up-down counting mode"]
            pub const UP_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Timer Overflow Interrupt Enable"]
    pub mod TOIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Timer Overflow Flag"]
    pub mod TOF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow"]
            pub const OVERFLOW: u32 = 0x01;
        }
    }
    #[doc = "DMA Enable"]
    pub mod DMA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Counter"]
pub mod CNT {
    #[doc = "Counter Value"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Modulo"]
pub mod MOD {
    #[doc = "Modulo Value"]
    pub mod MOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture and Compare Status"]
pub mod STATUS {
    #[doc = "Channel 0 Flag"]
    pub mod CH0F {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not occurred"]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Event occurred"]
            pub const EVENT: u32 = 0x01;
        }
    }
    #[doc = "Channel 1 Flag"]
    pub mod CH1F {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not occurred"]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Event occurred"]
            pub const EVENT: u32 = 0x01;
        }
    }
    #[doc = "Channel 2 Flag"]
    pub mod CH2F {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not occurred"]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Event occurred"]
            pub const EVENT: u32 = 0x01;
        }
    }
    #[doc = "Channel 3 Flag"]
    pub mod CH3F {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not occurred"]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Event occurred"]
            pub const EVENT: u32 = 0x01;
        }
    }
    #[doc = "Timer Overflow Flag"]
    pub mod TOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow"]
            pub const OVERFLOW: u32 = 0x01;
        }
    }
}
#[doc = "Combine Channel"]
pub mod COMBINE {
    #[doc = "Combine Channels 0 and 1"]
    pub mod COMBINE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Independent"]
            pub const NO_COMBINE: u32 = 0;
            #[doc = "Combined"]
            pub const COMBINE: u32 = 0x01;
        }
    }
    #[doc = "Combine Channel 0 and 1 Swap"]
    pub mod COMSWAP0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Even channel"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Odd channel"]
            pub const SWAP: u32 = 0x01;
        }
    }
    #[doc = "Combine Channels 2 and 3"]
    pub mod COMBINE1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Independent"]
            pub const NO_COMBINE: u32 = 0;
            #[doc = "Combined"]
            pub const COMBINE: u32 = 0x01;
        }
    }
    #[doc = "Combine Channels 2 and 3 Swap"]
    pub mod COMSWAP1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Even channel"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Odd channel"]
            pub const SWAP: u32 = 0x01;
        }
    }
}
#[doc = "Channel Trigger"]
pub mod TRIG {
    #[doc = "Channel 0 Trigger"]
    pub mod TRIG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Configures trigger input 0 to be used by channel 0"]
            pub const USE_TRIG: u32 = 0x01;
        }
    }
    #[doc = "Channel 1 Trigger"]
    pub mod TRIG1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Configures trigger input 1 to be used by channel 1"]
            pub const USE_TRIG: u32 = 0x01;
        }
    }
    #[doc = "Channel 2 Trigger"]
    pub mod TRIG2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Configures trigger input 0 to be used by channel 2"]
            pub const USE_TRIG: u32 = 0x01;
        }
    }
    #[doc = "Channel 3 Trigger"]
    pub mod TRIG3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Configures trigger input 1 to be used by channel 3"]
            pub const USE_TRIG: u32 = 0x01;
        }
    }
}
#[doc = "Channel Polarity"]
pub mod POL {
    #[doc = "Channel 0 Polarity"]
    pub mod POL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active high"]
            pub const HIGH: u32 = 0;
            #[doc = "Active low"]
            pub const LOW: u32 = 0x01;
        }
    }
    #[doc = "Channel 1 Polarity"]
    pub mod POL1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active high"]
            pub const HIGH: u32 = 0;
            #[doc = "Active low"]
            pub const LOW: u32 = 0x01;
        }
    }
    #[doc = "Channel 2 Polarity"]
    pub mod POL2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active high"]
            pub const HIGH: u32 = 0;
            #[doc = "Active low"]
            pub const LOW: u32 = 0x01;
        }
    }
    #[doc = "Channel 3 Polarity"]
    pub mod POL3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active high"]
            pub const HIGH: u32 = 0;
            #[doc = "Active low"]
            pub const LOW: u32 = 0x01;
        }
    }
}
#[doc = "Filter Control"]
pub mod FILTER {
    #[doc = "Channel 0 Filter Value"]
    pub mod CH0FVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 1 Filter Value"]
    pub mod CH1FVAL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 2 Filter Value"]
    pub mod CH2FVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 3 Filter Value"]
    pub mod CH3FVAL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Quadrature Decoder Control and Status"]
pub mod QDCTRL {
    #[doc = "Quadrature Decoder Enable"]
    pub mod QUADEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Timer Overflow Direction"]
    pub mod TOFDIR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bottom of counting"]
            pub const BOTTOM: u32 = 0;
            #[doc = "Top of counting"]
            pub const TOP: u32 = 0x01;
        }
    }
    #[doc = "Counter Direction in Quadrature Decode Mode"]
    pub mod QUADIR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Decreasing (counter decrement)"]
            pub const DOWN: u32 = 0;
            #[doc = "Increasing (counter increment)"]
            pub const UP: u32 = 0x01;
        }
    }
    #[doc = "Quadrature Decoder Mode"]
    pub mod QUADMODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Phase encoding mode"]
            pub const PHASE: u32 = 0;
            #[doc = "Count and direction encoding mode"]
            pub const COUNT_DIR: u32 = 0x01;
        }
    }
}
#[doc = "Configuration"]
pub mod CONF {
    #[doc = "Doze Enable"]
    pub mod DOZEEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM counter continues"]
            pub const COUNT: u32 = 0;
            #[doc = "TPM counter pauses"]
            pub const NO_COUNT: u32 = 0x01;
        }
    }
    #[doc = "Debug Mode"]
    pub mod DBGMODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM counter pauses"]
            pub const NO_COUNT: u32 = 0;
            #[doc = "TPM counter continues"]
            pub const COUNT: u32 = 0x03;
        }
    }
    #[doc = "GTB Synchronization"]
    pub mod GTBSYNC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GTB Enable"]
    pub mod GTBEEN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internally generated TPM counter"]
            pub const DISABLE: u32 = 0;
            #[doc = "Externally generated GTB counter"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Counter Start on Trigger"]
    pub mod CSOT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter starts immediately"]
            pub const NO: u32 = 0;
            #[doc = "Counter starts after detection of a rising edge on the selected input trigger"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Counter Stop on Overflow"]
    pub mod CSOO {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM counter continues"]
            pub const NO: u32 = 0;
            #[doc = "TPM counter stops"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Counter Reload on Trigger"]
    pub mod CROT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reload"]
            pub const NO: u32 = 0;
            #[doc = "Reload"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Counter Pause on Trigger"]
    pub mod CPOT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TPM counter continues"]
            pub const NO: u32 = 0;
            #[doc = "TPM counter pauses"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Trigger Polarity"]
    pub mod TRGPOL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active high"]
            pub const HIGH: u32 = 0;
            #[doc = "Active low"]
            pub const LOW: u32 = 0x01;
        }
    }
    #[doc = "Trigger Source"]
    pub mod TRGSRC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External"]
            pub const EXTERNAL: u32 = 0;
            #[doc = "Internal (channel pin input capture)"]
            pub const INTERNAL: u32 = 0x01;
        }
    }
    #[doc = "Trigger Select"]
    pub mod TRGSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel 0 pin input capture"]
            pub const CH_0: u32 = 0x01;
            #[doc = "Channel 1 pin input capture"]
            pub const CH_1: u32 = 0x02;
            #[doc = "Channel 0 or channel 1 pin input capture"]
            pub const CH_0_1: u32 = 0x03;
        }
    }
}
pub mod channel {
    #[doc = "Array of registers: CSC, CV"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Channel n Status and Control"]
        pub CSC: crate::RWRegister<u32>,
        #[doc = "Channel n Value"]
        pub CV: crate::RWRegister<u32>,
    }
    #[doc = "Channel n Status and Control"]
    pub mod CSC {
        #[doc = "DMA Enable"]
        pub mod DMA {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Edge or Level Select A"]
        pub mod ELSA {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Edge or Level Select B"]
        pub mod ELSB {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Channel Mode Select A"]
        pub mod MSA {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Channel Mode Select B"]
        pub mod MSB {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Channel Interrupt Enable"]
        pub mod CHIE {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Channel Flag"]
        pub mod CHF {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Event not occurred"]
                pub const NO_EVENT: u32 = 0;
                #[doc = "Event occurred"]
                pub const EVENT: u32 = 0x01;
            }
        }
    }
    #[doc = "Channel n Value"]
    pub mod CV {
        #[doc = "Channel Value"]
        pub mod VAL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
