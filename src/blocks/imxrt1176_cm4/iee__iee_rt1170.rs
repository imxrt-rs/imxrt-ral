#[doc = "IEE"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "IEE Global Configuration"]
    pub GCFG: crate::RWRegister<u32>,
    #[doc = "IEE Status"]
    pub STA: crate::RORegister<u32>,
    #[doc = "IEE Test Mode Register"]
    pub TSTMD: crate::RWRegister<u32>,
    #[doc = "AES Mask Generation Seed"]
    pub DPAMS: crate::WORegister<u32>,
    _reserved0: [u8; 0x10],
    #[doc = "Performance Counter, AES Slave Latency Threshold Value"]
    pub PC_S_LT: crate::RWRegister<u32>,
    #[doc = "Performance Counter, AES Master Latency Threshold"]
    pub PC_M_LT: crate::RWRegister<u32>,
    _reserved1: [u8; 0x18],
    #[doc = "Performance Counter, Number of AES Block Encryptions"]
    pub PC_BLK_ENC: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Number of AES Block Decryptions"]
    pub PC_BLK_DEC: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Performance Counter, Number of AXI Slave Read Transactions"]
    pub PC_SR_TRANS: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Number of AXI Slave Write Transactions"]
    pub PC_SW_TRANS: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Number of AXI Master Read Transactions"]
    pub PC_MR_TRANS: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Number of AXI Master Write Transactions"]
    pub PC_MW_TRANS: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Performance Counter, Number of AXI Master Merge Buffer Read Transactions"]
    pub PC_M_MBR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x08],
    #[doc = "Performance Counter, Upper Slave Read Transactions Byte Count"]
    pub PC_SR_TBC_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Slave Read Transactions Byte Count"]
    pub PC_SR_TBC_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Slave Write Transactions Byte Count"]
    pub PC_SW_TBC_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Slave Write Transactions Byte Count"]
    pub PC_SW_TBC_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Master Read Transactions Byte Count"]
    pub PC_MR_TBC_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Master Read Transactions Byte Count"]
    pub PC_MR_TBC_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Master Write Transactions Byte Count"]
    pub PC_MW_TBC_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Master Write Transactions Byte Count"]
    pub PC_MW_TBC_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Number of AXI Slave Read Transactions with Latency Greater than the Threshold"]
    pub PC_SR_TLGTT: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Number of AXI Slave Write Transactions with Latency Greater than the Threshold"]
    pub PC_SW_TLGTT: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Number of AXI Master Read Transactions with Latency Greater than the Threshold"]
    pub PC_MR_TLGTT: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Number of AXI Master Write Transactions with Latency Greater than the Threshold"]
    pub PC_MW_TLGTT: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Slave Read Latency Count"]
    pub PC_SR_TLAT_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Slave Read Latency Count"]
    pub PC_SR_TLAT_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Slave Write Latency Count"]
    pub PC_SW_TLAT_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Slave Write Latency Count"]
    pub PC_SW_TLAT_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Master Read Latency Count"]
    pub PC_MR_TLAT_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Master Read Latency Count"]
    pub PC_MR_TLAT_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Master Write Latency Count"]
    pub PC_MW_TLAT_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Master Write Latency Count"]
    pub PC_MW_TLAT_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Slave Read Total Non-Responding Time"]
    pub PC_SR_TNRT_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Slave Read Total Non-Responding Time"]
    pub PC_SR_TNRT_L: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Upper Slave Write Total Non-Responding Time"]
    pub PC_SW_TNRT_U: crate::RWRegister<u32>,
    #[doc = "Performance Counter, Lower Slave Write Total Non-Responding Time"]
    pub PC_SW_TNRT_L: crate::RWRegister<u32>,
    _reserved5: [u8; 0x20],
    #[doc = "IEE Version ID Register 1"]
    pub VIDR1: crate::RORegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "IEE AES Version ID Register"]
    pub AESVID: crate::RORegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "Region Registers"]
    pub REGX: [regx::RegisterBlock; 8usize],
    _reserved8: [u8; 0x0600],
    #[doc = "IEE AES Test Mode Data Buffer"]
    pub AES_TST_DB: [crate::RWRegister<u32>; 32usize],
}
#[doc = "IEE Global Configuration"]
pub mod GCFG {
    #[doc = "Region lock 0 bit"]
    pub mod RL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked."]
            pub const RL0_0: u32 = 0;
            #[doc = "Key, Offset and Attribute registers are locked."]
            pub const RL0_1: u32 = 0x01;
        }
    }
    #[doc = "Region lock 1 bit"]
    pub mod RL1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked."]
            pub const RL1_0: u32 = 0;
            #[doc = "Key, Offset and Attribute registers are locked."]
            pub const RL1_1: u32 = 0x01;
        }
    }
    #[doc = "Region lock 2 bit"]
    pub mod RL2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked."]
            pub const RL2_0: u32 = 0;
            #[doc = "Key, Offset and Attribute registers are locked."]
            pub const RL2_1: u32 = 0x01;
        }
    }
    #[doc = "Region lock 3 bit"]
    pub mod RL3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked."]
            pub const RL3_0: u32 = 0;
            #[doc = "Key, Offset and Attribute registers are locked."]
            pub const RL3_1: u32 = 0x01;
        }
    }
    #[doc = "Region lock 4 bit"]
    pub mod RL4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked."]
            pub const RL4_0: u32 = 0;
            #[doc = "Key, Offset and Attribute registers are locked."]
            pub const RL4_1: u32 = 0x01;
        }
    }
    #[doc = "Region lock 5 bit"]
    pub mod RL5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked."]
            pub const RL5_0: u32 = 0;
            #[doc = "Key, Offset and Attribute registers are locked."]
            pub const RL5_1: u32 = 0x01;
        }
    }
    #[doc = "Region lock 6 bit"]
    pub mod RL6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked."]
            pub const RL6_0: u32 = 0;
            #[doc = "Key, Offset and Attribute registers are locked."]
            pub const RL6_1: u32 = 0x01;
        }
    }
    #[doc = "Region lock 7 bit"]
    pub mod RL7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlocked."]
            pub const RL7_0: u32 = 0;
            #[doc = "Key, Offset and Attribute registers are locked."]
            pub const RL7_1: u32 = 0x01;
        }
    }
    #[doc = "Test mode enable bit"]
    pub mod TME {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const TME_0: u32 = 0;
            #[doc = "Enabled."]
            pub const TME_1: u32 = 0x01;
        }
    }
    #[doc = "Test mode disable bit"]
    pub mod TMD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Test mode is usable."]
            pub const TMD_0: u32 = 0;
            #[doc = "Test mode is disabled."]
            pub const TMD_1: u32 = 0x01;
        }
    }
    #[doc = "Key read disable bit"]
    pub mod KEY_RD_DIS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key read enabled. Reading the key registers is allowed."]
            pub const KEY_RD_DIS_0: u32 = 0;
            #[doc = "Key read disabled. Reading the key registers is disabled."]
            pub const KEY_RD_DIS_1: u32 = 0x01;
        }
    }
    #[doc = "Monitor enable bit"]
    pub mod MON_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Performance monitoring disabled. Writing of the performance counter registers is enabled."]
            pub const MON_EN_0: u32 = 0;
            #[doc = "Performance monitoring enabled. Writing of the performance counter registers is disabled."]
            pub const MON_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Clear monitor bit"]
    pub mod CLR_MON {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not reset."]
            pub const CLR_MON_0: u32 = 0;
            #[doc = "Reset performance counters."]
            pub const CLR_MON_1: u32 = 0x01;
        }
    }
    #[doc = "Reset bit"]
    pub mod RST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do Not Reset."]
            pub const RST_0: u32 = 0;
            #[doc = "Reset IEE."]
            pub const RST_1: u32 = 0x01;
        }
    }
}
#[doc = "IEE Status"]
pub mod STA {
    #[doc = "DPA seed request bit"]
    pub mod DSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No seed request present"]
            pub const DSR_0: u32 = 0;
            #[doc = "Seed request present"]
            pub const DSR_1: u32 = 0x01;
        }
    }
    #[doc = "AES fault detected bit"]
    pub mod AFD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No fault detected"]
            pub const AFD_0: u32 = 0;
            #[doc = "Fault detected"]
            pub const AFD_1: u32 = 0x01;
        }
    }
}
#[doc = "IEE Test Mode Register"]
pub mod TSTMD {
    #[doc = "Test mode ready bit. All AXI transactions have stopped and test can begin."]
    pub mod TMRDY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not Ready."]
            pub const TMRDY_0: u32 = 0;
            #[doc = "Ready."]
            pub const TMRDY_1: u32 = 0x01;
        }
    }
    #[doc = "Test mode run bit"]
    pub mod TMR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not running. May be written if IEE_GCFG\\[TME\\] = 1"]
            pub const TMR_0: u32 = 0;
            #[doc = "Run AES Test until TMDONE is indicated."]
            pub const TMR_1: u32 = 0x01;
        }
    }
    #[doc = "Test mode encrypt/decrypt bit."]
    pub mod TMENCR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AES Test mode will do decryption."]
            pub const TMENCR_0: u32 = 0;
            #[doc = "AES Test mode will do encryption."]
            pub const TMENCR_1: u32 = 0x01;
        }
    }
    #[doc = "Test mode continue bit. Set to indicate that operation will be followed by more data."]
    pub mod TMCONT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not continue. This is the last block of data for AES."]
            pub const TMCONT_0: u32 = 0;
            #[doc = "Continue. Do not initialize AES after this block."]
            pub const TMCONT_1: u32 = 0x01;
        }
    }
    #[doc = "Test mode done bit"]
    pub mod TMDONE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not Done."]
            pub const TMDONE_0: u32 = 0;
            #[doc = "Test Done."]
            pub const TMDONE_1: u32 = 0x01;
        }
    }
    #[doc = "Test mode length field"]
    pub mod TMLEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES Mask Generation Seed"]
pub mod DPAMS {
    #[doc = "DPA mask seed"]
    pub mod DPAMS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, AES Slave Latency Threshold Value"]
pub mod PC_S_LT {
    #[doc = "Slave write latency threshold in AXI clock cycles."]
    pub mod SW_LT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Slave read latency threshold in AXI clock cycles."]
    pub mod SR_LT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, AES Master Latency Threshold"]
pub mod PC_M_LT {
    #[doc = "Master write latency threshold in AXI clock cycles."]
    pub mod MW_LT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Master read latency threshold in AXI clock cycles."]
    pub mod MR_LT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AES Block Encryptions"]
pub mod PC_BLK_ENC {
    #[doc = "Number of AES block encryptions. Does not roll over if value maxes out."]
    pub mod BLK_ENC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AES Block Decryptions"]
pub mod PC_BLK_DEC {
    #[doc = "Number of AES block decryptions. Does not roll over if value maxes out."]
    pub mod BLK_DEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Slave Read Transactions"]
pub mod PC_SR_TRANS {
    #[doc = "Number of slave read transactions."]
    pub mod SR_TRANS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Slave Write Transactions"]
pub mod PC_SW_TRANS {
    #[doc = "Number of slave write transactions."]
    pub mod SW_TRANS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Master Read Transactions"]
pub mod PC_MR_TRANS {
    #[doc = "Number of master read transactions."]
    pub mod MR_TRANS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Master Write Transactions"]
pub mod PC_MW_TRANS {
    #[doc = "Number of master write transactions."]
    pub mod MW_TRANS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Master Merge Buffer Read Transactions"]
pub mod PC_M_MBR {
    #[doc = "Number of master merge buffer read transactions."]
    pub mod M_MBR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Slave Read Transactions Byte Count"]
pub mod PC_SR_TBC_U {
    #[doc = "Number of bytes in slave read transactions. Upper 16 bits of SR_TBC\\[47:0\\]."]
    pub mod SR_TBC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Slave Read Transactions Byte Count"]
pub mod PC_SR_TBC_L {
    #[doc = "Number of bytes in slave read transactions. Lower 32 bits of SR_TBC\\[47:0\\]."]
    pub mod SR_TBC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Slave Write Transactions Byte Count"]
pub mod PC_SW_TBC_U {
    #[doc = "Number of bytes in slave write transactions. Upper 16 bits of SW_TBC\\[47:0\\]."]
    pub mod SW_TBC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Slave Write Transactions Byte Count"]
pub mod PC_SW_TBC_L {
    #[doc = "Number of bytes in slave write transactions. Lower 32 bits of SW_TBC\\[47:0\\]."]
    pub mod SW_TBC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Master Read Transactions Byte Count"]
pub mod PC_MR_TBC_U {
    #[doc = "Number of bytes in master read transactions. 44 MSBs. Upper 16 bits of MR_TBC\\[43:0\\]."]
    pub mod MR_TBC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Master Read Transactions Byte Count"]
pub mod PC_MR_TBC_L {
    #[doc = "Number of bytes in master read transactions. 4 LSBs, always 0."]
    pub mod MR_TBC_LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of bytes in master read transactions. 44 MSBs. Lower 28 bits of MR_TBC\\[43:0\\]."]
    pub mod MR_TBC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Master Write Transactions Byte Count"]
pub mod PC_MW_TBC_U {
    #[doc = "Number of bytes in master write transactions. 44 MSBs. Upper 16 bits of MW_TBC\\[43:0\\]."]
    pub mod MW_TBC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Master Write Transactions Byte Count"]
pub mod PC_MW_TBC_L {
    #[doc = "Number of bytes in master write transactions. 4 LSBs, always 0."]
    pub mod MW_TBC_LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of bytes in master write transactions. 44 MSBs. Lower 28 bits of MR_TBC\\[43:0\\]."]
    pub mod MW_TBC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Slave Read Transactions with Latency Greater than the Threshold"]
pub mod PC_SR_TLGTT {
    #[doc = "Number of slave read transactions with latency greater than the threshold."]
    pub mod SR_TLGTT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Slave Write Transactions with Latency Greater than the Threshold"]
pub mod PC_SW_TLGTT {
    #[doc = "Number of slave write transactions with latency greater than the threshold."]
    pub mod SW_TLGTT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Master Read Transactions with Latency Greater than the Threshold"]
pub mod PC_MR_TLGTT {
    #[doc = "Number of master read transactions with latency greater than the threshold."]
    pub mod MR_TLGTT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Number of AXI Master Write Transactions with Latency Greater than the Threshold"]
pub mod PC_MW_TLGTT {
    #[doc = "Number of master write transactions with latency greater than the threshold."]
    pub mod MW_TGTT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Slave Read Latency Count"]
pub mod PC_SR_TLAT_U {
    #[doc = "Total slave read latency in AXI clock cycles. Upper 16 bits of SR_TLAT\\[47:0\\]."]
    pub mod SR_TLAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Slave Read Latency Count"]
pub mod PC_SR_TLAT_L {
    #[doc = "Total slave read latency in AXI clock cycles. Lower 32 bits of SR_TLAT\\[47:0\\]."]
    pub mod SR_TLAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Slave Write Latency Count"]
pub mod PC_SW_TLAT_U {
    #[doc = "Total slave write latency in AXI clock cycles. Upper 16 bits of SW_TLAT\\[47:0\\]."]
    pub mod SW_TLAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Slave Write Latency Count"]
pub mod PC_SW_TLAT_L {
    #[doc = "Total slave write latency in AXI clock cycles. Lower 32 bits of SW_TLAT\\[47:0\\]."]
    pub mod SW_TLAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Master Read Latency Count"]
pub mod PC_MR_TLAT_U {
    #[doc = "Total master read latency in AXI clock cycles. Upper 16 bits of MR_TLAT\\[47:0\\]."]
    pub mod MR_TLAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Master Read Latency Count"]
pub mod PC_MR_TLAT_L {
    #[doc = "Total master read latency in AXI clock cycles. Lower 32 bits of MR_TLAT\\[47:0\\]."]
    pub mod MR_TLAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Master Write Latency Count"]
pub mod PC_MW_TLAT_U {
    #[doc = "Total master write latency in AXI clock cycles. Upper 16 bits of MW_TLAT\\[47:0\\]."]
    pub mod MW_TLAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Master Write Latency Count"]
pub mod PC_MW_TLAT_L {
    #[doc = "Total master write latency in AXI clock cycles. Lower 32 bits of MW_TLAT\\[47:0\\]."]
    pub mod MW_TLAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Slave Read Total Non-Responding Time"]
pub mod PC_SR_TNRT_U {
    #[doc = "Total slave read non-responding time in AXI clock cycles. Upper 16 bits of SR_TNRT\\[47:0\\]."]
    pub mod SR_TNRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Slave Read Total Non-Responding Time"]
pub mod PC_SR_TNRT_L {
    #[doc = "Total slave read non-responding time in AXI clock cycles. Lower 32 bits of SR_TNRT\\[47:0\\]."]
    pub mod SR_TNRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Upper Slave Write Total Non-Responding Time"]
pub mod PC_SW_TNRT_U {
    #[doc = "Total slave write non-responding time in AXI clock cycles. Upper 16 bits of SW_TNRT\\[47:0\\]."]
    pub mod SW_TNRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Counter, Lower Slave Write Total Non-Responding Time"]
pub mod PC_SW_TNRT_L {
    #[doc = "Total slave write non-responding time in AXI clock cycles. Lower 32 bits of SW_TNRT\\[47:0\\]."]
    pub mod SW_TNRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IEE Version ID Register 1"]
pub mod VIDR1 {
    #[doc = "Minor revision number for IEE."]
    pub mod MIN_REV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major revision number for IEE."]
    pub mod MAJ_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID for IEE."]
    pub mod IP_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IEE AES Version ID Register"]
pub mod AESVID {
    #[doc = "AES revision number."]
    pub mod AESRN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES version ID."]
    pub mod AESVID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IEE AES Test Mode Data Buffer"]
pub mod AES_TST_DB {
    #[doc = "AES test mode data buffer."]
    pub mod AES_TST_DB0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod regx {
    #[doc = "Region Registers"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "IEE Region REGION Attribute Register."]
        pub REGATTR: crate::RWRegister<u32>,
        _reserved0: [u8; 0x04],
        #[doc = "IEE Region REGION Page Offset Register"]
        pub REGPO: crate::RWRegister<u32>,
        _reserved1: [u8; 0x34],
        #[doc = "IEE Region REGION Key 1 Register"]
        pub REGKEY1_: [crate::WORegister<u32>; 8usize],
        _reserved2: [u8; 0x20],
        #[doc = "IEE Region REGION Key 2 Register"]
        pub REGKEY2_: [crate::WORegister<u32>; 8usize],
        _reserved3: [u8; 0x60],
    }
    #[doc = "IEE Region REGION Attribute Register."]
    pub mod REGATTR {
        #[doc = "AES key size."]
        pub mod KS {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "128 bits (CTR), 256 bits (XTS)."]
                pub const KS_0: u32 = 0;
                #[doc = "256 bits (CTR), 512 bits (XTS)."]
                pub const KS_1: u32 = 0x01;
            }
        }
        #[doc = "AES Mode."]
        pub mod MD {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "None (AXI error if accessed)"]
                pub const MD_0: u32 = 0;
                #[doc = "XTS"]
                pub const MD_1: u32 = 0x01;
                #[doc = "CTR w/ address binding"]
                pub const MD_2: u32 = 0x02;
                #[doc = "CTR w/o address binding"]
                pub const MD_3: u32 = 0x03;
                #[doc = "CTR keystream only"]
                pub const MD_4: u32 = 0x04;
                #[doc = "Undefined, AXI error if used"]
                pub const MD_5: u32 = 0x05;
                #[doc = "Undefined, AXI error if used"]
                pub const MD_6: u32 = 0x06;
                #[doc = "Undefined, AXI error if used"]
                pub const MD_7: u32 = 0x07;
            }
        }
        #[doc = "AES Bypass."]
        pub mod BYP {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "use MD field"]
                pub const BYP_0: u32 = 0;
                #[doc = "Bypass AES, no encrypt/decrypt"]
                pub const BYP_1: u32 = 0x01;
            }
        }
    }
    #[doc = "IEE Region REGION Page Offset Register"]
    pub mod REGPO {
        #[doc = "This field represents a 4Kb page offset"]
        pub mod PGOFF {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x00ff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "IEE Region REGION Key 1 Register"]
    pub mod REGKEY1_ {
        #[doc = "Key 1."]
        pub mod KEY1 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "IEE Region REGION Key 2 Register"]
    pub mod REGKEY2_ {
        #[doc = "Key 2."]
        pub mod KEY2 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
