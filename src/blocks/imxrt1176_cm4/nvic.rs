#[doc = "Nested Vectored Interrupt Controller"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Interrupt Set Enable Register n"]
    pub NVICISER0: crate::RWRegister<u32>,
    #[doc = "Interrupt Set Enable Register n"]
    pub NVICISER1: crate::RWRegister<u32>,
    #[doc = "Interrupt Set Enable Register n"]
    pub NVICISER2: crate::RWRegister<u32>,
    #[doc = "Interrupt Set Enable Register n"]
    pub NVICISER3: crate::RWRegister<u32>,
    _reserved0: [u8; 0x70],
    #[doc = "Interrupt Clear Enable Register n"]
    pub NVICICER0: crate::RWRegister<u32>,
    #[doc = "Interrupt Clear Enable Register n"]
    pub NVICICER1: crate::RWRegister<u32>,
    #[doc = "Interrupt Clear Enable Register n"]
    pub NVICICER2: crate::RWRegister<u32>,
    #[doc = "Interrupt Clear Enable Register n"]
    pub NVICICER3: crate::RWRegister<u32>,
    _reserved1: [u8; 0x70],
    #[doc = "Interrupt Set Pending Register n"]
    pub NVICISPR0: crate::RWRegister<u32>,
    #[doc = "Interrupt Set Pending Register n"]
    pub NVICISPR1: crate::RWRegister<u32>,
    #[doc = "Interrupt Set Pending Register n"]
    pub NVICISPR2: crate::RWRegister<u32>,
    #[doc = "Interrupt Set Pending Register n"]
    pub NVICISPR3: crate::RWRegister<u32>,
    _reserved2: [u8; 0x70],
    #[doc = "Interrupt Clear Pending Register n"]
    pub NVICICPR0: crate::RWRegister<u32>,
    #[doc = "Interrupt Clear Pending Register n"]
    pub NVICICPR1: crate::RWRegister<u32>,
    #[doc = "Interrupt Clear Pending Register n"]
    pub NVICICPR2: crate::RWRegister<u32>,
    #[doc = "Interrupt Clear Pending Register n"]
    pub NVICICPR3: crate::RWRegister<u32>,
    _reserved3: [u8; 0x70],
    #[doc = "Interrupt Active bit Register n"]
    pub NVICIABR0: crate::RWRegister<u32>,
    #[doc = "Interrupt Active bit Register n"]
    pub NVICIABR1: crate::RWRegister<u32>,
    #[doc = "Interrupt Active bit Register n"]
    pub NVICIABR2: crate::RWRegister<u32>,
    #[doc = "Interrupt Active bit Register n"]
    pub NVICIABR3: crate::RWRegister<u32>,
    _reserved4: [u8; 0xf0],
    #[doc = "Interrupt Priority Register 0"]
    pub NVICIP0: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 1"]
    pub NVICIP1: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 2"]
    pub NVICIP2: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 3"]
    pub NVICIP3: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 4"]
    pub NVICIP4: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 5"]
    pub NVICIP5: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 6"]
    pub NVICIP6: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 7"]
    pub NVICIP7: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 8"]
    pub NVICIP8: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 9"]
    pub NVICIP9: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 10"]
    pub NVICIP10: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 11"]
    pub NVICIP11: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 12"]
    pub NVICIP12: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 13"]
    pub NVICIP13: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 14"]
    pub NVICIP14: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 15"]
    pub NVICIP15: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 16"]
    pub NVICIP16: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 17"]
    pub NVICIP17: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 18"]
    pub NVICIP18: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 19"]
    pub NVICIP19: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 20"]
    pub NVICIP20: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 21"]
    pub NVICIP21: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 22"]
    pub NVICIP22: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 23"]
    pub NVICIP23: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 24"]
    pub NVICIP24: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 25"]
    pub NVICIP25: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 26"]
    pub NVICIP26: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 27"]
    pub NVICIP27: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 28"]
    pub NVICIP28: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 29"]
    pub NVICIP29: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 30"]
    pub NVICIP30: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 31"]
    pub NVICIP31: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 32"]
    pub NVICIP32: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 33"]
    pub NVICIP33: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 34"]
    pub NVICIP34: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 35"]
    pub NVICIP35: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 36"]
    pub NVICIP36: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 37"]
    pub NVICIP37: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 38"]
    pub NVICIP38: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 39"]
    pub NVICIP39: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 40"]
    pub NVICIP40: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 41"]
    pub NVICIP41: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 42"]
    pub NVICIP42: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 43"]
    pub NVICIP43: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 44"]
    pub NVICIP44: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 45"]
    pub NVICIP45: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 46"]
    pub NVICIP46: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 47"]
    pub NVICIP47: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 48"]
    pub NVICIP48: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 49"]
    pub NVICIP49: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 50"]
    pub NVICIP50: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 51"]
    pub NVICIP51: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 52"]
    pub NVICIP52: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 53"]
    pub NVICIP53: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 54"]
    pub NVICIP54: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 55"]
    pub NVICIP55: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 56"]
    pub NVICIP56: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 57"]
    pub NVICIP57: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 58"]
    pub NVICIP58: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 59"]
    pub NVICIP59: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 60"]
    pub NVICIP60: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 61"]
    pub NVICIP61: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 62"]
    pub NVICIP62: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 63"]
    pub NVICIP63: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 64"]
    pub NVICIP64: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 65"]
    pub NVICIP65: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 66"]
    pub NVICIP66: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 67"]
    pub NVICIP67: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 68"]
    pub NVICIP68: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 69"]
    pub NVICIP69: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 70"]
    pub NVICIP70: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 71"]
    pub NVICIP71: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 72"]
    pub NVICIP72: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 73"]
    pub NVICIP73: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 74"]
    pub NVICIP74: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 75"]
    pub NVICIP75: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 76"]
    pub NVICIP76: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 77"]
    pub NVICIP77: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 78"]
    pub NVICIP78: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 79"]
    pub NVICIP79: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 80"]
    pub NVICIP80: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 81"]
    pub NVICIP81: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 82"]
    pub NVICIP82: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 83"]
    pub NVICIP83: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 84"]
    pub NVICIP84: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 85"]
    pub NVICIP85: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 86"]
    pub NVICIP86: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 87"]
    pub NVICIP87: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 88"]
    pub NVICIP88: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 89"]
    pub NVICIP89: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 90"]
    pub NVICIP90: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 91"]
    pub NVICIP91: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 92"]
    pub NVICIP92: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 93"]
    pub NVICIP93: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 94"]
    pub NVICIP94: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 95"]
    pub NVICIP95: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 96"]
    pub NVICIP96: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 97"]
    pub NVICIP97: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 98"]
    pub NVICIP98: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 99"]
    pub NVICIP99: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 100"]
    pub NVICIP100: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 101"]
    pub NVICIP101: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 102"]
    pub NVICIP102: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 103"]
    pub NVICIP103: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 104"]
    pub NVICIP104: crate::RWRegister<u8>,
    #[doc = "Interrupt Priority Register 105"]
    pub NVICIP105: crate::RWRegister<u8>,
    _reserved5: [u8; 0x0a96],
    #[doc = "Software Trigger Interrupt Register"]
    pub NVICSTIR: crate::RWRegister<u32>,
}
#[doc = "Interrupt Set Enable Register n"]
pub mod NVICISER0 {
    #[doc = "Interrupt set enable bits"]
    pub mod SETENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Set Enable Register n"]
pub mod NVICISER1 {
    #[doc = "Interrupt set enable bits"]
    pub mod SETENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Set Enable Register n"]
pub mod NVICISER2 {
    #[doc = "Interrupt set enable bits"]
    pub mod SETENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Set Enable Register n"]
pub mod NVICISER3 {
    #[doc = "Interrupt set enable bits"]
    pub mod SETENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Clear Enable Register n"]
pub mod NVICICER0 {
    #[doc = "Interrupt clear-enable bits"]
    pub mod CLRENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Clear Enable Register n"]
pub mod NVICICER1 {
    #[doc = "Interrupt clear-enable bits"]
    pub mod CLRENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Clear Enable Register n"]
pub mod NVICICER2 {
    #[doc = "Interrupt clear-enable bits"]
    pub mod CLRENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Clear Enable Register n"]
pub mod NVICICER3 {
    #[doc = "Interrupt clear-enable bits"]
    pub mod CLRENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Set Pending Register n"]
pub mod NVICISPR0 {
    #[doc = "Interrupt set-pending bits"]
    pub mod SETPEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Set Pending Register n"]
pub mod NVICISPR1 {
    #[doc = "Interrupt set-pending bits"]
    pub mod SETPEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Set Pending Register n"]
pub mod NVICISPR2 {
    #[doc = "Interrupt set-pending bits"]
    pub mod SETPEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Set Pending Register n"]
pub mod NVICISPR3 {
    #[doc = "Interrupt set-pending bits"]
    pub mod SETPEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Clear Pending Register n"]
pub mod NVICICPR0 {
    #[doc = "Interrupt clear-pending bits"]
    pub mod CLRPEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Clear Pending Register n"]
pub mod NVICICPR1 {
    #[doc = "Interrupt clear-pending bits"]
    pub mod CLRPEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Clear Pending Register n"]
pub mod NVICICPR2 {
    #[doc = "Interrupt clear-pending bits"]
    pub mod CLRPEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Clear Pending Register n"]
pub mod NVICICPR3 {
    #[doc = "Interrupt clear-pending bits"]
    pub mod CLRPEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Active bit Register n"]
pub mod NVICIABR0 {
    #[doc = "Interrupt active flags"]
    pub mod ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Active bit Register n"]
pub mod NVICIABR1 {
    #[doc = "Interrupt active flags"]
    pub mod ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Active bit Register n"]
pub mod NVICIABR2 {
    #[doc = "Interrupt active flags"]
    pub mod ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Active bit Register n"]
pub mod NVICIABR3 {
    #[doc = "Interrupt active flags"]
    pub mod ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 0"]
pub mod NVICIP0 {
    #[doc = "Priority of interrupt 0"]
    pub mod PRI0 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 1"]
pub mod NVICIP1 {
    #[doc = "Priority of interrupt 1"]
    pub mod PRI1 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 2"]
pub mod NVICIP2 {
    #[doc = "Priority of interrupt 2"]
    pub mod PRI2 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 3"]
pub mod NVICIP3 {
    #[doc = "Priority of interrupt 3"]
    pub mod PRI3 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 4"]
pub mod NVICIP4 {
    #[doc = "Priority of interrupt 4"]
    pub mod PRI4 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 5"]
pub mod NVICIP5 {
    #[doc = "Priority of interrupt 5"]
    pub mod PRI5 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 6"]
pub mod NVICIP6 {
    #[doc = "Priority of interrupt 6"]
    pub mod PRI6 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 7"]
pub mod NVICIP7 {
    #[doc = "Priority of interrupt 7"]
    pub mod PRI7 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 8"]
pub mod NVICIP8 {
    #[doc = "Priority of interrupt 8"]
    pub mod PRI8 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 9"]
pub mod NVICIP9 {
    #[doc = "Priority of interrupt 9"]
    pub mod PRI9 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 10"]
pub mod NVICIP10 {
    #[doc = "Priority of interrupt 10"]
    pub mod PRI10 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 11"]
pub mod NVICIP11 {
    #[doc = "Priority of interrupt 11"]
    pub mod PRI11 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 12"]
pub mod NVICIP12 {
    #[doc = "Priority of interrupt 12"]
    pub mod PRI12 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 13"]
pub mod NVICIP13 {
    #[doc = "Priority of interrupt 13"]
    pub mod PRI13 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 14"]
pub mod NVICIP14 {
    #[doc = "Priority of interrupt 14"]
    pub mod PRI14 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 15"]
pub mod NVICIP15 {
    #[doc = "Priority of interrupt 15"]
    pub mod PRI15 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 16"]
pub mod NVICIP16 {
    #[doc = "Priority of interrupt 16"]
    pub mod PRI16 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 17"]
pub mod NVICIP17 {
    #[doc = "Priority of interrupt 17"]
    pub mod PRI17 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 18"]
pub mod NVICIP18 {
    #[doc = "Priority of interrupt 18"]
    pub mod PRI18 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 19"]
pub mod NVICIP19 {
    #[doc = "Priority of interrupt 19"]
    pub mod PRI19 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 20"]
pub mod NVICIP20 {
    #[doc = "Priority of interrupt 20"]
    pub mod PRI20 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 21"]
pub mod NVICIP21 {
    #[doc = "Priority of interrupt 21"]
    pub mod PRI21 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 22"]
pub mod NVICIP22 {
    #[doc = "Priority of interrupt 22"]
    pub mod PRI22 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 23"]
pub mod NVICIP23 {
    #[doc = "Priority of interrupt 23"]
    pub mod PRI23 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 24"]
pub mod NVICIP24 {
    #[doc = "Priority of interrupt 24"]
    pub mod PRI24 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 25"]
pub mod NVICIP25 {
    #[doc = "Priority of interrupt 25"]
    pub mod PRI25 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 26"]
pub mod NVICIP26 {
    #[doc = "Priority of interrupt 26"]
    pub mod PRI26 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 27"]
pub mod NVICIP27 {
    #[doc = "Priority of interrupt 27"]
    pub mod PRI27 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 28"]
pub mod NVICIP28 {
    #[doc = "Priority of interrupt 28"]
    pub mod PRI28 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 29"]
pub mod NVICIP29 {
    #[doc = "Priority of interrupt 29"]
    pub mod PRI29 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 30"]
pub mod NVICIP30 {
    #[doc = "Priority of interrupt 30"]
    pub mod PRI30 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 31"]
pub mod NVICIP31 {
    #[doc = "Priority of interrupt 31"]
    pub mod PRI31 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 32"]
pub mod NVICIP32 {
    #[doc = "Priority of interrupt 32"]
    pub mod PRI32 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 33"]
pub mod NVICIP33 {
    #[doc = "Priority of interrupt 33"]
    pub mod PRI33 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 34"]
pub mod NVICIP34 {
    #[doc = "Priority of interrupt 34"]
    pub mod PRI34 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 35"]
pub mod NVICIP35 {
    #[doc = "Priority of interrupt 35"]
    pub mod PRI35 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 36"]
pub mod NVICIP36 {
    #[doc = "Priority of interrupt 36"]
    pub mod PRI36 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 37"]
pub mod NVICIP37 {
    #[doc = "Priority of interrupt 37"]
    pub mod PRI37 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 38"]
pub mod NVICIP38 {
    #[doc = "Priority of interrupt 38"]
    pub mod PRI38 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 39"]
pub mod NVICIP39 {
    #[doc = "Priority of interrupt 39"]
    pub mod PRI39 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 40"]
pub mod NVICIP40 {
    #[doc = "Priority of interrupt 40"]
    pub mod PRI40 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 41"]
pub mod NVICIP41 {
    #[doc = "Priority of interrupt 41"]
    pub mod PRI41 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 42"]
pub mod NVICIP42 {
    #[doc = "Priority of interrupt 42"]
    pub mod PRI42 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 43"]
pub mod NVICIP43 {
    #[doc = "Priority of interrupt 43"]
    pub mod PRI43 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 44"]
pub mod NVICIP44 {
    #[doc = "Priority of interrupt 44"]
    pub mod PRI44 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 45"]
pub mod NVICIP45 {
    #[doc = "Priority of interrupt 45"]
    pub mod PRI45 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 46"]
pub mod NVICIP46 {
    #[doc = "Priority of interrupt 46"]
    pub mod PRI46 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 47"]
pub mod NVICIP47 {
    #[doc = "Priority of interrupt 47"]
    pub mod PRI47 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 48"]
pub mod NVICIP48 {
    #[doc = "Priority of interrupt 48"]
    pub mod PRI48 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 49"]
pub mod NVICIP49 {
    #[doc = "Priority of interrupt 49"]
    pub mod PRI49 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 50"]
pub mod NVICIP50 {
    #[doc = "Priority of interrupt 50"]
    pub mod PRI50 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 51"]
pub mod NVICIP51 {
    #[doc = "Priority of interrupt 51"]
    pub mod PRI51 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 52"]
pub mod NVICIP52 {
    #[doc = "Priority of interrupt 52"]
    pub mod PRI52 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 53"]
pub mod NVICIP53 {
    #[doc = "Priority of interrupt 53"]
    pub mod PRI53 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 54"]
pub mod NVICIP54 {
    #[doc = "Priority of interrupt 54"]
    pub mod PRI54 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 55"]
pub mod NVICIP55 {
    #[doc = "Priority of interrupt 55"]
    pub mod PRI55 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 56"]
pub mod NVICIP56 {
    #[doc = "Priority of interrupt 56"]
    pub mod PRI56 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 57"]
pub mod NVICIP57 {
    #[doc = "Priority of interrupt 57"]
    pub mod PRI57 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 58"]
pub mod NVICIP58 {
    #[doc = "Priority of interrupt 58"]
    pub mod PRI58 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 59"]
pub mod NVICIP59 {
    #[doc = "Priority of interrupt 59"]
    pub mod PRI59 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 60"]
pub mod NVICIP60 {
    #[doc = "Priority of interrupt 60"]
    pub mod PRI60 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 61"]
pub mod NVICIP61 {
    #[doc = "Priority of interrupt 61"]
    pub mod PRI61 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 62"]
pub mod NVICIP62 {
    #[doc = "Priority of interrupt 62"]
    pub mod PRI62 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 63"]
pub mod NVICIP63 {
    #[doc = "Priority of interrupt 63"]
    pub mod PRI63 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 64"]
pub mod NVICIP64 {
    #[doc = "Priority of interrupt 64"]
    pub mod PRI64 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 65"]
pub mod NVICIP65 {
    #[doc = "Priority of interrupt 65"]
    pub mod PRI65 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 66"]
pub mod NVICIP66 {
    #[doc = "Priority of interrupt 66"]
    pub mod PRI66 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 67"]
pub mod NVICIP67 {
    #[doc = "Priority of interrupt 67"]
    pub mod PRI67 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 68"]
pub mod NVICIP68 {
    #[doc = "Priority of interrupt 68"]
    pub mod PRI68 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 69"]
pub mod NVICIP69 {
    #[doc = "Priority of interrupt 69"]
    pub mod PRI69 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 70"]
pub mod NVICIP70 {
    #[doc = "Priority of interrupt 70"]
    pub mod PRI70 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 71"]
pub mod NVICIP71 {
    #[doc = "Priority of interrupt 71"]
    pub mod PRI71 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 72"]
pub mod NVICIP72 {
    #[doc = "Priority of interrupt 72"]
    pub mod PRI72 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 73"]
pub mod NVICIP73 {
    #[doc = "Priority of interrupt 73"]
    pub mod PRI73 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 74"]
pub mod NVICIP74 {
    #[doc = "Priority of interrupt 74"]
    pub mod PRI74 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 75"]
pub mod NVICIP75 {
    #[doc = "Priority of interrupt 75"]
    pub mod PRI75 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 76"]
pub mod NVICIP76 {
    #[doc = "Priority of interrupt 76"]
    pub mod PRI76 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 77"]
pub mod NVICIP77 {
    #[doc = "Priority of interrupt 77"]
    pub mod PRI77 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 78"]
pub mod NVICIP78 {
    #[doc = "Priority of interrupt 78"]
    pub mod PRI78 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 79"]
pub mod NVICIP79 {
    #[doc = "Priority of interrupt 79"]
    pub mod PRI79 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 80"]
pub mod NVICIP80 {
    #[doc = "Priority of interrupt 80"]
    pub mod PRI80 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 81"]
pub mod NVICIP81 {
    #[doc = "Priority of interrupt 81"]
    pub mod PRI81 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 82"]
pub mod NVICIP82 {
    #[doc = "Priority of interrupt 82"]
    pub mod PRI82 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 83"]
pub mod NVICIP83 {
    #[doc = "Priority of interrupt 83"]
    pub mod PRI83 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 84"]
pub mod NVICIP84 {
    #[doc = "Priority of interrupt 84"]
    pub mod PRI84 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 85"]
pub mod NVICIP85 {
    #[doc = "Priority of interrupt 85"]
    pub mod PRI85 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 86"]
pub mod NVICIP86 {
    #[doc = "Priority of interrupt 86"]
    pub mod PRI86 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 87"]
pub mod NVICIP87 {
    #[doc = "Priority of interrupt 87"]
    pub mod PRI87 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 88"]
pub mod NVICIP88 {
    #[doc = "Priority of interrupt 88"]
    pub mod PRI88 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 89"]
pub mod NVICIP89 {
    #[doc = "Priority of interrupt 89"]
    pub mod PRI89 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 90"]
pub mod NVICIP90 {
    #[doc = "Priority of interrupt 90"]
    pub mod PRI90 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 91"]
pub mod NVICIP91 {
    #[doc = "Priority of interrupt 91"]
    pub mod PRI91 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 92"]
pub mod NVICIP92 {
    #[doc = "Priority of interrupt 92"]
    pub mod PRI92 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 93"]
pub mod NVICIP93 {
    #[doc = "Priority of interrupt 93"]
    pub mod PRI93 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 94"]
pub mod NVICIP94 {
    #[doc = "Priority of interrupt 94"]
    pub mod PRI94 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 95"]
pub mod NVICIP95 {
    #[doc = "Priority of interrupt 95"]
    pub mod PRI95 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 96"]
pub mod NVICIP96 {
    #[doc = "Priority of interrupt 96"]
    pub mod PRI96 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 97"]
pub mod NVICIP97 {
    #[doc = "Priority of interrupt 97"]
    pub mod PRI97 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 98"]
pub mod NVICIP98 {
    #[doc = "Priority of interrupt 98"]
    pub mod PRI98 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 99"]
pub mod NVICIP99 {
    #[doc = "Priority of interrupt 99"]
    pub mod PRI99 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 100"]
pub mod NVICIP100 {
    #[doc = "Priority of interrupt 100"]
    pub mod PRI100 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 101"]
pub mod NVICIP101 {
    #[doc = "Priority of interrupt 101"]
    pub mod PRI101 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 102"]
pub mod NVICIP102 {
    #[doc = "Priority of interrupt 102"]
    pub mod PRI102 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 103"]
pub mod NVICIP103 {
    #[doc = "Priority of interrupt 103"]
    pub mod PRI103 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 104"]
pub mod NVICIP104 {
    #[doc = "Priority of interrupt 104"]
    pub mod PRI104 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Priority Register 105"]
pub mod NVICIP105 {
    #[doc = "Priority of interrupt 105"]
    pub mod PRI105 {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software Trigger Interrupt Register"]
pub mod NVICSTIR {
    #[doc = "Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3."]
    pub mod INTID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
