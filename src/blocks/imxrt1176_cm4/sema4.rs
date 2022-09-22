#[doc = "IPS_Semaphores"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Semaphores Gate n Register"]
    pub GATE: [crate::RWRegister<u8>; 16usize],
    _reserved0: [u8; 0x30],
    #[doc = "Semaphores Processor n IRQ Notification Enable"]
    pub CPINE0: crate::RWRegister<u16>,
    _reserved1: [u8; 0x06],
    #[doc = "Semaphores Processor n IRQ Notification Enable"]
    pub CPINE1: crate::RWRegister<u16>,
    _reserved2: [u8; 0x36],
    #[doc = "Semaphores Processor n IRQ Notification"]
    pub CPNTF0: crate::RORegister<u16>,
    _reserved3: [u8; 0x06],
    #[doc = "Semaphores Processor n IRQ Notification"]
    pub CPNTF1: crate::RORegister<u16>,
    _reserved4: [u8; 0x76],
    #[doc = "Semaphores (Secure) Reset Gate n"]
    pub RSTGT: crate::RWRegister<u16>,
    _reserved5: [u8; 0x02],
    #[doc = "Semaphores (Secure) Reset IRQ Notification"]
    pub RSTNTF: crate::RWRegister<u16>,
}
#[doc = "Semaphores Gate n Register"]
pub mod GATE {
    #[doc = "Gate Finite State Machine."]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const GTFSM_0: u8 = 0;
            #[doc = "The gate has been locked by processor 0."]
            pub const GTFSM_1: u8 = 0x01;
            #[doc = "The gate has been locked by processor 1."]
            pub const GTFSM_2: u8 = 0x02;
            #[doc = "This state encoding is never used and therefore reserved. Attempted writes of 0x03 are treated as \"no operation\" and do not affect the gate state machine."]
            pub const GTFSM_3: u8 = 0x03;
        }
    }
}
#[doc = "Semaphores Processor n IRQ Notification Enable"]
pub mod CPINE0 {
    #[doc = "Interrupt Request Notification Enable 7. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 7."]
    pub mod INE7 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE7_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE7_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 6. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 6."]
    pub mod INE6 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE6_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE6_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 5. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 5."]
    pub mod INE5 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE5_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE5_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 4. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 4."]
    pub mod INE4 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE4_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE4_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 3"]
    pub mod INE3 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE3_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE3_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 2"]
    pub mod INE2 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE2_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE2_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 1"]
    pub mod INE1 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE1_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE1_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 0"]
    pub mod INE0 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE0_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE0_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 15. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 15."]
    pub mod INE15 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE15_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE15_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 14. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 14."]
    pub mod INE14 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE14_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE14_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 13. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 13."]
    pub mod INE13 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE13_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE13_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 12. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 12."]
    pub mod INE12 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE12_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE12_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 11. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 11."]
    pub mod INE11 {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE11_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE11_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 10. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 10."]
    pub mod INE10 {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE10_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE10_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 9. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 9."]
    pub mod INE9 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE9_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE9_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 8. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 8."]
    pub mod INE8 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE8_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE8_1: u16 = 0x01;
        }
    }
}
#[doc = "Semaphores Processor n IRQ Notification Enable"]
pub mod CPINE1 {
    #[doc = "Interrupt Request Notification Enable 7. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 7."]
    pub mod INE7 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE7_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE7_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 6. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 6."]
    pub mod INE6 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE6_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE6_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 5. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 5."]
    pub mod INE5 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE5_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE5_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 4. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 4."]
    pub mod INE4 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE4_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE4_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 3"]
    pub mod INE3 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE3_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE3_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 2"]
    pub mod INE2 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE2_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE2_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 1"]
    pub mod INE1 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE1_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE1_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 0"]
    pub mod INE0 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE0_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE0_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 15. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 15."]
    pub mod INE15 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE15_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE15_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 14. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 14."]
    pub mod INE14 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE14_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE14_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 13. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 13."]
    pub mod INE13 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE13_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE13_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 12. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 12."]
    pub mod INE12 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE12_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE12_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 11. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 11."]
    pub mod INE11 {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE11_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE11_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 10. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 10."]
    pub mod INE10 {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE10_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE10_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 9. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 9."]
    pub mod INE9 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE9_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE9_1: u16 = 0x01;
        }
    }
    #[doc = "Interrupt Request Notification Enable 8. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 8."]
    pub mod INE8 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The generation of the notification interrupt is disabled."]
            pub const INE8_0: u16 = 0;
            #[doc = "The generation of the notification interrupt is enabled."]
            pub const INE8_1: u16 = 0x01;
        }
    }
}
#[doc = "Semaphores Processor n IRQ Notification"]
pub mod CPNTF0 {
    #[doc = "Gate 7 Notification"]
    pub mod GN7 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 6 Notification"]
    pub mod GN6 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 5 Notification"]
    pub mod GN5 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 4 Notification"]
    pub mod GN4 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 3 Notification"]
    pub mod GN3 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 2 Notification"]
    pub mod GN2 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 1 Notification"]
    pub mod GN1 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 0 Notification"]
    pub mod GN0 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 15 Notification"]
    pub mod GN15 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 14 Notification"]
    pub mod GN14 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 13 Notification"]
    pub mod GN13 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 12 Notification"]
    pub mod GN12 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 11 Notification"]
    pub mod GN11 {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 10 Notification"]
    pub mod GN10 {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 9 Notification"]
    pub mod GN9 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 8 Notification"]
    pub mod GN8 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Semaphores Processor n IRQ Notification"]
pub mod CPNTF1 {
    #[doc = "Gate 7 Notification"]
    pub mod GN7 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 6 Notification"]
    pub mod GN6 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 5 Notification"]
    pub mod GN5 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 4 Notification"]
    pub mod GN4 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 3 Notification"]
    pub mod GN3 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 2 Notification"]
    pub mod GN2 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 1 Notification"]
    pub mod GN1 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 0 Notification"]
    pub mod GN0 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 15 Notification"]
    pub mod GN15 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 14 Notification"]
    pub mod GN14 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 13 Notification"]
    pub mod GN13 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 12 Notification"]
    pub mod GN12 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 11 Notification"]
    pub mod GN11 {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 10 Notification"]
    pub mod GN10 {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 9 Notification"]
    pub mod GN9 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate 8 Notification"]
    pub mod GN8 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Semaphores (Secure) Reset Gate n"]
pub mod RSTGT {
    #[doc = "This field contains sub-fields that vary depending on whether it is being read or written"]
    pub mod RSTGSM_RSTGMS_RSTGDP {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Gate Number"]
    pub mod RSTGTN {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Semaphores (Secure) Reset IRQ Notification"]
pub mod RSTNTF {
    #[doc = "This field contains sub-fields that vary depending on whether it is being read or written"]
    pub mod RSTNSM_RSTNMS_RSTNDP {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Notification Number"]
    pub mod RSTNTN {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
