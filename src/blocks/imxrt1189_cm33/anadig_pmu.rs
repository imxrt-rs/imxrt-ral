#[doc = "RT1180_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4600],
    #[doc = "PMU_BIAS_CTRL_REGISTER"]
    pub PMU_BIAS_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "PMU_BIAS_CTRL2_REGISTER"]
    pub PMU_BIAS_CTRL2: crate::RWRegister<u32>,
    _reserved2: [u8; 0x2c],
    #[doc = "PMU_LDO_PLL_REGISTER"]
    pub PMU_LDO_PLL: crate::RWRegister<u32>,
    _reserved3: [u8; 0xbc],
    #[doc = "PMU_POWER_DETECT_CTRL_REGISTER"]
    pub PMU_POWER_DETECT_CTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "PMU_REF_CTRL_REGISTER"]
    pub PMU_REF_CTRL: crate::RWRegister<u32>,
}
#[doc = "PMU_BIAS_CTRL_REGISTER"]
pub mod PMU_BIAS_CTRL {
    #[doc = "wb_cfg_1p8"]
    pub mod WB_CFG_1P8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wb_vdd_sel_1p8"]
    pub mod WB_VDD_SEL_1P8 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VDD_LV1"]
            pub const LV1: u32 = 0;
            #[doc = "VDD_LV2"]
            pub const LV2: u32 = 0x01;
        }
    }
    #[doc = "standby enable bit of fbb m7"]
    pub mod FBB_M7_STBY_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FBB_M7 will be still on when gpc give standby request. After the mode is switched to gpc mode, keep this bit as it is."]
            pub const ENABLE: u32 = 0;
            #[doc = "FBB_M7 will standby when gpc give standby request."]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "wb_pw_lvl_1p8"]
    pub mod WB_PW_LVL_1P8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wb_nw_lvl_1p8"]
    pub mod WB_NW_LVL_1P8 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PMU_BIAS_CTRL2_REGISTER"]
pub mod PMU_BIAS_CTRL2 {
    #[doc = "MODSEL_wb_tst_md_1p8"]
    pub mod WB_PWR_SW_EN_1P8 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WELL connected to no back-biasing power supplies"]
            pub const BB012: u32 = 0;
            #[doc = "WELL connected to back biasing generators."]
            pub const BB0: u32 = 0x01;
        }
    }
    #[doc = "wb_adj_1p8"]
    pub mod WB_ADJ_1P8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cref= 0fF Cspl= 0fF DeltaC= 0fF"]
            pub const WB00000000: u32 = 0;
            #[doc = "Cref= 0fF Cspl= 30fF DeltaC= -30fF"]
            pub const WB00000001: u32 = 0x01;
            #[doc = "Cref= 0fF Cspl= 43fF DeltaC= -43fF"]
            pub const WB00000010: u32 = 0x02;
            #[doc = "Cref= 0fF Cspl= 62fF DeltaC=-62fF"]
            pub const WB00000011: u32 = 0x03;
            #[doc = "Cref= 0fF Cspl=105fF DeltaC=-105fF"]
            pub const WB00000100: u32 = 0x04;
            #[doc = "Cref= 30fF Cspl= 0fF DeltaC= 30fF"]
            pub const WB00000101: u32 = 0x05;
            #[doc = "Cref= 30fF Cspl= 43fF DeltaC= -12fF"]
            pub const WB00000110: u32 = 0x06;
            #[doc = "Cref= 30fF Cspl=105fF DeltaC= -75fF"]
            pub const WB00000111: u32 = 0x07;
            #[doc = "Cref= 43fF Cspl= 0fF DeltaC= 43fF"]
            pub const WB00001000: u32 = 0x08;
            #[doc = "Cref= 43fF Cspl= 30fF DeltaC= 13fF"]
            pub const WB00001001: u32 = 0x09;
            #[doc = "Cref= 43fF Cspl= 62fF DeltaC= -19fF"]
            pub const WB00001010: u32 = 0x0a;
            #[doc = "Cref= 62fF Cspl= 0fF DeltaC= 62fF"]
            pub const WB00001011: u32 = 0x0b;
            #[doc = "Cref= 62fF Cspl= 43fF DeltaC= 19fF"]
            pub const WB00001100: u32 = 0x0c;
            #[doc = "Cref=105fF Cspl= 0fF DeltaC= 105fF"]
            pub const WB00001101: u32 = 0x0d;
            #[doc = "Cref=105fF Cspl=30fF DeltaC= 75fF"]
            pub const WB00001110: u32 = 0x0e;
            #[doc = "Cref=0fF Cspl=0fF DeltaC= 0fF"]
            pub const WB00001111: u32 = 0x0f;
        }
    }
    #[doc = "wb_en"]
    pub mod WB_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Digital Output pin."]
    pub mod WB_OK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Regulator is unstable."]
            pub const UNSTABLE: u32 = 0;
            #[doc = "Regulator is stable."]
            pub const STABLE: u32 = 0x01;
        }
    }
}
#[doc = "PMU_LDO_PLL_REGISTER"]
pub mod PMU_LDO_PLL {
    #[doc = "LDO_PLL_CONTROL_MODE"]
    pub mod LDO_PLL_CONTROL_MODE {
        pub const offset: u32 = 1;
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
    #[doc = "standby enable bit of ldopll"]
    pub mod LDO_PLL_STBY_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "phy_ldo will be still on when gpc gives standby request. After the mode is switched to gpc mode, keep this bit as it is."]
            pub const DISABLE: u32 = 0;
            #[doc = "phy_ldo will standby when gpc give standby request"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "PMU_POWER_DETECT_CTRL_REGISTER"]
pub mod PMU_POWER_DETECT_CTRL {
    #[doc = "ckgb_aon1p0"]
    pub mod CKGB_AON1P0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "To disable lpsr_1p0, ckgb need to be disabled first."]
            pub const DISABLE: u32 = 0;
            #[doc = "After lpsr_1p0 start up, wait for 1ms and then set ckgb bit."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "PMU_REF_CTRL_REGISTER"]
pub mod PMU_REF_CTRL {
    #[doc = "REF_CONTROL_MODE"]
    pub mod REF_CONTROL_MODE {
        pub const offset: u32 = 3;
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
    #[doc = "en_pll_vol_ref_buffer"]
    pub mod EN_PLL_VOL_REF_BUFFER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "standby enable bit of reftop"]
    pub mod REF_STBY_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
