#[doc = "CM7_MCM"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "Interrupt Status and Control Register"]
    pub ISCR: crate::RWRegister<u32>,
    #[doc = "SysTick Calibration Register"]
    pub CFGSTCALIB: crate::RWRegister<u32>,
    #[doc = "Process ID"]
    pub PID: crate::RWRegister<u32>,
}
#[doc = "Interrupt Status and Control Register"]
pub mod ISCR {
    #[doc = "Write Abort on Slave"]
    pub mod WABS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No abort"]
            pub const NOABORT: u32 = 0;
            #[doc = "Abort"]
            pub const ABORT: u32 = 0x01;
        }
    }
    #[doc = "Write Abort on Slave Overrun"]
    pub mod WABSO {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No write abort overrun"]
            pub const NO: u32 = 0;
            #[doc = "Write abort overrun occurred"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "FPU Invalid Operation interrupt Status"]
    pub mod FIOC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "FPU Divide-by-Zero Interrupt Status"]
    pub mod FDZC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "FPU Overflow Interrupt Status"]
    pub mod FOFC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "FPU Underflow Interrupt Status"]
    pub mod FUFC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "FPU Inexact Interrupt Status"]
    pub mod FIXC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "FPU Input Denormal Interrupt Status"]
    pub mod FIDC {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "TCM Write Abort Interrupt Enable"]
    pub mod WABE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FPU Invalid Operation Interrupt Enable"]
    pub mod FIOCE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FPU Divide-by-Zero Interrupt Enable"]
    pub mod FDZCE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FPU Overflow Interrupt Enable"]
    pub mod FOFCE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FPU Underflow Interrupt Enable"]
    pub mod FUFCE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FPU Inexact Interrupt Enable"]
    pub mod FIXCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FPU Input Denormal Interrupt Enable"]
    pub mod FIDCE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "SysTick Calibration Register"]
pub mod CFGSTCALIB {
    #[doc = "Ten milliseconds"]
    pub mod TENMS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Skew"]
    pub mod SKEW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock skew"]
            pub const NO: u32 = 0;
            #[doc = "Having clock skew"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "No Reference Clock"]
    pub mod NOREF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Process ID"]
pub mod PID {
    #[doc = "Process ID"]
    pub mod PID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
