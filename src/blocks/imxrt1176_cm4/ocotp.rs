#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "OTP Controller Control and Status Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "OTP Controller Control and Status Register"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "OTP Controller Control and Status Register"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "OTP Controller Control and Status Register"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "OTP Controller PDN Register"]
    pub PDN: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "OTP Controller Write Data Register"]
    pub DATA: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "OTP Controller Read Control Register"]
    pub READ_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x5c],
    #[doc = "8K OTP Memory STATUS Register"]
    pub OUT_STATUS: crate::RWRegister<u32>,
    #[doc = "8K OTP Memory STATUS Register"]
    pub OUT_STATUS_SET: crate::RWRegister<u32>,
    #[doc = "8K OTP Memory STATUS Register"]
    pub OUT_STATUS_CLR: crate::RWRegister<u32>,
    #[doc = "8K OTP Memory STATUS Register"]
    pub OUT_STATUS_TOG: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "OTP Controller Version Register"]
    pub VERSION: crate::RORegister<u32>,
    _reserved4: [u8; 0x4c],
    #[doc = "OTP Controller Read Data 0 Register"]
    pub READ_FUSE_DATA0: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "OTP Controller Read Data 1 Register"]
    pub READ_FUSE_DATA1: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "OTP Controller Read Data 2 Register"]
    pub READ_FUSE_DATA2: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "OTP Controller Read Data 3 Register"]
    pub READ_FUSE_DATA3: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "SW_LOCK Register"]
    pub SW_LOCK: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "BIT_LOCK Register"]
    pub BIT_LOCK: crate::RWRegister<u32>,
    _reserved10: [u8; 0x04ac],
    #[doc = "OTP Controller Program Locked Status 0 Register"]
    pub LOCKED0: crate::RORegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "OTP Controller Program Locked Status 1 Register"]
    pub LOCKED1: crate::RORegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "OTP Controller Program Locked Status 2 Register"]
    pub LOCKED2: crate::RORegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "OTP Controller Program Locked Status 3 Register"]
    pub LOCKED3: crate::RORegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "OTP Controller Program Locked Status 4 Register"]
    pub LOCKED4: crate::RORegister<u32>,
    _reserved15: [u8; 0x01bc],
    #[doc = "no description available"]
    pub FUSEN: [fusen::RegisterBlock; 144usize],
}
#[doc = "OTP Controller Control and Status Register"]
pub mod CTRL {
    #[doc = "OTP write and read access address register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTP controller status bit"]
    pub mod BUSY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No write or read access to OTP started."]
            pub const NOT_BUSY: u32 = 0;
            #[doc = "Write or read access to OTP started."]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "Locked Region Access Error"]
    pub mod ERROR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error - access to a locked region requested."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Reload Shadow Registers"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not force shadow register re-load."]
            pub const SHADOW_NOFORCE_RELOAD: u32 = 0;
            #[doc = "Force shadow register re-load. This bit is cleared automatically after shadow registers are re-loaded."]
            pub const SHADOW_FORCE_RELOAD: u32 = 0x01;
        }
    }
    #[doc = "Lock fuse word"]
    pub mod WORDLOCK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change to LOCK bit when programming a word using redundancy"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "LOCK bit for fuse word will be set after successfully programming a word using redundancy"]
            pub const LOCK: u32 = 0x01;
        }
    }
    #[doc = "Write unlock"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OTP write access is locked."]
            pub const OTP_W_LOCKED: u32 = 0;
            #[doc = "OTP write access is unlocked."]
            pub const OTP_W_UNLOCKED: u32 = 0x3e77;
        }
    }
}
#[doc = "OTP Controller Control and Status Register"]
pub mod CTRL_SET {
    #[doc = "OTP write and read access address register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTP controller status bit"]
    pub mod BUSY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Locked Region Access Error"]
    pub mod ERROR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reload Shadow Registers"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock fuse word"]
    pub mod WORDLOCK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write unlock"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Control and Status Register"]
pub mod CTRL_CLR {
    #[doc = "OTP write and read access address register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTP controller status bit"]
    pub mod BUSY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Locked Region Access Error"]
    pub mod ERROR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reload Shadow Registers"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock fuse word"]
    pub mod WORDLOCK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write unlock"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Control and Status Register"]
pub mod CTRL_TOG {
    #[doc = "OTP write and read access address register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTP controller status bit"]
    pub mod BUSY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Locked Region Access Error"]
    pub mod ERROR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reload Shadow Registers"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock fuse word"]
    pub mod WORDLOCK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write unlock"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller PDN Register"]
pub mod PDN {
    #[doc = "PDN value"]
    pub mod PDN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OTP memory is not powered"]
            pub const POWER_OFF: u32 = 0;
            #[doc = "OTP memory is powered"]
            pub const POWER_ON: u32 = 0x01;
        }
    }
}
#[doc = "OTP Controller Write Data Register"]
pub mod DATA {
    #[doc = "DATA"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Read Control Register"]
pub mod READ_CTRL {
    #[doc = "Read Fuse"]
    pub mod READ_FUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not initiate a read from OTP"]
            pub const DO_NOT_START_RD_OP: u32 = 0;
            #[doc = "Initiate a read from OTP"]
            pub const START_RD_OP: u32 = 0x01;
        }
    }
    #[doc = "Number of words to read."]
    pub mod READ_FUSE_CNTR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 word"]
            pub const ONE_WORD: u32 = 0;
            #[doc = "2 words"]
            pub const TWO_WORDS: u32 = 0x01;
            #[doc = "3 words"]
            pub const THREE_WORDS: u32 = 0x02;
            #[doc = "4 words"]
            pub const FOUR_WORDS: u32 = 0x03;
        }
    }
    #[doc = "Enable read-done interrupt"]
    pub mod READ_FUSE_DONE_INTR_ENA {
        pub const offset: u32 = 3;
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
    #[doc = "Enable read-error interrupt"]
    pub mod READ_FUSE_ERROR_INTR_ENA {
        pub const offset: u32 = 4;
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
#[doc = "8K OTP Memory STATUS Register"]
pub mod OUT_STATUS {
    #[doc = "Single Error Correct"]
    pub mod SEC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word Locked"]
    pub mod LOCKED {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Programming failed"]
    pub mod PROGFAIL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Acknowledge"]
    pub mod ACK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power OK"]
    pub mod PWOK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flag state"]
    pub mod FLAGSTATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates single error correction occured on reload"]
    pub mod SEC_RELOAD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates double error detection occured on reload"]
    pub mod DED_RELOAD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Calibrated status"]
    pub mod CALIBRATED {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read fuse done"]
    pub mod READ_DONE_INTR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fuse read error"]
    pub mod READ_ERROR_INTR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read operation finished with out any error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Read operation finished with an error"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Double error detect"]
    pub mod DED0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "8K OTP Memory STATUS Register"]
pub mod OUT_STATUS_SET {
    #[doc = "Single Error Correct"]
    pub mod SEC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word Locked"]
    pub mod LOCKED {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Programming failed"]
    pub mod PROGFAIL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Acknowledge"]
    pub mod ACK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power OK"]
    pub mod PWOK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flag state"]
    pub mod FLAGSTATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates single error correction occured on reload"]
    pub mod SEC_RELOAD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates double error detection occured on reload"]
    pub mod DED_RELOAD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Calibrated status"]
    pub mod CALIBRATED {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read fuse done"]
    pub mod READ_DONE_INTR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fuse read error"]
    pub mod READ_ERROR_INTR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "8K OTP Memory STATUS Register"]
pub mod OUT_STATUS_CLR {
    #[doc = "Single Error Correct"]
    pub mod SEC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word Locked"]
    pub mod LOCKED {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Programming failed"]
    pub mod PROGFAIL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Acknowledge"]
    pub mod ACK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power OK"]
    pub mod PWOK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flag state"]
    pub mod FLAGSTATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates single error correction occured on reload"]
    pub mod SEC_RELOAD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates double error detection occured on reload"]
    pub mod DED_RELOAD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Calibrated status"]
    pub mod CALIBRATED {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read fuse done"]
    pub mod READ_DONE_INTR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fuse read error"]
    pub mod READ_ERROR_INTR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "8K OTP Memory STATUS Register"]
pub mod OUT_STATUS_TOG {
    #[doc = "Single Error Correct"]
    pub mod SEC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word Locked"]
    pub mod LOCKED {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Programming failed"]
    pub mod PROGFAIL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Acknowledge"]
    pub mod ACK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power OK"]
    pub mod PWOK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flag state"]
    pub mod FLAGSTATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates single error correction occured on reload"]
    pub mod SEC_RELOAD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates double error detection occured on reload"]
    pub mod DED_RELOAD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Calibrated status"]
    pub mod CALIBRATED {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read fuse done"]
    pub mod READ_DONE_INTR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fuse read error"]
    pub mod READ_ERROR_INTR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double error detect"]
    pub mod DED3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Version Register"]
pub mod VERSION {
    #[doc = "STEP"]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MINOR"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAJOR"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Read Data 0 Register"]
pub mod READ_FUSE_DATA0 {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Read Data 1 Register"]
pub mod READ_FUSE_DATA1 {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Read Data 2 Register"]
pub mod READ_FUSE_DATA2 {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Read Data 3 Register"]
pub mod READ_FUSE_DATA3 {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SW_LOCK Register"]
pub mod SW_LOCK {
    #[doc = "This register contains lock information, which has the same function as the RLOCK fuse words (supplementary fuse words 8 (0x880) and 9 (0x890)) in fuse memory"]
    pub mod SW_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BIT_LOCK Register"]
pub mod BIT_LOCK {
    #[doc = "Each bit controls the corresponding bit in supplementary fuse word 13 and its shadow register"]
    pub mod BIT_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Program Locked Status 0 Register"]
pub mod LOCKED0 {
    #[doc = "Stores program locked status for fuse words 0-15."]
    pub mod LOCKED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Program Locked Status 1 Register"]
pub mod LOCKED1 {
    #[doc = "Stores program locked status for fuse words 16-47"]
    pub mod LOCKED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Program Locked Status 2 Register"]
pub mod LOCKED2 {
    #[doc = "Stores program locked status for fuse words 48-79"]
    pub mod LOCKED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Program Locked Status 3 Register"]
pub mod LOCKED3 {
    #[doc = "Stores program locked status for fuse words 80-111"]
    pub mod LOCKED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTP Controller Program Locked Status 4 Register"]
pub mod LOCKED4 {
    #[doc = "Stores program locked status for fuse words 112-143"]
    pub mod LOCKED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod fusen {
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Value of fuse word index"]
        pub FUSE: crate::RORegister<u32>,
        _reserved0: [u8; 0x0c],
    }
    #[doc = "Value of fuse word index"]
    pub mod FUSE {
        #[doc = "Reflects value of the fuse word"]
        pub mod BITS {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
