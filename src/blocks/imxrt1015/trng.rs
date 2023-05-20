#[doc = "TRNG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Miscellaneous Control Register"]
    pub MCTL: crate::RWRegister<u32>,
    #[doc = "Statistical Check Miscellaneous Register"]
    pub SCMISC: crate::RWRegister<u32>,
    #[doc = "Poker Range Register"]
    pub PKRRNG: crate::RWRegister<u32>,
    #[doc = "Poker Maximum Limit Register"]
    pub PKRMAX: crate::RWRegister<u32>,
    #[doc = "Seed Control Register"]
    pub SDCTL: crate::RWRegister<u32>,
    #[doc = "Sparse Bit Limit Register"]
    pub SBLIM: crate::RWRegister<u32>,
    #[doc = "Frequency Count Minimum Limit Register"]
    pub FRQMIN: crate::RWRegister<u32>,
    #[doc = "Frequency Count Maximum Limit Register"]
    pub FRQMAX: crate::RWRegister<u32>,
    #[doc = "Statistical Check Monobit Limit Register"]
    pub SCML: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 1 Limit Register"]
    pub SCR1L: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 2 Limit Register"]
    pub SCR2L: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 3 Limit Register"]
    pub SCR3L: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 4 Limit Register"]
    pub SCR4L: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 5 Limit Register"]
    pub SCR5L: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 6+ Limit Register"]
    pub SCR6PL: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STATUS: crate::RORegister<u32>,
    #[doc = "Entropy Read Register"]
    pub ENT: [crate::RORegister<u32>; 16usize],
    #[doc = "Statistical Check Poker Count 1 and 0 Register"]
    pub PKRCNT10: crate::RORegister<u32>,
    #[doc = "Statistical Check Poker Count 3 and 2 Register"]
    pub PKRCNT32: crate::RORegister<u32>,
    #[doc = "Statistical Check Poker Count 5 and 4 Register"]
    pub PKRCNT54: crate::RORegister<u32>,
    #[doc = "Statistical Check Poker Count 7 and 6 Register"]
    pub PKRCNT76: crate::RORegister<u32>,
    #[doc = "Statistical Check Poker Count 9 and 8 Register"]
    pub PKRCNT98: crate::RORegister<u32>,
    #[doc = "Statistical Check Poker Count B and A Register"]
    pub PKRCNTBA: crate::RORegister<u32>,
    #[doc = "Statistical Check Poker Count D and C Register"]
    pub PKRCNTDC: crate::RORegister<u32>,
    #[doc = "Statistical Check Poker Count F and E Register"]
    pub PKRCNTFE: crate::RORegister<u32>,
    #[doc = "Security Configuration Register"]
    pub SEC_CFG: crate::RWRegister<u32>,
    #[doc = "Interrupt Control Register"]
    pub INT_CTRL: crate::RWRegister<u32>,
    #[doc = "Mask Register"]
    pub INT_MASK: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Register"]
    pub INT_STATUS: crate::RORegister<u32>,
    _reserved0: [u8; 0x40],
    #[doc = "Version ID Register (MS)"]
    pub VID1: crate::RORegister<u32>,
    #[doc = "Version ID Register (LS)"]
    pub VID2: crate::RORegister<u32>,
}
#[doc = "Miscellaneous Control Register"]
pub mod MCTL {
    #[doc = "Sample Mode"]
    pub mod SAMP_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
            pub const SAMP_MODE_0: u32 = 0;
            #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
            pub const SAMP_MODE_1: u32 = 0x01;
            #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
            pub const SAMP_MODE_2: u32 = 0x02;
            #[doc = "undefined/reserved."]
            pub const SAMP_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "Oscillator Divide"]
    pub mod OSC_DIV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "use ring oscillator with no divide"]
            pub const OSC_DIV_0: u32 = 0;
            #[doc = "use ring oscillator divided-by-2"]
            pub const OSC_DIV_1: u32 = 0x01;
            #[doc = "use ring oscillator divided-by-4"]
            pub const OSC_DIV_2: u32 = 0x02;
            #[doc = "use ring oscillator divided-by-8"]
            pub const OSC_DIV_3: u32 = 0x03;
        }
    }
    #[doc = "This bit is unused. Always reads zero."]
    pub mod UNUSED4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRNG Access Mode"]
    pub mod TRNG_ACC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Defaults"]
    pub mod RST_DEF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force System Clock"]
    pub mod FOR_SCLK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read only: Frequency Count Fail"]
    pub mod FCT_FAIL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
    pub mod FCT_VAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read only: Entropy Valid"]
    pub mod ENT_VAL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read only: Test point inside ring oscillator."]
    pub mod TST_OUT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read: Error status"]
    pub mod ERR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRNG_OK_TO_STOP"]
    pub mod TSTOP_OK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Long run count continues between entropy generations"]
    pub mod LRUN_CONT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Programming Mode Select"]
    pub mod PRGM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Miscellaneous Register"]
pub mod SCMISC {
    #[doc = "LONG RUN MAX LIMIT"]
    pub mod LRUN_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RETRY COUNT"]
    pub mod RTY_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Poker Range Register"]
pub mod PKRRNG {
    #[doc = "Poker Range"]
    pub mod PKR_RNG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Poker Maximum Limit Register"]
pub mod PKRMAX {
    #[doc = "Poker Maximum Limit."]
    pub mod PKR_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Seed Control Register"]
pub mod SDCTL {
    #[doc = "Sample Size"]
    pub mod SAMP_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Entropy Delay"]
    pub mod ENT_DLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Sparse Bit Limit Register"]
pub mod SBLIM {
    #[doc = "Sparse Bit Limit"]
    pub mod SB_LIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frequency Count Minimum Limit Register"]
pub mod FRQMIN {
    #[doc = "Frequency Count Minimum Limit"]
    pub mod FRQ_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frequency Count Maximum Limit Register"]
pub mod FRQMAX {
    #[doc = "Frequency Counter Maximum Limit"]
    pub mod FRQ_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Monobit Limit Register"]
pub mod SCML {
    #[doc = "Monobit Maximum Limit"]
    pub mod MONO_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Monobit Range"]
    pub mod MONO_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 1 Limit Register"]
pub mod SCR1L {
    #[doc = "Run Length 1 Maximum Limit"]
    pub mod RUN1_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 1 Range"]
    pub mod RUN1_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 2 Limit Register"]
pub mod SCR2L {
    #[doc = "Run Length 2 Maximum Limit"]
    pub mod RUN2_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 2 Range"]
    pub mod RUN2_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 3 Limit Register"]
pub mod SCR3L {
    #[doc = "Run Length 3 Maximum Limit"]
    pub mod RUN3_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 3 Range"]
    pub mod RUN3_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 4 Limit Register"]
pub mod SCR4L {
    #[doc = "Run Length 4 Maximum Limit"]
    pub mod RUN4_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 4 Range"]
    pub mod RUN4_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 5 Limit Register"]
pub mod SCR5L {
    #[doc = "Run Length 5 Maximum Limit"]
    pub mod RUN5_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 5 Range"]
    pub mod RUN5_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 6+ Limit Register"]
pub mod SCR6PL {
    #[doc = "Run Length 6+ Maximum Limit"]
    pub mod RUN6P_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 6+ Range"]
    pub mod RUN6P_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STATUS {
    #[doc = "Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
    pub mod TF1BR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
    pub mod TF1BR1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
    pub mod TF2BR0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
    pub mod TF2BR1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
    pub mod TF3BR0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
    pub mod TF3BR1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
    pub mod TF4BR0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
    pub mod TF4BR1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
    pub mod TF5BR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
    pub mod TF5BR1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 6 Plus Bit Run, Sampling 0s"]
    pub mod TF6PBR0 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 6 Plus Bit Run, Sampling 1s"]
    pub mod TF6PBR1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
    pub mod TFSB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
    pub mod TFLR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, Poker. If TFP=1, the Poker Test has failed."]
    pub mod TFP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
    pub mod TFMB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RETRY COUNT"]
    pub mod RETRY_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Entropy Read Register"]
pub mod ENT {
    #[doc = "Entropy Value"]
    pub mod ENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Poker Count 1 and 0 Register"]
pub mod PKRCNT10 {
    #[doc = "Poker 0h Count"]
    pub mod PKR_0_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Poker 1h Count"]
    pub mod PKR_1_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Poker Count 3 and 2 Register"]
pub mod PKRCNT32 {
    #[doc = "Poker 2h Count"]
    pub mod PKR_2_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Poker 3h Count"]
    pub mod PKR_3_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Poker Count 5 and 4 Register"]
pub mod PKRCNT54 {
    #[doc = "Poker 4h Count"]
    pub mod PKR_4_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Poker 5h Count"]
    pub mod PKR_5_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Poker Count 7 and 6 Register"]
pub mod PKRCNT76 {
    #[doc = "Poker 6h Count"]
    pub mod PKR_6_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Poker 7h Count"]
    pub mod PKR_7_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Poker Count 9 and 8 Register"]
pub mod PKRCNT98 {
    #[doc = "Poker 8h Count"]
    pub mod PKR_8_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Poker 9h Count"]
    pub mod PKR_9_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Poker Count B and A Register"]
pub mod PKRCNTBA {
    #[doc = "Poker Ah Count"]
    pub mod PKR_A_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Poker Bh Count"]
    pub mod PKR_B_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Poker Count D and C Register"]
pub mod PKRCNTDC {
    #[doc = "Poker Ch Count"]
    pub mod PKR_C_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Poker Dh Count"]
    pub mod PKR_D_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Poker Count F and E Register"]
pub mod PKRCNTFE {
    #[doc = "Poker Eh Count"]
    pub mod PKR_E_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Poker Fh Count"]
    pub mod PKR_F_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Security Configuration Register"]
pub mod SEC_CFG {
    #[doc = "This bit is unused. Ignore."]
    pub mod UNUSED0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set, the TRNG registers cannot be programmed"]
    pub mod NO_PRGM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
            pub const NO_PRGM_0: u32 = 0;
            #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
            pub const NO_PRGM_1: u32 = 0x01;
        }
    }
    #[doc = "This bit is unused. Ignore."]
    pub mod UNUSED2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Control Register"]
pub mod INT_CTRL {
    #[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS register has been asserted."]
    pub mod HW_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding bit of INT_STATUS register cleared."]
            pub const HW_ERR_0: u32 = 0;
            #[doc = "Corresponding bit of INT_STATUS register active."]
            pub const HW_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "Same behavior as bit 0 of this register."]
    pub mod ENT_VAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Same behavior as bit 0 of this register."]
            pub const ENT_VAL_0: u32 = 0;
            #[doc = "Same behavior as bit 0 of this register."]
            pub const ENT_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Same behavior as bit 0 of this register."]
    pub mod FRQ_CT_FAIL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Same behavior as bit 0 of this register."]
            pub const FRQ_CT_FAIL_0: u32 = 0;
            #[doc = "Same behavior as bit 0 of this register."]
            pub const FRQ_CT_FAIL_1: u32 = 0x01;
        }
    }
}
#[doc = "Mask Register"]
pub mod INT_MASK {
    #[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    pub mod HW_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding interrupt of INT_STATUS is masked."]
            pub const HW_ERR_0: u32 = 0;
            #[doc = "Corresponding bit of INT_STATUS is active."]
            pub const HW_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "Same behavior as bit 0 of this register."]
    pub mod ENT_VAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Same behavior as bit 0 of this register."]
            pub const ENT_VAL_0: u32 = 0;
            #[doc = "Same behavior as bit 0 of this register."]
            pub const ENT_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Same behavior as bit 0 of this register."]
    pub mod FRQ_CT_FAIL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Same behavior as bit 0 of this register."]
            pub const FRQ_CT_FAIL_0: u32 = 0;
            #[doc = "Same behavior as bit 0 of this register."]
            pub const FRQ_CT_FAIL_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Status Register"]
pub mod INT_STATUS {
    #[doc = "Read: Error status"]
    pub mod HW_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no error"]
            pub const HW_ERR_0: u32 = 0;
            #[doc = "error detected."]
            pub const HW_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "Read only: Entropy Valid"]
    pub mod ENT_VAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Busy generation entropy. Any value read is invalid."]
            pub const ENT_VAL_0: u32 = 0;
            #[doc = "TRNG can be stopped and entropy is valid if read."]
            pub const ENT_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Read only: Frequency Count Fail"]
    pub mod FRQ_CT_FAIL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No hardware nor self test frequency errors."]
            pub const FRQ_CT_FAIL_0: u32 = 0;
            #[doc = "The frequency counter has detected a failure."]
            pub const FRQ_CT_FAIL_1: u32 = 0x01;
        }
    }
}
#[doc = "Version ID Register (MS)"]
pub mod VID1 {
    #[doc = "Shows the IP's Minor revision of the TRNG."]
    pub mod MIN_REV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minor revision number for TRNG."]
            pub const MIN_REV_0: u32 = 0;
        }
    }
    #[doc = "Shows the IP's Major revision of the TRNG."]
    pub mod MAJ_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Major revision number for TRNG."]
            pub const MAJ_REV_1: u32 = 0x01;
        }
    }
    #[doc = "Shows the IP ID."]
    pub mod IP_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ID for TRNG."]
            pub const IP_ID_48: u32 = 0x30;
        }
    }
}
#[doc = "Version ID Register (LS)"]
pub mod VID2 {
    #[doc = "Shows the IP's Configuaration options for the TRNG."]
    pub mod CONFIG_OPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRNG_CONFIG_OPT for TRNG."]
            pub const CONFIG_OPT_0: u32 = 0;
        }
    }
    #[doc = "Shows the IP's ECO revision of the TRNG."]
    pub mod ECO_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRNG_ECO_REV for TRNG."]
            pub const ECO_REV_0: u32 = 0;
        }
    }
    #[doc = "Shows the integration options for the TRNG."]
    pub mod INTG_OPT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "INTG_OPT for TRNG."]
            pub const INTG_OPT_0: u32 = 0;
        }
    }
    #[doc = "Shows the compile options for the TRNG."]
    pub mod ERA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "COMPILE_OPT for TRNG."]
            pub const ERA_0: u32 = 0;
        }
    }
}
