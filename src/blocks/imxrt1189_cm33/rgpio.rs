#[doc = "GPIO"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RORegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Lock"]
    pub LOCK: crate::RWRegister<u32>,
    #[doc = "Pin Control Non-Secure"]
    pub PCNS: crate::RWRegister<u32>,
    #[doc = "Interrupt Control Non-Secure"]
    pub ICNS: crate::RWRegister<u32>,
    #[doc = "Pin Control Non-Privilege"]
    pub PCNP: crate::RWRegister<u32>,
    #[doc = "Interrupt Control Non-Privilege"]
    pub ICNP: crate::RWRegister<u32>,
    _reserved1: [u8; 0x20],
    #[doc = "Port Data Output Register"]
    pub PDOR: crate::RWRegister<u32>,
    #[doc = "Port Set Output Register"]
    pub PSOR: crate::RWRegister<u32>,
    #[doc = "Port Clear Output Register"]
    pub PCOR: crate::RWRegister<u32>,
    #[doc = "Port Toggle Output Register"]
    pub PTOR: crate::RWRegister<u32>,
    #[doc = "Port Data Input Register"]
    pub PDIR: crate::RORegister<u32>,
    #[doc = "Port Data Direction Register"]
    pub PDDR: crate::RWRegister<u32>,
    #[doc = "Port Input Disable Register"]
    pub PIDR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Pin Data Register a"]
    pub PDR: [crate::RWRegister<u8>; 32usize],
    #[doc = "Interrupt Control Register %s"]
    pub ICR: [crate::RWRegister<u32>; 32usize],
    #[doc = "Global Interrupt Control Low Register"]
    pub GICLR: crate::RWRegister<u32>,
    #[doc = "Global Interrupt Control High Register"]
    pub GICHR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x18],
    #[doc = "Interrupt Status Flag Register"]
    pub ISFR: [crate::RWRegister<u32>; 2usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Basic implementation."]
            pub const FEATURE0: u32 = 0;
            #[doc = "Protection registers implemented."]
            pub const FEATURE1: u32 = 0x01;
        }
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "Interrupt Number"]
    pub mod IRQNUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lock"]
pub mod LOCK {
    #[doc = "Lock PCNS"]
    pub mod PCNS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PCNS register is writable by software in Secure-Privilege state."]
            pub const PCNS0: u32 = 0;
            #[doc = "PCNS register is not writable until the next reset."]
            pub const PCNS1: u32 = 0x01;
        }
    }
    #[doc = "Lock ICNS"]
    pub mod ICNS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ICNS register is writable by software in Secure-Privilege state."]
            pub const ICNS0: u32 = 0;
            #[doc = "ICNS register is not writable until the next reset."]
            pub const ICNS1: u32 = 0x01;
        }
    }
    #[doc = "Lock PCNP"]
    pub mod PCNP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PCNP register is writable by software in Secure-Privilege state."]
            pub const PCNP0: u32 = 0;
            #[doc = "PCNP register is not writable until the next reset."]
            pub const PCNP1: u32 = 0x01;
        }
    }
    #[doc = "Lock ICNP"]
    pub mod ICNP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ICNP register is writable by software in Secure-Privilege state."]
            pub const ICNP0: u32 = 0;
            #[doc = "ICNP register is not writable until the next reset."]
            pub const ICNP1: u32 = 0x01;
        }
    }
}
#[doc = "Pin Control Non-Secure"]
pub mod PCNS {
    #[doc = "Non-Secure Enable"]
    pub mod NSE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Secure state. When the corresponding pin's registers are accessed by software in Non-Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE0: u32 = 0;
            #[doc = "The pin is configured for Non-Secure access. Read or write access to the corresponding pin's registers and bit fields is only allowed by software in Non-Secure state. When the corresponding pin's registers are accessed by software in Secure state, all bits in the registers related to that pin are read zero and write ignored."]
            pub const NSE1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Control Non-Secure"]
pub mod ICNS {
    #[doc = "Non-Secure Enable"]
    pub mod NSE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt or DMA request is configured for Secure access. Only software in Secure state can configure a pin to use the corresponding interrupt or DMA request or reconfigure a pin that is already configured to use the corresponding interrupt or DMA request."]
            pub const NSE0: u32 = 0;
            #[doc = "The interrupt or DMA request is configured for Non-Secure access. Only software in Non-Secure state can configure a pin to use the corresponding interrupt or DMA request or reconfigure a pin that is already configured to use the corresponding interrupt or DMA request."]
            pub const NSE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Secure Enable"]
    pub mod NSE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt or DMA request is configured for Secure access. Only software in Secure state can configure a pin to use the corresponding interrupt or DMA request or reconfigure a pin that is already configured to use the corresponding interrupt or DMA request."]
            pub const NSE0: u32 = 0;
            #[doc = "The interrupt or DMA request is configured for Non-Secure access. Only software in Non-Secure state can configure a pin to use the corresponding interrupt or DMA request or reconfigure a pin that is already configured to use the corresponding interrupt or DMA request."]
            pub const NSE1: u32 = 0x01;
        }
    }
}
#[doc = "Pin Control Non-Privilege"]
pub mod PCNP {
    #[doc = "Non-Privilege Enable"]
    pub mod NPE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Write access to the corresponding pin's registers and bit fields is allowed only by software in Privilege state. When the corresponding pin's registers and bit fields are accessed by software in Non-Privilege state, all bits related to that pin in this GPIO are readable but write ignored."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access, Read or write access to the corresponding pin's registers is allowed by software in both Privilege and Non-Privilege state."]
            pub const NPE1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Control Non-Privilege"]
pub mod ICNP {
    #[doc = "Non-Privilege Enable"]
    pub mod NPE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Only software in Privilege state can configure a pin to use the corresponding interrupt/DMA request or reconfigure a pin that is already configured to use the corresponding interrupt/DMA request."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access. Software in either Privilege or Non-Privilege state can configure a pin to use the corresponding interrupt/DMA request or reconfigure a pin that is already configured to use the corresponding interrupt/DMA request."]
            pub const NPE1: u32 = 0x01;
        }
    }
    #[doc = "Non-Privilege Enable"]
    pub mod NPE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The pin is configured for Privilege access. Only software in Privilege state can configure a pin to use the corresponding interrupt/DMA request or reconfigure a pin that is already configured to use the corresponding interrupt/DMA request."]
            pub const NPE0: u32 = 0;
            #[doc = "The pin is configured for Non-Privilege access. Software in either Privilege or Non-Privilege state can configure a pin to use the corresponding interrupt/DMA request or reconfigure a pin that is already configured to use the corresponding interrupt/DMA request."]
            pub const NPE1: u32 = 0x01;
        }
    }
}
#[doc = "Port Data Output Register"]
pub mod PDOR {
    #[doc = "Port Data Output"]
    pub mod PDO0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Output"]
    pub mod PDO31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic level 0 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO0: u32 = 0;
            #[doc = "Logic level 1 is driven on pin, if the pin is configured for general-purpose output."]
            pub const PDO1: u32 = 0x01;
        }
    }
}
#[doc = "Port Set Output Register"]
pub mod PSOR {
    #[doc = "Port Set Output"]
    pub mod PTSO0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
    #[doc = "Port Set Output"]
    pub mod PTSO31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTSO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to logic 1."]
            pub const PTSO1: u32 = 0x01;
        }
    }
}
#[doc = "Port Clear Output Register"]
pub mod PCOR {
    #[doc = "Port Clear Output"]
    pub mod PTCO0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
    #[doc = "Port Clear Output"]
    pub mod PTCO31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTCO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is cleared to logic 0."]
            pub const PTCO1: u32 = 0x01;
        }
    }
}
#[doc = "Port Toggle Output Register"]
pub mod PTOR {
    #[doc = "Port Toggle Output"]
    pub mod PTTO0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
    #[doc = "Port Toggle Output"]
    pub mod PTTO31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Corresponding field of PDOR\\[PDOn\\] does not change."]
            pub const PTTO0: u32 = 0;
            #[doc = "Corresponding field of PDOR\\[PDOn\\] is set to the inverse of its current logic state."]
            pub const PTTO1: u32 = 0x01;
        }
    }
}
#[doc = "Port Data Input Register"]
pub mod PDIR {
    #[doc = "Port Data Input"]
    pub mod PDI0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Input"]
    pub mod PDI31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic 0 or is not configured for use by digital function."]
            pub const PDI0: u32 = 0;
            #[doc = "Pin logic level is logic 1."]
            pub const PDI1: u32 = 0x01;
        }
    }
}
#[doc = "Port Data Direction Register"]
pub mod PDDR {
    #[doc = "Port Data Direction"]
    pub mod PDD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
    #[doc = "Port Data Direction"]
    pub mod PDD31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured as general-purpose input for the GPIO function."]
            pub const PDD0: u32 = 0;
            #[doc = "Pin is configured as general-purpose output for the GPIO function."]
            pub const PDD1: u32 = 0x01;
        }
    }
}
#[doc = "Port Input Disable Register"]
pub mod PIDR {
    #[doc = "Port Input Disable"]
    pub mod PID0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
    #[doc = "Port Input Disable"]
    pub mod PID31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is configured for general-purpose input provided, the pin is configured for any digital function."]
            pub const PID0: u32 = 0;
            #[doc = "Pin is disabled for general-purpose input."]
            pub const PID1: u32 = 0x01;
        }
    }
}
#[doc = "Pin Data Register a"]
pub mod PDR {
    #[doc = "Pin Data (input and output)"]
    pub mod PD {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin logic level is logic zero or not configured for use by digital function."]
            pub const PD0: u8 = 0;
            #[doc = "Pin logic level is logic one."]
            pub const PD1: u8 = 0x01;
        }
    }
}
#[doc = "Interrupt Control Register %s"]
pub mod ICR {
    #[doc = "Interrupt Configuration"]
    pub mod IRQC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt Status Flag (ISF) is disabled."]
            pub const IRQC0: u32 = 0;
            #[doc = "ISF flag and DMA request on rising edge."]
            pub const IRQC1: u32 = 0x01;
            #[doc = "ISF flag and DMA request on falling edge."]
            pub const IRQC2: u32 = 0x02;
            #[doc = "ISF flag and DMA request on either edge."]
            pub const IRQC3: u32 = 0x03;
            #[doc = "ISF flag sets on rising edge."]
            pub const IRQC5: u32 = 0x05;
            #[doc = "ISF flag sets on falling edge."]
            pub const IRQC6: u32 = 0x06;
            #[doc = "ISF flag sets on either edge."]
            pub const IRQC7: u32 = 0x07;
            #[doc = "ISF flag and Interrupt when logic 0."]
            pub const IRQC8: u32 = 0x08;
            #[doc = "ISF flag and Interrupt on rising-edge."]
            pub const IRQC9: u32 = 0x09;
            #[doc = "ISF flag and Interrupt on falling-edge."]
            pub const IRQC10: u32 = 0x0a;
            #[doc = "ISF flag and Interrupt on either edge."]
            pub const IRQC11: u32 = 0x0b;
            #[doc = "ISF flag and Interrupt when logic 1."]
            pub const IRQC12: u32 = 0x0c;
        }
    }
    #[doc = "Interrupt Select"]
    pub mod IRQS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt/DMA request 0."]
            pub const IRQS0: u32 = 0;
            #[doc = "Interrupt/DMA request 1."]
            pub const IRQS1: u32 = 0x01;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt configuration by ICR\\[23:0\\] is not locked and can be updated."]
            pub const LK0: u32 = 0;
            #[doc = "Interrupt configuration by ICR\\[23:0\\] is locked and cannot be updated until next system reset."]
            pub const LK1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
}
#[doc = "Global Interrupt Control Low Register"]
pub mod GICLR {
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Data"]
    pub mod GIWD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Global Interrupt Control High Register"]
pub mod GICHR {
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE17 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE18 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE19 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE20 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE21 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE22 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE23 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE24 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE25 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE26 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE27 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE28 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE29 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE30 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Enable"]
    pub mod GIWE31 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is not updated with the value in GIWD."]
            pub const GIWE0: u32 = 0;
            #[doc = "Upper 16-bit of corresponding Interrupt Control Register is updated with the value in GIWD."]
            pub const GIWE1: u32 = 0x01;
        }
    }
    #[doc = "Global Interrupt Write Data"]
    pub mod GIWD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Flag Register"]
pub mod ISFR {
    #[doc = "Interrupt Status Flag"]
    pub mod ISF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Status Flag"]
    pub mod ISF31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configured interrupt is not detected on the pin of the same number."]
            pub const ISF0: u32 = 0;
            #[doc = "Configured interrupt is detected on the pin of the same number. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
            pub const ISF1: u32 = 0x01;
        }
    }
}
