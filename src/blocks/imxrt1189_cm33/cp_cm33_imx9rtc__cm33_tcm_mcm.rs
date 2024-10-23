#[doc = "CM33_TCM_MCM"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "TCM ECC Control"]
    pub TCMECCR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x18],
    #[doc = "Interrupt Status"]
    pub INT_STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Enable"]
    pub INT_STAT_EN: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub INT_SIG_EN: crate::RWRegister<u32>,
    _reserved2: [u8; 0x30],
    #[doc = "Code TCM Single-Bit ECC Error Information"]
    pub CODE_TCM_ECC_SINGLE_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "Code TCM Single-Bit ECC Error Address"]
    pub CODE_TCM_ECC_SINGLE_ERROR_ADDR: crate::RORegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Code TCM Multibit ECC Error Information"]
    pub CODE_TCM_ECC_MULTI_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "Code TCM Multibit ECC Error Address"]
    pub CODE_TCM_ECC_MULTI_ERROR_ADDR: crate::RORegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "System TCM Single-Bit ECC Error Information"]
    pub SYS_TCM_ECC_SINGLE_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "System TCM Single-Bit ECC Error Address"]
    pub SYS_TCM_ECC_SINGLE_ERROR_ADDR: crate::RORegister<u32>,
    _reserved5: [u8; 0x04],
    #[doc = "System TCM Multibit ECC Error Information"]
    pub SYS_TCM_ECC_MULTI_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "System TCM Multibit ECC Error Address"]
    pub SYS_TCM_ECC_MULTI_ERROR_ADDR: crate::RORegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "Code TCM ECC Error Injection"]
    pub CODE_TCM_ECC_ERROR_INJEC: crate::RWRegister<u32>,
    #[doc = "System TCM ECC Error Injection"]
    pub SYS_TCM_ECC_ERROR_INJEC: crate::RWRegister<u32>,
}
#[doc = "TCM ECC Control"]
pub mod TCMECCR {
    #[doc = "TCM ECC Write Generation Disable"]
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
    #[doc = "TCM ECC Read Check Disable"]
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
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Status"]
    pub mod CODE_TCM_ECC_ERRM_INT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error"]
            pub const ENABLE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Status"]
    pub mod CODE_TCM_ECC_ERRS_INT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error"]
            pub const ENABLE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Status"]
    pub mod SYS_TCM_ECC_ERRM_INT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error"]
            pub const ENABLE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Status"]
    pub mod SYS_TCM_ECC_ERRS_INT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error"]
            pub const ENABLE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Enable"]
pub mod INT_STAT_EN {
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Status Enable"]
    pub mod CODE_TCM_ERRM_INT_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Status Enable"]
    pub mod CODE_TCM_ERRS_INT_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Status Enable"]
    pub mod SYS_TCM_ERRM_INT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Status Enable"]
    pub mod SYS_TCM_ERRS_INT_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod INT_SIG_EN {
    #[doc = "Code TCM Access Multibit ECC Error Interrupt Signal Enable"]
    pub mod CODE_TCM_ERRM_INT_SIG_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Code TCM Access Single-Bit ECC Error Interrupt Signal Enable"]
    pub mod CODE_TCM_ERRS_INT_SIG_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "System TCM Access Multibit ECC Error Interrupt Signal Enable"]
    pub mod SYS_TCM_ERRM_INT_SIG_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "System TCM Access Single-Bit ECC Error Interrupt Signal Enable"]
    pub mod SYS_TCM_ERRS_INT_SIG_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mask"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Code TCM Single-Bit ECC Error Information"]
pub mod CODE_TCM_ECC_SINGLE_ERROR_INFO {
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Access Size"]
    pub mod CODE_TCM_ECCS_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Master"]
    pub mod CODE_TCM_ECCS_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code TCM Single-Bit ECC Error for Corresponding TCM Access Protection"]
    pub mod CODE_TCM_ECCS_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code TCM Single-Bit ECC Error Corresponding Syndrome"]
    pub mod CODE_TCM_ECCS_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Code TCM Single-Bit ECC Error Address"]
pub mod CODE_TCM_ECC_SINGLE_ERROR_ADDR {
    #[doc = "Code TCM Single-Bit ECC Error Address"]
    pub mod CODE_TCM_ECCS_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Code TCM Multibit ECC Error Information"]
pub mod CODE_TCM_ECC_MULTI_ERROR_INFO {
    #[doc = "Code TCM Multibit ECC Error for Corresponding TCM Access Size"]
    pub mod CODE_TCM_ECCM_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code TCM Multibit ECC Error for Corresponding TCM Master"]
    pub mod CODE_TCM_ECCM_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CODE_TCM Multibit ECC Error for Corresponding Access Protection Attribute"]
    pub mod CODE_TCM_ECCM_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code TCM Multibit ECC Error for Corresponding Syndrome"]
    pub mod CODE_TCM_ECCM_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Code TCM Multibit ECC Error Address"]
pub mod CODE_TCM_ECC_MULTI_ERROR_ADDR {
    #[doc = "Code TCM Multibit ECC Error Address"]
    pub mod CODE_TCM_ECCM_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System TCM Single-Bit ECC Error Information"]
pub mod SYS_TCM_ECC_SINGLE_ERROR_INFO {
    #[doc = "System TCM Single-Bit ECC Error for Corresponding TCM Access Size"]
    pub mod SYS_TCM_ECCS_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding TCM Master"]
    pub mod SYS_TCM_ECCS_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding Access Protection Attribute"]
    pub mod SYS_TCM_ECCS_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System TCM Single-Bit ECC Error for Corresponding Syndrome"]
    pub mod SYS_TCM_ECCS_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System TCM Single-Bit ECC Error Address"]
pub mod SYS_TCM_ECC_SINGLE_ERROR_ADDR {
    #[doc = "System TCM Single-Bit ECC Error Address"]
    pub mod SYS_TCM_ECCS_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System TCM Multibit ECC Error Information"]
pub mod SYS_TCM_ECC_MULTI_ERROR_INFO {
    #[doc = "System TCM Multibit ECC Error for Corresponding TCM Access Size"]
    pub mod SYS_TCM_ECCM_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding TCM Master"]
    pub mod SYS_TCM_ECCM_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding Access Protection Attribute"]
    pub mod SYS_TCM_ECCM_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System TCM Multibit ECC Error for Corresponding Syndrome"]
    pub mod SYS_TCM_ECCM_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System TCM Multibit ECC Error Address"]
pub mod SYS_TCM_ECC_MULTI_ERROR_ADDR {
    #[doc = "System TCM Multibit ECC Error Address"]
    pub mod SYS_TCM_ECCM_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Code TCM ECC Error Injection"]
pub mod CODE_TCM_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod CODE_TCM_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod CODE_TCM_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on Code TCM Write Access"]
    pub mod CODE_TCM_FR11BI {
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
    #[doc = "Force One Noncorrectable Data Inversion on Code TCM Write Access"]
    pub mod CODE_TCM_FR1NCI {
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
    #[doc = "Force Continuous 1-Bit Data Inversions on Code TCM Write Access"]
    pub mod CODE_TCM_FRC1BI {
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
    #[doc = "Force Continuous Noncorrectable Data Inversions on Code TCM Write Access"]
    pub mod CODE_TCM_FRCNCI {
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
#[doc = "System TCM ECC Error Injection"]
pub mod SYS_TCM_ECC_ERROR_INJEC {
    #[doc = "Position of First Bit to Inject ECC Error"]
    pub mod SYS_TCM_ERR1BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position of Second Bit to Inject ECC Error"]
    pub mod SYS_TCM_ERR2BIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force One 1-Bit Data Inversion on System TCM Write Access"]
    pub mod SYS_TCM_FR11BI {
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
    #[doc = "Force One Noncorrectable Data Inversion on System TCM Write Access"]
    pub mod SYS_TCM_FR1NCI {
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
    #[doc = "Force Continuous 1-Bit Data Inversions on System TCM Write Access"]
    pub mod SYS_TCM_FRC1BI {
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
    #[doc = "Force Continuous Noncorrectable Data Inversions on System TCM Write Access"]
    pub mod SYS_TCM_FRCNCI {
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
