#[doc = "PGMC_MIF"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "MIF Authentication Control"]
    pub MIF_AUTHEN_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "MIF MLPL control of SLEEP"]
    pub MIF_MLPL_SLEEP: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "MIF MLPL control of IG"]
    pub MIF_MLPL_IG: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "MIF MLPL control of LS"]
    pub MIF_MLPL_LS: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "MIF MLPL control of HS"]
    pub MIF_MLPL_HS: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "MIF MLPL control of STDBY"]
    pub MIF_MLPL_STDBY: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "MIF MLPL control of array power down"]
    pub MIF_MLPL_ARR_PDN: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "MIF MLPL control of peripheral power down"]
    pub MIF_MLPL_PER_PDN: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "MIF MLPL control of INITN"]
    pub MIF_MLPL_INITN: crate::RWRegister<u32>,
    _reserved9: [u8; 0x2c],
    #[doc = "MIF MLPL control of isolation enable"]
    pub MIF_MLPL_ISO: crate::RWRegister<u32>,
}
#[doc = "MIF Authentication Control"]
pub mod MIF_AUTHEN_CTRL {
    #[doc = "Configuration lock"]
    pub mod LOCK_CFG {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of SLEEP"]
pub mod MIF_MLPL_SLEEP {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of IG"]
pub mod MIF_MLPL_IG {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of LS"]
pub mod MIF_MLPL_LS {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of HS"]
pub mod MIF_MLPL_HS {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of STDBY"]
pub mod MIF_MLPL_STDBY {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of array power down"]
pub mod MIF_MLPL_ARR_PDN {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of peripheral power down"]
pub mod MIF_MLPL_PER_PDN {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of INITN"]
pub mod MIF_MLPL_INITN {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass vdd_ok. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod BYPASS_VDD_OK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of isolation enable"]
pub mod MIF_MLPL_ISO {
    #[doc = "Signal behavior at each MLPL"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
