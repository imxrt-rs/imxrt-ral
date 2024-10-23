#[doc = "VREF"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "VREF Version ID"]
    pub VERID: crate::RORegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "VREF Control and Status Register"]
    pub CSR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "VREF User Trim"]
    pub UTRIM: crate::RWRegister<u32>,
}
#[doc = "VREF Version ID"]
pub mod VERID {
    #[doc = "FEATURE"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MINOR"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAJOR"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VREF Control and Status Register"]
pub mod CSR {
    #[doc = "High Accuracy Bandgap enabled"]
    pub mod HCBGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HC Bandgap is disabled"]
            pub const HC_BG_DIS: u32 = 0;
            #[doc = "HC Bandgap is enabled"]
            pub const HC_BG_ENB: u32 = 0x01;
        }
    }
    #[doc = "Low Power Bandgap enable"]
    pub mod LPBGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP Bandgap is disabled"]
            pub const LP_BG_DIS: u32 = 0;
            #[doc = "LP Bandgap is enabled"]
            pub const LP_BG_ENB: u32 = 0x01;
        }
    }
    #[doc = "Chop oscillator enable. When set, the internal chopping operation is enabled and the internal analog offset will be minimized."]
    pub mod CHOPEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Chop oscillator is disabled."]
            pub const CHP_OSC_DIS: u32 = 0;
            #[doc = "Chop oscillator is enabled."]
            pub const CHP_OSC_ENB: u32 = 0x01;
        }
    }
    #[doc = "Second order curvature compensation enable"]
    pub mod ICOMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DIS: u32 = 0;
            #[doc = "Enabled"]
            pub const ENB: u32 = 0x01;
        }
    }
    #[doc = "Regulator enable"]
    pub mod REGEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal 1.75 V regulator is disabled."]
            pub const INT_REG_DIS: u32 = 0;
            #[doc = "Internal 1.75 V regulator is enabled."]
            pub const INT_REG_ENB: u32 = 0x01;
        }
    }
    #[doc = "Buffer mode control"]
    pub mod HI_PWR_LV {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "buffer is in low power mode"]
            pub const LOW_MODE: u32 = 0;
            #[doc = "buffer is in high power mode"]
            pub const HIGH_MODE: u32 = 0x01;
        }
    }
    #[doc = "Internal buffer enable"]
    pub mod BUF21EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "buffer is disabled"]
            pub const BUFFER_DIS: u32 = 0;
            #[doc = "buffer is enabled"]
            pub const BUFFER_ENB: u32 = 0x01;
        }
    }
    #[doc = "Internal High Accuracy Voltage Reference stable"]
    pub mod VREFST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The module is disabled or not stable."]
            pub const MOD_DISABLE: u32 = 0;
            #[doc = "The module is stable."]
            pub const MOD_STABLE: u32 = 0x01;
        }
    }
}
#[doc = "VREF User Trim"]
pub mod UTRIM {
    #[doc = "VREF Trim bits"]
    pub mod VREFTRIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Min"]
            pub const MIN: u32 = 0;
            #[doc = "Max-31*(4/3) mV"]
            pub const MAX_31: u32 = 0x01;
            #[doc = "Max"]
            pub const MAX: u32 = 0x3f;
        }
    }
}
