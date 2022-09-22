#[doc = "CCM_OBS"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Clock root section."]
    pub OBSERVE: [observe::RegisterBlock; 6usize],
}
pub mod observe {
    #[doc = "Clock root section."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Observe control"]
        pub OBSERVE_CONTROL: crate::RWRegister<u32>,
        #[doc = "Observe control"]
        pub OBSERVE_CONTROL_SET: crate::RWRegister<u32>,
        #[doc = "Observe control"]
        pub OBSERVE_CONTROL_CLR: crate::RWRegister<u32>,
        #[doc = "Observe control"]
        pub OBSERVE_CONTROL_TOG: crate::RWRegister<u32>,
        _reserved0: [u8; 0x10],
        #[doc = "Observe status"]
        pub OBSERVE_STATUS0: crate::RORegister<u32>,
        _reserved1: [u8; 0x0c],
        #[doc = "Observe access control"]
        pub OBSERVE_AUTHEN: crate::RWRegister<u32>,
        #[doc = "Observe access control"]
        pub OBSERVE_AUTHEN_SET: crate::RWRegister<u32>,
        #[doc = "Observe access control"]
        pub OBSERVE_AUTHEN_CLR: crate::RWRegister<u32>,
        #[doc = "Observe access control"]
        pub OBSERVE_AUTHEN_TOG: crate::RWRegister<u32>,
        #[doc = "Current frequency detected"]
        pub OBSERVE_FREQUENCY_CURRENT: crate::RORegister<u32>,
        #[doc = "Minimum frequency detected"]
        pub OBSERVE_FREQUENCY_MIN: crate::RORegister<u32>,
        #[doc = "Maximum frequency detected"]
        pub OBSERVE_FREQUENCY_MAX: crate::RORegister<u32>,
        _reserved2: [u8; 0x34],
    }
    #[doc = "Observe control"]
    pub mod OBSERVE_CONTROL {
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Select divided signal."]
                pub const RAW_0: u32 = 0;
                #[doc = "Select raw signal."]
                pub const RAW_1: u32 = 0x01;
            }
        }
        #[doc = "Invert"]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Clock phase remain same."]
                pub const INV_0: u32 = 0;
                #[doc = "Invert clock phase before measurement or send to IO."]
                pub const INV_1: u32 = 0x01;
            }
        }
        #[doc = "Reset observe divider"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No reset"]
                pub const RESET_0: u32 = 0;
                #[doc = "Reset observe divider"]
                pub const RESET_1: u32 = 0x01;
            }
        }
        #[doc = "Divider for observe signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "observe slice is on"]
                pub const OFF_0: u32 = 0;
                #[doc = "observe slice is off"]
                pub const OFF_1: u32 = 0x01;
            }
        }
    }
    #[doc = "Observe control"]
    pub mod OBSERVE_CONTROL_SET {
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Invert"]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Reset observe divider"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Divider for observe signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Observe control"]
    pub mod OBSERVE_CONTROL_CLR {
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Invert"]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Reset observe divider"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Divider for observe signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Observe control"]
    pub mod OBSERVE_CONTROL_TOG {
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Invert"]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Reset observe divider"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Divider for observe signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Observe status"]
    pub mod OBSERVE_STATUS0 {
        #[doc = "Select value"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Divided signal is selected"]
                pub const RAW_0: u32 = 0;
                #[doc = "Raw signal is selected"]
                pub const RAW_1: u32 = 0x01;
            }
        }
        #[doc = "Polarity of the observe target"]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Polarity is not inverted"]
                pub const INV_0: u32 = 0;
                #[doc = "Polarity of the observe target is inverted"]
                pub const INV_1: u32 = 0x01;
            }
        }
        #[doc = "Reset state"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Observe divider is not in reset state"]
                pub const RESET_0: u32 = 0;
                #[doc = "Observe divider is in reset state"]
                pub const RESET_1: u32 = 0x01;
            }
        }
        #[doc = "Divide value status. The clock will be divided by DIVIDE + 1."]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Turn off slice"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "observe slice is on"]
                pub const OFF_0: u32 = 0;
                #[doc = "observe slice is off"]
                pub const OFF_1: u32 = 0x01;
            }
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN {
        #[doc = "User access"]
        pub mod TZ_USER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Clock cannot be changed in user mode."]
                pub const TZ_USER_0: u32 = 0;
                #[doc = "Clock can be changed in user mode."]
                pub const TZ_USER_1: u32 = 0x01;
            }
        }
        #[doc = "Non-secure access"]
        pub mod TZ_NS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Cannot be changed in Non-secure mode."]
                pub const TZ_NS_0: u32 = 0;
                #[doc = "Can be changed in Non-secure mode."]
                pub const TZ_NS_1: u32 = 0x01;
            }
        }
        #[doc = "Lock truszone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Trustzone setting is not locked."]
                pub const LOCK_TZ_0: u32 = 0;
                #[doc = "Trustzone setting is locked."]
                pub const LOCK_TZ_1: u32 = 0x01;
            }
        }
        #[doc = "White list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No domain can change."]
                pub const WHITE_LIST_0: u32 = 0;
                #[doc = "Domain 0 can change."]
                pub const WHITE_LIST_1: u32 = 0x01;
                #[doc = "Domain 1 can change."]
                pub const WHITE_LIST_2: u32 = 0x02;
                #[doc = "Domain 0 and domain 1 can change."]
                pub const WHITE_LIST_3: u32 = 0x03;
                #[doc = "Domain 2 can change."]
                pub const WHITE_LIST_4: u32 = 0x04;
                #[doc = "All domain can change."]
                pub const WHITE_LIST_15: u32 = 0x0f;
            }
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "White list is not locked."]
                pub const LOCK_LIST_0: u32 = 0;
                #[doc = "White list is locked."]
                pub const LOCK_LIST_1: u32 = 0x01;
            }
        }
        #[doc = "Low power and access control by domain"]
        pub mod DOMAIN_MODE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Clock does not work in domain mode."]
                pub const DOMAIN_MODE_0: u32 = 0;
                #[doc = "Clock works in domain mode."]
                pub const DOMAIN_MODE_1: u32 = 0x01;
            }
        }
        #[doc = "Lock low power and access mode"]
        pub mod LOCK_MODE {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "MODE is not locked."]
                pub const LOCK_MODE_0: u32 = 0;
                #[doc = "MODE is locked."]
                pub const LOCK_MODE_1: u32 = 0x01;
            }
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN_SET {
        #[doc = "User access"]
        pub mod TZ_USER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Non-secure access"]
        pub mod TZ_NS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock truszone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "White list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Low power and access control by domain"]
        pub mod DOMAIN_MODE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock low power and access mode"]
        pub mod LOCK_MODE {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN_CLR {
        #[doc = "User access"]
        pub mod TZ_USER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Non-secure access"]
        pub mod TZ_NS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock truszone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "White list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Low power and access control by domain"]
        pub mod DOMAIN_MODE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock low power and access mode"]
        pub mod LOCK_MODE {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN_TOG {
        #[doc = "User access"]
        pub mod TZ_USER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Non-secure access"]
        pub mod TZ_NS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock truszone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "White list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Low power and access control by domain"]
        pub mod DOMAIN_MODE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Lock low power and access mode"]
        pub mod LOCK_MODE {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Current frequency detected"]
    pub mod OBSERVE_FREQUENCY_CURRENT {
        #[doc = "Frequency"]
        pub mod FREQUENCY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Minimum frequency detected"]
    pub mod OBSERVE_FREQUENCY_MIN {
        #[doc = "Frequency"]
        pub mod FREQUENCY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Maximum frequency detected"]
    pub mod OBSERVE_FREQUENCY_MAX {
        #[doc = "Frequency"]
        pub mod FREQUENCY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
