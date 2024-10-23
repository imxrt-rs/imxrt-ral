#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Array of registers: _CM_AUTHEN_CTRL, _CM_MISC, _CM_MODE_CTRL, _CM_MODE_STAT, _CM_IRQ_WAKEUP_MASK_0, _CM_IRQ_WAKEUP_MASK_1, _CM_IRQ_WAKEUP_MASK_2, _CM_IRQ_WAKEUP_MASK_3, _CM_IRQ_WAKEUP_MASK_4, _CM_IRQ_WAKEUP_MASK_5, _CM_IRQ_WAKEUP_MASK_6, _CM_IRQ_WAKEUP_MASK_7, _CM_NON_IRQ_WAKEUP_MASK, _CM_IRQ_WAKEUP_STAT_0, _CM_IRQ_WAKEUP_STAT_1, _CM_IRQ_WAKEUP_STAT_2, _CM_IRQ_WAKEUP_STAT_3, _CM_IRQ_WAKEUP_STAT_4, _CM_IRQ_WAKEUP_STAT_5, _CM_IRQ_WAKEUP_STAT_6, _CM_IRQ_WAKEUP_STAT_7, _CM_NON_IRQ_WAKEUP_STAT, _CM_SLEEP_SSAR_CTRL, _CM_SLEEP_LPCG_CTRL, _CM_SLEEP_PLL_CTRL, _CM_SLEEP_ISO_CTRL, _CM_SLEEP_RESET_CTRL, _CM_SLEEP_POWER_CTRL, _CM_WAKEUP_POWER_CTRL, _CM_WAKEUP_RESET_CTRL, _CM_WAKEUP_ISO_CTRL, _CM_WAKEUP_PLL_CTRL, _CM_WAKEUP_LPCG_CTRL, _CM_WAKEUP_SSAR_CTRL, _CM_SYS_SLEEP_CTRL"]
    pub AUTHEN: [authen::RegisterBlock; 2usize],
}
pub mod authen {
    #[doc = "Array of registers: _CM_AUTHEN_CTRL, _CM_MISC, _CM_MODE_CTRL, _CM_MODE_STAT, _CM_IRQ_WAKEUP_MASK_0, _CM_IRQ_WAKEUP_MASK_1, _CM_IRQ_WAKEUP_MASK_2, _CM_IRQ_WAKEUP_MASK_3, _CM_IRQ_WAKEUP_MASK_4, _CM_IRQ_WAKEUP_MASK_5, _CM_IRQ_WAKEUP_MASK_6, _CM_IRQ_WAKEUP_MASK_7, _CM_NON_IRQ_WAKEUP_MASK, _CM_IRQ_WAKEUP_STAT_0, _CM_IRQ_WAKEUP_STAT_1, _CM_IRQ_WAKEUP_STAT_2, _CM_IRQ_WAKEUP_STAT_3, _CM_IRQ_WAKEUP_STAT_4, _CM_IRQ_WAKEUP_STAT_5, _CM_IRQ_WAKEUP_STAT_6, _CM_IRQ_WAKEUP_STAT_7, _CM_NON_IRQ_WAKEUP_STAT, _CM_SLEEP_SSAR_CTRL, _CM_SLEEP_LPCG_CTRL, _CM_SLEEP_PLL_CTRL, _CM_SLEEP_ISO_CTRL, _CM_SLEEP_RESET_CTRL, _CM_SLEEP_POWER_CTRL, _CM_WAKEUP_POWER_CTRL, _CM_WAKEUP_RESET_CTRL, _CM_WAKEUP_ISO_CTRL, _CM_WAKEUP_PLL_CTRL, _CM_WAKEUP_LPCG_CTRL, _CM_WAKEUP_SSAR_CTRL, _CM_SYS_SLEEP_CTRL"]
    #[repr(C)]
    pub struct RegisterBlock {
        _reserved0: [u8; 0x04],
        #[doc = "CM Authentication Control"]
        pub _CM_AUTHEN_CTRL: crate::RWRegister<u32>,
        _reserved1: [u8; 0x04],
        #[doc = "Miscellaneous"]
        pub _CM_MISC: crate::RWRegister<u32>,
        #[doc = "CPU mode control"]
        pub _CM_MODE_CTRL: crate::RWRegister<u32>,
        #[doc = "CM CPU mode Status"]
        pub _CM_MODE_STAT: crate::RORegister<u32>,
        _reserved2: [u8; 0xe8],
        #[doc = "CM IRQ0~31 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_0: crate::RWRegister<u32>,
        #[doc = "CM IRQ32~63 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_1: crate::RWRegister<u32>,
        #[doc = "CM IRQ64~95 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_2: crate::RWRegister<u32>,
        #[doc = "CM IRQ96~127 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_3: crate::RWRegister<u32>,
        #[doc = "CM IRQ128~159 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_4: crate::RWRegister<u32>,
        #[doc = "CM IRQ160~191 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_5: crate::RWRegister<u32>,
        #[doc = "CM IRQ192~223 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_6: crate::RWRegister<u32>,
        #[doc = "CM IRQ224~255 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_7: crate::RWRegister<u32>,
        _reserved3: [u8; 0x20],
        #[doc = "CM non-IRQ wakeup mask"]
        pub _CM_NON_IRQ_WAKEUP_MASK: crate::RWRegister<u32>,
        _reserved4: [u8; 0x0c],
        #[doc = "CM IRQ0~31 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_0: crate::RORegister<u32>,
        #[doc = "CM IRQ32~63 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_1: crate::RORegister<u32>,
        #[doc = "CM IRQ64~95 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_2: crate::RORegister<u32>,
        #[doc = "CM IRQ96~127 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_3: crate::RORegister<u32>,
        #[doc = "CM IRQ128~159 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_4: crate::RORegister<u32>,
        #[doc = "CM IRQ160~191 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_5: crate::RORegister<u32>,
        #[doc = "CM IRQ192~223 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_6: crate::RORegister<u32>,
        #[doc = "CM IRQ224~255 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_7: crate::RORegister<u32>,
        _reserved5: [u8; 0x20],
        #[doc = "CM non-irq wakeup status"]
        pub _CM_NON_IRQ_WAKEUP_STAT: crate::RORegister<u32>,
        _reserved6: [u8; 0x6c],
        #[doc = "CM sleep SSAR control"]
        pub _CM_SLEEP_SSAR_CTRL: crate::RWRegister<u32>,
        _reserved7: [u8; 0x04],
        #[doc = "CM sleep LPCG control"]
        pub _CM_SLEEP_LPCG_CTRL: crate::RWRegister<u32>,
        _reserved8: [u8; 0x04],
        #[doc = "CM sleep PLL control"]
        pub _CM_SLEEP_PLL_CTRL: crate::RWRegister<u32>,
        _reserved9: [u8; 0x04],
        #[doc = "CM sleep isolation control"]
        pub _CM_SLEEP_ISO_CTRL: crate::RWRegister<u32>,
        _reserved10: [u8; 0x04],
        #[doc = "CM sleep reset control"]
        pub _CM_SLEEP_RESET_CTRL: crate::RWRegister<u32>,
        _reserved11: [u8; 0x04],
        #[doc = "CM sleep power control"]
        pub _CM_SLEEP_POWER_CTRL: crate::RWRegister<u32>,
        _reserved12: [u8; 0x64],
        #[doc = "CM wakeup power control"]
        pub _CM_WAKEUP_POWER_CTRL: crate::RWRegister<u32>,
        _reserved13: [u8; 0x04],
        #[doc = "CM wakeup reset control"]
        pub _CM_WAKEUP_RESET_CTRL: crate::RWRegister<u32>,
        _reserved14: [u8; 0x04],
        #[doc = "CM wakeup isolation control"]
        pub _CM_WAKEUP_ISO_CTRL: crate::RWRegister<u32>,
        _reserved15: [u8; 0x04],
        #[doc = "CM wakeup PLL control"]
        pub _CM_WAKEUP_PLL_CTRL: crate::RWRegister<u32>,
        _reserved16: [u8; 0x04],
        #[doc = "CM wakeup LPCG control"]
        pub _CM_WAKEUP_LPCG_CTRL: crate::RWRegister<u32>,
        _reserved17: [u8; 0x0c],
        #[doc = "CM wakeup SSAR control"]
        pub _CM_WAKEUP_SSAR_CTRL: crate::RWRegister<u32>,
        _reserved18: [u8; 0xbc],
        #[doc = "CM system sleep control"]
        pub _CM_SYS_SLEEP_CTRL: crate::RWRegister<u32>,
        _reserved19: [u8; 0x047c],
    }
    #[doc = "CM Authentication Control"]
    pub mod _CM_AUTHEN_CTRL {
        #[doc = "Configuration lock"]
        pub mod LOCK_CFG {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The value of low power configuration fields are not locked."]
                pub const B0: u32 = 0;
                #[doc = "The value of low power configuration fields are locked. It locks the CPUx_CM registers which are marked as \"Locked by LOCK_CFG field\" in the function field."]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "Allow user mode access"]
        pub mod USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Allow only privilege mode to access CPU mode control registers"]
                pub const B0: u32 = 0;
                #[doc = "Allow both privilege and user mode to access CPU mode control registers"]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "Allow non-secure mode access"]
        pub mod NONSECURE {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Allow only secure mode to access CPU mode control"]
                pub const B0: u32 = 0;
                #[doc = "Allow both secure and non-secure mode to access CPU mode control registers"]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "Lock NONSECURE and USER"]
        pub mod LOCK_SETTING {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "NONSECURE and USER fields are not locked"]
                pub const B0: u32 = 0;
                #[doc = "NONSECURE and USER fields are locked"]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "White list lock"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "WHITE_LIST is not locked"]
                pub const B0: u32 = 0;
                #[doc = "WHITE_LIST is locked"]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "Domain ID white list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Miscellaneous"]
    pub mod _CM_MISC {
        #[doc = "Non-masked interrupt status"]
        pub mod NMI_STAT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "NMI is not asserted"]
                pub const B0: u32 = 0;
                #[doc = "NMI is asserted"]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "Allow cpu_sleep_hold_req to assert during CPU low power status"]
        pub mod SLEEP_HOLD_EN {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable cpu_sleep_hold_req"]
                pub const B0: u32 = 0;
                #[doc = "Allow cpu_sleep_hold_req to assert during CPU low power status"]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "CPU sleep hold status"]
        pub mod SLEEP_HOLD_STAT {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "CPU sleep hold is acknowledged"]
                pub const B0: u32 = 0;
                #[doc = "CPU is not in sleep hold"]
                pub const B1: u32 = 0x01;
            }
        }
    }
    #[doc = "CPU mode control"]
    pub mod _CM_MODE_CTRL {
        #[doc = "The CPU mode the CPU platform should transit to on next sleep event"]
        pub mod CPU_MODE_TARGET {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Stay in RUN mode"]
                pub const B0: u32 = 0;
                #[doc = "Transit to WAIT mode"]
                pub const B1: u32 = 0x01;
                #[doc = "Transit to STOP mode"]
                pub const B2: u32 = 0x02;
                #[doc = "Transit to SUSPEND mode"]
                pub const B3: u32 = 0x03;
            }
        }
        #[doc = "WFE assertion can be sleep event"]
        pub mod WFE_EN {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "WFE assertion can not trigger low power"]
                pub const B0: u32 = 0;
                #[doc = "WFE assertion can trigger low power"]
                pub const B1: u32 = 0x01;
            }
        }
    }
    #[doc = "CM CPU mode Status"]
    pub mod _CM_MODE_STAT {
        #[doc = "Current CPU mode"]
        pub mod CPU_MODE_CURRENT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "CPU is currently in RUN mode"]
                pub const B0: u32 = 0;
                #[doc = "CPU is currently in WAIT mode"]
                pub const B1: u32 = 0x01;
                #[doc = "CPU is currently in STOP mode"]
                pub const B2: u32 = 0x02;
                #[doc = "CPU is currently in SUSPEND mode"]
                pub const B3: u32 = 0x03;
            }
        }
        #[doc = "Previous CPU mode"]
        pub mod CPU_MODE_PREVIOUS {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "CPU was previously in RUN mode"]
                pub const B0: u32 = 0;
                #[doc = "CPU was previously in WAIT mode"]
                pub const B1: u32 = 0x01;
                #[doc = "CPU was previously in STOP mode"]
                pub const B2: u32 = 0x02;
                #[doc = "CPU was previously in SUSPEND mode"]
                pub const B3: u32 = 0x03;
            }
        }
    }
    #[doc = "CM IRQ0~31 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_0 {
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_0_31 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ32~63 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_1 {
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_32_63 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ64~95 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_2 {
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_64_95 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ96~127 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_3 {
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_96_127 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ128~159 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_4 {
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_128_159 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ160~191 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_5 {
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_160_191 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ192~223 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_6 {
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_192_223 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ224~255 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_7 {
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_224_255 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM non-IRQ wakeup mask"]
    pub mod _CM_NON_IRQ_WAKEUP_MASK {
        #[doc = "\"1\" means the debug_wakeup_request cannot wakeup CPU platform"]
        pub mod DEBUG_WAKEUP_MASK {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ0~31 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_0 {
        #[doc = "IRQ status"]
        pub mod STAT_0_31 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ32~63 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_1 {
        #[doc = "IRQ status"]
        pub mod STAT_32_63 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ64~95 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_2 {
        #[doc = "IRQ status"]
        pub mod STAT_64_95 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ96~127 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_3 {
        #[doc = "IRQ status"]
        pub mod STAT_96_127 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ128~159 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_4 {
        #[doc = "IRQ status"]
        pub mod STAT_128_159 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ160~191 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_5 {
        #[doc = "IRQ status"]
        pub mod STAT_160_191 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ192~223 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_6 {
        #[doc = "IRQ status"]
        pub mod STAT_192_223 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM IRQ224~255 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_7 {
        #[doc = "IRQ status"]
        pub mod STAT_224_255 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "CM non-irq wakeup status"]
    pub mod _CM_NON_IRQ_WAKEUP_STAT {
        #[doc = "Debug wakeup status"]
        pub mod DEBUG_WAKEUP_STAT {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No debug wakeup is requested"]
                pub const B0: u32 = 0;
                #[doc = "Debug wakeup is requested"]
                pub const B1: u32 = 0x01;
            }
        }
    }
    #[doc = "CM sleep SSAR control"]
    pub mod _CM_SLEEP_SSAR_CTRL {
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
    #[doc = "CM sleep LPCG control"]
    pub mod _CM_SLEEP_LPCG_CTRL {
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
    #[doc = "CM sleep PLL control"]
    pub mod _CM_SLEEP_PLL_CTRL {
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
    #[doc = "CM sleep isolation control"]
    pub mod _CM_SLEEP_ISO_CTRL {
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
    #[doc = "CM sleep reset control"]
    pub mod _CM_SLEEP_RESET_CTRL {
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
    #[doc = "CM sleep power control"]
    pub mod _CM_SLEEP_POWER_CTRL {
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
    #[doc = "CM wakeup power control"]
    pub mod _CM_WAKEUP_POWER_CTRL {
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
    #[doc = "CM wakeup reset control"]
    pub mod _CM_WAKEUP_RESET_CTRL {
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
    #[doc = "CM wakeup isolation control"]
    pub mod _CM_WAKEUP_ISO_CTRL {
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
    #[doc = "CM wakeup PLL control"]
    pub mod _CM_WAKEUP_PLL_CTRL {
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
    #[doc = "CM wakeup LPCG control"]
    pub mod _CM_WAKEUP_LPCG_CTRL {
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
    #[doc = "CM wakeup SSAR control"]
    pub mod _CM_WAKEUP_SSAR_CTRL {
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
    #[doc = "CM system sleep control"]
    pub mod _CM_SYS_SLEEP_CTRL {
        #[doc = "Request system sleep when CPU is in WAIT mode"]
        pub mod SS_WAIT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Do not request system sleep when CPU is in WAIT mode"]
                pub const B0: u32 = 0;
                #[doc = "Request system sleep when CPU is in WAIT mode"]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "Request system sleep when CPU is in STOP mode"]
        pub mod SS_STOP {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Do not request system sleep when CPU is in STOP mode"]
                pub const B0: u32 = 0;
                #[doc = "Request system sleep when CPU is in STOP mode"]
                pub const B1: u32 = 0x01;
            }
        }
        #[doc = "Request system sleep when CPU is in SUSPEND mode"]
        pub mod SS_SUSPEND {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Do not request system sleep when CPU is in SUSPEND mode"]
                pub const B0: u32 = 0;
                #[doc = "Request system sleep when CPU is in SUSPEND mode"]
                pub const B1: u32 = 0x01;
            }
        }
    }
}
