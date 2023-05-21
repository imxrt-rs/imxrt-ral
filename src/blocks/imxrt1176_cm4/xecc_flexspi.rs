#[doc = "XECC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ECC Control Register"]
    pub ECC_CTRL: crate::RWRegister<u32>,
    #[doc = "Error Interrupt Status Register"]
    pub ERR_STATUS: crate::RWRegister<u32>,
    #[doc = "Error Interrupt Status Enable Register"]
    pub ERR_STAT_EN: crate::RWRegister<u32>,
    #[doc = "Error Interrupt Enable Register"]
    pub ERR_SIG_EN: crate::RWRegister<u32>,
    #[doc = "Error Injection On Write Data"]
    pub ERR_DATA_INJ: crate::RWRegister<u32>,
    #[doc = "Error Injection On ECC Code of Write Data"]
    pub ERR_ECC_INJ: crate::RWRegister<u32>,
    #[doc = "Single Error Address"]
    pub SINGLE_ERR_ADDR: crate::RORegister<u32>,
    #[doc = "Single Error Read Data"]
    pub SINGLE_ERR_DATA: crate::RORegister<u32>,
    #[doc = "Single Error ECC Code"]
    pub SINGLE_ERR_ECC: crate::RORegister<u32>,
    #[doc = "Single Error Bit Position"]
    pub SINGLE_ERR_POS: crate::RORegister<u32>,
    #[doc = "Single Error Bit Field"]
    pub SINGLE_ERR_BIT_FIELD: crate::RORegister<u32>,
    #[doc = "Multiple Error Address"]
    pub MULTI_ERR_ADDR: crate::RORegister<u32>,
    #[doc = "Multiple Error Read Data"]
    pub MULTI_ERR_DATA: crate::RORegister<u32>,
    #[doc = "Multiple Error ECC code"]
    pub MULTI_ERR_ECC: crate::RORegister<u32>,
    #[doc = "Multiple Error Bit Field"]
    pub MULTI_ERR_BIT_FIELD: crate::RORegister<u32>,
    #[doc = "ECC Region 0 Base Address"]
    pub ECC_BASE_ADDR0: crate::RWRegister<u32>,
    #[doc = "ECC Region 0 End Address"]
    pub ECC_END_ADDR0: crate::RWRegister<u32>,
    #[doc = "ECC Region 1 Base Address"]
    pub ECC_BASE_ADDR1: crate::RWRegister<u32>,
    #[doc = "ECC Region 1 End Address"]
    pub ECC_END_ADDR1: crate::RWRegister<u32>,
    #[doc = "ECC Region 2 Base Address"]
    pub ECC_BASE_ADDR2: crate::RWRegister<u32>,
    #[doc = "ECC Region 2 End Address"]
    pub ECC_END_ADDR2: crate::RWRegister<u32>,
    #[doc = "ECC Region 3 Base Address"]
    pub ECC_BASE_ADDR3: crate::RWRegister<u32>,
    #[doc = "ECC Region 3 End Address"]
    pub ECC_END_ADDR3: crate::RWRegister<u32>,
}
#[doc = "ECC Control Register"]
pub mod ECC_CTRL {
    #[doc = "ECC Function Enable"]
    pub mod ECC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const ECC_EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const ECC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Write ECC Encode Function Enable"]
    pub mod WECC_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const WECC_EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const WECC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Read ECC Function Enable"]
    pub mod RECC_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const RECC_EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const RECC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Swap Data Enable"]
    pub mod SWAP_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const SWAP_EN_0: u32 = 0;
            #[doc = "Enable"]
            pub const SWAP_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Interrupt Status Register"]
pub mod ERR_STATUS {
    #[doc = "Single Bit Error"]
    pub mod SINGLE_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single bit error does not happen."]
            pub const SINGLE_ERR_0: u32 = 0;
            #[doc = "Single bit error happens."]
            pub const SINGLE_ERR_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error"]
    pub mod MULTI_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multiple bits error does not happen."]
            pub const MULTI_ERR_0: u32 = 0;
            #[doc = "Multiple bits error happens."]
            pub const MULTI_ERR_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Interrupt Status Enable Register"]
pub mod ERR_STAT_EN {
    #[doc = "Single Bit Error Status Enable"]
    pub mod SINGLE_ERR_STAT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const SINGLE_ERR_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Status Enable"]
    pub mod MULIT_ERR_STAT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const MULIT_ERR_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULIT_ERR_STAT_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Interrupt Enable Register"]
pub mod ERR_SIG_EN {
    #[doc = "Single Bit Error Interrupt Enable"]
    pub mod SINGLE_ERR_SIG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const SINGLE_ERR_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const SINGLE_ERR_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Multiple Bits Error Interrupt Enable"]
    pub mod MULTI_ERR_SIG_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const MULTI_ERR_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const MULTI_ERR_SIG_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Error Injection On Write Data"]
pub mod ERR_DATA_INJ {
    #[doc = "Error Injection On Write Data"]
    pub mod ERR_DATA_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection On ECC Code of Write Data"]
pub mod ERR_ECC_INJ {
    #[doc = "Error Injection On ECC Code of Write Data"]
    pub mod ERR_ECC_INJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error Address"]
pub mod SINGLE_ERR_ADDR {
    #[doc = "Single Error Address"]
    pub mod SINGLE_ERR_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error Read Data"]
pub mod SINGLE_ERR_DATA {
    #[doc = "Single Error Read Data"]
    pub mod SINGLE_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error ECC Code"]
pub mod SINGLE_ERR_ECC {
    #[doc = "Single Error ECC code"]
    pub mod SINGLE_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error Bit Position"]
pub mod SINGLE_ERR_POS {
    #[doc = "Single Error bit Position"]
    pub mod SINGLE_ERR_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Error Bit Field"]
pub mod SINGLE_ERR_BIT_FIELD {
    #[doc = "Single Error Bit Field"]
    pub mod SINGLE_ERR_BIT_FIELD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Error Address"]
pub mod MULTI_ERR_ADDR {
    #[doc = "Multiple Error Address"]
    pub mod MULTI_ERR_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Error Read Data"]
pub mod MULTI_ERR_DATA {
    #[doc = "Multiple Error Read Data"]
    pub mod MULTI_ERR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Error ECC code"]
pub mod MULTI_ERR_ECC {
    #[doc = "Multiple Error ECC code"]
    pub mod MULTI_ERR_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Error Bit Field"]
pub mod MULTI_ERR_BIT_FIELD {
    #[doc = "Multiple Error Bit Field"]
    pub mod MULTI_ERR_BIT_FIELD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC Region 0 Base Address"]
pub mod ECC_BASE_ADDR0 {
    #[doc = "ECC Region 0 Base Address"]
    pub mod ECC_BASE_ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC Region 0 End Address"]
pub mod ECC_END_ADDR0 {
    #[doc = "ECC Region 0 End Address"]
    pub mod ECC_END_ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC Region 1 Base Address"]
pub mod ECC_BASE_ADDR1 {
    #[doc = "ECC Region 1 Base Address"]
    pub mod ECC_BASE_ADDR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC Region 1 End Address"]
pub mod ECC_END_ADDR1 {
    #[doc = "ECC Region 1 End Address"]
    pub mod ECC_END_ADDR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC Region 2 Base Address"]
pub mod ECC_BASE_ADDR2 {
    #[doc = "ECC Region 2 Base Address"]
    pub mod ECC_BASE_ADDR2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC Region 2 End Address"]
pub mod ECC_END_ADDR2 {
    #[doc = "ECC Region 2 End Address"]
    pub mod ECC_END_ADDR2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC Region 3 Base Address"]
pub mod ECC_BASE_ADDR3 {
    #[doc = "ECC Region 3 Base Address"]
    pub mod ECC_BASE_ADDR3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ECC Region 3 End Address"]
pub mod ECC_END_ADDR3 {
    #[doc = "ECC Region 3 End Address"]
    pub mod ECC_END_ADDR3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
