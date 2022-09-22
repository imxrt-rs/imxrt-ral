#[doc = "IOMUXC LPSR GPR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPR0 General Purpose Register"]
    pub GPR0: crate::RWRegister<u32>,
    #[doc = "GPR1 General Purpose Register"]
    pub GPR1: crate::RWRegister<u32>,
    #[doc = "GPR2 General Purpose Register"]
    pub GPR2: crate::RWRegister<u32>,
    #[doc = "GPR3 General Purpose Register"]
    pub GPR3: crate::RWRegister<u32>,
    #[doc = "GPR4 General Purpose Register"]
    pub GPR4: crate::RWRegister<u32>,
    #[doc = "GPR5 General Purpose Register"]
    pub GPR5: crate::RWRegister<u32>,
    #[doc = "GPR6 General Purpose Register"]
    pub GPR6: crate::RWRegister<u32>,
    #[doc = "GPR7 General Purpose Register"]
    pub GPR7: crate::RWRegister<u32>,
    #[doc = "GPR8 General Purpose Register"]
    pub GPR8: crate::RWRegister<u32>,
    #[doc = "GPR9 General Purpose Register"]
    pub GPR9: crate::RWRegister<u32>,
    #[doc = "GPR10 General Purpose Register"]
    pub GPR10: crate::RWRegister<u32>,
    #[doc = "GPR11 General Purpose Register"]
    pub GPR11: crate::RWRegister<u32>,
    #[doc = "GPR12 General Purpose Register"]
    pub GPR12: crate::RWRegister<u32>,
    #[doc = "GPR13 General Purpose Register"]
    pub GPR13: crate::RWRegister<u32>,
    #[doc = "GPR14 General Purpose Register"]
    pub GPR14: crate::RWRegister<u32>,
    #[doc = "GPR15 General Purpose Register"]
    pub GPR15: crate::RWRegister<u32>,
    #[doc = "GPR16 General Purpose Register"]
    pub GPR16: crate::RWRegister<u32>,
    #[doc = "GPR17 General Purpose Register"]
    pub GPR17: crate::RWRegister<u32>,
    #[doc = "GPR18 General Purpose Register"]
    pub GPR18: crate::RWRegister<u32>,
    #[doc = "GPR19 General Purpose Register"]
    pub GPR19: crate::RWRegister<u32>,
    #[doc = "GPR20 General Purpose Register"]
    pub GPR20: crate::RWRegister<u32>,
    #[doc = "GPR21 General Purpose Register"]
    pub GPR21: crate::RWRegister<u32>,
    #[doc = "GPR22 General Purpose Register"]
    pub GPR22: crate::RWRegister<u32>,
    #[doc = "GPR23 General Purpose Register"]
    pub GPR23: crate::RWRegister<u32>,
    #[doc = "GPR24 General Purpose Register"]
    pub GPR24: crate::RWRegister<u32>,
    #[doc = "GPR25 General Purpose Register"]
    pub GPR25: crate::RWRegister<u32>,
    #[doc = "GPR26 General Purpose Register"]
    pub GPR26: crate::RWRegister<u32>,
    _reserved0: [u8; 0x18],
    #[doc = "GPR33 General Purpose Register"]
    pub GPR33: crate::RWRegister<u32>,
    #[doc = "GPR34 General Purpose Register"]
    pub GPR34: crate::RWRegister<u32>,
    #[doc = "GPR35 General Purpose Register"]
    pub GPR35: crate::RWRegister<u32>,
    #[doc = "GPR36 General Purpose Register"]
    pub GPR36: crate::RWRegister<u32>,
    #[doc = "GPR37 General Purpose Register"]
    pub GPR37: crate::RWRegister<u32>,
    #[doc = "GPR38 General Purpose Register"]
    pub GPR38: crate::RWRegister<u32>,
    #[doc = "GPR39 General Purpose Register"]
    pub GPR39: crate::RWRegister<u32>,
    #[doc = "GPR40 General Purpose Register"]
    pub GPR40: crate::RORegister<u32>,
    #[doc = "GPR41 General Purpose Register"]
    pub GPR41: crate::RORegister<u32>,
}
#[doc = "GPR0 General Purpose Register"]
pub mod GPR0 {
    #[doc = "CM4 Vector table offset value lower bits out of reset"]
    pub mod CM4_INIT_VTOR_LOW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR1 General Purpose Register"]
pub mod GPR1 {
    #[doc = "CM4 Vector table offset value higher bits out of reset"]
    pub mod CM4_INIT_VTOR_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR2 General Purpose Register"]
pub mod GPR2 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-0"]
    pub mod APC_AC_R0_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR3 General Purpose Register"]
pub mod GPR3 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-0"]
    pub mod APC_AC_R0_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR4 General Purpose Register"]
pub mod GPR4 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-1"]
    pub mod APC_AC_R1_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR5 General Purpose Register"]
pub mod GPR5 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-1"]
    pub mod APC_AC_R1_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR6 General Purpose Register"]
pub mod GPR6 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-2"]
    pub mod APC_AC_R2_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR7 General Purpose Register"]
pub mod GPR7 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-2"]
    pub mod APC_AC_R2_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR8 General Purpose Register"]
pub mod GPR8 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-3"]
    pub mod APC_AC_R3_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR9 General Purpose Register"]
pub mod GPR9 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-3"]
    pub mod APC_AC_R3_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR10 General Purpose Register"]
pub mod GPR10 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-4"]
    pub mod APC_AC_R4_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR11 General Purpose Register"]
pub mod GPR11 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-4"]
    pub mod APC_AC_R4_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR12 General Purpose Register"]
pub mod GPR12 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-5"]
    pub mod APC_AC_R5_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR13 General Purpose Register"]
pub mod GPR13 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-5"]
    pub mod APC_AC_R5_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR14 General Purpose Register"]
pub mod GPR14 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-6"]
    pub mod APC_AC_R6_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR15 General Purpose Register"]
pub mod GPR15 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-6"]
    pub mod APC_AC_R6_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR16 General Purpose Register"]
pub mod GPR16 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC start address of memory region-7"]
    pub mod APC_AC_R7_BOT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR17 General Purpose Register"]
pub mod GPR17 {
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access to bit 31:1 is not blocked"]
            pub const NO: u32 = 0;
            #[doc = "Write access to bit 31:1 is blocked"]
            pub const BLOCK: u32 = 0x01;
        }
    }
    #[doc = "APC end address of memory region-7"]
    pub mod APC_AC_R7_TOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR18 General Purpose Register"]
pub mod GPR18 {
    #[doc = "APC memory region-0 encryption enable"]
    pub mod APC_R0_ENCRYPT_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Encryption enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR19 General Purpose Register"]
pub mod GPR19 {
    #[doc = "APC memory region-1 encryption enable"]
    pub mod APC_R1_ENCRYPT_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Encryption enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR20 General Purpose Register"]
pub mod GPR20 {
    #[doc = "APC memory region-2 encryption enable"]
    pub mod APC_R2_ENCRYPT_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Encryption enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR21 General Purpose Register"]
pub mod GPR21 {
    #[doc = "APC memory region-3 encryption enable"]
    pub mod APC_R3_ENCRYPT_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Encryption enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR22 General Purpose Register"]
pub mod GPR22 {
    #[doc = "APC memory region-4 encryption enable"]
    pub mod APC_R4_ENCRYPT_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Encryption enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR23 General Purpose Register"]
pub mod GPR23 {
    #[doc = "APC memory region-5 encryption enable"]
    pub mod APC_R5_ENCRYPT_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Encryption enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR24 General Purpose Register"]
pub mod GPR24 {
    #[doc = "APC memory region-6 encryption enable"]
    pub mod APC_R6_ENCRYPT_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Encryption enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR25 General Purpose Register"]
pub mod GPR25 {
    #[doc = "APC memory region-7 encryption enable"]
    pub mod APC_R7_ENCRYPT_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Encryption enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "APC global enable bit"]
    pub mod APC_VALID {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DIS: u32 = 0;
            #[doc = "Enable encryption for GPRx\\[APC_x_ENCRYPT_ENABLE\\] (valid for GPR2-GPR25)"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR26 General Purpose Register"]
pub mod GPR26 {
    #[doc = "Vector table offset register out of reset. See the ARM v7-M Architecture Reference Manual for more information about the vector table offset register (VTOR)."]
    pub mod CM7_INIT_VTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General purpose bits"]
    pub mod FIELD_0 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR33 General Purpose Register"]
pub mod GPR33 {
    #[doc = "Clear CM4 NMI holding register"]
    pub mod M4_NMI_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear USBPHY1 wakeup interrupt holding register"]
    pub mod USBPHY1_WAKEUP_IRQ_CLEAR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear USBPHY1 wakeup interrupt holding register"]
    pub mod USBPHY2_WAKEUP_IRQ_CLEAR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR34 General Purpose Register"]
pub mod GPR34 {
    #[doc = "GPIO_LPSR IO bank supply voltage range selection"]
    pub mod GPIO_LPSR_HIGH_RANGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO_LPSR IO bank supply voltage range selection"]
    pub mod GPIO_LPSR_LOW_RANGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask CM7 NMI pin input"]
    pub mod M7_NMI_MASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NMI input from IO to CM7 is not blocked"]
            pub const DISABLE: u32 = 0;
            #[doc = "NMI input from IO to CM7 is blocked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Mask CM4 NMI pin input"]
    pub mod M4_NMI_MASK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NMI input from IO to CM4 is not blocked"]
            pub const DISABLE: u32 = 0;
            #[doc = "NMI input from IO to CM4 is blocked"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CM4 sleep request selection"]
    pub mod M4_GPC_SLEEP_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CM4 SLEEPDEEP is sent to GPC"]
            pub const DISABLE: u32 = 0;
            #[doc = "CM4 SLEEPING is sent to GPC"]
            pub const NABLE: u32 = 0x01;
        }
    }
    #[doc = "Security error response enable"]
    pub mod SEC_ERR_RESP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OKEY response"]
            pub const DISABLE: u32 = 0;
            #[doc = "SLVError (default)"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR35 General Purpose Register"]
pub mod GPR35 {
    #[doc = "ADC1 doze mode"]
    pub mod ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ADC1 stop request"]
    pub mod ADC1_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ADC1 stop mode selection. This bitfield cannot change when ADC1_STOP_REQ is asserted."]
    pub mod ADC1_IPG_STOP_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "ADC2 doze mode"]
    pub mod ADC2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ADC2 stop request"]
    pub mod ADC2_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ADC2 stop mode selection. This bitfield cannot change when ADC2_STOP_REQ is asserted."]
    pub mod ADC2_IPG_STOP_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "CAN3 doze mode"]
    pub mod CAAM_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CAAM stop request"]
    pub mod CAAM_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CAN1 doze mode"]
    pub mod CAN1_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CAN1 stop request"]
    pub mod CAN1_STOP_REQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CAN2 doze mode"]
    pub mod CAN2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CAN2 stop request"]
    pub mod CAN2_STOP_REQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CAN3 doze mode"]
    pub mod CAN3_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CAN3 stop request"]
    pub mod CAN3_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EDMA stop request"]
    pub mod EDMA_STOP_REQ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EDMA_LPSR stop request"]
    pub mod EDMA_LPSR_STOP_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ENET doze mode"]
    pub mod ENET_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ENET stop request"]
    pub mod ENET_STOP_REQ {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ENET1G doze mode"]
    pub mod ENET1G_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ENET1G stop request"]
    pub mod ENET1G_STOP_REQ {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO2 doze mode"]
    pub mod FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO2 doze mode"]
    pub mod FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXSPI1 doze mode"]
    pub mod FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXSPI1 stop request"]
    pub mod FLEXSPI1_STOP_REQ {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXSPI2 doze mode"]
    pub mod FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXSPI2 stop request"]
    pub mod FLEXSPI2_STOP_REQ {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR36 General Purpose Register"]
pub mod GPR36 {
    #[doc = "GPT1 doze mode"]
    pub mod GPT1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GPT2 doze mode"]
    pub mod GPT2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GPT3 doze mode"]
    pub mod GPT3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GPT4 doze mode"]
    pub mod GPT4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GPT5 doze mode"]
    pub mod GPT5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GPT6 doze mode"]
    pub mod GPT6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C1 doze mode"]
    pub mod LPI2C1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C1 stop request"]
    pub mod LPI2C1_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C1 stop mode selection. This bitfield cannot change when LPI2C1_STOP_REQ is asserted."]
    pub mod LPI2C1_IPG_STOP_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPI2C2 doze mode"]
    pub mod LPI2C2_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C2 stop request"]
    pub mod LPI2C2_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C2 stop mode selection. This bitfield cannot change when LPI2C2_STOP_REQ is asserted."]
    pub mod LPI2C2_IPG_STOP_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 doze mode"]
    pub mod LPI2C3_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 stop request"]
    pub mod LPI2C3_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C3 stop mode selection. This bitfield cannot change when LPI2C3_STOP_REQ is asserted."]
    pub mod LPI2C3_IPG_STOP_MODE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 doze mode"]
    pub mod LPI2C4_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 stop request"]
    pub mod LPI2C4_STOP_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C4 stop mode selection. This bitfield cannot change when LPI2C4_STOP_REQ is asserted."]
    pub mod LPI2C4_IPG_STOP_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPI2C5 doze mode"]
    pub mod LPI2C5_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C5 stop request"]
    pub mod LPI2C5_STOP_REQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C5 stop mode selection. This bitfield cannot change when LPI2C5_STOP_REQ is asserted."]
    pub mod LPI2C5_IPG_STOP_MODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPI2C6 doze mode"]
    pub mod LPI2C6_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C6 stop request"]
    pub mod LPI2C6_STOP_REQ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI2C6 stop mode selection. This bitfield cannot change when LPI2C6_STOP_REQ is asserted."]
    pub mod LPI2C6_IPG_STOP_MODE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPSPI1 doze mode"]
    pub mod LPSPI1_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI1 stop request"]
    pub mod LPSPI1_STOP_REQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI1 stop mode selection. This bitfield cannot change when LPSPI1_STOP_REQ is asserted."]
    pub mod LPSPI1_IPG_STOP_MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR37 General Purpose Register"]
pub mod GPR37 {
    #[doc = "LPSPI2 doze mode"]
    pub mod LPSPI2_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI2 stop request"]
    pub mod LPSPI2_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI2 stop mode selection. This bitfield cannot change when LPSPI2_STOP_REQ is asserted."]
    pub mod LPSPI2_IPG_STOP_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPSPI3 doze mode"]
    pub mod LPSPI3_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI3 stop request"]
    pub mod LPSPI3_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI3 stop mode selection. This bitfield cannot change when LPSPI3_STOP_REQ is asserted."]
    pub mod LPSPI3_IPG_STOP_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 doze mode"]
    pub mod LPSPI4_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 stop request"]
    pub mod LPSPI4_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI4 stop mode selection. This bitfield cannot change when LPSPI4_STOP_REQ is asserted."]
    pub mod LPSPI4_IPG_STOP_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPSPI5 doze mode"]
    pub mod LPSPI5_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI5 stop request"]
    pub mod LPSPI5_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI5 stop mode selection. This bitfield cannot change when LPSPI5_STOP_REQ is asserted."]
    pub mod LPSPI5_IPG_STOP_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPSPI6 doze mode"]
    pub mod LPSPI6_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI6 stop request"]
    pub mod LPSPI6_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPSPI6 stop mode selection. This bitfield cannot change when LPSPI6_STOP_REQ is asserted."]
    pub mod LPSPI6_IPG_STOP_MODE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 doze mode"]
    pub mod LPUART1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 stop request"]
    pub mod LPUART1_STOP_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART1 stop mode selection. This bitfield cannot change when LPUART1_STOP_REQ is asserted."]
    pub mod LPUART1_IPG_STOP_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART2 doze mode"]
    pub mod LPUART2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART2 stop request"]
    pub mod LPUART2_STOP_REQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART2 stop mode selection. This bitfield cannot change when LPUART2_STOP_REQ is asserted."]
    pub mod LPUART2_IPG_STOP_MODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART3 doze mode"]
    pub mod LPUART3_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART3 stop request"]
    pub mod LPUART3_STOP_REQ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART3 stop mode selection. This bitfield cannot change when LPUART3_STOP_REQ is asserted."]
    pub mod LPUART3_IPG_STOP_MODE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART4 doze mode"]
    pub mod LPUART4_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART4 stop request"]
    pub mod LPUART4_STOP_REQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART4 stop mode selection. This bitfield cannot change when LPUART4_STOP_REQ is asserted."]
    pub mod LPUART4_IPG_STOP_MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR38 General Purpose Register"]
pub mod GPR38 {
    #[doc = "LPUART5 doze mode"]
    pub mod LPUART5_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART5 stop request"]
    pub mod LPUART5_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART5 stop mode selection. This bitfield cannot change when LPUART5_STOP_REQ is asserted."]
    pub mod LPUART5_IPG_STOP_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART6 doze mode"]
    pub mod LPUART6_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART6 stop request"]
    pub mod LPUART6_STOP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART6 stop mode selection. This bitfield cannot change when LPUART6_STOP_REQ is asserted."]
    pub mod LPUART6_IPG_STOP_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART7 doze mode"]
    pub mod LPUART7_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART7 stop request"]
    pub mod LPUART7_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART7 stop mode selection. This bitfield cannot change when LPUART7_STOP_REQ is asserted."]
    pub mod LPUART7_IPG_STOP_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART8 doze mode"]
    pub mod LPUART8_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART8 stop request"]
    pub mod LPUART8_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART8 stop mode selection. This bitfield cannot change when LPUART8_STOP_REQ is asserted."]
    pub mod LPUART8_IPG_STOP_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART9 doze mode"]
    pub mod LPUART9_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART9 stop request"]
    pub mod LPUART9_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART9 stop mode selection. This bitfield cannot change when LPUART9_STOP_REQ is asserted."]
    pub mod LPUART9_IPG_STOP_MODE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART10 doze mode"]
    pub mod LPUART10_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART10 stop request"]
    pub mod LPUART10_STOP_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART10 stop mode selection. This bitfield cannot change when LPUART10_STOP_REQ is asserted."]
    pub mod LPUART10_IPG_STOP_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART11 doze mode"]
    pub mod LPUART11_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART11 stop request"]
    pub mod LPUART11_STOP_REQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART11 stop mode selection. This bitfield cannot change when LPUART11_STOP_REQ is asserted."]
    pub mod LPUART11_IPG_STOP_MODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "LPUART12 doze mode"]
    pub mod LPUART12_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART12 stop request"]
    pub mod LPUART12_STOP_REQ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPUART12 stop mode selection. This bitfield cannot change when LPUART12_STOP_REQ is asserted."]
    pub mod LPUART12_IPG_STOP_MODE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "MIC doze mode"]
    pub mod MIC_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MIC stop request"]
    pub mod MIC_STOP_REQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MIC stop mode selection. This bitfield cannot change when MIC_STOP_REQ is asserted."]
    pub mod MIC_IPG_STOP_MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This module is functional in Stop Mode"]
            pub const FUNC: u32 = 0;
            #[doc = "This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'."]
            pub const NONFUNC: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR39 General Purpose Register"]
pub mod GPR39 {
    #[doc = "PIT1 stop request"]
    pub mod PIT1_STOP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PIT2 stop request"]
    pub mod PIT2_STOP_REQ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SEMC stop request"]
    pub mod SEMC_STOP_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SIM1 doze mode"]
    pub mod SIM1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SIM2 doze mode"]
    pub mod SIM2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SNVS_HP doze mode"]
    pub mod SNVS_HP_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SNVS_HP stop request"]
    pub mod SNVS_HP_STOP_REQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "WDOG1 doze mode"]
    pub mod WDOG1_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "WDOG2 doze mode"]
    pub mod WDOG2_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in doze mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "In doze mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SAI1 stop request"]
    pub mod SAI1_STOP_REQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SAI2 stop request"]
    pub mod SAI2_STOP_REQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SAI3 stop request"]
    pub mod SAI3_STOP_REQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SAI4 stop request"]
    pub mod SAI4_STOP_REQ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO1 bus clock domain stop request"]
    pub mod FLEXIO1_STOP_REQ_BUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO1 peripheral clock domain stop request"]
    pub mod FLEXIO1_STOP_REQ_PER {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO2 bus clock domain stop request"]
    pub mod FLEXIO2_STOP_REQ_BUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FLEXIO2 peripheral clock domain stop request"]
    pub mod FLEXIO2_STOP_REQ_PER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop request off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Stop request on"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Domain write protection"]
    pub mod DWP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Both cores are allowed"]
            pub const FORBID_NONE: u32 = 0;
            #[doc = "CM7 is forbidden"]
            pub const FORBID_CM7: u32 = 0x01;
            #[doc = "CM4 is forbidden"]
            pub const FORBID_CM4: u32 = 0x02;
            #[doc = "Both cores are forbidden"]
            pub const FORBID_BOTH: u32 = 0x03;
        }
    }
    #[doc = "Domain write protection lock"]
    pub mod DWP_LOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Neither of DWP bits is locked"]
            pub const LOCK_NONE: u32 = 0;
            #[doc = "The lower DWP bit is locked"]
            pub const LOCK_LOW: u32 = 0x01;
            #[doc = "The higher DWP bit is locked"]
            pub const LOCK_HIGH: u32 = 0x02;
            #[doc = "Both DWP bits are locked"]
            pub const LOCK_BOTH: u32 = 0x03;
        }
    }
}
#[doc = "GPR40 General Purpose Register"]
pub mod GPR40 {
    #[doc = "ADC1 stop acknowledge"]
    pub mod ADC1_STOP_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC2 stop acknowledge"]
    pub mod ADC2_STOP_ACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAAM stop acknowledge"]
    pub mod CAAM_STOP_ACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN1 stop acknowledge"]
    pub mod CAN1_STOP_ACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN2 stop acknowledge"]
    pub mod CAN2_STOP_ACK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN3 stop acknowledge"]
    pub mod CAN3_STOP_ACK {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EDMA stop acknowledge"]
    pub mod EDMA_STOP_ACK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EDMA_LPSR stop acknowledge"]
    pub mod EDMA_LPSR_STOP_ACK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET stop acknowledge"]
    pub mod ENET_STOP_ACK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ENET1G stop acknowledge"]
    pub mod ENET1G_STOP_ACK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXSPI1 stop acknowledge"]
    pub mod FLEXSPI1_STOP_ACK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXSPI2 stop acknowledge"]
    pub mod FLEXSPI2_STOP_ACK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C1 stop acknowledge"]
    pub mod LPI2C1_STOP_ACK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C2 stop acknowledge"]
    pub mod LPI2C2_STOP_ACK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C3 stop acknowledge"]
    pub mod LPI2C3_STOP_ACK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C4 stop acknowledge"]
    pub mod LPI2C4_STOP_ACK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C5 stop acknowledge"]
    pub mod LPI2C5_STOP_ACK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI2C6 stop acknowledge"]
    pub mod LPI2C6_STOP_ACK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI1 stop acknowledge"]
    pub mod LPSPI1_STOP_ACK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI2 stop acknowledge"]
    pub mod LPSPI2_STOP_ACK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI3 stop acknowledge"]
    pub mod LPSPI3_STOP_ACK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI4 stop acknowledge"]
    pub mod LPSPI4_STOP_ACK {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI5 stop acknowledge"]
    pub mod LPSPI5_STOP_ACK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSPI6 stop acknowledge"]
    pub mod LPSPI6_STOP_ACK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART1 stop acknowledge"]
    pub mod LPUART1_STOP_ACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART2 stop acknowledge"]
    pub mod LPUART2_STOP_ACK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART3 stop acknowledge"]
    pub mod LPUART3_STOP_ACK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART4 stop acknowledge"]
    pub mod LPUART4_STOP_ACK {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART5 stop acknowledge"]
    pub mod LPUART5_STOP_ACK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART6 stop acknowledge"]
    pub mod LPUART6_STOP_ACK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART7 stop acknowledge"]
    pub mod LPUART7_STOP_ACK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART8 stop acknowledge"]
    pub mod LPUART8_STOP_ACK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPR41 General Purpose Register"]
pub mod GPR41 {
    #[doc = "LPUART9 stop acknowledge"]
    pub mod LPUART9_STOP_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART10 stop acknowledge"]
    pub mod LPUART10_STOP_ACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART11 stop acknowledge"]
    pub mod LPUART11_STOP_ACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPUART12 stop acknowledge"]
    pub mod LPUART12_STOP_ACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIC stop acknowledge"]
    pub mod MIC_STOP_ACK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIT1 stop acknowledge"]
    pub mod PIT1_STOP_ACK {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIT2 stop acknowledge"]
    pub mod PIT2_STOP_ACK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SEMC stop acknowledge"]
    pub mod SEMC_STOP_ACK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SNVS_HP stop acknowledge"]
    pub mod SNVS_HP_STOP_ACK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI1 stop acknowledge"]
    pub mod SAI1_STOP_ACK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI2 stop acknowledge"]
    pub mod SAI2_STOP_ACK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI3 stop acknowledge"]
    pub mod SAI3_STOP_ACK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAI4 stop acknowledge"]
    pub mod SAI4_STOP_ACK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO1 stop acknowledge of bus clock domain"]
    pub mod FLEXIO1_STOP_ACK_BUS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO1 stop acknowledge of peripheral clock domain"]
    pub mod FLEXIO1_STOP_ACK_PER {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO2 stop acknowledge of bus clock domain"]
    pub mod FLEXIO2_STOP_ACK_BUS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FLEXIO2 stop acknowledge of peripheral clock domain"]
    pub mod FLEXIO2_STOP_ACK_PER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ROM read lock status bit"]
    pub mod ROM_READ_LOCKED {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
