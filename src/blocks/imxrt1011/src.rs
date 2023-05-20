#[doc = "SRC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SRC Control Register"]
    pub SCR: crate::RWRegister<u32>,
    #[doc = "SRC Boot Mode Register 1"]
    pub SBMR1: crate::RORegister<u32>,
    #[doc = "SRC Reset Status Register"]
    pub SRSR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x10],
    #[doc = "SRC Boot Mode Register 2"]
    pub SBMR2: crate::RORegister<u32>,
    #[doc = "SRC General Purpose Register 1"]
    pub GPR1: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register 2"]
    pub GPR2: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register 3"]
    pub GPR3: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register 4"]
    pub GPR4: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register 5"]
    pub GPR5: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register 6"]
    pub GPR6: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register 7"]
    pub GPR7: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register 8"]
    pub GPR8: crate::RWRegister<u32>,
    #[doc = "SRC General Purpose Register 9"]
    pub GPR9: crate::RORegister<u32>,
    #[doc = "SRC General Purpose Register 10"]
    pub GPR10: crate::RWRegister<u32>,
}
#[doc = "SRC Control Register"]
pub mod SCR {
    #[doc = "lockup reset enable bit"]
    pub mod LOCKUP_RST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disabled"]
            pub const LOCKUP_RST_0: u32 = 0;
            #[doc = "enabled"]
            pub const LOCKUP_RST_1: u32 = 0x01;
        }
    }
    #[doc = "Mask wdog_rst_b source"]
    pub mod MASK_WDOG_RST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "wdog_rst_b is masked"]
            pub const MASK_WDOG_RST_5: u32 = 0x05;
            #[doc = "wdog_rst_b is not masked (default)"]
            pub const MASK_WDOG_RST_10: u32 = 0x0a;
        }
    }
    #[doc = "Software reset for core0 only"]
    pub mod CORE0_RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "do not assert core0 reset"]
            pub const CORE0_RST_0: u32 = 0;
            #[doc = "assert core0 reset"]
            pub const CORE0_RST_1: u32 = 0x01;
        }
    }
    #[doc = "Software reset for core0 debug only"]
    pub mod CORE0_DBG_RST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "do not assert core0 debug reset"]
            pub const CORE0_DBG_RST_0: u32 = 0;
            #[doc = "assert core0 debug reset"]
            pub const CORE0_DBG_RST_1: u32 = 0x01;
        }
    }
    #[doc = "Do not assert debug resets after power gating event of core"]
    pub mod DBG_RST_MSK_PG {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "do not mask core debug resets (debug resets will be asserted after power gating event)"]
            pub const DBG_RST_MSK_PG_0: u32 = 0;
            #[doc = "mask core debug resets (debug resets won't be asserted after power gating event)"]
            pub const DBG_RST_MSK_PG_1: u32 = 0x01;
        }
    }
    #[doc = "Mask wdog3_rst_b source"]
    pub mod MASK_WDOG3_RST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "wdog3_rst_b is masked"]
            pub const MASK_WDOG3_RST_5: u32 = 0x05;
            #[doc = "wdog3_rst_b is not masked"]
            pub const MASK_WDOG3_RST_10: u32 = 0x0a;
        }
    }
}
#[doc = "SRC Boot Mode Register 1"]
pub mod SBMR1 {
    #[doc = "Refer to fusemap."]
    pub mod BOOT_CFG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refer to fusemap."]
    pub mod BOOT_CFG2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refer to fusemap."]
    pub mod BOOT_CFG3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refer to fusemap."]
    pub mod BOOT_CFG4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRC Reset Status Register"]
pub mod SRSR {
    #[doc = "Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    pub mod IPP_RESET_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of ipp_reset_b pin."]
            pub const IPP_RESET_B_0: u32 = 0;
            #[doc = "Reset is a result of ipp_reset_b pin."]
            pub const IPP_RESET_B_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates a reset has been caused by CPU lockup."]
    #[deprecated(since = "0.5.1", note = "Use LOCKUP_SYSRESETREQ")]
    pub mod LOCKUP {
        pub use super::LOCKUP_SYSRESETREQ::*;
    }
    #[doc = "Indicates a reset has been caused by CPU lockup."]
    pub mod LOCKUP_SYSRESETREQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const LOCKUP_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const LOCKUP_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the reset was the result of the csu_reset_b input."]
    pub mod CSU_RESET_B {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the csu_reset_b event."]
            pub const CSU_RESET_B_0: u32 = 0;
            #[doc = "Reset is a result of the csu_reset_b event."]
            pub const CSU_RESET_B_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    pub mod IPP_USER_RESET_B {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
            pub const IPP_USER_RESET_B_0: u32 = 0;
            #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
            pub const IPP_USER_RESET_B_1: u32 = 0x01;
        }
    }
    #[doc = "IC Watchdog Time-out reset"]
    pub mod WDOG_RST_B {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const WDOG_RST_B_0: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const WDOG_RST_B_1: u32 = 0x01;
        }
    }
    #[doc = "HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    pub mod JTAG_RST_B {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
            pub const JTAG_RST_B_0: u32 = 0;
            #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
            pub const JTAG_RST_B_1: u32 = 0x01;
        }
    }
    #[doc = "JTAG software reset"]
    pub mod JTAG_SW_RST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const JTAG_SW_RST_0: u32 = 0;
            #[doc = "Reset is not a result of the mentioned case."]
            pub const JTAG_SW_RST_1: u32 = 0x01;
        }
    }
    #[doc = "IC Watchdog3 Time-out reset"]
    pub mod WDOG3_RST_B {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of the watchdog3 time-out event."]
            pub const WDOG3_RST_B_0: u32 = 0;
            #[doc = "Reset is a result of the watchdog3 time-out event."]
            pub const WDOG3_RST_B_1: u32 = 0x01;
        }
    }
    #[doc = "Temper Sensor software reset"]
    pub mod TEMPSENSE_RST_B {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not a result of software reset from Temperature Sensor."]
            pub const TEMPSENSE_RST_B_0: u32 = 0;
            #[doc = "Reset is a result of software reset from Temperature Sensor."]
            pub const TEMPSENSE_RST_B_1: u32 = 0x01;
        }
    }
}
#[doc = "SRC Boot Mode Register 2"]
pub mod SBMR2 {
    #[doc = "SECONFIG\\[1\\] shows the state of the SECONFIG\\[1\\] fuse"]
    pub mod SEC_CONFIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DIR_BT_DIS shows the state of the DIR_BT_DIS fuse"]
    pub mod DIR_BT_DIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BT_FUSE_SEL (connected to gpio bt_fuse_sel) shows the state of the BT_FUSE_SEL fuse"]
    pub mod BT_FUSE_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BMOD\\[1:0\\] shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B"]
    pub mod BMOD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRC General Purpose Register 1"]
pub mod GPR1 {
    #[doc = "Holds entry function for core0 for waking-up from low power mode"]
    pub mod PERSISTENT_ENTRY0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRC General Purpose Register 2"]
pub mod GPR2 {
    #[doc = "Holds argument of entry function for core0 for waking-up from low power mode"]
    pub mod PERSISTENT_ARG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRC General Purpose Register 10"]
pub mod GPR10 {
    #[doc = "This field identifies which image must be used - 0/1/2/3"]
    pub mod PERSIST_REDUNDANT_BOOT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit identifies which image must be used - primary and secondary"]
    pub mod PERSIST_SECONDARY_BOOT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
