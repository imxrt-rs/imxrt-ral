#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "System Sleep Authentication Control"]
    pub SS_AUTHEN_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "System Sleep Misc"]
    pub SS_MISC: crate::RWRegister<u32>,
    _reserved2: [u8; 0x30],
    #[doc = "PMIC standby control"]
    pub PMIC_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0xac],
    #[doc = "System Sleep STEP0 (BIAS) in control"]
    pub SS_STEP0_IN_CTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "System Sleep STEP1 (PLDO) in control"]
    pub SS_STEP1_IN_CTRL: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "System Sleep STEP2 (BANDGAP) in control"]
    pub SS_STEP2_IN_CTRL: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "System Sleep STEP3 (LDO) in control"]
    pub SS_STEP3_IN_CTRL: crate::RWRegister<u32>,
    _reserved7: [u8; 0x1c],
    #[doc = "System Sleep DCDC in control"]
    pub SS_DCDC_IN_CTRL: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "System Sleep PMIC in control"]
    pub SS_PMIC_IN_CTRL: crate::RWRegister<u32>,
    _reserved9: [u8; 0xac],
    #[doc = "System Sleep PMIC out control"]
    pub SS_PMIC_OUT_CTRL: crate::RWRegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "System Sleep DCDC out control"]
    pub SS_DCDC_OUT_CTRL: crate::RWRegister<u32>,
    _reserved11: [u8; 0x24],
    #[doc = "System Sleep STEP3 (LDO) out control"]
    pub SS_STEP3_OUT_CTRL: crate::RWRegister<u32>,
    _reserved12: [u8; 0x04],
    #[doc = "System Sleep STEP2 (BANDGAP) out control"]
    pub SS_STEP2_OUT_CTRL: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "System Sleep STEP1 (PLDO) out control"]
    pub SS_STEP1_OUT_CTRL: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "System Sleep STEP0 (BIAS) out control"]
    pub SS_STEP0_OUT_CTRL: crate::RWRegister<u32>,
}
#[doc = "System Sleep Authentication Control"]
pub mod SS_AUTHEN_CTRL {
    #[doc = "Configuration lock"]
    pub mod LOCK_CFG {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The value of low power configuration fields are not locked."]
            pub const B0: u32 = 0;
            #[doc = "The value of low power configuration fields are locked. Refer to the function field of each gpc_sys_sleep_ctrl registers."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep Misc"]
pub mod SS_MISC {
    #[doc = "Force CPU0 to request system sleep mode"]
    pub mod FORCE_CPU0_SYS_SLEEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not force CPU0 to request system sleep mode"]
            pub const B0: u32 = 0;
            #[doc = "Force CPU0 to request system sleep mode"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Force CPU1 to request system sleep mode"]
    pub mod FORCE_CPU1_SYS_SLEEP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not force CPU1 to request system sleep mode"]
            pub const B0: u32 = 0;
            #[doc = "Force CPU1 to request system sleep mode"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "PMIC standby control"]
pub mod PMIC_CTRL {
    #[doc = "Assert the PMIC standby request when system sleep"]
    pub mod PMIC_STBY_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not assert PMIC_STBY_REQ when system sleep is entered"]
            pub const B0: u32 = 0;
            #[doc = "Assert PMIC_STBY_REQ when system sleep is entered"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "PMIC_READY pin status"]
    pub mod PMIC_READY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMIC_READY not asserted"]
            pub const B0: u32 = 0;
            #[doc = "PMIC_READY asserted"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "PMIC_READY is driven from pad"]
    pub mod PMIC_READY_EXIST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMIC_READY is not driven."]
            pub const B0: u32 = 0;
            #[doc = "PMIC_READY is driven from pad. If PMIC_READY_EXIST = 1, PMIC_READY = 1 means external PMIC drives PMIC_READY pin."]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Software PMIC standby trigger"]
    pub mod PMIC_STBY_SOFT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Exit PMIC standby"]
            pub const B0: u32 = 0;
            #[doc = "Trigger PMIC standby"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep STEP0 (BIAS) in control"]
pub mod SS_STEP0_IN_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep STEP1 (PLDO) in control"]
pub mod SS_STEP1_IN_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep STEP2 (BANDGAP) in control"]
pub mod SS_STEP2_IN_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep STEP3 (LDO) in control"]
pub mod SS_STEP3_IN_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep DCDC in control"]
pub mod SS_DCDC_IN_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep PMIC in control"]
pub mod SS_PMIC_IN_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep PMIC out control"]
pub mod SS_PMIC_OUT_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep DCDC out control"]
pub mod SS_DCDC_OUT_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep STEP3 (LDO) out control"]
pub mod SS_STEP3_OUT_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep STEP2 (BANDGAP) out control"]
pub mod SS_STEP2_OUT_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep STEP1 (PLDO) out control"]
pub mod SS_STEP1_OUT_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "System Sleep STEP0 (BIAS) out control"]
pub mod SS_STEP0_OUT_CTRL {
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This step is enabled."]
            pub const B0: u32 = 0;
            #[doc = "This step is disabled. GPC will skip this step and not send any request."]
            pub const B1: u32 = 0x01;
        }
    }
}
