#[doc = "SRC General"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "Authentication Control"]
    pub AUTHEN_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "SRC Control Register"]
    pub SCR: crate::RWRegister<u32>,
    #[doc = "SRC Reset Trigger Mode Register"]
    pub SRTMR: crate::RWRegister<u32>,
    #[doc = "SRC Reset Mask Register"]
    pub SRMASK: crate::RWRegister<u32>,
    _reserved2: [u8; 0x24],
    #[doc = "SRC Boot Mode Register 1"]
    pub SBMR1: crate::RORegister<u32>,
    #[doc = "SRC Boot Mode Register 2"]
    pub SBMR2: crate::RORegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "SRC Reset Status Register backup in BBSM domain"]
    pub SRSR_BBSM: crate::RWRegister<u32>,
    #[doc = "SRC Reset Status Register"]
    pub SRSR: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register"]
    pub GPR: [crate::RWRegister<u32>; 20usize],
}
#[doc = "Authentication Control"]
pub mod AUTHEN_CTRL {
    #[doc = "Configuration lock"]
    pub mod LOCK_CFG {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "General registers are not locked."]
            pub const DISABLE: u32 = 0;
            #[doc = "LOCK_CFG and registers in the list are locked."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode write"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "General registers can only be written in privilege mode."]
            pub const DIS: u32 = 0;
            #[doc = "General registers can be written either in privilege mode or user mode."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure mode access"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "General registers can only be written in secure mode."]
            pub const DIS: u32 = 0;
            #[doc = "General registers can be written either in secure mode or non-secure mode."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock Trust Zone Non Secure(TZ_NS) and Trust Zone User(TZ_USER) bits"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TZ_NS and TZ_USER values can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "LOCK_TZ, TZ_NS and TZ_USER values cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WHITE_LIST value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "LOCK_LIST and WHITE_LIST values cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Core with domain ID=0 can write General registers."]
            pub const DOMAIN0: u32 = 0x01;
            #[doc = "Core with domain ID=1 can write General registers."]
            pub const DOMAIN1: u32 = 0x02;
            #[doc = "Core with domain ID=2 can write General registers."]
            pub const DOMAIN2: u32 = 0x04;
            #[doc = "Core with domain ID=3 can write General registers."]
            pub const DOMAIN3: u32 = 0x08;
            #[doc = "Core with domain ID=4 can write General registers."]
            pub const DOMAIN4: u32 = 0x10;
            #[doc = "Core with domain ID=5 can write General registers."]
            pub const DOMAIN5: u32 = 0x20;
            #[doc = "Core with domain ID=6 can write General registers."]
            pub const DOMAIN6: u32 = 0x40;
            #[doc = "Core with domain ID=7 can write General registers."]
            pub const DOMAIN7: u32 = 0x80;
            #[doc = "Core with domain ID=8 can write General registers."]
            pub const DOMAIN8: u32 = 0x0100;
            #[doc = "Core with domain ID=9 can write General registers."]
            pub const DOMAIN9: u32 = 0x0200;
            #[doc = "Core with domain ID=10 can write General registers."]
            pub const DOMAIN10: u32 = 0x0400;
            #[doc = "Core with domain ID=11 can write General registers."]
            pub const DOMAIN11: u32 = 0x0800;
            #[doc = "Core with domain ID=12 can write General registers."]
            pub const DOMAIN12: u32 = 0x1000;
            #[doc = "Core with domain ID=13 can write General registers."]
            pub const DOMAIN13: u32 = 0x2000;
            #[doc = "Core with domain ID=14 can write General registers."]
            pub const DOMAIN14: u32 = 0x4000;
            #[doc = "Core with domain ID=15 can write General registers."]
            pub const DOMAIN15: u32 = 0x8000;
        }
    }
}
#[doc = "SRC Control Register"]
pub mod SCR {
    #[doc = "Boot release M7"]
    pub mod BT_RELEASE_M7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Holds M7 Core reset."]
            pub const DISABLE: u32 = 0;
            #[doc = "Releases M7 Core reset and let it run. After this bit is set, it cannot be cleared by SW write."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "SRC Reset Trigger Mode Register"]
pub mod SRTMR {
    #[doc = "Wdog1 reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod WDOG1_TRIG_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "Wdog2 reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod WDOG2_TRIG_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "Wdog3 reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod WDOG3_TRIG_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "Wdog4 reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod WDOG4_TRIG_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "Wdog5 reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod WDOG5_TRIG_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "TempSense reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod TEMPSENSE_TRIG_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "Edgelock reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod EDGELOCK_TRIG_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "Jtagsw reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod JTAGSW_TRIG_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "CM33 reset trigger mode configuration, locked by LOCK_CFG field."]
    pub mod CM33_RESET_TRIG_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "CM33 lockup trigger mode configuration, locked by LOCK_CFG field."]
    pub mod CM33_LOCKUP_TRIG_MODE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "CM7 reset trigger mode configuration, locked by LOCK_CFG field"]
    pub mod CM7_RESET_TRIG_MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "CM7 lockup trigger mode configuration, locked by LOCK_CFG field"]
    pub mod CM7_LOCKUP_TRIG_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "DCDC over voltage trigger mode configuration, locked by LOCK_CFG field"]
    pub mod DCDC_OVVT_TRIG_MODE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
    #[doc = "ECAT reset output mode configuration, locked by LOCK_CFG field"]
    pub mod ECAT_RSTO_TRIG_MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level-sensitive: System stays in reset until the reset source deasserts."]
            pub const LEVEL: u32 = 0;
            #[doc = "Edge-sensitive: System resets once, even if the reset source remains asserted."]
            pub const EDGE: u32 = 0x01;
        }
    }
}
#[doc = "SRC Reset Mask Register"]
pub mod SRMASK {
    #[doc = "WDOG1 reset mask"]
    pub mod WDOG1_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG2 reset mask"]
    pub mod WDOG2_MASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG3 reset mask"]
    pub mod WDOG3_MASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG4 reset mask"]
    pub mod WDOG4_MASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG5 reset mask"]
    pub mod WDOG5_MASK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "TempSense reset mask"]
    pub mod TEMPSENSE_MASK {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Edgelock reset mask"]
    pub mod EDGELOCK_MASK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "JTAGSW reset mask"]
    pub mod JTAGSW_MASK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "CM33 reset mask"]
    pub mod CM33_RESET_MASK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "CM33 lockup mask"]
    pub mod CM33_LOCKUP_MASK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "CM7 reset mask"]
    pub mod CM7_RESET_MASK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "CM7 lockup reset mask"]
    pub mod CM7_LOCKUP_MASK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "DCDC over voltage mask"]
    pub mod DCDC_OVVT_MASK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "ECAT reset output mask"]
    pub mod ECAT_RSTO_MASK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The reset source can work"]
            pub const DIS: u32 = 0;
            #[doc = "The reset source is masked, cannot work"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock WDOG1_MASK"]
    pub mod WDOG1_MASK_LOCKED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This bit and WDOG1_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and WDOG1_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock WDOG2_MASK"]
    pub mod WDOG2_MASK_LOCKED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This bit and WDOG2_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and WDOG2_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock WDOG3_MASK"]
    pub mod WDOG3_MASK_LOCKED {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This bit and WDOG3_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and WDOG3_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock WDOG4_MASK"]
    pub mod WDOG4_MASK_LOCKED {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This bit and WDOG4_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and WDOG4_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock WDOG5_MASK"]
    pub mod WDOG5_MASK_LOCKED {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This bit and WDOG5_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and WDOG5_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock TEMPSENSE_MASK"]
    pub mod TEMPSENSE_MASK_LOCKED {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TEMPSENSE_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and TEMPSENSE_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock EDGELOCK_MASK"]
    pub mod EDGELOCK_MASK_LOCKED {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EDGELOCK_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and EDGELOCK_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock JTAGSW_MASK"]
    pub mod JTAGSW_MASK_LOCKED {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "JTAGSW_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and JTAGSW_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock CM33_RESET_MASK"]
    pub mod CM33_RESET_MASK_LOCKED {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CM33_RESET_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and CM33_RESET_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock CM33_LOCKUP_MASK"]
    pub mod CM33_LOCKUP_MASK_LOCKED {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CM33_LOCKUP_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and CM33_LOCKUP_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock CM7 reset mask bit"]
    pub mod CM7_RESET_MASK_LOCKED {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CM7_RESET_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and CM7_RESET_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock CM7_LOCKUP_MASK"]
    pub mod CM7_LOCKUP_MASK_LOCKED {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CM7_LOCKUP_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and CM7_LOCKUP_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock DCDC_OVVT_MASK"]
    pub mod DCDC_OVVT_MASK_LOCKED {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DCDC_OVVT_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and DCDC_OVVT_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Lock ECAT_RSTO_MASK"]
    pub mod ECAT_RSTO_MASK_LOCKED {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECAT_RSTO_MASK's value can be changed."]
            pub const DIS: u32 = 0;
            #[doc = "This bit and ECAT_RSTO_MASK's value cannot be changed."]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "SRC Boot Mode Register 1"]
pub mod SBMR1 {}
#[doc = "SRC Boot Mode Register 2"]
pub mod SBMR2 {
    #[doc = "IPP_BOOT_MODE\\[5:4\\] reserved"]
    pub mod IPP_BOOT_MODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Boot from internal Fuses"]
            pub const BOOTOPT0: u32 = 0;
            #[doc = "Serial Downloader: USB1 or LPUART1"]
            pub const BOOTOPT1: u32 = 0x01;
            #[doc = "USDHC1 8-bit eMMC 5.1"]
            pub const BOOTOPT2: u32 = 0x02;
            #[doc = "USDHC2 4-bit SD 3.0"]
            pub const BOOTOPT3: u32 = 0x03;
            #[doc = "FlexSPI Serial NOR with SFDP (JESD-216) discoverable parameters"]
            pub const BOOTOPT4: u32 = 0x04;
            #[doc = "FlexSPI Serial NAND 2k page"]
            pub const BOOTOPT5: u32 = 0x05;
            #[doc = "FlexSPI Serial NAND 4k page"]
            pub const BOOTOPT6: u32 = 0x06;
            #[doc = "Test mode/Infinite loop mode"]
            pub const BOOTOPT7: u32 = 0x07;
        }
    }
}
#[doc = "SRC Reset Status Register backup in BBSM domain"]
pub mod SRSR_BBSM {
    #[doc = "Indicates whether the reset was the result of power up or chip PAD POR_B."]
    pub mod POR_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of power up or chip PAD POR_B."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of power up or chip PAD POR_B."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog1 time-out event."]
    pub mod WDOG1_RST_B {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog2 time-out event."]
    pub mod WDOG2_RST_B {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog3 time-out"]
    pub mod WDOG3_RST_B {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog3 time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog3 time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog4 time-out"]
    pub mod WDOG4_RST_B {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog5 time-out"]
    pub mod WDOG5_RST_B {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "TempSensor software reset. Indicates whether the reset was the result of software reset from on-chip Temperature Sensor."]
    pub mod TEMPSENSE_RST_B {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of software reset from Temperature Sensor."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of software reset from Temperature Sensor."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the reset was the result of the Edgelock's reset input."]
    pub mod EDGELOCK_RESET_B {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the Edgelock's reset event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the Edgelock's reset event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    pub mod JTAG_SW_RST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of software reset from JTAG."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of software reset from JTAG."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether reset was the result of cm33 reset request"]
    pub mod CM33_REQUEST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of cm33 reset request."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of cm33 reset request."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by cm33 CPU lockup"]
    pub mod CM33_LOCKUP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the cm33 lockup."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the cm33 lockup."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether reset was the result of cm7 reset request"]
    pub mod CM7_REQUEST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of cm7 reset request."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of cm7 reset request."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by CM7 CPU"]
    pub mod CM7_LOCKUP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the cm7 lockup."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the cm7 lockup."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by DCDC over voltage"]
    pub mod DCDC_OVVT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the DCDC over voltage."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the DCDC over voltage."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by ECAT reset output"]
    pub mod ECAT_RSTO {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the ECAT reset output."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the ECAT reset output."]
            pub const YES: u32 = 0x01;
        }
    }
}
#[doc = "SRC Reset Status Register"]
pub mod SRSR {
    #[doc = "Indicates whether the reset was the result of POR."]
    pub mod POR_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of POR."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of POR."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog1 time-out event."]
    pub mod WDOG1_RST_B {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog2 time-out event."]
    pub mod WDOG2_RST_B {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog3 time-out"]
    pub mod WDOG3_RST_B {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog3 time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog3 time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog4 time-out"]
    pub mod WDOG4_RST_B {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Time-out reset. Indicates whether the reset was the result of the watchdog5 time-out"]
    pub mod WDOG5_RST_B {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Temper Sensor software reset. Indicates whether the reset was the result of software reset from on-chip Temperature Sensor."]
    pub mod TEMPSENSE_RST_B {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of software reset from Temperature Sensor."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of software reset from Temperature Sensor."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the reset was the result of the Edgelock's reset input."]
    pub mod EDGELOCK_RESET_B {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the Edgelock's reset event."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the Edgelock's reset event."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    pub mod JTAG_SW_RST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of software reset from JTAG."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of software reset from JTAG."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether reset was the result of cm33 reset request"]
    pub mod CM33_REQUEST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of cm33 reset request."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of cm33 reset request."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by cm33 CPU lockup"]
    pub mod CM33_LOCKUP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the cm33 lockup."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the cm33 lockup."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether reset was the result of cm7 reset request"]
    pub mod CM7_REQUEST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of cm7 reset request."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of cm7 reset request."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by CM7 CPU"]
    pub mod CM7_LOCKUP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the cm7 lockup."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the cm7 lockup."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by DCDC over voltage"]
    pub mod DCDC_OVVT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the DCDC over voltage."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the DCDC over voltage."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by ECAT reset output"]
    pub mod ECAT_RSTO {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the ECAT reset output."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of the ECAT reset output."]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the reset was the result of chip PAD POR_B."]
    pub mod IPP_POR_B {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of chip PAD POR_B."]
            pub const NO: u32 = 0;
            #[doc = "Reset is a result of chip PAD POR_B."]
            pub const YES: u32 = 0x01;
        }
    }
}
#[doc = "SRC General Purpose Register"]
pub mod GPR {
    #[doc = "General Purpose Register."]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
