#[doc = "AIPSTZ Control Registers"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Master Priviledge Registers"]
    pub MPR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x3c],
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    pub OPACR: crate::RWRegister<u32>,
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    pub OPACR1: crate::RWRegister<u32>,
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    pub OPACR2: crate::RWRegister<u32>,
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    pub OPACR3: crate::RWRegister<u32>,
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    pub OPACR4: crate::RWRegister<u32>,
}
#[doc = "Master Priviledge Registers"]
pub mod MPR {
    #[doc = "Master 5 Priviledge, Buffer, Read, Write Control."]
    pub mod MPROT5 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
            pub const MPL0: u32 = 0;
            #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
            pub const MPL1: u32 = 0x01;
        }
    }
    #[doc = "Master 3 Priviledge, Buffer, Read, Write Control."]
    pub mod MPROT3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
            pub const MPL0: u32 = 0;
            #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
            pub const MPL1: u32 = 0x01;
        }
    }
    #[doc = "Master 2 Priviledge, Buffer, Read, Write Control"]
    pub mod MPROT2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
            pub const MPL0: u32 = 0;
            #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
            pub const MPL1: u32 = 0x01;
        }
    }
    #[doc = "Master 1 Priviledge, Buffer, Read, Write Control"]
    pub mod MPROT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
            pub const MPL0: u32 = 0;
            #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
            pub const MPL1: u32 = 0x01;
        }
    }
    #[doc = "Master 0 Priviledge, Buffer, Read, Write Control"]
    pub mod MPROT0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\] access attribute."]
            pub const MPL0: u32 = 0;
            #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\] access attribute is used directly to determine ips_supervisor_access."]
            pub const MPL1: u32 = 0x01;
        }
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod OPACR {
    #[doc = "Off-platform Peripheral Access Control 7"]
    pub mod OPAC7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 6"]
    pub mod OPAC6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 5"]
    pub mod OPAC5 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 4"]
    pub mod OPAC4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 3"]
    pub mod OPAC3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 2"]
    pub mod OPAC2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 1"]
    pub mod OPAC1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 0"]
    pub mod OPAC0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod OPACR1 {
    #[doc = "Off-platform Peripheral Access Control 15"]
    pub mod OPAC15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 14"]
    pub mod OPAC14 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 13"]
    pub mod OPAC13 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 12"]
    pub mod OPAC12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 11"]
    pub mod OPAC11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 10"]
    pub mod OPAC10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 9"]
    pub mod OPAC9 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 8"]
    pub mod OPAC8 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod OPACR2 {
    #[doc = "Off-platform Peripheral Access Control 23"]
    pub mod OPAC23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 22"]
    pub mod OPAC22 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 21"]
    pub mod OPAC21 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 20"]
    pub mod OPAC20 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 19"]
    pub mod OPAC19 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 18"]
    pub mod OPAC18 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 17"]
    pub mod OPAC17 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 16"]
    pub mod OPAC16 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod OPACR3 {
    #[doc = "Off-platform Peripheral Access Control 31"]
    pub mod OPAC31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 30"]
    pub mod OPAC30 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 29"]
    pub mod OPAC29 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 28"]
    pub mod OPAC28 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 27"]
    pub mod OPAC27 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 26"]
    pub mod OPAC26 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 25"]
    pub mod OPAC25 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 24"]
    pub mod OPAC24 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod OPACR4 {
    #[doc = "Off-platform Peripheral Access Control 33"]
    pub mod OPAC33 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
    #[doc = "Off-platform Peripheral Access Control 32"]
    pub mod OPAC32 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accesses from an untrusted master are allowed."]
            pub const TP0: u32 = 0;
            #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
            pub const TP1: u32 = 0x01;
        }
    }
}
