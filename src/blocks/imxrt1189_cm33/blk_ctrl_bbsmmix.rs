#[doc = "blk_ctrl_bbsmmix"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "BBSM miscellaneous register"]
    pub BBSM_MISC: crate::RWRegister<u32>,
    #[doc = "BBSM TRIM register"]
    pub BBSM_TRIM: crate::RWRegister<u32>,
}
#[doc = "BBSM miscellaneous register"]
pub mod BBSM_MISC {
    #[doc = "LDO_BBSM_ANA bypass enable"]
    pub mod BBSM_BYPASS_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable bypass"]
            pub const NO: u32 = 0;
            #[doc = "Enable bypass"]
            pub const OVER: u32 = 0x01;
        }
    }
    #[doc = "32K OSC ok flag"]
    pub mod BBSM_XTAL_CLK_OK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32K oscillator is NOT stable into normal operation"]
            pub const UNSTABLE: u32 = 0;
            #[doc = "32K oscillator is stable into normal operation"]
            pub const STABLE: u32 = 0x01;
        }
    }
}
#[doc = "BBSM TRIM register"]
pub mod BBSM_TRIM {
    #[doc = "BBSM core voltage detect trim select"]
    pub mod BBSM_CORE_VOLT_DET_TRIM_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The trimming codes are selected from eFuse"]
            pub const OVER1: u32 = 0;
            #[doc = "The trimming codes of core voltage detectors used to change the voltage falling trip point are selected from BBSM_CORE_VOLT_DET_TRIM"]
            pub const NO1: u32 = 0x01;
        }
    }
    #[doc = "BBSM core voltage detect trim"]
    pub mod BBSM_CORE_VOLT_DET_TRIM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BBSM OSC load capacitor trim select"]
    pub mod BBSM_CAP_TRIM_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The trimming codes are selected from eFuse"]
            pub const OVER: u32 = 0;
            #[doc = "The trimming codes are used from BBSM_OSC_CAP_TRIM (osc32k's load capacitor)"]
            pub const NO: u32 = 0x01;
        }
    }
    #[doc = "BBSM OSC load capacitor trim"]
    pub mod BBSM_OSC_CAP_TRIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
