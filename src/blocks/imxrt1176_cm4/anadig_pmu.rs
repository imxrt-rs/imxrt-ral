#[doc = "MX6RT_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    #[doc = "PMU_LDO_PLL_REGISTER"]
    pub PMU_LDO_PLL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4c],
    #[doc = "PMU_BIAS_CTRL_REGISTER"]
    pub PMU_BIAS_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "PMU_BIAS_CTRL2_REGISTER"]
    pub PMU_BIAS_CTRL2: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "PMU_REF_CTRL_REGISTER"]
    pub PMU_REF_CTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "PMU_POWER_DETECT_CTRL_REGISTER"]
    pub PMU_POWER_DETECT_CTRL: crate::RWRegister<u32>,
    _reserved5: [u8; 0x7c],
    #[doc = "LDO_PLL_ENABLE_SP_REGISTER"]
    pub LDO_PLL_ENABLE_SP: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "LDO_LPSR_ANA_ENABLE_SP_REGISTER"]
    pub LDO_LPSR_ANA_ENABLE_SP: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "LDO_LPSR_ANA_LP_MODE_SP_REGISTER"]
    pub LDO_LPSR_ANA_LP_MODE_SP: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "LDO_LPSR_ANA_TRACKING_EN_SP_REGISTER"]
    pub LDO_LPSR_ANA_TRACKING_EN_SP: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "LDO_LPSR_ANA_BYPASS_EN_SP_REGISTER"]
    pub LDO_LPSR_ANA_BYPASS_EN_SP: crate::RWRegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "LDO_LPSR_ANA_STBY_EN_SP_REGISTER"]
    pub LDO_LPSR_ANA_STBY_EN_SP: crate::RWRegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_ENABLE_SP_REGISTER"]
    pub LDO_LPSR_DIG_ENABLE_SP: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_TRG_SP0_REGISTER"]
    pub LDO_LPSR_DIG_TRG_SP0: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_TRG_SP1_REGISTER"]
    pub LDO_LPSR_DIG_TRG_SP1: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_TRG_SP2_REGISTER"]
    pub LDO_LPSR_DIG_TRG_SP2: crate::RWRegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_TRG_SP3_REGISTER"]
    pub LDO_LPSR_DIG_TRG_SP3: crate::RWRegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_LP_MODE_SP_REGISTER"]
    pub LDO_LPSR_DIG_LP_MODE_SP: crate::RWRegister<u32>,
    _reserved17: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_TRACKING_EN_SP_REGISTER"]
    pub LDO_LPSR_DIG_TRACKING_EN_SP: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_BYPASS_EN_SP_REGISTER"]
    pub LDO_LPSR_DIG_BYPASS_EN_SP: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0c],
    #[doc = "LDO_LPSR_DIG_STBY_EN_SP_REGISTER"]
    pub LDO_LPSR_DIG_STBY_EN_SP: crate::RWRegister<u32>,
    _reserved20: [u8; 0x0c],
    #[doc = "BANDGAP_ENABLE_SP_REGISTER"]
    pub BANDGAP_ENABLE_SP: crate::RWRegister<u32>,
    _reserved21: [u8; 0x0c],
    #[doc = "FBB_M7_ENABLE_SP_REGISTER"]
    pub FBB_M7_ENABLE_SP: crate::RWRegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "RBB_SOC_ENABLE_SP_REGISTER"]
    pub RBB_SOC_ENABLE_SP: crate::RWRegister<u32>,
    _reserved23: [u8; 0x0c],
    #[doc = "RBB_LPSR_ENABLE_SP_REGISTER"]
    pub RBB_LPSR_ENABLE_SP: crate::RWRegister<u32>,
    _reserved24: [u8; 0x0c],
    #[doc = "BANDGAP_STBY_EN_SP_REGISTER"]
    pub BANDGAP_STBY_EN_SP: crate::RWRegister<u32>,
    _reserved25: [u8; 0x0c],
    #[doc = "PLL_LDO_STBY_EN_SP_REGISTER"]
    pub PLL_LDO_STBY_EN_SP: crate::RWRegister<u32>,
    _reserved26: [u8; 0x0c],
    #[doc = "FBB_M7_STBY_EN_SP_REGISTER"]
    pub FBB_M7_STBY_EN_SP: crate::RWRegister<u32>,
    _reserved27: [u8; 0x0c],
    #[doc = "RBB_SOC_STBY_EN_SP_REGISTER"]
    pub RBB_SOC_STBY_EN_SP: crate::RWRegister<u32>,
    _reserved28: [u8; 0x0c],
    #[doc = "RBB_LPSR_STBY_EN_SP_REGISTER"]
    pub RBB_LPSR_STBY_EN_SP: crate::RWRegister<u32>,
    _reserved29: [u8; 0x0c],
    #[doc = "FBB_M7_CONFIGURE_REGISTER"]
    pub FBB_M7_CONFIGURE: crate::RWRegister<u32>,
    _reserved30: [u8; 0x0c],
    #[doc = "RBB_LPSR_CONFIGURE_REGISTER"]
    pub RBB_LPSR_CONFIGURE: crate::RWRegister<u32>,
    _reserved31: [u8; 0x0c],
    #[doc = "RBB_SOC_CONFIGURE_REGISTER"]
    pub RBB_SOC_CONFIGURE: crate::RWRegister<u32>,
    _reserved32: [u8; 0x0c],
    #[doc = "REFTOP_OTP_TRIM_VALUE_REGISTER"]
    pub REFTOP_OTP_TRIM_VALUE: crate::RORegister<u32>,
    _reserved33: [u8; 0x1c],
    #[doc = "LPSR_1P8_LDO_OTP_TRIM_VALUE_REGISTER"]
    pub LPSR_1P8_LDO_OTP_TRIM_VALUE: crate::RORegister<u32>,
}
#[doc = "PMU_LDO_PLL_REGISTER"]
pub mod PMU_LDO_PLL {
    #[doc = "LDO_PLL_ENABLE"]
    pub mod LDO_PLL_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
    #[doc = "ldo_pll_ai_toggle"]
    pub mod LDO_PLL_AI_TOGGLE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ldo_pll_busy"]
    pub mod LDO_PLL_AI_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
}
#[doc = "PMU_BIAS_CTRL2_REGISTER"]
pub mod PMU_BIAS_CTRL2 {
    #[doc = "TMOD_wb_tst_md_1p8"]
    pub mod WB_TST_MD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MODSEL_wb_tst_md_1p8"]
    pub mod WB_PWR_SW_EN_1P8 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No BB"]
            pub const BB0: u32 = 0x01;
            #[doc = "BB"]
            pub const BB1: u32 = 0x02;
            #[doc = "BB"]
            pub const BB2: u32 = 0x04;
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
    #[doc = "FBB_M7_CONTROL_MODE"]
    pub mod FBB_M7_CONTROL_MODE {
        pub const offset: u32 = 21;
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
    #[doc = "RBB_SOC_CONTROL_MODE"]
    pub mod RBB_SOC_CONTROL_MODE {
        pub const offset: u32 = 22;
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
    #[doc = "RBB_LPSR_CONTROL_MODE"]
    pub mod RBB_LPSR_CONTROL_MODE {
        pub const offset: u32 = 23;
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
    #[doc = "wb_en"]
    pub mod WB_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Digital output"]
    pub mod WB_TST_DIG_OUT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Digital Output pin."]
    pub mod WB_OK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PMU_REF_CTRL_REGISTER"]
pub mod PMU_REF_CTRL {
    #[doc = "ref_ai_toggle"]
    pub mod REF_AI_TOGGLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ref_ai_busy"]
    pub mod REF_AI_BUSY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REF_ENABLE"]
    pub mod REF_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
}
#[doc = "PMU_POWER_DETECT_CTRL_REGISTER"]
pub mod PMU_POWER_DETECT_CTRL {
    #[doc = "ckgb_lpsr1p0"]
    pub mod CKGB_LPSR1P0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LDO_PLL_ENABLE_SP_REGISTER"]
pub mod LDO_PLL_ENABLE_SP {
    #[doc = "ON_OFF_SETPOINT0"]
    pub mod ON_OFF_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT1"]
    pub mod ON_OFF_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT2"]
    pub mod ON_OFF_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT3"]
    pub mod ON_OFF_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT4"]
    pub mod ON_OFF_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT5"]
    pub mod ON_OFF_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT6"]
    pub mod ON_OFF_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT7"]
    pub mod ON_OFF_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT8"]
    pub mod ON_OFF_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT9"]
    pub mod ON_OFF_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT10"]
    pub mod ON_OFF_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT11"]
    pub mod ON_OFF_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT12"]
    pub mod ON_OFF_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT13"]
    pub mod ON_OFF_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT14"]
    pub mod ON_OFF_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT15"]
    pub mod ON_OFF_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_ANA_ENABLE_SP_REGISTER"]
pub mod LDO_LPSR_ANA_ENABLE_SP {
    #[doc = "ON_OFF_SETPOINT0"]
    pub mod ON_OFF_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT1"]
    pub mod ON_OFF_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT2"]
    pub mod ON_OFF_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT3"]
    pub mod ON_OFF_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT4"]
    pub mod ON_OFF_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT5"]
    pub mod ON_OFF_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT6"]
    pub mod ON_OFF_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT7"]
    pub mod ON_OFF_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT8"]
    pub mod ON_OFF_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT9"]
    pub mod ON_OFF_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT10"]
    pub mod ON_OFF_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT11"]
    pub mod ON_OFF_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT12"]
    pub mod ON_OFF_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT13"]
    pub mod ON_OFF_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT14"]
    pub mod ON_OFF_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT15"]
    pub mod ON_OFF_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_ANA_LP_MODE_SP_REGISTER"]
pub mod LDO_LPSR_ANA_LP_MODE_SP {
    #[doc = "LP_MODE_SETPOINT0"]
    pub mod LP_MODE_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT1"]
    pub mod LP_MODE_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT2"]
    pub mod LP_MODE_SETPONIT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT3"]
    pub mod LP_MODE_SETPONIT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT4"]
    pub mod LP_MODE_SETPONIT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT5"]
    pub mod LP_MODE_SETPONIT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT6"]
    pub mod LP_MODE_SETPONIT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT7"]
    pub mod LP_MODE_SETPONIT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT8"]
    pub mod LP_MODE_SETPONIT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT9"]
    pub mod LP_MODE_SETPONIT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT10"]
    pub mod LP_MODE_SETPONIT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT11"]
    pub mod LP_MODE_SETPONIT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT12"]
    pub mod LP_MODE_SETPONIT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT13"]
    pub mod LP_MODE_SETPONIT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT14"]
    pub mod LP_MODE_SETPONIT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT15"]
    pub mod LP_MODE_SETPONIT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_ANA_TRACKING_EN_SP_REGISTER"]
pub mod LDO_LPSR_ANA_TRACKING_EN_SP {
    #[doc = "TRACKING_EN_SETPOINT0"]
    pub mod TRACKING_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT1"]
    pub mod TRACKING_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT2"]
    pub mod TRACKING_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT3"]
    pub mod TRACKING_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT4"]
    pub mod TRACKING_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT5"]
    pub mod TRACKING_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT6"]
    pub mod TRACKING_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT7"]
    pub mod TRACKING_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT8"]
    pub mod TRACKING_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT9"]
    pub mod TRACKING_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT10"]
    pub mod TRACKING_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT11"]
    pub mod TRACKING_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT12"]
    pub mod TRACKING_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT13"]
    pub mod TRACKING_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT14"]
    pub mod TRACKING_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT15"]
    pub mod TRACKING_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_ANA_BYPASS_EN_SP_REGISTER"]
pub mod LDO_LPSR_ANA_BYPASS_EN_SP {
    #[doc = "BYPASS_EN_SETPOINT0"]
    pub mod BYPASS_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT1"]
    pub mod BYPASS_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT2"]
    pub mod BYPASS_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT3"]
    pub mod BYPASS_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT4"]
    pub mod BYPASS_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT5"]
    pub mod BYPASS_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT6"]
    pub mod BYPASS_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT7"]
    pub mod BYPASS_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT"]
    pub mod BYPASS_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT9"]
    pub mod BYPASS_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT10"]
    pub mod BYPASS_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT11"]
    pub mod BYPASS_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT12"]
    pub mod BYPASS_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT13"]
    pub mod BYPASS_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT14"]
    pub mod BYPASS_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT15"]
    pub mod BYPASS_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_ANA_STBY_EN_SP_REGISTER"]
pub mod LDO_LPSR_ANA_STBY_EN_SP {
    #[doc = "STBY_EN_SETPOINT0"]
    pub mod STBY_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT1"]
    pub mod STBY_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT2"]
    pub mod STBY_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT3"]
    pub mod STBY_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT4"]
    pub mod STBY_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT5"]
    pub mod STBY_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT6"]
    pub mod STBY_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT7"]
    pub mod STBY_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT8"]
    pub mod STBY_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT9"]
    pub mod STBY_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT10"]
    pub mod STBY_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT11"]
    pub mod STBY_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT12"]
    pub mod STBY_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT13"]
    pub mod STBY_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT14"]
    pub mod STBY_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT15"]
    pub mod STBY_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_DIG_ENABLE_SP_REGISTER"]
pub mod LDO_LPSR_DIG_ENABLE_SP {
    #[doc = "ON_OFF_SETPOINT0"]
    pub mod ON_OFF_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT1"]
    pub mod ON_OFF_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT2"]
    pub mod ON_OFF_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT3"]
    pub mod ON_OFF_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT4"]
    pub mod ON_OFF_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT5"]
    pub mod ON_OFF_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT6"]
    pub mod ON_OFF_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT7"]
    pub mod ON_OFF_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT8"]
    pub mod ON_OFF_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT9"]
    pub mod ON_OFF_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT10"]
    pub mod ON_OFF_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT11"]
    pub mod ON_OFF_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT12"]
    pub mod ON_OFF_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT13"]
    pub mod ON_OFF_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT14"]
    pub mod ON_OFF_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT15"]
    pub mod ON_OFF_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_DIG_TRG_SP0_REGISTER"]
pub mod LDO_LPSR_DIG_TRG_SP0 {
    #[doc = "VOLTAGE_SETPOINT0"]
    pub mod VOLTAGE_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT1"]
    pub mod VOLTAGE_SETPOINT1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT2"]
    pub mod VOLTAGE_SETPOINT2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT3"]
    pub mod VOLTAGE_SETPOINT3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LDO_LPSR_DIG_TRG_SP1_REGISTER"]
pub mod LDO_LPSR_DIG_TRG_SP1 {
    #[doc = "VOLTAGE_SETPOINT4"]
    pub mod VOLTAGE_SETPOINT4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT5"]
    pub mod VOLTAGE_SETPOINT5 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT6"]
    pub mod VOLTAGE_SETPOINT6 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT7"]
    pub mod VOLTAGE_SETPOINT7 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LDO_LPSR_DIG_TRG_SP2_REGISTER"]
pub mod LDO_LPSR_DIG_TRG_SP2 {
    #[doc = "VOLTAGE_SETPOINT8"]
    pub mod VOLTAGE_SETPOINT8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT9"]
    pub mod VOLTAGE_SETPOINT9 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT10"]
    pub mod VOLTAGE_SETPOINT10 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT11"]
    pub mod VOLTAGE_SETPOINT11 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LDO_LPSR_DIG_TRG_SP3_REGISTER"]
pub mod LDO_LPSR_DIG_TRG_SP3 {
    #[doc = "VOLTAGE_SETPOINT12"]
    pub mod VOLTAGE_SETPOINT12 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT13"]
    pub mod VOLTAGE_SETPOINT13 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT14"]
    pub mod VOLTAGE_SETPOINT14 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOLTAGE_SETPOINT15"]
    pub mod VOLTAGE_SETPOINT15 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LDO_LPSR_DIG_LP_MODE_SP_REGISTER"]
pub mod LDO_LPSR_DIG_LP_MODE_SP {
    #[doc = "LP_MODE_SETPOINT0"]
    pub mod LP_MODE_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT1"]
    pub mod LP_MODE_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT2"]
    pub mod LP_MODE_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT3"]
    pub mod LP_MODE_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT4"]
    pub mod LP_MODE_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT5"]
    pub mod LP_MODE_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT6"]
    pub mod LP_MODE_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT7"]
    pub mod LP_MODE_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT8"]
    pub mod LP_MODE_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT9"]
    pub mod LP_MODE_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT10"]
    pub mod LP_MODE_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT11"]
    pub mod LP_MODE_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT12"]
    pub mod LP_MODE_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT13"]
    pub mod LP_MODE_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT14"]
    pub mod LP_MODE_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
    #[doc = "LP_MODE_SETPOINT15"]
    pub mod LP_MODE_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP"]
            pub const LP0: u32 = 0;
            #[doc = "HP"]
            pub const HP1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_DIG_TRACKING_EN_SP_REGISTER"]
pub mod LDO_LPSR_DIG_TRACKING_EN_SP {
    #[doc = "TRACKING_EN_SETPOINT0"]
    pub mod TRACKING_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT1"]
    pub mod TRACKING_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT2"]
    pub mod TRACKING_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT3"]
    pub mod TRACKING_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT4"]
    pub mod TRACKING_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT5"]
    pub mod TRACKING_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT6"]
    pub mod TRACKING_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT7"]
    pub mod TRACKING_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT8"]
    pub mod TRACKING_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT9"]
    pub mod TRACKING_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT10"]
    pub mod TRACKING_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT11"]
    pub mod TRACKING_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT12"]
    pub mod TRACKING_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT13"]
    pub mod TRACKING_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT14"]
    pub mod TRACKING_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
    #[doc = "TRACKING_EN_SETPOINT15"]
    pub mod TRACKING_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TRACK0: u32 = 0;
            #[doc = "Enabled"]
            pub const TRACK1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_DIG_BYPASS_EN_SP_REGISTER"]
pub mod LDO_LPSR_DIG_BYPASS_EN_SP {
    #[doc = "BYPASS_EN_SETPOINT0"]
    pub mod BYPASS_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT1"]
    pub mod BYPASS_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT2"]
    pub mod BYPASS_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT3"]
    pub mod BYPASS_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT4"]
    pub mod BYPASS_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT5"]
    pub mod BYPASS_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT6"]
    pub mod BYPASS_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT7"]
    pub mod BYPASS_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT8"]
    pub mod BYPASS_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT9"]
    pub mod BYPASS_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT10"]
    pub mod BYPASS_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT11"]
    pub mod BYPASS_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT12"]
    pub mod BYPASS_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT13"]
    pub mod BYPASS_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT14"]
    pub mod BYPASS_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
    #[doc = "BYPASS_EN_SETPOINT15"]
    pub mod BYPASS_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const BYP0: u32 = 0;
            #[doc = "Enabled"]
            pub const BYP1: u32 = 0x01;
        }
    }
}
#[doc = "LDO_LPSR_DIG_STBY_EN_SP_REGISTER"]
pub mod LDO_LPSR_DIG_STBY_EN_SP {
    #[doc = "STBY_EN_SETPOINT0"]
    pub mod STBY_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT1"]
    pub mod STBY_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT2"]
    pub mod STBY_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT3"]
    pub mod STBY_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT4"]
    pub mod STBY_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT5"]
    pub mod STBY_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT6"]
    pub mod STBY_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT7"]
    pub mod STBY_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT8"]
    pub mod STBY_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT9"]
    pub mod STBY_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT10"]
    pub mod STBY_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT11"]
    pub mod STBY_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT12"]
    pub mod STBY_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT13"]
    pub mod STBY_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT14"]
    pub mod STBY_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT15"]
    pub mod STBY_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "BANDGAP_ENABLE_SP_REGISTER"]
pub mod BANDGAP_ENABLE_SP {
    #[doc = "ON_OFF_SETPOINT0"]
    pub mod ON_OFF_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT1"]
    pub mod ON_OFF_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT2"]
    pub mod ON_OFF_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT3"]
    pub mod ON_OFF_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT4"]
    pub mod ON_OFF_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT5"]
    pub mod ON_OFF_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT5"]
    pub mod ON_OFF_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT7"]
    pub mod ON_OFF_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT8"]
    pub mod ON_OFF_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT9"]
    pub mod ON_OFF_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT10"]
    pub mod ON_OFF_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT11"]
    pub mod ON_OFF_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT12"]
    pub mod ON_OFF_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT13"]
    pub mod ON_OFF_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT14"]
    pub mod ON_OFF_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT15"]
    pub mod ON_OFF_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
}
#[doc = "FBB_M7_ENABLE_SP_REGISTER"]
pub mod FBB_M7_ENABLE_SP {
    #[doc = "ON_OFF_SETPOINT0"]
    pub mod ON_OFF_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT1"]
    pub mod ON_OFF_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT2"]
    pub mod ON_OFF_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT3"]
    pub mod ON_OFF_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT4"]
    pub mod ON_OFF_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT5"]
    pub mod ON_OFF_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT6"]
    pub mod ON_OFF_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT7"]
    pub mod ON_OFF_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT8"]
    pub mod ON_OFF_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT9"]
    pub mod ON_OFF_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT10"]
    pub mod ON_OFF_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT11"]
    pub mod ON_OFF_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT12"]
    pub mod ON_OFF_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT13"]
    pub mod ON_OFF_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT14"]
    pub mod ON_OFF_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT15"]
    pub mod ON_OFF_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
}
#[doc = "RBB_SOC_ENABLE_SP_REGISTER"]
pub mod RBB_SOC_ENABLE_SP {
    #[doc = "ON_OFF_SETPOINT0"]
    pub mod ON_OFF_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT1"]
    pub mod ON_OFF_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT2"]
    pub mod ON_OFF_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT3"]
    pub mod ON_OFF_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT4"]
    pub mod ON_OFF_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT5"]
    pub mod ON_OFF_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT6"]
    pub mod ON_OFF_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT7"]
    pub mod ON_OFF_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT8"]
    pub mod ON_OFF_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT9"]
    pub mod ON_OFF_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT10"]
    pub mod ON_OFF_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT11"]
    pub mod ON_OFF_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT12"]
    pub mod ON_OFF_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT13"]
    pub mod ON_OFF_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT14"]
    pub mod ON_OFF_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT15"]
    pub mod ON_OFF_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
}
#[doc = "RBB_LPSR_ENABLE_SP_REGISTER"]
pub mod RBB_LPSR_ENABLE_SP {
    #[doc = "ON_OFF_SETPOINT0"]
    pub mod ON_OFF_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT1"]
    pub mod ON_OFF_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT2"]
    pub mod ON_OFF_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT3"]
    pub mod ON_OFF_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT4"]
    pub mod ON_OFF_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT5"]
    pub mod ON_OFF_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT6"]
    pub mod ON_OFF_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT7"]
    pub mod ON_OFF_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT8"]
    pub mod ON_OFF_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT9"]
    pub mod ON_OFF_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT10"]
    pub mod ON_OFF_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT11"]
    pub mod ON_OFF_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT12"]
    pub mod ON_OFF_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT13"]
    pub mod ON_OFF_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT14"]
    pub mod ON_OFF_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
    #[doc = "ON_OFF_SETPOINT15"]
    pub mod ON_OFF_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ON"]
            pub const S0: u32 = 0;
            #[doc = "OFF"]
            pub const S1: u32 = 0x01;
        }
    }
}
#[doc = "BANDGAP_STBY_EN_SP_REGISTER"]
pub mod BANDGAP_STBY_EN_SP {
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "STBY_EN_SETPOINT"]
    pub mod STBY_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "PLL_LDO_STBY_EN_SP_REGISTER"]
pub mod PLL_LDO_STBY_EN_SP {
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "FBB_M7_STBY_EN_SP_REGISTER"]
pub mod FBB_M7_STBY_EN_SP {
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "RBB_SOC_STBY_EN_SP_REGISTER"]
pub mod RBB_SOC_STBY_EN_SP {
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "RBB_LPSR_STBY_EN_SP_REGISTER"]
pub mod RBB_LPSR_STBY_EN_SP {
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Standby mode"]
    pub mod STBY_EN_SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const B0: u32 = 0;
            #[doc = "Enabled"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "FBB_M7_CONFIGURE_REGISTER"]
pub mod FBB_M7_CONFIGURE {
    #[doc = "wb_cfg_pw"]
    pub mod WB_CFG_PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wb_cfg_nw"]
    pub mod WB_CFG_NW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "oscillator_bits"]
    pub mod OSCILLATOR_BITS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "regulator_strength"]
    pub mod REGULATOR_STRENGTH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RBB_LPSR_CONFIGURE_REGISTER"]
pub mod RBB_LPSR_CONFIGURE {
    #[doc = "wb_cfg_pw"]
    pub mod WB_CFG_PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wb_cfg_nw"]
    pub mod WB_CFG_NW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "oscillator_bits"]
    pub mod OSCILLATOR_BITS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "regulator_strength"]
    pub mod REGULATOR_STRENGTH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RBB_SOC_CONFIGURE_REGISTER"]
pub mod RBB_SOC_CONFIGURE {
    #[doc = "wb_cfg_pw"]
    pub mod WB_CFG_PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wb_cfg_nw"]
    pub mod WB_CFG_NW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "oscillator_bits"]
    pub mod OSCILLATOR_BITS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "regulator_strength"]
    pub mod REGULATOR_STRENGTH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "REFTOP_OTP_TRIM_VALUE_REGISTER"]
pub mod REFTOP_OTP_TRIM_VALUE {
    #[doc = "REFTOP_IBZTCADJ"]
    pub mod REFTOP_IBZTCADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REFTOP_VBGADJ"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REFTOP_TRIM_EN"]
    pub mod REFTOP_TRIM_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LPSR_1P8_LDO_OTP_TRIM_VALUE_REGISTER"]
pub mod LPSR_1P8_LDO_OTP_TRIM_VALUE {
    #[doc = "LPSR_LDO_1P8_TRIM"]
    pub mod LPSR_LDO_1P8_TRIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPSR_LDO_1P8_TRIM_EN"]
    pub mod LPSR_LDO_1P8_TRIM_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
