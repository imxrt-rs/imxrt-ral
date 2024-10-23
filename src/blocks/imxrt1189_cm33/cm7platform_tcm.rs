#[doc = "MEM Type I with PSW"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "MPC Control"]
    pub MIF_CTRL: crate::RWRegister<u32>,
    #[doc = "MIF Status"]
    pub MIF_STAT: crate::RORegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "MIF MLPL control of LS"]
    pub MIF_MLPL_LS: crate::RWRegister<u32>,
    #[doc = "MIF Delay of LS"]
    pub MIF_DLY_LS: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "MIF MLPL control of HS"]
    pub MIF_MLPL_HS: crate::RWRegister<u32>,
    #[doc = "MIF Delay of HS"]
    pub MIF_DLY_HS: crate::RWRegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "MIF MLPL control of Input Gating (IG)"]
    pub MIF_MLPL_IG: crate::RWRegister<u32>,
    #[doc = "MIF Delay of IG"]
    pub MIF_DLY_IG: crate::RWRegister<u32>,
    _reserved4: [u8; 0x08],
    #[doc = "MIF MLPL control of STDBY"]
    pub MIF_MLPL_STDBY: crate::RWRegister<u32>,
    #[doc = "MIF Delay of STDBY"]
    pub MIF_DLY_STDBY: crate::RWRegister<u32>,
    _reserved5: [u8; 0x08],
    #[doc = "MIF MLPL control of SLEEP"]
    pub MIF_MLPL_SLEEP: crate::RWRegister<u32>,
    #[doc = "MIF Delay of SLEEP"]
    pub MIF_DLY_SLEEP: crate::RWRegister<u32>,
    _reserved6: [u8; 0x08],
    #[doc = "MIF MLPL control of array power down"]
    pub MIF_MLPL_ARR_PDN: crate::RWRegister<u32>,
    #[doc = "MIF Delay of array high-fanout power switch"]
    pub MIF_DLY_ARR_HF: crate::RWRegister<u32>,
}
#[doc = "MPC Control"]
pub mod MIF_CTRL {
    #[doc = "Memory low power pins controlled by SW"]
    pub mod SW_CTRL_PIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use CURRENT_MLPL field to select MLPL_CTRL to control low power signal."]
            pub const DIS: u32 = 0;
            #[doc = "Use SW_* field to control low power signal directly."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Memory power status will be considered when determining slice power status."]
    pub mod MEM_PWR_ST_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory power status will not be considered when determining slice power status."]
            pub const EN: u32 = 0;
            #[doc = "Memory power status will be considered when determining slice power status."]
            pub const DIS: u32 = 0x01;
        }
    }
    #[doc = "Configuration lock"]
    pub mod LOCK_CFG {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The fields are not locked."]
            pub const DISABLE: u32 = 0;
            #[doc = "The fields are locked."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MIF Status"]
pub mod MIF_STAT {
    #[doc = "Current state of CURRENT_MLPL"]
    pub mod MLPL_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current state of LS"]
    pub mod LS_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LS is 0."]
            pub const DIS: u32 = 0;
            #[doc = "LS is 1."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Current state of HS"]
    pub mod HS_STATE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HS is 0."]
            pub const DIS: u32 = 0;
            #[doc = "HS is 1."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Current state of IG"]
    pub mod IG_STATE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IG is 0."]
            pub const DIS: u32 = 0;
            #[doc = "IG is 1."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Current state of STDBY"]
    pub mod STDBY_STATE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STDBY is 0."]
            pub const DIS: u32 = 0;
            #[doc = "STDBY is 1."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "SLEEP status"]
    pub mod SLEEP_STATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SLEEP is 0."]
            pub const DIS: u32 = 0;
            #[doc = "SLEEP is 1."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "ARR_HF_OFF status"]
    pub mod ARR_HF_STATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ARR_HS is 0."]
            pub const DIS: u32 = 0;
            #[doc = "ARR_HS is 1."]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "MIF MLPL control of LS"]
pub mod MIF_MLPL_LS {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software control LS"]
    pub mod SW_LS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LS is 0."]
            pub const DIS: u32 = 0;
            #[doc = "LS is 1."]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "MIF Delay of LS"]
pub mod MIF_DLY_LS {
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    pub mod PRE_HI_DLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    pub mod PRE_LO_DLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of HS"]
pub mod MIF_MLPL_HS {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software control HS"]
    pub mod SW_HS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HS is 0."]
            pub const DIS: u32 = 0;
            #[doc = "HS is 1."]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "MIF Delay of HS"]
pub mod MIF_DLY_HS {
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    pub mod PRE_HI_DLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    pub mod PRE_LO_DLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of Input Gating (IG)"]
pub mod MIF_MLPL_IG {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software control IG"]
    pub mod SW_IG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IG is 0."]
            pub const DIS: u32 = 0;
            #[doc = "IG is 1."]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "MIF Delay of IG"]
pub mod MIF_DLY_IG {
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    pub mod PRE_HI_DLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    pub mod PRE_LO_DLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of STDBY"]
pub mod MIF_MLPL_STDBY {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software control STDBY"]
    pub mod SW_STDBY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STDBY is 0."]
            pub const DIS: u32 = 0;
            #[doc = "STDBY is 1."]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "MIF Delay of STDBY"]
pub mod MIF_DLY_STDBY {
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field"]
    pub mod PRE_HI_DLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field"]
    pub mod PRE_LO_DLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of SLEEP"]
pub mod MIF_MLPL_SLEEP {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software control SLEEP"]
    pub mod SW_SLEEP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SLEEP is 0."]
            pub const DIS: u32 = 0;
            #[doc = "SLEEP is 1."]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "MIF Delay of SLEEP"]
pub mod MIF_DLY_SLEEP {
    #[doc = "Delay before asserting signal to high, locked by LOCK_CFG field."]
    pub mod PRE_HI_DLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay before de-asserting signal to low, locked by LOCK_CFG field."]
    pub mod PRE_LO_DLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MIF MLPL control of array power down"]
pub mod MIF_MLPL_ARR_PDN {
    #[doc = "Signal behavior at 8 different MLPL settings"]
    pub mod MLPL_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software control arr pdn"]
    pub mod SW_ARR_PDN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ARR_PDN is 0."]
            pub const DIS: u32 = 0;
            #[doc = "ARR_PDN is 1."]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "MIF Delay of array high-fanout power switch"]
pub mod MIF_DLY_ARR_HF {
    #[doc = "Delay before turn off the high-fanout power switch, locked by LOCK_CFG field"]
    pub mod PRE_HI_DLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay before turn on the high-fanout power switch, locked by LOCK_CFG field"]
    pub mod PRE_LO_DLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
