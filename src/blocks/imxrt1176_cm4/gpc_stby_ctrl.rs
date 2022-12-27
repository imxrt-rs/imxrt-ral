#[doc = "GPC_STBY"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "Standby Authentication Control"]
    pub STBY_AUTHEN_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "STBY Misc"]
    pub STBY_MISC: crate::RWRegister<u32>,
    _reserved2: [u8; 0xe0],
    #[doc = "STBY lpcg_in control"]
    pub STBY_LPCG_IN_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "STBY pll_in control"]
    pub STBY_PLL_IN_CTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "STBY bias_in control"]
    pub STBY_BIAS_IN_CTRL: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "STBY pldo_in control"]
    pub STBY_PLDO_IN_CTRL: crate::RWRegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "STBY bandgap_in control"]
    pub STBY_BANDGAP_IN_CTRL: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "STBY ldo_in control"]
    pub STBY_LDO_IN_CTRL: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "STBY dcdc_in control"]
    pub STBY_DCDC_IN_CTRL: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "STBY PMIC in control"]
    pub STBY_PMIC_IN_CTRL: crate::RWRegister<u32>,
    _reserved10: [u8; 0xac],
    #[doc = "STBY PMIC out control"]
    pub STBY_PMIC_OUT_CTRL: crate::RWRegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "STBY DCDC out control"]
    pub STBY_DCDC_OUT_CTRL: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "STBY LDO out control"]
    pub STBY_LDO_OUT_CTRL: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "STBY bandgap out control"]
    pub STBY_BANDGAP_OUT_CTRL: crate::RWRegister<u32>,
    _reserved14: [u8; 0x04],
    #[doc = "STBY pldo out control"]
    pub STBY_PLDO_OUT_CTRL: crate::RWRegister<u32>,
    _reserved15: [u8; 0x04],
    #[doc = "STBY bias out control"]
    pub STBY_BIAS_OUT_CTRL: crate::RWRegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "STBY PLL out control"]
    pub STBY_PLL_OUT_CTRL: crate::RWRegister<u32>,
    _reserved17: [u8; 0x0c],
    #[doc = "STBY LPCG out control"]
    pub STBY_LPCG_OUT_CTRL: crate::RWRegister<u32>,
}
#[doc = "Standby Authentication Control"]
pub mod STBY_AUTHEN_CTRL {
    #[doc = "Configuration lock"]
    pub mod LOCK_CFG {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY Misc"]
pub mod STBY_MISC {
    #[doc = "Force CPU0 requesting standby mode"]
    pub mod FORCE_CPU0_STBY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force CPU0 requesting standby mode"]
    pub mod FORCE_CPU1_STBY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force CPU2 requesting standby mode"]
    pub mod FORCE_CPU2_STBY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force CPU3 requesting standby mode"]
    pub mod FORCE_CPU3_STBY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY lpcg_in control"]
pub mod STBY_LPCG_IN_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY pll_in control"]
pub mod STBY_PLL_IN_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY bias_in control"]
pub mod STBY_BIAS_IN_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY pldo_in control"]
pub mod STBY_PLDO_IN_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY bandgap_in control"]
pub mod STBY_BANDGAP_IN_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY ldo_in control"]
pub mod STBY_LDO_IN_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY dcdc_in control"]
pub mod STBY_DCDC_IN_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY PMIC in control"]
pub mod STBY_PMIC_IN_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY PMIC out control"]
pub mod STBY_PMIC_OUT_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY DCDC out control"]
pub mod STBY_DCDC_OUT_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY LDO out control"]
pub mod STBY_LDO_OUT_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY bandgap out control"]
pub mod STBY_BANDGAP_OUT_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY pldo out control"]
pub mod STBY_PLDO_OUT_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY bias out control"]
pub mod STBY_BIAS_OUT_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY PLL out control"]
pub mod STBY_PLL_OUT_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STBY LPCG out control"]
pub mod STBY_LPCG_OUT_CTRL {
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
