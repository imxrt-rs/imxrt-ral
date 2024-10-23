#[doc = "Pseudo MAC port"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Port pseudo MAC status register"]
    pub PPMSR: crate::RORegister<u32>,
    _reserved0: [u8; 0x7c],
    #[doc = "Port pseudo MAC receive octets counter"]
    pub PPMROCR: [crate::RORegister<u32>; 2usize],
    #[doc = "Port pseudo MAC receive unicast frame counter register"]
    pub PPMRUFCR: [crate::RORegister<u32>; 2usize],
    #[doc = "Port pseudo MAC receive multicast frame counter register"]
    pub PPMRMFCR: [crate::RORegister<u32>; 2usize],
    #[doc = "Port pseudo MAC receive broadcast frame counter register"]
    pub PPMRBFCR: [crate::RORegister<u32>; 2usize],
    _reserved1: [u8; 0x20],
    #[doc = "Port pseudo MAC transmit octets counter"]
    pub PPMTOCR: [crate::RORegister<u32>; 2usize],
    #[doc = "Port pseudo MAC transmit unicast frame counter register"]
    pub PPMTUFCR: [crate::RORegister<u32>; 2usize],
    #[doc = "Port pseudo MAC transmit multicast frame counter register"]
    pub PPMTMFCR: [crate::RORegister<u32>; 2usize],
    #[doc = "Port pseudo MAC transmit broadcast frame counter register"]
    pub PPMTBFCR: [crate::RORegister<u32>; 2usize],
}
#[doc = "Port pseudo MAC status register"]
pub mod PPMSR {
    #[doc = "Local link end's state 0 - Link is down 1 - Link is up The operational state is always \"Link is up\" for the pseudo link"]
    pub mod LSTATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote link end's state 0 - Link is down 1 - Link is up The operational state is always \"Link is up\" for the pseudo link"]
    pub mod RSTATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pseudo MAC receive octets counter"]
pub mod PPMROCR {
    #[doc = "Incremented for each octet received (on the link) (that is, Ethernet header, payload, pad and FCS)."]
    pub mod ROCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pseudo MAC receive unicast frame counter register"]
pub mod PPMRUFCR {
    #[doc = "Incremented for each valid frame received (on the link) in which bit 0 of the destination address was 0"]
    pub mod RUCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pseudo MAC receive multicast frame counter register"]
pub mod PPMRMFCR {
    #[doc = "Incremented for each frame received (on the link) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1)"]
    pub mod RMCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pseudo MAC receive broadcast frame counter register"]
pub mod PPMRBFCR {
    #[doc = "Incremented for each frame received (on the link) in which all bits of the destination address were 1"]
    pub mod RBCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pseudo MAC transmit octets counter"]
pub mod PPMTOCR {
    #[doc = "Incremented for each octet transmitted (on the link) (that is, Ethernet header, payload, pad and FCS)"]
    pub mod TOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pseudo MAC transmit unicast frame counter register"]
pub mod PPMTUFCR {
    #[doc = "Incremented for each frame transmitted (on the link) in which bit 0 of the destination address was 0"]
    pub mod TUCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pseudo MAC transmit multicast frame counter register"]
pub mod PPMTMFCR {
    #[doc = "Incremented for each frame transmitted (on the link) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1)"]
    pub mod TMCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pseudo MAC transmit broadcast frame counter register"]
pub mod PPMTBFCR {
    #[doc = "Incremented for each frame transmitted (on the link) in which all bits of the destination address were 1"]
    pub mod TBCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
