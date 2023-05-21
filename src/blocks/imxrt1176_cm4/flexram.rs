#[doc = "FLEXRAM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "TCM CRTL Register"]
    pub TCM_CTRL: crate::RWRegister<u32>,
    #[doc = "OCRAM Magic Address Register"]
    pub OCRAM_MAGIC_ADDR: crate::RWRegister<u32>,
    #[doc = "DTCM Magic Address Register"]
    pub DTCM_MAGIC_ADDR: crate::RWRegister<u32>,
    #[doc = "ITCM Magic Address Register"]
    pub ITCM_MAGIC_ADDR: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Register"]
    pub INT_STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Enable Register"]
    pub INT_STAT_EN: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INT_SIG_EN: crate::RWRegister<u32>,
    #[doc = "OCRAM single-bit ECC Error Information Register"]
    pub OCRAM_ECC_SINGLE_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "OCRAM single-bit ECC Error Address Register"]
    pub OCRAM_ECC_SINGLE_ERROR_ADDR: crate::RORegister<u32>,
    #[doc = "OCRAM single-bit ECC Error Data Register"]
    pub OCRAM_ECC_SINGLE_ERROR_DATA_LSB: crate::RORegister<u32>,
    #[doc = "OCRAM single-bit ECC Error Data Register"]
    pub OCRAM_ECC_SINGLE_ERROR_DATA_MSB: crate::RORegister<u32>,
    #[doc = "OCRAM multi-bit ECC Error Information Register"]
    pub OCRAM_ECC_MULTI_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "OCRAM multi-bit ECC Error Address Register"]
    pub OCRAM_ECC_MULTI_ERROR_ADDR: crate::RORegister<u32>,
    #[doc = "OCRAM multi-bit ECC Error Data Register"]
    pub OCRAM_ECC_MULTI_ERROR_DATA_LSB: crate::RORegister<u32>,
    #[doc = "OCRAM multi-bit ECC Error Data Register"]
    pub OCRAM_ECC_MULTI_ERROR_DATA_MSB: crate::RORegister<u32>,
    #[doc = "ITCM single-bit ECC Error Information Register"]
    pub ITCM_ECC_SINGLE_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "ITCM single-bit ECC Error Address Register"]
    pub ITCM_ECC_SINGLE_ERROR_ADDR: crate::RORegister<u32>,
    #[doc = "ITCM single-bit ECC Error Data Register"]
    pub ITCM_ECC_SINGLE_ERROR_DATA_LSB: crate::RORegister<u32>,
    #[doc = "ITCM single-bit ECC Error Data Register"]
    pub ITCM_ECC_SINGLE_ERROR_DATA_MSB: crate::RORegister<u32>,
    #[doc = "ITCM multi-bit ECC Error Information Register"]
    pub ITCM_ECC_MULTI_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "ITCM multi-bit ECC Error Address Register"]
    pub ITCM_ECC_MULTI_ERROR_ADDR: crate::RORegister<u32>,
    #[doc = "ITCM multi-bit ECC Error Data Register"]
    pub ITCM_ECC_MULTI_ERROR_DATA_LSB: crate::RORegister<u32>,
    #[doc = "ITCM multi-bit ECC Error Data Register"]
    pub ITCM_ECC_MULTI_ERROR_DATA_MSB: crate::RORegister<u32>,
    #[doc = "D0TCM single-bit ECC Error Information Register"]
    pub D0TCM_ECC_SINGLE_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "D0TCM single-bit ECC Error Address Register"]
    pub D0TCM_ECC_SINGLE_ERROR_ADDR: crate::RORegister<u32>,
    #[doc = "D0TCM single-bit ECC Error Data Register"]
    pub D0TCM_ECC_SINGLE_ERROR_DATA: crate::RORegister<u32>,
    #[doc = "D0TCM multi-bit ECC Error Information Register"]
    pub D0TCM_ECC_MULTI_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "D0TCM multi-bit ECC Error Address Register"]
    pub D0TCM_ECC_MULTI_ERROR_ADDR: crate::RORegister<u32>,
    #[doc = "D0TCM multi-bit ECC Error Data Register"]
    pub D0TCM_ECC_MULTI_ERROR_DATA: crate::RORegister<u32>,
    #[doc = "D1TCM single-bit ECC Error Information Register"]
    pub D1TCM_ECC_SINGLE_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "D1TCM single-bit ECC Error Address Register"]
    pub D1TCM_ECC_SINGLE_ERROR_ADDR: crate::RORegister<u32>,
    #[doc = "D1TCM single-bit ECC Error Data Register"]
    pub D1TCM_ECC_SINGLE_ERROR_DATA: crate::RORegister<u32>,
    #[doc = "D1TCM multi-bit ECC Error Information Register"]
    pub D1TCM_ECC_MULTI_ERROR_INFO: crate::RORegister<u32>,
    #[doc = "D1TCM multi-bit ECC Error Address Register"]
    pub D1TCM_ECC_MULTI_ERROR_ADDR: crate::RORegister<u32>,
    #[doc = "D1TCM multi-bit ECC Error Data Register"]
    pub D1TCM_ECC_MULTI_ERROR_DATA: crate::RORegister<u32>,
    _reserved0: [u8; 0x7c],
    #[doc = "FlexRAM feature Control register"]
    pub FLEXRAM_CTRL: crate::RWRegister<u32>,
    #[doc = "OCRAM Pipeline Status register"]
    pub OCRAM_PIPELINE_STATUS: crate::RORegister<u32>,
}
#[doc = "TCM CRTL Register"]
pub mod TCM_CTRL {
    #[doc = "TCM Write Wait Mode Enable"]
    pub mod TCM_WWAIT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
            pub const TCM_WWAIT_EN_0: u32 = 0;
            #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
            pub const TCM_WWAIT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "TCM Read Wait Mode Enable"]
    pub mod TCM_RWAIT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
            pub const TCM_RWAIT_EN_0: u32 = 0;
            #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
            pub const TCM_RWAIT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Force RAM Clock Always On"]
    pub mod FORCE_CLK_ON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM Magic Address Register"]
pub mod OCRAM_MAGIC_ADDR {
    #[doc = "OCRAM Write Read Select"]
    pub mod OCRAM_WR_RD_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When OCRAM read access hits magic address, it will generate interrupt."]
            pub const OCRAM_WR_RD_SEL_0: u32 = 0;
            #[doc = "When OCRAM write access hits magic address, it will generate interrupt."]
            pub const OCRAM_WR_RD_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Magic Address"]
    pub mod OCRAM_MAGIC_ADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DTCM Magic Address Register"]
pub mod DTCM_MAGIC_ADDR {
    #[doc = "DTCM Write Read Select"]
    pub mod DTCM_WR_RD_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When DTCM read access hits magic address, it will generate interrupt."]
            pub const DTCM_WR_RD_SEL_0: u32 = 0;
            #[doc = "When DTCM write access hits magic address, it will generate interrupt."]
            pub const DTCM_WR_RD_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "DTCM Magic Address"]
    pub mod DTCM_MAGIC_ADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM Magic Address Register"]
pub mod ITCM_MAGIC_ADDR {
    #[doc = "ITCM Write Read Select"]
    pub mod ITCM_WR_RD_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When ITCM read access hits magic address, it will generate interrupt."]
            pub const ITCM_WR_RD_SEL_0: u32 = 0;
            #[doc = "When ITCM write access hits magic address, it will generate interrupt."]
            pub const ITCM_WR_RD_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Magic Address"]
    pub mod ITCM_MAGIC_ADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Register"]
pub mod INT_STATUS {
    #[doc = "ITCM Magic Address Match Status"]
    pub mod ITCM_MAM_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ITCM did not access magic address."]
            pub const ITCM_MAM_STATUS_0: u32 = 0;
            #[doc = "ITCM accessed magic address."]
            pub const ITCM_MAM_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "DTCM Magic Address Match Status"]
    pub mod DTCM_MAM_STATUS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DTCM did not access magic address."]
            pub const DTCM_MAM_STATUS_0: u32 = 0;
            #[doc = "DTCM accessed magic address."]
            pub const DTCM_MAM_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Magic Address Match Status"]
    pub mod OCRAM_MAM_STATUS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM did not access magic address."]
            pub const OCRAM_MAM_STATUS_0: u32 = 0;
            #[doc = "OCRAM accessed magic address."]
            pub const OCRAM_MAM_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Access Error Status"]
    pub mod ITCM_ERR_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ITCM access error does not happen"]
            pub const ITCM_ERR_STATUS_0: u32 = 0;
            #[doc = "ITCM access error happens."]
            pub const ITCM_ERR_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "DTCM Access Error Status"]
    pub mod DTCM_ERR_STATUS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DTCM access error does not happen"]
            pub const DTCM_ERR_STATUS_0: u32 = 0;
            #[doc = "DTCM access error happens."]
            pub const DTCM_ERR_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Status"]
    pub mod OCRAM_ERR_STATUS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM access error does not happen"]
            pub const OCRAM_ERR_STATUS_0: u32 = 0;
            #[doc = "OCRAM access error happens."]
            pub const OCRAM_ERR_STATUS_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM access multi-bit ECC Error Interrupt Status"]
    pub mod OCRAM_ECC_ERRM_INT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM multi-bit ECC error does not happen"]
            pub const OCRAM_ECC_ERRM_INT_0: u32 = 0;
            #[doc = "OCRAM multi-bit ECC error happens."]
            pub const OCRAM_ECC_ERRM_INT_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM access single-bit ECC Error Interrupt Status"]
    pub mod OCRAM_ECC_ERRS_INT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM single-bit ECC error does not happen"]
            pub const OCRAM_ECC_ERRS_INT_0: u32 = 0;
            #[doc = "OCRAM single-bit ECC error happens."]
            pub const OCRAM_ECC_ERRS_INT_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Access multi-bit ECC Error Interrupt Status"]
    pub mod ITCM_ECC_ERRM_INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ITCM multi-bit ECC error does not happen"]
            pub const ITCM_ECC_ERRM_INT_0: u32 = 0;
            #[doc = "ITCM multi-bit ECC error happens."]
            pub const ITCM_ECC_ERRM_INT_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM access single-bit ECC Error Interrupt Status"]
    pub mod ITCM_ECC_ERRS_INT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ITCM single-bit ECC error does not happen"]
            pub const ITCM_ECC_ERRS_INT_0: u32 = 0;
            #[doc = "ITCM single-bit ECC error happens."]
            pub const ITCM_ECC_ERRS_INT_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM access multi-bit ECC Error Interrupt Status"]
    pub mod D0TCM_ECC_ERRM_INT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "D0TCM multi-bit ECC error does not happen"]
            pub const D0TCM_ECC_ERRM_INT_0: u32 = 0;
            #[doc = "D0TCM multi-bit ECC error happens."]
            pub const D0TCM_ECC_ERRM_INT_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM access single-bit ECC Error Interrupt Status"]
    pub mod D0TCM_ECC_ERRS_INT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "D0TCM single-bit ECC error does not happen"]
            pub const D0TCM_ECC_ERRS_INT_0: u32 = 0;
            #[doc = "D0TCM single-bit ECC error happens."]
            pub const D0TCM_ECC_ERRS_INT_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM access multi-bit ECC Error Interrupt Status"]
    pub mod D1TCM_ECC_ERRM_INT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "D1TCM multi-bit ECC error does not happen"]
            pub const D1TCM_ECC_ERRM_INT_0: u32 = 0;
            #[doc = "D1TCM multi-bit ECC error happens."]
            pub const D1TCM_ECC_ERRM_INT_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM access single-bit ECC Error Interrupt Status"]
    pub mod D1TCM_ECC_ERRS_INT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "D1TCM single-bit ECC error does not happen"]
            pub const D1TCM_ECC_ERRS_INT_0: u32 = 0;
            #[doc = "D1TCM single-bit ECC error happens."]
            pub const D1TCM_ECC_ERRS_INT_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Partial Write Interrupt Status"]
    pub mod ITCM_PARTIAL_WR_INT_S {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ITCM Partial Write does not happen"]
            pub const ITCM_PARTIAL_WR_INT_S_0: u32 = 0;
            #[doc = "ITCM Partial Write happens."]
            pub const ITCM_PARTIAL_WR_INT_S_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM Partial Write Interrupt Status"]
    pub mod D0TCM_PARTIAL_WR_INT_S {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "D0TCM Partial Write does not happen"]
            pub const D0TCM_PARTIAL_WR_INT_S_0: u32 = 0;
            #[doc = "D0TCM Partial Write happens."]
            pub const D0TCM_PARTIAL_WR_INT_S_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM Partial Write Interrupt Status"]
    pub mod D1TCM_PARTIAL_WR_INT_S {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "D1TCM Partial Write does not happen"]
            pub const D1TCM_PARTIAL_WR_INT_S_0: u32 = 0;
            #[doc = "D1TCM Partial Write happens."]
            pub const D1TCM_PARTIAL_WR_INT_S_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Partial Write Interrupt Status"]
    pub mod OCRAM_PARTIAL_WR_INT_S {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCRAM Partial Write does not happen"]
            pub const OCRAM_PARTIAL_WR_INT_S_0: u32 = 0;
            #[doc = "OCRAM Partial Write happens."]
            pub const OCRAM_PARTIAL_WR_INT_S_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Status Enable Register"]
pub mod INT_STAT_EN {
    #[doc = "ITCM Magic Address Match Status Enable"]
    pub mod ITCM_MAM_STAT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_MAM_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_MAM_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "DTCM Magic Address Match Status Enable"]
    pub mod DTCM_MAM_STAT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DTCM_MAM_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DTCM_MAM_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Magic Address Match Status Enable"]
    pub mod OCRAM_MAM_STAT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_MAM_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_MAM_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Access Error Status Enable"]
    pub mod ITCM_ERR_STAT_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_ERR_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_ERR_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "DTCM Access Error Status Enable"]
    pub mod DTCM_ERR_STAT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DTCM_ERR_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DTCM_ERR_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Status Enable"]
    pub mod OCRAM_ERR_STAT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_ERR_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_ERR_STAT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access multi-bit ECC Error Interrupt Status Enable"]
    pub mod OCRAM_ERRM_INT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_ERRM_INT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_ERRM_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access single-bit ECC Error Interrupt Status Enable"]
    pub mod OCRAM_ERRS_INT_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_ERRS_INT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_ERRS_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Access multi-bit ECC Error Interrupt Status Enable"]
    pub mod ITCM_ERRM_INT_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_ERRM_INT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_ERRM_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Access single-bit ECC Error Interrupt Status Enable"]
    pub mod ITCM_ERRS_INT_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_ERRS_INT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_ERRS_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM Access multi-bit ECC Error Interrupt Status Enable"]
    pub mod D0TCM_ERRM_INT_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D0TCM_ERRM_INT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D0TCM_ERRM_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM Access single-bit ECC Error Interrupt Status Enable"]
    pub mod D0TCM_ERRS_INT_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D0TCM_ERRS_INT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D0TCM_ERRS_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM Access multi-bit ECC Error Interrupt Status Enable"]
    pub mod D1TCM_ERRM_INT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D1TCM_ERRM_INT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D1TCM_ERRM_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM Access single-bit ECC Error Interrupt Status Enable"]
    pub mod D1TCM_ERRS_INT_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D1TCM_ERRS_INT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D1TCM_ERRS_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Partial Write Interrupt Status Enable"]
    pub mod ITCM_PARTIAL_WR_INT_S_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_PARTIAL_WR_INT_S_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_PARTIAL_WR_INT_S_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM Partial Write Interrupt Status Enable"]
    pub mod D0TCM_PARTIAL_WR_INT_S_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D0TCM_PARTIAL_WR_INT_S_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D0TCM_PARTIAL_WR_INT_S_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM Partial Write Interrupt Status EN"]
    pub mod D1TCM_PARTIAL_WR_INT_S_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D1TCM_PARTIAL_WR_INT_S_EN_0: u32 = 0;
            #[doc = "Enbaled"]
            pub const D1TCM_PARTIAL_WR_INT_S_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Partial Write Interrupt Status"]
    pub mod OCRAM_PARTIAL_WR_INT_S_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_PARTIAL_WR_INT_S_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_PARTIAL_WR_INT_S_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INT_SIG_EN {
    #[doc = "ITCM Magic Address Match Interrupt Enable"]
    pub mod ITCM_MAM_SIG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_MAM_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_MAM_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "DTCM Magic Address Match Interrupt Enable"]
    pub mod DTCM_MAM_SIG_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DTCM_MAM_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DTCM_MAM_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Magic Address Match Interrupt Enable"]
    pub mod OCRAM_MAM_SIG_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_MAM_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_MAM_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Access Error Interrupt Enable"]
    pub mod ITCM_ERR_SIG_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_ERR_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_ERR_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "DTCM Access Error Interrupt Enable"]
    pub mod DTCM_ERR_SIG_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DTCM_ERR_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DTCM_ERR_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access Error Interrupt Enable"]
    pub mod OCRAM_ERR_SIG_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_ERR_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_ERR_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access multi-bit ECC Error Interrupt Signal Enable"]
    pub mod OCRAM_ERRM_INT_SIG_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_ERRM_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_ERRM_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Access single-bit ECC Error Interrupt Signal Enable"]
    pub mod OCRAM_ERRS_INT_SIG_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_ERRS_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_ERRS_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Access multi-bit ECC Error Interrupt Signal Enable"]
    pub mod ITCM_ERRM_INT_SIG_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_ERRM_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_ERRM_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Access single-bit ECC Error Interrupt Signal Enable"]
    pub mod ITCM_ERRS_INT_SIG_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_ERRS_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_ERRS_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM Access multi-bit ECC Error Interrupt Signal Enable"]
    pub mod D0TCM_ERRM_INT_SIG_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D0TCM_ERRM_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D0TCM_ERRM_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM Access single-bit ECC Error Interrupt Signal Enable"]
    pub mod D0TCM_ERRS_INT_SIG_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D0TCM_ERRS_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D0TCM_ERRS_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM Access multi-bit ECC Error Interrupt Signal Enable"]
    pub mod D1TCM_ERRM_INT_SIG_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D1TCM_ERRM_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D1TCM_ERRM_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM Access single-bit ECC Error Interrupt Signal Enable"]
    pub mod D1TCM_ERRS_INT_SIG_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D1TCM_ERRS_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D1TCM_ERRS_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "ITCM Partial Write Interrupt Signal Enable Enable"]
    pub mod ITCM_PARTIAL_WR_INT_SIG_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ITCM_PARTIAL_WR_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_PARTIAL_WR_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D0TCM Partial Write Interrupt Signal Enable Enable"]
    pub mod D0TCM_PARTIAL_WR_INT_SIG_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D0TCM_PARTIAL_WR_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const D0TCM_PARTIAL_WR_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "D1TCM Partial Write Interrupt Signal Enable EN"]
    pub mod D1TCM_PARTIAL_WR_INT_SIG_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const D1TCM_PARTIAL_WR_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enbaled"]
            pub const D1TCM_PARTIAL_WR_INT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "OCRAM Partial Write Interrupt Signal Enable"]
    pub mod OCRAM_PARTIAL_WR_INT_SIG_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const OCRAM_PARTIAL_WR_INT_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_PARTIAL_WR_INT_SIG_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "OCRAM single-bit ECC Error Information Register"]
pub mod OCRAM_ECC_SINGLE_ERROR_INFO {
    #[doc = "corresponding ECC cipher of OCRAM single-bit ECC error"]
    pub mod OCRAM_ECCS_ERRED_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "corresponding ECC syndrome of OCRAM single-bit ECC error"]
    pub mod OCRAM_ECCS_ERRED_SYN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM single-bit ECC Error Address Register"]
pub mod OCRAM_ECC_SINGLE_ERROR_ADDR {
    #[doc = "OCRAM single-bit ECC error address"]
    pub mod OCRAM_ECCS_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM single-bit ECC Error Data Register"]
pub mod OCRAM_ECC_SINGLE_ERROR_DATA_LSB {
    #[doc = "OCRAM single-bit ECC error data \\[31:0\\]"]
    pub mod OCRAM_ECCS_ERRED_DATA_LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM single-bit ECC Error Data Register"]
pub mod OCRAM_ECC_SINGLE_ERROR_DATA_MSB {
    #[doc = "OCRAM single-bit ECC error data \\[63:32\\]"]
    pub mod OCRAM_ECCS_ERRED_DATA_MSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM multi-bit ECC Error Information Register"]
pub mod OCRAM_ECC_MULTI_ERROR_INFO {
    #[doc = "OCRAM multi-bit ECC error corresponding ECC value"]
    pub mod OCRAM_ECCM_ERRED_ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM multi-bit ECC Error Address Register"]
pub mod OCRAM_ECC_MULTI_ERROR_ADDR {
    #[doc = "OCRAM multi-bit ECC error address"]
    pub mod OCRAM_ECCM_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM multi-bit ECC Error Data Register"]
pub mod OCRAM_ECC_MULTI_ERROR_DATA_LSB {
    #[doc = "OCRAM multi-bit ECC error data \\[31:0\\]"]
    pub mod OCRAM_ECCM_ERRED_DATA_LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM multi-bit ECC Error Data Register"]
pub mod OCRAM_ECC_MULTI_ERROR_DATA_MSB {
    #[doc = "OCRAM multi-bit ECC error data \\[63:32\\]"]
    pub mod OCRAM_ECCM_ERRED_DATA_MSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM single-bit ECC Error Information Register"]
pub mod ITCM_ECC_SINGLE_ERROR_INFO {
    #[doc = "ITCM single-bit ECC error corresponding TCM_WR value."]
    pub mod ITCM_ECCS_EFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITCM single-bit ECC error corresponding TCM size"]
    pub mod ITCM_ECCS_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITCM single-bit ECC error corresponding TCM_MASTER."]
    pub mod ITCM_ECCS_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITCM single-bit ECC error corresponding TCM_PRIV."]
    pub mod ITCM_ECCS_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITCM single-bit ECC error corresponding syndrome"]
    pub mod ITCM_ECCS_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM single-bit ECC Error Address Register"]
pub mod ITCM_ECC_SINGLE_ERROR_ADDR {
    #[doc = "ITCM single-bit ECC error address"]
    pub mod ITCM_ECCS_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM single-bit ECC Error Data Register"]
pub mod ITCM_ECC_SINGLE_ERROR_DATA_LSB {
    #[doc = "ITCM single-bit ECC error data \\[31:0\\]"]
    pub mod ITCM_ECCS_ERRED_DATA_LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM single-bit ECC Error Data Register"]
pub mod ITCM_ECC_SINGLE_ERROR_DATA_MSB {
    #[doc = "ITCM single-bit ECC error data \\[63:32\\]"]
    pub mod ITCM_ECCS_ERRED_DATA_MSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM multi-bit ECC Error Information Register"]
pub mod ITCM_ECC_MULTI_ERROR_INFO {
    #[doc = "ITCM multi-bit ECC error corresponding TCM_WR value"]
    pub mod ITCM_ECCM_EFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITCM multi-bit ECC error corresponding tcm access size"]
    pub mod ITCM_ECCM_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITCM multi-bit ECC error corresponding TCM_MASTER"]
    pub mod ITCM_ECCM_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITCM multi-bit ECC error corresponding TCM_PRIV"]
    pub mod ITCM_ECCM_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITCM multi-bit ECC error corresponding syndrome"]
    pub mod ITCM_ECCM_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM multi-bit ECC Error Address Register"]
pub mod ITCM_ECC_MULTI_ERROR_ADDR {
    #[doc = "ITCM multi-bit ECC error address"]
    pub mod ITCM_ECCM_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM multi-bit ECC Error Data Register"]
pub mod ITCM_ECC_MULTI_ERROR_DATA_LSB {
    #[doc = "ITCM multi-bit ECC error data \\[31:0\\]"]
    pub mod ITCM_ECCM_ERRED_DATA_LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ITCM multi-bit ECC Error Data Register"]
pub mod ITCM_ECC_MULTI_ERROR_DATA_MSB {
    #[doc = "ITCM multi-bit ECC error data \\[63:32\\]"]
    pub mod ITCM_ECCM_ERRED_DATA_MSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D0TCM single-bit ECC Error Information Register"]
pub mod D0TCM_ECC_SINGLE_ERROR_INFO {
    #[doc = "D0TCM single-bit ECC error corresponding TCM_WR value"]
    pub mod D0TCM_ECCS_EFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D0TCM single-bit ECC error corresponding tcm access size"]
    pub mod D0TCM_ECCS_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D0TCM single-bit ECC error corresponding TCM_MASTER"]
    pub mod D0TCM_ECCS_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D0TCM single-bit ECC error corresponding TCM_PRIV"]
    pub mod D0TCM_ECCS_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D0TCM single-bit ECC error corresponding syndrome"]
    pub mod D0TCM_ECCS_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D0TCM single-bit ECC Error Address Register"]
pub mod D0TCM_ECC_SINGLE_ERROR_ADDR {
    #[doc = "D0TCM single-bit ECC error address"]
    pub mod D0TCM_ECCS_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D0TCM single-bit ECC Error Data Register"]
pub mod D0TCM_ECC_SINGLE_ERROR_DATA {
    #[doc = "D0TCM single-bit ECC error data"]
    pub mod D0TCM_ECCS_ERRED_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D0TCM multi-bit ECC Error Information Register"]
pub mod D0TCM_ECC_MULTI_ERROR_INFO {
    #[doc = "D0TCM multi-bit ECC error corresponding TCM_WR value"]
    pub mod D0TCM_ECCM_EFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D0TCM multi-bit ECC error corresponding tcm access size"]
    pub mod D0TCM_ECCM_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D0TCM multi-bit ECC error corresponding TCM_MASTER"]
    pub mod D0TCM_ECCM_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D0TCM multi-bit ECC error corresponding TCM_PRIV"]
    pub mod D0TCM_ECCM_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D0TCM multi-bit ECC error corresponding syndrome"]
    pub mod D0TCM_ECCM_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D0TCM multi-bit ECC Error Address Register"]
pub mod D0TCM_ECC_MULTI_ERROR_ADDR {
    #[doc = "D0TCM multi-bit ECC error address"]
    pub mod D0TCM_ECCM_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D0TCM multi-bit ECC Error Data Register"]
pub mod D0TCM_ECC_MULTI_ERROR_DATA {
    #[doc = "D0TCM multi-bit ECC error data"]
    pub mod D0TCM_ECCM_ERRED_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D1TCM single-bit ECC Error Information Register"]
pub mod D1TCM_ECC_SINGLE_ERROR_INFO {
    #[doc = "D1TCM single-bit ECC error corresponding TCM_WR value"]
    pub mod D1TCM_ECCS_EFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D1TCM single-bit ECC error corresponding tcm access size"]
    pub mod D1TCM_ECCS_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D1TCM single-bit ECC error corresponding TCM_MASTER"]
    pub mod D1TCM_ECCS_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D1TCM single-bit ECC error corresponding TCM_PRIV"]
    pub mod D1TCM_ECCS_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D1TCM single-bit ECC error corresponding syndrome"]
    pub mod D1TCM_ECCS_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D1TCM single-bit ECC Error Address Register"]
pub mod D1TCM_ECC_SINGLE_ERROR_ADDR {
    #[doc = "D1TCM single-bit ECC error address"]
    pub mod D1TCM_ECCS_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D1TCM single-bit ECC Error Data Register"]
pub mod D1TCM_ECC_SINGLE_ERROR_DATA {
    #[doc = "D1TCM single-bit ECC error data"]
    pub mod D1TCM_ECCS_ERRED_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D1TCM multi-bit ECC Error Information Register"]
pub mod D1TCM_ECC_MULTI_ERROR_INFO {
    #[doc = "D1TCM multi-bit ECC error corresponding TCM_WR value"]
    pub mod D1TCM_ECCM_EFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D1TCM multi-bit ECC error corresponding tcm access size"]
    pub mod D1TCM_ECCM_EFSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D1TCM multi-bit ECC error corresponding TCM_MASTER"]
    pub mod D1TCM_ECCM_EFMST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D1TCM multi-bit ECC error corresponding TCM_PRIV"]
    pub mod D1TCM_ECCM_EFPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D1TCM multi-bit ECC error corresponding syndrome"]
    pub mod D1TCM_ECCM_EFSYN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D1TCM multi-bit ECC Error Address Register"]
pub mod D1TCM_ECC_MULTI_ERROR_ADDR {
    #[doc = "D1TCM multi-bit ECC error address"]
    pub mod D1TCM_ECCM_ERRED_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "D1TCM multi-bit ECC Error Data Register"]
pub mod D1TCM_ECC_MULTI_ERROR_DATA {
    #[doc = "D1TCM multi-bit ECC error data"]
    pub mod D1TCM_ECCM_ERRED_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FlexRAM feature Control register"]
pub mod FLEXRAM_CTRL {
    #[doc = "Read Data Wait Enable"]
    pub mod OCRAM_RDATA_WAIT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Address Pipeline Enable"]
    pub mod OCRAM_RADDR_PIPELINE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data Pipeline Enable"]
    pub mod OCRAM_WRDATA_PIPELINE_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Address Pipeline Enable"]
    pub mod OCRAM_WRADDR_PIPELINE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OCRAM ECC enable"]
    pub mod OCRAM_ECC_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TCM ECC enable"]
    pub mod TCM_ECC_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OCRAM Pipeline Status register"]
pub mod OCRAM_PIPELINE_STATUS {
    #[doc = "Read Data Wait Enable Pending"]
    pub mod OCRAM_RDATA_WAIT_EN_UPDATA_PENDING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Address Pipeline Enable Pending"]
    pub mod OCRAM_RADDR_PIPELINE_EN_UPDATA_PENDING {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data Pipeline Enable Pending"]
    pub mod OCRAM_WRDATA_PIPELINE_EN_UPDATA_PENDING {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Address Pipeline Enable Pending"]
    pub mod OCRAM_WRADDR_PIPELINE_EN_UPDATA_PENDING {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
