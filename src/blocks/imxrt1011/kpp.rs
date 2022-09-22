#[doc = "KPP Registers"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Keypad Control Register"]
    pub KPCR: crate::RWRegister<u16>,
    #[doc = "Keypad Status Register"]
    pub KPSR: crate::RWRegister<u16>,
    #[doc = "Keypad Data Direction Register"]
    pub KDDR: crate::RWRegister<u16>,
    #[doc = "Keypad Data Register"]
    pub KPDR: crate::RWRegister<u16>,
}
#[doc = "Keypad Control Register"]
pub mod KPCR {
    #[doc = "Keypad Row Enable"]
    pub mod KRE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Row is not included in the keypad key press detect."]
            pub const KRE_0: u16 = 0;
            #[doc = "Row is included in the keypad key press detect."]
            pub const KRE_1: u16 = 0x01;
        }
    }
    #[doc = "Keypad Column Strobe Open-Drain Enable"]
    pub mod KCO {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Column strobe output is totem pole drive."]
            pub const TOTEM_POLE: u16 = 0;
            #[doc = "Column strobe output is open drain."]
            pub const OPEN_DRAIN: u16 = 0x01;
        }
    }
}
#[doc = "Keypad Status Register"]
pub mod KPSR {
    #[doc = "Keypad Key Depress"]
    pub mod KPKD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key presses detected"]
            pub const KPKD_0: u16 = 0;
            #[doc = "A key has been depressed"]
            pub const KPKD_1: u16 = 0x01;
        }
    }
    #[doc = "Keypad Key Release"]
    pub mod KPKR {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key release detected"]
            pub const KPKR_0: u16 = 0;
            #[doc = "All keys have been released"]
            pub const KPKR_1: u16 = 0x01;
        }
    }
    #[doc = "Key Depress Synchronizer Clear"]
    pub mod KDSC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const KDSC_0: u16 = 0;
            #[doc = "Set bits that clear the keypad depress synchronizer chain"]
            pub const KDSC_1: u16 = 0x01;
        }
    }
    #[doc = "Key Release Synchronizer Set"]
    pub mod KRSS {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const KRSS_0: u16 = 0;
            #[doc = "Set bits which sets keypad release synchronizer chain"]
            pub const KRSS_1: u16 = 0x01;
        }
    }
    #[doc = "Keypad Key Depress Interrupt Enable"]
    pub mod KDIE {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt request is generated when KPKD is set."]
            pub const KDIE_0: u16 = 0;
            #[doc = "An interrupt request is generated when KPKD is set."]
            pub const KDIE_1: u16 = 0x01;
        }
    }
    #[doc = "Keypad Release Interrupt Enable"]
    pub mod KRIE {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt request is generated when KPKR is set."]
            pub const KRIE_0: u16 = 0;
            #[doc = "An interrupt request is generated when KPKR is set."]
            pub const KRIE_1: u16 = 0x01;
        }
    }
}
#[doc = "Keypad Data Direction Register"]
pub mod KDDR {
    #[doc = "Keypad Row Data Direction"]
    pub mod KRDD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROWn pin configured as an input."]
            pub const INPUT: u16 = 0;
            #[doc = "ROWn pin configured as an output."]
            pub const OUTPUT: u16 = 0x01;
        }
    }
    #[doc = "Keypad Column Data Direction Register"]
    pub mod KCDD {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "COLn pin is configured as an input."]
            pub const INPUT: u16 = 0;
            #[doc = "COLn pin is configured as an output."]
            pub const OUTPUT: u16 = 0x01;
        }
    }
}
#[doc = "Keypad Data Register"]
pub mod KPDR {
    #[doc = "Keypad Row Data"]
    pub mod KRD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Keypad Column Data"]
    pub mod KCD {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
