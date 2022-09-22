#[doc = "WDOG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Watchdog Control Register"]
    pub WCR: crate::RWRegister<u16>,
    #[doc = "Watchdog Service Register"]
    pub WSR: crate::RWRegister<u16>,
    #[doc = "Watchdog Reset Status Register"]
    pub WRSR: crate::RORegister<u16>,
    #[doc = "Watchdog Interrupt Control Register"]
    pub WICR: crate::RWRegister<u16>,
    #[doc = "Watchdog Miscellaneous Control Register"]
    pub WMCR: crate::RWRegister<u16>,
}
#[doc = "Watchdog Control Register"]
pub mod WCR {
    #[doc = "WDZST"]
    pub mod WDZST {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continue timer operation (Default)."]
            pub const WDZST_0: u16 = 0;
            #[doc = "Suspend the watchdog timer."]
            pub const WDZST_1: u16 = 0x01;
        }
    }
    #[doc = "WDBG"]
    pub mod WDBG {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continue WDOG timer operation (Default)."]
            pub const WDBG_0: u16 = 0;
            #[doc = "Suspend the watchdog timer."]
            pub const WDBG_1: u16 = 0x01;
        }
    }
    #[doc = "WDE"]
    pub mod WDE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the Watchdog (Default)."]
            pub const WDE_0: u16 = 0;
            #[doc = "Enable the Watchdog."]
            pub const WDE_1: u16 = 0x01;
        }
    }
    #[doc = "WDT"]
    pub mod WDT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect on WDOG_B (Default)."]
            pub const WDT_0: u16 = 0;
            #[doc = "Assert WDOG_B upon a Watchdog Time-out event."]
            pub const WDT_1: u16 = 0x01;
        }
    }
    #[doc = "SRS"]
    pub mod SRS {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Assert system reset signal."]
            pub const SRS_0: u16 = 0;
            #[doc = "No effect on the system (Default)."]
            pub const SRS_1: u16 = 0x01;
        }
    }
    #[doc = "WDA"]
    pub mod WDA {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Assert WDOG_B output."]
            pub const WDA_0: u16 = 0;
            #[doc = "No effect on system (Default)."]
            pub const WDA_1: u16 = 0x01;
        }
    }
    #[doc = "software reset extension, an option way to generate software reset"]
    pub mod SRE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "using original way to generate software reset (default)"]
            pub const SRE_0: u16 = 0;
            #[doc = "using new way to generate software reset."]
            pub const SRE_1: u16 = 0x01;
        }
    }
    #[doc = "WDW"]
    pub mod WDW {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continue WDOG timer operation (Default)."]
            pub const WDW_0: u16 = 0;
            #[doc = "Suspend WDOG timer operation."]
            pub const WDW_1: u16 = 0x01;
        }
    }
    #[doc = "WT"]
    pub mod WT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "- 0.5 Seconds (Default)."]
            pub const WT_0: u16 = 0;
            #[doc = "- 1.0 Seconds."]
            pub const WT_1: u16 = 0x01;
            #[doc = "- 1.5 Seconds."]
            pub const WT_2: u16 = 0x02;
            #[doc = "- 2.0 Seconds."]
            pub const WT_3: u16 = 0x03;
            #[doc = "- 128 Seconds."]
            pub const WT_255: u16 = 0xff;
        }
    }
}
#[doc = "Watchdog Service Register"]
pub mod WSR {
    #[doc = "WSR"]
    pub mod WSR {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
            pub const WSR_21845: u16 = 0x5555;
            #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
            pub const WSR_43690: u16 = 0xaaaa;
        }
    }
}
#[doc = "Watchdog Reset Status Register"]
pub mod WRSR {
    #[doc = "SFTW"]
    pub mod SFTW {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not the result of a software reset."]
            pub const SFTW_0: u16 = 0;
            #[doc = "Reset is the result of a software reset."]
            pub const SFTW_1: u16 = 0x01;
        }
    }
    #[doc = "TOUT"]
    pub mod TOUT {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not the result of a WDOG timeout."]
            pub const TOUT_0: u16 = 0;
            #[doc = "Reset is the result of a WDOG timeout."]
            pub const TOUT_1: u16 = 0x01;
        }
    }
    #[doc = "POR"]
    pub mod POR {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not the result of a power on reset."]
            pub const POR_0: u16 = 0;
            #[doc = "Reset is the result of a power on reset."]
            pub const POR_1: u16 = 0x01;
        }
    }
}
#[doc = "Watchdog Interrupt Control Register"]
pub mod WICR {
    #[doc = "WICT"]
    pub mod WICT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 0 seconds."]
            pub const WICT_0: u16 = 0;
            #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 0.5 seconds."]
            pub const WICT_1: u16 = 0x01;
            #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 2 seconds (Default)."]
            pub const WICT_4: u16 = 0x04;
            #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 127.5 seconds."]
            pub const WICT_255: u16 = 0xff;
        }
    }
    #[doc = "WTIS"]
    pub mod WTIS {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt has occurred (Default)."]
            pub const WTIS_0: u16 = 0;
            #[doc = "Interrupt has occurred"]
            pub const WTIS_1: u16 = 0x01;
        }
    }
    #[doc = "WIE"]
    pub mod WIE {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Interrupt (Default)."]
            pub const WIE_0: u16 = 0;
            #[doc = "Enable Interrupt."]
            pub const WIE_1: u16 = 0x01;
        }
    }
}
#[doc = "Watchdog Miscellaneous Control Register"]
pub mod WMCR {
    #[doc = "PDE"]
    pub mod PDE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power Down Counter of WDOG is disabled."]
            pub const PDE_0: u16 = 0;
            #[doc = "Power Down Counter of WDOG is enabled (Default)."]
            pub const PDE_1: u16 = 0x01;
        }
    }
}
