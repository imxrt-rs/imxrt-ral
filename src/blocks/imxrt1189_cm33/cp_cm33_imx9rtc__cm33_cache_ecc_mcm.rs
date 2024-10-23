#[doc = "CM33_CACHE_ECC_MCM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CACHE ECC Control"]
    pub CACHE_ECCR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x1c],
    #[doc = "Interrupt Status"]
    pub INT_STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Enable"]
    pub INT_STAT_EN: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub INT_SIG_EN: crate::RWRegister<u32>,
    _reserved1: [u8; 0x30],
    #[doc = "Code Cache Single-Bit ECC Error Information"]
    pub CODE_CACHE_ECC_SINGLE_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "Code Cache Single-Bit ECC Error Address"]
    pub CODE_CACHE_ECC_SINGLE_ERROR_ADDR: crate::RORegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Code Cache Multibit ECC Error Information"]
    pub CODE_CACHE_ECC_MULTI_ERROR_INFO: crate::RORegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "System Cache Single-Bit ECC Error Information"]
    pub SYSTEM_CACHE_ECC_SINGLE_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "System Cache Single-Bit ECC Error Address"]
    pub SYSTEM_CACHE_ECC_SINGLE_ERROR_ADDR: crate::RORegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "System Cache Multibit ECC Error Information"]
    pub SYSTEM_CACHE_ECC_MULTI_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "System Cache Multibit ECC Error Data"]
    pub SYSTEM_CACHE_ECC_MULTI_ERROR_DATA: crate::RORegister<u32>,
    _reserved5: [u8; 0x04],
    #[doc = "Code Cache TAG0 ECC Error Injection"]
    pub CODE_CACHE_TAG0_ECC_ERROR_INJEC: crate::RWRegister<u32>,
    #[doc = "Code Cache TAG1 ECC Error Injection"]
    pub CODE_CACHE_TAG1_ECC_ERROR_INJEC: crate::RWRegister<u32>,
    #[doc = "Code Cache DATA0 ECC Error Injection"]
    pub CODE_CACHE_DATA0_ECC_ERROR_INJEC: crate::RWRegister<u32>,
    #[doc = "Code Cache DATA1 ECC Error Injection"]
    pub CODE_CACHE_DATA1_ECC_ERROR_INJEC: crate::RWRegister<u32>,
    #[doc = "System Cache TAG0 ECC Error Injection"]
    pub SYTEM_CACHE_TAG0_ECC_ERROR_INJEC: crate::RWRegister<u32>,
    #[doc = "System Cache TAG1 ECC Error Injection"]
    pub SYSTEM_CACHE_TAG1_ECC_ERROR_INJEC: crate::RWRegister<u32>,
    #[doc = "System Cache DATA0 ECC Error Injection"]
    pub SYSTEM_CACHE_DATA0_ECC_ERROR_INJEC: crate::RWRegister<u32>,
    #[doc = "System Cache DATA1 ECC Error Injection"]
    pub STSTEM_CACHE_DATA1_ECC_ERROR_INJEC: crate::RWRegister<u32>,
}
#[doc = "CACHE ECC Control"]
pub mod CACHE_ECCR {
    #[doc = "Disable CACHE ECC Write Generation"]
    pub mod WECC_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Cache ECC Read Check"]
    pub mod RECC_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Status"]
pub mod INT_STATUS {
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Status"]
    pub mod CODE_CACHE_ECC_ERRM_INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Error"]
            pub const ECC_ERR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Status"]
    pub mod CODE_CACHE_ECC_ERRS_INT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Error"]
            pub const ECC_ERR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Status"]
    pub mod CODE_CACHE_ECC_ERRM_OVER_INT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not more than one error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Multiple errors"]
            pub const ECC_ERR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Status"]
    pub mod CODE_CACHE_ECC_ERRS_OVER_INT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not more than one error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Multiple errors"]
            pub const ECC_ERR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Status"]
    pub mod SYSTEM_CACHE_ECC_ERRM_INT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Error"]
            pub const ECC_ERR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Status"]
    pub mod SYSTEM_CACHE_ECC_ERRS_INT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Error"]
            pub const ECC_ERR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Status"]
    pub mod SYSTEM_CACHE_ECC_ERRM_OVER_INT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not more than one error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Multiple errors"]
            pub const ECC_ERR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Status"]
    pub mod SYSTEM_CACHE_ECC_ERRS_OVER_INT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not more than one error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Multiple errors"]
            pub const ECC_ERR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Enable"]
pub mod INT_STAT_EN {
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Status Enable"]
    pub mod CODE_CACHE_ERRM_INT_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Status Enable"]
    pub mod CODE_CACHE_ERRS_INT_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Status Enable"]
    pub mod CODE_CACHE_ERRM_OVER_INT_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Status Enable"]
    pub mod CODE_CACHE_ERRS_OVER_INT_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Status Enable"]
    pub mod SYSTEM_CACHE_ECC_ERRM_INT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Status Enable"]
    pub mod SYSTEM_CACHE_ECC_ERRS_INT_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Status Enable"]
    pub mod SYSTEM_CACHE_ECC_ERRM_OVER_INT_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Status Enable"]
    pub mod SYSTEM_CACHE_ECC_ERRS_OVER_INT_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod INT_SIG_EN {
    #[doc = "Code Cache Access Multibit ECC Error Interrupt Signal Enable"]
    pub mod CODE_CACHE_ERRM_INT_SIG_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Access Single-Bit ECC Error Interrupt Signal Enable"]
    pub mod CODE_CACHE_ERRS_INT_SIG_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Access Multiple Multibit ECC Error Interrupt Signal Enable"]
    pub mod CODE_CACHE_ERRM_OVER_INT_SIG_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Access Multiple Single-Bit ECC Error Interrupt Signal Enable"]
    pub mod CODE_CACHE_ERRS_OVER_INT_SIG_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "System Cache Access Multibit ECC Error Interrupt Signal Enable"]
    pub mod SYSTEM_CACHE_ERRM_INT_SIG_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "System Cache Access Single-Bit ECC Error Interrupt Signal Enable"]
    pub mod SYSTEM_CACHE_ERRS_INT_SIG_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "System Cache Access Multiple Multibit ECC Error Interrupt Signal Enable"]
    pub mod SYSTEM_CACHE_ERRM_OVER_INT_SIG_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const MASKED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "System Cache Access Multiple Single-Bit ECC Error Interrupt Signal Enable"]
    pub mod SYSTEM_CACHE_ERRS_OVER_INT_SIG_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const MASKED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "Code Cache Single-Bit ECC Error Information"]
pub mod CODE_CACHE_ECC_SINGLE_ERROR_INFO {
    #[doc = "Code Cache Single-Bit ECC Error"]
    pub mod CODE_CACHE_ECCS_TAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data"]
            pub const DATA: u32 = 0;
            #[doc = "Tag"]
            pub const TAG: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Single-Bit ECC Error on Cache Command"]
    pub mod CODE_CACHE_ECCS_CMD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Error"]
            pub const ECC_ERR: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Single-Bit ECC Error Master Number"]
    pub mod CODE_CACHE_ECCS_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Cache Single-Bit ECC Error Protection"]
    pub mod CODE_CACHE_ECCS_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Cache Single-Bit ECC Error Corresponding Syndrome"]
    pub mod CODE_CACHE_ECCS_EFSYN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Code Cache Single-Bit ECC Error Address"]
pub mod CODE_CACHE_ECC_SINGLE_ERROR_ADDR {
    #[doc = "Code Cache Single-Bit ECC Error Address"]
    pub mod CODE_CACHE_ECCS_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Code Cache Multibit ECC Error Information"]
pub mod CODE_CACHE_ECC_MULTI_ERROR_INFO {
    #[doc = "Code Cache Multibit ECC Error"]
    pub mod CODE_CACHE_ECCM_TAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data"]
            pub const DATA: u32 = 0;
            #[doc = "Tag"]
            pub const TAG: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Multibit ECC Error on Code Cache Command"]
    pub mod CODE_CACHE_ECCM_CMD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Error"]
            pub const ECC_ERR: u32 = 0x01;
        }
    }
    #[doc = "Code Cache Multibit ECC Error Master Number"]
    pub mod CODE_CACHE_ECCM_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Cache Multibit ECC Error Protection"]
    pub mod CODE_CACHE_ECCM_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Cache Multibit ECC Error Corresponding Syndrome"]
    pub mod CODE_CACHE_ECCM_EFSYN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Cache Single-Bit ECC Error Information"]
pub mod SYSTEM_CACHE_ECC_SINGLE_ERROR_INFO {
    #[doc = "System Cache Single-Bit ECC Error"]
    pub mod SYSTEM_CACHE_ECCS_TAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data"]
            pub const DATA: u32 = 0;
            #[doc = "Tag"]
            pub const TAG: u32 = 0x01;
        }
    }
    #[doc = "System Cache Single-Bit ECC Error on Cache Command"]
    pub mod SYSTEM_CACHE_ECCS_CMD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ECC_ERR: u32 = 0;
            #[doc = "Error"]
            pub const ECC_ERR: u32 = 0x01;
        }
    }
    #[doc = "System Cache Single-Bit ECC Error Master Number"]
    pub mod SYSTEM_CACHE_ECCS_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Cache Single-Bit ECC Error Protection"]
    pub mod SYSTEM_CACHE_ECCS_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Cache Single-Bit ECC Error Corresponding Syndrome"]
    pub mod SYSTEM_CACHE_ECCS_EFSYN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Cache Single-Bit ECC Error Address"]
pub mod SYSTEM_CACHE_ECC_SINGLE_ERROR_ADDR {
    #[doc = "System Cache Single-Bit ECC Error Address"]
    pub mod SYSTEM_CACHE_ECCS_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Cache Multibit ECC Error Information"]
pub mod SYSTEM_CACHE_ECC_MULTI_ERROR_INFO {
    #[doc = "System Cache Multibit ECC Error"]
    pub mod SYSTEM_CACHE_ECCM_TAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data"]
            pub const DATA: u32 = 0;
            #[doc = "Tag"]
            pub const TAG: u32 = 0x01;
        }
    }
    #[doc = "System Cache Multibit ECC Error on System Cache Command"]
    pub mod SYSTEM_CACHE_ECCM_CMD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "System Cache Multibit ECC Error Master Number"]
    pub mod SYSTEM_CACHE_ECCM_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Cache Multibit ECC Error Protection"]
    pub mod SYSTEM_CACHE_ECCM_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Cache Multibit ECC Error Corresponding Syndrome"]
    pub mod SYSTEM_CACHE_ECCM_EFSYN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Cache Multibit ECC Error Data"]
pub mod SYSTEM_CACHE_ECC_MULTI_ERROR_DATA {
    #[doc = "System Cache Multibit ECC Error Data"]
    pub mod SYSTEM_CACHE_ECCM_ERRED_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Code Cache TAG0 ECC Error Injection"]
pub mod CODE_CACHE_TAG0_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod CODE_CACHE_TAG0_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod CODE_CACHE_TAG0_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache TAG0 Write Access"]
    pub mod CODE_CACHE_TAG0_FR11BI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache TAG0 Write Access"]
    pub mod CODE_CACHE_TAG0_FR1NCI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache TAG0 Write Access"]
    pub mod CODE_CACHE_TAG0_FRC1BI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache TAG0 Write Access"]
    pub mod CODE_CACHE_TAG0_FRCNCI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Code Cache TAG1 ECC Error Injection"]
pub mod CODE_CACHE_TAG1_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod CODE_CACHE_TAG1_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod CODE_CACHE_TAG1_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache TAG1 Write Access"]
    pub mod CODE_CACHE_TAG1_FR11BI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache TAG1 Write Access"]
    pub mod CODE_CACHE_TAG1_FR1NCI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache TAG1 Write Access"]
    pub mod CODE_CACHE_TAG1_FRC1BI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache TAG1 Write Access"]
    pub mod CODE_CACHE_TAG1_FRCNCI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Code Cache DATA0 ECC Error Injection"]
pub mod CODE_CACHE_DATA0_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod CODE_CACHE_DATA0_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod CODE_CACHE_DATA0_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache DATA0 Write Access"]
    pub mod CODE_CACHE_DATA0_FR11BI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache DATA0 Write Access"]
    pub mod CODE_CACHE_DATA0_FR1NCI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache DATA0 Write Access"]
    pub mod CODE_CACHE_DATA0_FRC1BI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache DATA0 Write Access"]
    pub mod CODE_CACHE_DATA0_FRCNCI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Code Cache DATA1 ECC Error Injection"]
pub mod CODE_CACHE_DATA1_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod CODE_CACHE_DATA1_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod CODE_CACHE_DATA1_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on Code Cache DATA1 Write Access"]
    pub mod CODE_CACHE_DATA1_FR11BI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force One Noncorrectable Data Inversion on Code Cache DATA1 Write Access"]
    pub mod CODE_CACHE_DATA1_FR1NCI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on Code Cache DATA1 Write Access"]
    pub mod CODE_CACHE_DATA1_FRC1BI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code Cache DATA1 Write Access"]
    pub mod CODE_CACHE_DATA1_FRCNCI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "System Cache TAG0 ECC Error Injection"]
pub mod SYTEM_CACHE_TAG0_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod SYSTEM_CACHE_TAG0_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod SYSTEM_CACHE_TAG0_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache TAG0 Write Access"]
    pub mod SYSTEM_CACHE_TAG0_FR11BI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache TAG0 Write Access"]
    pub mod SYSTEM_CACHE_TAG0_FR1NCI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache TAG0 Write Access"]
    pub mod SYSTEM_CACHE_TAG0_FRC1BI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache TAG0 Write Access"]
    pub mod SYSTEM_CACHE_TAG0_FRCNCI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "System Cache TAG1 ECC Error Injection"]
pub mod SYSTEM_CACHE_TAG1_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod SYSTEM_CACHE_TAG1_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod SYSTEMCACHE_TAG1_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache TAG1 Write Access"]
    pub mod SYSTEM_CACHE_TAG1_FR11BI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache TAG1 Write Access"]
    pub mod SYSTEM_CACHE_TAG1_FR1NCI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache TAG1 Write Access"]
    pub mod SYSTEM_CACHE_TAG1_FRC1BI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache TAG1 Write Access"]
    pub mod SYSTEM_CACHE_TAG1_FRCNCI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "System Cache DATA0 ECC Error Injection"]
pub mod SYSTEM_CACHE_DATA0_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod SYSTEM_CACHE_DATA0_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod SYSTEM_CACHE_DATA0_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache DATA0 Write Access"]
    pub mod SYSTEM_CACHE_DATA0_FR11BI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache DATA0 Write Access"]
    pub mod SYSTEM_CACHE_DATA0_FR1NCI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache DATA0 Write Access"]
    pub mod SYSTEM_CACHE_DATA0_FRC1BI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache DATA0 Write Access"]
    pub mod SYSTEM_CACHE_DATA0_FRCNCI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "System Cache DATA1 ECC Error Injection"]
pub mod STSTEM_CACHE_DATA1_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod SYSTEM_CACHE_DATA1_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod SYSTEM_CACHE_DATA1_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on System Cache DATA1 Write Access"]
    pub mod SYSTEM_CACHE_DATA1_FR11BI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force One Noncorrectable Data Inversion on System Cache DATA1 Write Access"]
    pub mod SYSTEM_CACHE_DATA1_FR1NCI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous 1-Bit Data Inversions on System Cache DATA1 Write Access"]
    pub mod SYSTEM_CACHE_DATA1_FRC1BI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Force Continuous Noncorrectable Data Inversions on System Cache DATA1 Write Access"]
    pub mod SYSTEM_CACHE_DATA1_FRCNCI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable injection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable injection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
