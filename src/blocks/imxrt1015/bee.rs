#[doc = "Bus Encryption Engine"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "BEE Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub ADDR_OFFSET0: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub ADDR_OFFSET1: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub AES_KEY0_W0: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub AES_KEY0_W1: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub AES_KEY0_W2: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub AES_KEY0_W3: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub CTR_NONCE0_W0: crate::WORegister<u32>,
    #[doc = "no description available"]
    pub CTR_NONCE0_W1: crate::WORegister<u32>,
    #[doc = "no description available"]
    pub CTR_NONCE0_W2: crate::WORegister<u32>,
    #[doc = "no description available"]
    pub CTR_NONCE0_W3: crate::WORegister<u32>,
    #[doc = "no description available"]
    pub CTR_NONCE1_W0: crate::WORegister<u32>,
    #[doc = "no description available"]
    pub CTR_NONCE1_W1: crate::WORegister<u32>,
    #[doc = "no description available"]
    pub CTR_NONCE1_W2: crate::WORegister<u32>,
    #[doc = "no description available"]
    pub CTR_NONCE1_W3: crate::WORegister<u32>,
    #[doc = "no description available"]
    pub REGION1_TOP: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub REGION1_BOT: crate::RWRegister<u32>,
}
#[doc = "BEE Control Register"]
pub mod CTRL {
    #[doc = "BEE enable bit"]
    pub mod BEE_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable BEE"]
            pub const BEE_ENABLE_0: u32 = 0;
            #[doc = "Enable BEE"]
            pub const BEE_ENABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Clock enable input, low inactive"]
    pub mod CTRL_CLK_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Soft reset input, low active"]
    pub mod CTRL_SFTRST_N {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES-128 key is ready"]
    pub mod KEY_VALID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES key region select"]
    pub mod KEY_REGION_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Load AES key for region0"]
            pub const KEY_REGION_SEL_0: u32 = 0;
            #[doc = "Load AES key for region1"]
            pub const KEY_REGION_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
    pub mod AC_PROT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Endian swap control for the 16 bytes input and output data of AES core."]
    pub mod LITTLE_ENDIAN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
            pub const LITTLE_ENDIAN_0: u32 = 0;
            #[doc = "The input and output data of AES core is not swapped."]
            pub const LITTLE_ENDIAN_1: u32 = 0x01;
        }
    }
    #[doc = "Security level of the allowed access for memory region0"]
    pub mod SECURITY_LEVEL_R0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES mode of region0"]
    pub mod CTRL_AES_MODE_R0 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECB"]
            pub const CTRL_AES_MODE_R0_0: u32 = 0;
            #[doc = "CTR"]
            pub const CTRL_AES_MODE_R0_1: u32 = 0x01;
        }
    }
    #[doc = "Security level of the allowed access for memory region1"]
    pub mod SECURITY_LEVEL_R1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES mode of region1"]
    pub mod CTRL_AES_MODE_R1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECB"]
            pub const CTRL_AES_MODE_R1_0: u32 = 0;
            #[doc = "CTR"]
            pub const CTRL_AES_MODE_R1_1: u32 = 0x01;
        }
    }
    #[doc = "Lock bit for bee_enable"]
    pub mod BEE_ENABLE_LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for ctrl_clk_en"]
    pub mod CTRL_CLK_EN_LOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for ctrl_sftrst"]
    pub mod CTRL_SFTRST_N_LOCK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for region1 address boundary"]
    pub mod REGION1_ADDR_LOCK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for key_valid"]
    pub mod KEY_VALID_LOCK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for key_region_sel"]
    pub mod KEY_REGION_SEL_LOCK {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for ac_prot"]
    pub mod AC_PROT_EN_LOCK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for little_endian"]
    pub mod LITTLE_ENDIAN_LOCK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bits for security_level_r0"]
    pub mod SECURITY_LEVEL_R0_LOCK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for region0 ctrl_aes_mode"]
    pub mod CTRL_AES_MODE_R0_LOCK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for region0 AES key"]
    pub mod REGION0_KEY_LOCK {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bits for security_level_r1"]
    pub mod SECURITY_LEVEL_R1_LOCK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for region1 ctrl_aes_mode"]
    pub mod CTRL_AES_MODE_R1_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for region1 AES key"]
    pub mod REGION1_KEY_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod ADDR_OFFSET0 {
    #[doc = "Signed offset for BEE region 0"]
    pub mod ADDR_OFFSET0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bits for addr_offset0"]
    pub mod ADDR_OFFSET0_LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod ADDR_OFFSET1 {
    #[doc = "Signed offset for BEE region 1"]
    pub mod ADDR_OFFSET1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bits for addr_offset1"]
    pub mod ADDR_OFFSET1_LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod AES_KEY0_W0 {
    #[doc = "AES 128 key from software"]
    pub mod KEY0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod AES_KEY0_W1 {
    #[doc = "AES 128 key from software"]
    pub mod KEY1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod AES_KEY0_W2 {
    #[doc = "AES 128 key from software"]
    pub mod KEY2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod AES_KEY0_W3 {
    #[doc = "AES 128 key from software"]
    pub mod KEY3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod STATUS {
    #[doc = "bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
    pub mod IRQ_VEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'b1: BEE is idle; 1'b0: BEE is active"]
    pub mod BEE_IDLE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod CTR_NONCE0_W0 {
    #[doc = "Nonce0 from software for CTR, for region0. Nonce0={Nonce03,Nonce02,Nonce01,Nonce00}"]
    pub mod NONCE00 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod CTR_NONCE0_W1 {
    #[doc = "Nonce0 from software for CTR, for region0. Nonce0={Nonce03,Nonce02,Nonce01,Nonce00}"]
    pub mod NONCE01 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod CTR_NONCE0_W2 {
    #[doc = "Nonce0 from software for CTR, for region0. Nonce0={Nonce03,Nonce02,Nonce01,Nonce00}"]
    pub mod NONCE02 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod CTR_NONCE0_W3 {
    #[doc = "Nonce0 from software for CTR, for region0. Nonce0={Nonce03,Nonce02,Nonce01,Nonce00}"]
    pub mod NONCE03 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod CTR_NONCE1_W0 {
    #[doc = "Nonce1 from software for CTR, for region1. Nonce1={Nonce13,Nonce12,Nonce11,Nonce10}"]
    pub mod NONCE10 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod CTR_NONCE1_W1 {
    #[doc = "Nonce1 from software for CTR, for region1. Nonce1={Nonce13,Nonce12,Nonce11,Nonce10}"]
    pub mod NONCE11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod CTR_NONCE1_W2 {
    #[doc = "Nonce1 from software for CTR, for region1. Nonce1={Nonce13,Nonce12,Nonce11,Nonce10}"]
    pub mod NONCE12 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod CTR_NONCE1_W3 {
    #[doc = "Nonce1 from software for CTR, for region1. Nonce1={Nonce13,Nonce12,Nonce11,Nonce10}"]
    pub mod NONCE13 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod REGION1_TOP {
    #[doc = "Address upper limit of region1"]
    pub mod REGION1_TOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod REGION1_BOT {
    #[doc = "Address lower limit of region1"]
    pub mod REGION1_BOT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
