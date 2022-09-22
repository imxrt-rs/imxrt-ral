#[doc = "MX6RT_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0510],
    #[doc = "PMU_LDO_LPSR_ANA_REGISTER"]
    pub PMU_LDO_LPSR_ANA: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "PMU_LDO_LPSR_DIG_2_REGISTER"]
    pub PMU_LDO_LPSR_DIG_2: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "PMU_LDO_LPSR_DIG_REGISTER"]
    pub PMU_LDO_LPSR_DIG: crate::RWRegister<u32>,
}
#[doc = "PMU_LDO_LPSR_ANA_REGISTER"]
pub mod PMU_LDO_LPSR_ANA {
    #[doc = "reg_lp_en"]
    pub mod REG_LP_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_disable"]
    pub mod REG_DISABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pull_down_2ma_en"]
    pub mod PULL_DOWN_2MA_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSR_ANA_CONTROL_MODE"]
    pub mod LPSR_ANA_CONTROL_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SW Control"]
            pub const SW: u32 = 0;
            #[doc = "HW Control"]
            pub const HW: u32 = 0x01;
        }
    }
    #[doc = "bypass_mode_en"]
    pub mod BYPASS_MODE_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "standby_en"]
    pub mod STANDBY_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "always_4ma_pulldown_en"]
    pub mod ALWAYS_4MA_PULLDOWN_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Track Mode Enable"]
    pub mod TRACK_MODE_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal use"]
            pub const NORMAL: u32 = 0;
            #[doc = "Switch preparation"]
            pub const SWITCH: u32 = 0x01;
        }
    }
    #[doc = "pull_down_20ua_en"]
    pub mod PULL_DOWN_20UA_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PMU_LDO_LPSR_DIG_2_REGISTER"]
pub mod PMU_LDO_LPSR_DIG_2 {
    #[doc = "voltage_step_inc"]
    pub mod VOLTAGE_STEP_INC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PMU_LDO_LPSR_DIG_REGISTER"]
pub mod PMU_LDO_LPSR_DIG {
    #[doc = "ENABLE_ILIMIT"]
    pub mod REG_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSR_DIG_CONTROL_MODE"]
    pub mod LPSR_DIG_CONTROL_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SW Control"]
            pub const SW: u32 = 0;
            #[doc = "HW Control"]
            pub const HW: u32 = 0x01;
        }
    }
    #[doc = "standby_en"]
    pub mod STANDBY_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tracking_mode"]
    pub mod TRACKING_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bypass_mode"]
    pub mod BYPASS_MODE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SELECT"]
    pub mod VOLTAGE_SELECT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL0: u32 = 0;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL1: u32 = 0x01;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL2: u32 = 0x02;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL3: u32 = 0x03;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL4: u32 = 0x04;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL5: u32 = 0x05;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL6: u32 = 0x06;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL7: u32 = 0x07;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL8: u32 = 0x08;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL9: u32 = 0x09;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL10: u32 = 0x0a;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL11: u32 = 0x0b;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL12: u32 = 0x0c;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL13: u32 = 0x0d;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL14: u32 = 0x0e;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL15: u32 = 0x0f;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL16: u32 = 0x10;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL17: u32 = 0x11;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL18: u32 = 0x12;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL19: u32 = 0x13;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL20: u32 = 0x14;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL21: u32 = 0x15;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL22: u32 = 0x16;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL23: u32 = 0x17;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL24: u32 = 0x18;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL25: u32 = 0x19;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL26: u32 = 0x1a;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL27: u32 = 0x1b;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL28: u32 = 0x1c;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL29: u32 = 0x1d;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL30: u32 = 0x1e;
            #[doc = "Stable Voltage (range)"]
            pub const BITVAL31: u32 = 0x1f;
        }
    }
}
