#[doc = "PGMC_CPC"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "CPC Authentication Control"]
    pub CPC_AUTHEN_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "CPC Core Mode"]
    pub CPC_CORE_MODE: crate::RWRegister<u32>,
    #[doc = "CPC core power control"]
    pub CPC_CORE_POWER_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x14],
    #[doc = "CPC flag"]
    pub CPC_FLAG: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "CPC Cache Mode"]
    pub CPC_CACHE_MODE: crate::RWRegister<u32>,
    #[doc = "CPC cache CPU mode control"]
    pub CPC_CACHE_CM_CTRL: crate::RWRegister<u32>,
    #[doc = "CPC cache Setpoint control 0"]
    pub CPC_CACHE_SP_CTRL_0: crate::RWRegister<u32>,
    #[doc = "CPC cache Setpoint control 1"]
    pub CPC_CACHE_SP_CTRL_1: crate::RWRegister<u32>,
    _reserved4: [u8; 0x70],
    #[doc = "CPC local memory Mode"]
    pub CPC_LMEM_MODE: crate::RWRegister<u32>,
    #[doc = "CPC local memory CPU mode control"]
    pub CPC_LMEM_CM_CTRL: crate::RWRegister<u32>,
    #[doc = "CPC local memory Setpoint control 0"]
    pub CPC_LMEM_SP_CTRL_0: crate::RWRegister<u32>,
    #[doc = "CPC local memory Setpoint control 1"]
    pub CPC_LMEM_SP_CTRL_1: crate::RWRegister<u32>,
}
#[doc = "CPC Authentication Control"]
pub mod CPC_AUTHEN_CTRL {
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
#[doc = "CPC Core Mode"]
pub mod CPC_CORE_MODE {
    #[doc = "Control mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod CTRL_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not affected by any low power mode"]
            pub const CTRL_MODE_0: u32 = 0;
            #[doc = "Controlled by CPU power mode of the domain"]
            pub const CTRL_MODE_1: u32 = 0x01;
        }
    }
}
#[doc = "CPC core power control"]
pub mod CPC_CORE_POWER_CTRL {
    #[doc = "Power off when domain enters WAIT mode"]
    pub mod PWR_OFF_AT_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power off when domain enters STOP mode"]
    pub mod PWR_OFF_AT_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power off when domain enters SUSPEND mode"]
    pub mod PWR_OFF_AT_SUSPEND {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software isolation on trigger"]
    pub mod ISO_ON_SOFT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software power off trigger"]
    pub mod PSW_OFF_SOFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software power on trigger"]
    pub mod PSW_ON_SOFT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software isolation off trigger"]
    pub mod ISO_OFF_SOFT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPC flag"]
pub mod CPC_FLAG {
    #[doc = "set to 1 after core power switch off, cleared by writing 1"]
    pub mod CORE_PDN_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPC Cache Mode"]
pub mod CPC_CACHE_MODE {
    #[doc = "Control mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod CTRL_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not affected by any low power mode"]
            pub const CTRL_MODE_0: u32 = 0;
            #[doc = "Controlled by CPU power mode of the domain"]
            pub const CTRL_MODE_1: u32 = 0x01;
            #[doc = "Controlled by Setpoint"]
            pub const CTRL_MODE_2: u32 = 0x02;
        }
    }
}
#[doc = "CPC cache CPU mode control"]
pub mod CPC_CACHE_CM_CTRL {
    #[doc = "Memory Low Power Level (MLPL) at RUN mode"]
    pub mod MLPL_AT_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at WAIT mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_WAIT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at STOP mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at SUSPEND mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SUSPEND {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) software change request, keep 1 until MLPL transition complete"]
    pub mod MLPL_SOFT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPC cache Setpoint control 0"]
pub mod CPC_CACHE_SP_CTRL_0 {
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 0. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 1. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 2. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 3. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 4. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 5. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 6. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 7. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPC cache Setpoint control 1"]
pub mod CPC_CACHE_SP_CTRL_1 {
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 8. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 9. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP9 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 10. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 11. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP11 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 12. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP12 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 13. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP13 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 14. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP14 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 15. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP15 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPC local memory Mode"]
pub mod CPC_LMEM_MODE {
    #[doc = "Control mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod CTRL_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not affected by any low power mode"]
            pub const CTRL_MODE_0: u32 = 0;
            #[doc = "Controlled by CPU power mode of the domain"]
            pub const CTRL_MODE_1: u32 = 0x01;
            #[doc = "Controlled by Setpoint"]
            pub const CTRL_MODE_2: u32 = 0x02;
        }
    }
}
#[doc = "CPC local memory CPU mode control"]
pub mod CPC_LMEM_CM_CTRL {
    #[doc = "Memory Low Power Level (MLPL) at RUN mode"]
    pub mod MLPL_AT_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at WAIT mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_WAIT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at STOP mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at SUSPEND mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SUSPEND {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) software change request, keep 1 until MLPL transition complete"]
    pub mod MLPL_SOFT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPC local memory Setpoint control 0"]
pub mod CPC_LMEM_SP_CTRL_0 {
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 0. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 1. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 2. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 3. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 4. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 5. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 6. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 7. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPC local memory Setpoint control 1"]
pub mod CPC_LMEM_SP_CTRL_1 {
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 8. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 9. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP9 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 10. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 11. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP11 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 12. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP12 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 13. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP13 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 14. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP14 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Low Power Level (MLPL) at Setpoint 15. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod MLPL_AT_SP15 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
