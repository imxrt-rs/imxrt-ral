#[doc = "WDOG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Watchdog Control and Status Register"]
    pub CS: crate::RWRegister<u32>,
    #[doc = "Watchdog Counter Register"]
    pub CNT: crate::RWRegister<u32>,
    #[doc = "Watchdog Timeout Value Register"]
    pub TOVAL: crate::RWRegister<u32>,
    #[doc = "Watchdog Window Register"]
    pub WIN: crate::RWRegister<u32>,
}
#[doc = "Watchdog Control and Status Register"]
pub mod CS {
    #[doc = "Stop Enable"]
    pub mod STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog disabled in chip stop mode."]
            pub const STOP_0: u32 = 0;
            #[doc = "Watchdog enabled in chip stop mode."]
            pub const STOP_1: u32 = 0x01;
        }
    }
    #[doc = "Wait Enable"]
    pub mod WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog disabled in chip wait mode."]
            pub const WAIT_0: u32 = 0;
            #[doc = "Watchdog enabled in chip wait mode."]
            pub const WAIT_1: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog disabled in chip debug mode."]
            pub const DBG_0: u32 = 0;
            #[doc = "Watchdog enabled in chip debug mode."]
            pub const DBG_1: u32 = 0x01;
        }
    }
    #[doc = "Watchdog Test"]
    pub mod TST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog test mode disabled."]
            pub const TST_0: u32 = 0;
            #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
            pub const TST_1: u32 = 0x01;
            #[doc = "Watchdog test mode enabled, only the low byte is used. CNT\\[CNTLOW\\] is compared with TOVAL\\[TOVALLOW\\]."]
            pub const TST_2: u32 = 0x02;
            #[doc = "Watchdog test mode enabled, only the high byte is used. CNT\\[CNTHIGH\\] is compared with TOVAL\\[TOVALHIGH\\]."]
            pub const TST_3: u32 = 0x03;
        }
    }
    #[doc = "Allow updates"]
    pub mod UPDATE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
            pub const UPDATE_0: u32 = 0;
            #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
            pub const UPDATE_1: u32 = 0x01;
        }
    }
    #[doc = "Watchdog Interrupt"]
    pub mod INT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
            pub const INT_0: u32 = 0;
            #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks from the interrupt vector fetch."]
            pub const INT_1: u32 = 0x01;
        }
    }
    #[doc = "Watchdog Enable"]
    pub mod EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog disabled."]
            pub const EN_0: u32 = 0;
            #[doc = "Watchdog enabled."]
            pub const EN_1: u32 = 0x01;
        }
    }
    #[doc = "Watchdog Clock"]
    pub mod CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reconfiguration Success"]
    pub mod RCS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reconfiguring WDOG."]
            pub const RCS_0: u32 = 0;
            #[doc = "Reconfiguration is successful."]
            pub const RCS_1: u32 = 0x01;
        }
    }
    #[doc = "Unlock status"]
    pub mod ULK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WDOG is locked."]
            pub const ULK_0: u32 = 0;
            #[doc = "WDOG is unlocked."]
            pub const ULK_1: u32 = 0x01;
        }
    }
    #[doc = "Watchdog prescaler"]
    pub mod PRES {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "256 prescaler disabled."]
            pub const PRES_0: u32 = 0;
            #[doc = "256 prescaler enabled."]
            pub const PRES_1: u32 = 0x01;
        }
    }
    #[doc = "Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    pub mod CMD32EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
            pub const CMD32EN_0: u32 = 0;
            #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
            pub const CMD32EN_1: u32 = 0x01;
        }
    }
    #[doc = "Watchdog Interrupt Flag"]
    pub mod FLG {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt occurred."]
            pub const FLG_0: u32 = 0;
            #[doc = "An interrupt occurred."]
            pub const FLG_1: u32 = 0x01;
        }
    }
    #[doc = "Watchdog Window"]
    pub mod WIN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Window mode disabled."]
            pub const WIN_0: u32 = 0;
            #[doc = "Window mode enabled."]
            pub const WIN_1: u32 = 0x01;
        }
    }
}
#[doc = "Watchdog Counter Register"]
pub mod CNT {
    #[doc = "Low byte of the Watchdog Counter"]
    pub mod CNTLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High byte of the Watchdog Counter"]
    pub mod CNTHIGH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog Timeout Value Register"]
pub mod TOVAL {
    #[doc = "Low byte of the timeout value"]
    pub mod TOVALLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High byte of the timeout value"]
    pub mod TOVALHIGH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog Window Register"]
pub mod WIN {
    #[doc = "Low byte of Watchdog Window"]
    pub mod WINLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High byte of Watchdog Window"]
    pub mod WINHIGH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
