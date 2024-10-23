#[doc = "M33 Systick module"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SysTick Control and Status Register"]
    pub SYST_CSR: crate::RWRegister<u32>,
    #[doc = "SysTick Reload Value Register"]
    pub SYST_RVR: crate::RWRegister<u32>,
    #[doc = "SysTick Current Value Register"]
    pub SYST_CVR: crate::RWRegister<u32>,
    #[doc = "SysTick Calibration Value Register"]
    pub SYST_CALIB: crate::RORegister<u32>,
}
#[doc = "SysTick Control and Status Register"]
pub mod SYST_CSR {
    #[doc = "Enable/disable systick counter"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "counter disabled"]
            pub const COUNTER_DISABLED: u32 = 0;
            #[doc = "counter enabled"]
            pub const COUNTER_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Enable Systick interrupt."]
    pub mod TICKINT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "counting down to 0 does not assert the SysTick exception request"]
            pub const INTERRUPT_DISABLED: u32 = 0;
            #[doc = "counting down to 0 asserts the SysTick exception request"]
            pub const INTERRUPT_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Clock source selection."]
    pub mod CLKSOURCE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "external clock"]
            pub const EXTERNAL_CLOCK: u32 = 0;
            #[doc = "processor clock"]
            pub const PROCESSOR_CLOCK: u32 = 0x01;
        }
    }
    #[doc = "Returns 1 if timer counted to 0 since the last read of this register."]
    pub mod COUNTFLAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SysTick Reload Value Register"]
pub mod SYST_RVR {
    #[doc = "Value to load into the SysTick Current Value Register when the counter reaches 0"]
    pub mod RELOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SysTick Current Value Register"]
pub mod SYST_CVR {
    #[doc = "Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
    pub mod CURRENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SysTick Calibration Value Register"]
pub mod SYST_CALIB {
    #[doc = "Reload value to use for 10ms timing"]
    pub mod TENMS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the TENMS value is exact"]
    pub mod SKEW {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "10ms calibration value is exact"]
            pub const EXACT_VALUE: u32 = 0;
            #[doc = "10ms calibration value is inexact, because of the clock frequency"]
            pub const INEXACT_VALUE: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the device provides an alternative reference clock"]
    pub mod NOREF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The alternative reference clock is provided"]
            pub const CLOCK_PROVIDED: u32 = 0;
            #[doc = "The alternative reference clock is not provided"]
            pub const CLOCK_DISABLED: u32 = 0x01;
        }
    }
}
