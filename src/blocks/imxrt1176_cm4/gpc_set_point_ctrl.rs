#[doc = "GPC_SP"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "SP Authentication Control"]
    pub SP_AUTHEN_CTRL: crate::RWRegister<u32>,
    #[doc = "SP Interrupt Control"]
    pub SP_INT_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "CPU SP Request"]
    pub SP_CPU_REQ: crate::RORegister<u32>,
    #[doc = "SP System Status"]
    pub SP_SYS_STAT: crate::RORegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "SP ROSC Control"]
    pub SP_ROSC_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x20],
    #[doc = "SP0~7 Priority"]
    pub SP_PRIORITY_0_7: crate::RWRegister<u32>,
    #[doc = "SP8~15 Priority"]
    pub SP_PRIORITY_8_15: crate::RWRegister<u32>,
    _reserved4: [u8; 0xb8],
    #[doc = "SP SSAR save control"]
    pub SP_SSAR_SAVE_CTRL: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "SP LPCG off control"]
    pub SP_LPCG_OFF_CTRL: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "SP group down control"]
    pub SP_GROUP_DOWN_CTRL: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "SP root down control"]
    pub SP_ROOT_DOWN_CTRL: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "SP PLL off control"]
    pub SP_PLL_OFF_CTRL: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "SP ISO on control"]
    pub SP_ISO_ON_CTRL: crate::RWRegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "SP reset early control"]
    pub SP_RESET_EARLY_CTRL: crate::RWRegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "SP power off control"]
    pub SP_POWER_OFF_CTRL: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "SP bias off control"]
    pub SP_BIAS_OFF_CTRL: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "SP bandgap and PLL_LDO off control"]
    pub SP_BG_PLDO_OFF_CTRL: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "SP LDO pre control"]
    pub SP_LDO_PRE_CTRL: crate::RWRegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "SP DCDC down control"]
    pub SP_DCDC_DOWN_CTRL: crate::RWRegister<u32>,
    _reserved16: [u8; 0x4c],
    #[doc = "SP DCDC up control"]
    pub SP_DCDC_UP_CTRL: crate::RWRegister<u32>,
    _reserved17: [u8; 0x0c],
    #[doc = "SP LDO post control"]
    pub SP_LDO_POST_CTRL: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0c],
    #[doc = "SP bandgap and PLL_LDO on control"]
    pub SP_BG_PLDO_ON_CTRL: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0c],
    #[doc = "SP bias on control"]
    pub SP_BIAS_ON_CTRL: crate::RWRegister<u32>,
    _reserved20: [u8; 0x0c],
    #[doc = "SP power on control"]
    pub SP_POWER_ON_CTRL: crate::RWRegister<u32>,
    _reserved21: [u8; 0x0c],
    #[doc = "SP reset late control"]
    pub SP_RESET_LATE_CTRL: crate::RWRegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "SP ISO off control"]
    pub SP_ISO_OFF_CTRL: crate::RWRegister<u32>,
    _reserved23: [u8; 0x0c],
    #[doc = "SP PLL on control"]
    pub SP_PLL_ON_CTRL: crate::RWRegister<u32>,
    _reserved24: [u8; 0x0c],
    #[doc = "SP root up control"]
    pub SP_ROOT_UP_CTRL: crate::RWRegister<u32>,
    _reserved25: [u8; 0x0c],
    #[doc = "SP group up control"]
    pub SP_GROUP_UP_CTRL: crate::RWRegister<u32>,
    _reserved26: [u8; 0x0c],
    #[doc = "SP LPCG on control"]
    pub SP_LPCG_ON_CTRL: crate::RWRegister<u32>,
    _reserved27: [u8; 0x0c],
    #[doc = "SP SSAR restore control"]
    pub SP_SSAR_RESTORE_CTRL: crate::RWRegister<u32>,
}
#[doc = "SP Authentication Control"]
pub mod SP_AUTHEN_CTRL {
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow only privilege mode to access setpoint control registers"]
            pub const B0: u32 = 0;
            #[doc = "Allow both privilege and user mode to access setpoint control registers"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow only secure mode to access setpoint control registers"]
            pub const B0: u32 = 0;
            #[doc = "Allow both secure and non-secure mode to access setpoint control registers"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configuration lock"]
    pub mod LOCK_CFG {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SP Interrupt Control"]
pub mod SP_INT_CTRL {
    #[doc = "no_allowed_set_point interrupt enable"]
    pub mod NO_ALLOWED_SP_INT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "no_allowed_set_point interrupt"]
    pub mod NO_ALLOWED_SP_INT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPU SP Request"]
pub mod SP_CPU_REQ {
    #[doc = "Setpoint requested by CPU0"]
    pub mod SP_REQ_CPU0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setpoint requested by CPU1"]
    pub mod SP_REQ_CPU1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setpoint requested by CPU2"]
    pub mod SP_REQ_CPU2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setpoint requested by CPU3"]
    pub mod SP_REQ_CPU3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 Setpoint accepted by SP controller"]
    pub mod SP_ACCEPTED_CPU0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU1 Setpoint accepted by SP controller"]
    pub mod SP_ACCEPTED_CPU1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU2 Setpoint accepted by SP controller"]
    pub mod SP_ACCEPTED_CPU2 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU3 Setpoint accepted by SP controller"]
    pub mod SP_ACCEPTED_CPU3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SP System Status"]
pub mod SP_SYS_STAT {
    #[doc = "Allowed Setpoints by all current CPU Setpoint requests"]
    pub mod SYS_SP_ALLOWED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The Setpoint chosen as the target setpoint"]
    pub mod SYS_SP_TARGET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current Setpoint, only valid when not SP trans busy"]
    pub mod SYS_SP_CURRENT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Previous Setpoint, only valid when not SP trans busy"]
    pub mod SYS_SP_PREVIOUS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SP ROSC Control"]
pub mod SP_ROSC_CTRL {
    #[doc = "Allow shutting off the ROSC"]
    pub mod SP_ALLOW_ROSC_OFF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SP0~7 Priority"]
pub mod SP_PRIORITY_0_7 {
    #[doc = "priority of Setpoint 0"]
    pub mod SYS_SP0_PRIORITY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 1"]
    pub mod SYS_SP1_PRIORITY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 2"]
    pub mod SYS_SP2_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 3"]
    pub mod SYS_SP3_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 4"]
    pub mod SYS_SP4_PRIORITY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 5"]
    pub mod SYS_SP5_PRIORITY {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 6"]
    pub mod SYS_SP6_PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 7"]
    pub mod SYS_SP7_PRIORITY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SP8~15 Priority"]
pub mod SP_PRIORITY_8_15 {
    #[doc = "priority of Setpoint 8"]
    pub mod SYS_SP8_PRIORITY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 9"]
    pub mod SYS_SP9_PRIORITY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 10"]
    pub mod SYS_SP10_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 11"]
    pub mod SYS_SP11_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 12"]
    pub mod SYS_SP12_PRIORITY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 13"]
    pub mod SYS_SP13_PRIORITY {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 14"]
    pub mod SYS_SP14_PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "priority of Setpoint 15"]
    pub mod SYS_SP15_PRIORITY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SP SSAR save control"]
pub mod SP_SSAR_SAVE_CTRL {
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
#[doc = "SP LPCG off control"]
pub mod SP_LPCG_OFF_CTRL {
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
#[doc = "SP group down control"]
pub mod SP_GROUP_DOWN_CTRL {
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
#[doc = "SP root down control"]
pub mod SP_ROOT_DOWN_CTRL {
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
#[doc = "SP PLL off control"]
pub mod SP_PLL_OFF_CTRL {
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
#[doc = "SP ISO on control"]
pub mod SP_ISO_ON_CTRL {
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
#[doc = "SP reset early control"]
pub mod SP_RESET_EARLY_CTRL {
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
#[doc = "SP power off control"]
pub mod SP_POWER_OFF_CTRL {
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
#[doc = "SP bias off control"]
pub mod SP_BIAS_OFF_CTRL {
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
#[doc = "SP bandgap and PLL_LDO off control"]
pub mod SP_BG_PLDO_OFF_CTRL {
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
#[doc = "SP LDO pre control"]
pub mod SP_LDO_PRE_CTRL {
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
#[doc = "SP DCDC down control"]
pub mod SP_DCDC_DOWN_CTRL {
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
#[doc = "SP DCDC up control"]
pub mod SP_DCDC_UP_CTRL {
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
#[doc = "SP LDO post control"]
pub mod SP_LDO_POST_CTRL {
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
#[doc = "SP bandgap and PLL_LDO on control"]
pub mod SP_BG_PLDO_ON_CTRL {
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
#[doc = "SP bias on control"]
pub mod SP_BIAS_ON_CTRL {
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
#[doc = "SP power on control"]
pub mod SP_POWER_ON_CTRL {
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
#[doc = "SP reset late control"]
pub mod SP_RESET_LATE_CTRL {
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
#[doc = "SP ISO off control"]
pub mod SP_ISO_OFF_CTRL {
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
#[doc = "SP PLL on control"]
pub mod SP_PLL_ON_CTRL {
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
#[doc = "SP root up control"]
pub mod SP_ROOT_UP_CTRL {
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
#[doc = "SP group up control"]
pub mod SP_GROUP_UP_CTRL {
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
#[doc = "SP LPCG on control"]
pub mod SP_LPCG_ON_CTRL {
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
#[doc = "SP SSAR restore control"]
pub mod SP_SSAR_RESTORE_CTRL {
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
