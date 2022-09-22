#[doc = "EWM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u8>,
    #[doc = "Service Register"]
    pub SERV: crate::RWRegister<u8>,
    #[doc = "Compare Low Register"]
    pub CMPL: crate::RWRegister<u8>,
    #[doc = "Compare High Register"]
    pub CMPH: crate::RWRegister<u8>,
    #[doc = "Clock Control Register"]
    pub CLKCTRL: crate::RWRegister<u8>,
    #[doc = "Clock Prescaler Register"]
    pub CLKPRESCALER: crate::RWRegister<u8>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "EWM enable."]
    pub mod EWMEN {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EWM module is disabled."]
            pub const DISABLE: u8 = 0;
            #[doc = "EWM module is enabled."]
            pub const ENABLE: u8 = 0x01;
        }
    }
    #[doc = "EWM_in's Assertion State Select."]
    pub mod ASSIN {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default assert state of the EWM_in signal."]
            pub const DISABLE: u8 = 0;
            #[doc = "Inverts the assert state of EWM_in signal."]
            pub const ENABLE: u8 = 0x01;
        }
    }
    #[doc = "Input Enable."]
    pub mod INEN {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EWM_in port is disabled."]
            pub const DISABLE: u8 = 0;
            #[doc = "EWM_in port is enabled."]
            pub const ENABLE: u8 = 0x01;
        }
    }
    #[doc = "Interrupt Enable."]
    pub mod INTEN {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts the interrupt request."]
            pub const ZERO: u8 = 0;
            #[doc = "Generates an interrupt request, when EWM_OUT_b is asserted."]
            pub const INT_REQ: u8 = 0x01;
        }
    }
}
#[doc = "Service Register"]
pub mod SERV {
    #[doc = "SERVICE"]
    pub mod SERVICE {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Compare Low Register"]
pub mod CMPL {
    #[doc = "COMPAREL"]
    pub mod COMPAREL {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Compare High Register"]
pub mod CMPH {
    #[doc = "COMPAREH"]
    pub mod COMPAREH {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Control Register"]
pub mod CLKCTRL {
    #[doc = "CLKSEL"]
    pub mod CLKSEL {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Prescaler Register"]
pub mod CLKPRESCALER {
    #[doc = "CLK_DIV"]
    pub mod CLK_DIV {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
