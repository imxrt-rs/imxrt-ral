#[doc = "MX6RT_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "Chip Silicon Version Register"]
    pub MISC_DIFPROG: crate::RORegister<u32>,
    _reserved1: [u8; 0x1c],
    #[doc = "VDDSOC_AI_CTRL_REGISTER"]
    pub VDDSOC_AI_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "VDDSOC_AI_WDATA_REGISTER"]
    pub VDDSOC_AI_WDATA: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "VDDSOC_AI_RDATA_REGISTER"]
    pub VDDSOC_AI_RDATA: crate::RORegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "VDDSOC2PLL_AI_CTRL_1G_REGISTER"]
    pub VDDSOC2PLL_AI_CTRL_1G: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "VDDSOC2PLL_AI_WDATA_1G_REGISTER"]
    pub VDDSOC2PLL_AI_WDATA_1G: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "VDDSOC2PLL_AI_RDATA_1G_REGISTER"]
    pub VDDSOC2PLL_AI_RDATA_1G: crate::RORegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "VDDSOC_AI_CTRL_AUDIO_REGISTER"]
    pub VDDSOC2PLL_AI_CTRL_AUDIO: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "VDDSOC_AI_WDATA_AUDIO_REGISTER"]
    pub VDDSOC2PLL_AI_WDATA_AUDIO: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "VDDSOC2PLL_AI_RDATA_REGISTER"]
    pub VDDSOC2PLL_AI_RDATA_AUDIO: crate::RORegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "VDDSOC2PLL_AI_CTRL_VIDEO_REGISTER"]
    pub VDDSOC2PLL_AI_CTRL_VIDEO: crate::RWRegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "VDDSOC2PLL_AI_WDATA_VIDEO_REGISTER"]
    pub VDDSOC2PLL_AI_WDATA_VIDEO: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "VDDSOC2PLL_AI_RDATA_VIDEO_REGISTER"]
    pub VDDSOC2PLL_AI_RDATA_VIDEO: crate::RORegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "VDDSOC_AI_CTRL_REGISTER"]
    pub VDDLPSR_AI_CTRL: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "VDDLPSR_AI_WDATA_REGISTER"]
    pub VDDLPSR_AI_WDATA: crate::RWRegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "VDDLPSR_AI_RDATA_REFTOP_REGISTER"]
    pub VDDLPSR_AI_RDATA_REFTOP: crate::RORegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "VDDLPSR_AI_RDATA_TMPSNS_REGISTER"]
    pub VDDLPSR_AI_RDATA_TMPSNS: crate::RORegister<u32>,
    _reserved17: [u8; 0x0c],
    #[doc = "VDDLPSR_AI400M_CTRL_REGISTER"]
    pub VDDLPSR_AI400M_CTRL: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0c],
    #[doc = "VDDLPSR_AI400M_WDATA_REGISTER"]
    pub VDDLPSR_AI400M_WDATA: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0c],
    #[doc = "VDDLPSR_AI400M_RDATA_REGISTER"]
    pub VDDLPSR_AI400M_RDATA: crate::RORegister<u32>,
}
#[doc = "Chip Silicon Version Register"]
pub mod MISC_DIFPROG {
    #[doc = "Chip ID"]
    pub mod CHIPID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC_AI_CTRL_REGISTER"]
pub mod VDDSOC_AI_CTRL {
    #[doc = "VDDSOC_AI_ADDR"]
    pub mod VDDSOC_AI_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC_AIRWB"]
    pub mod VDDSOC_AIRWB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC_AI_WDATA_REGISTER"]
pub mod VDDSOC_AI_WDATA {
    #[doc = "VDDSOC_AI_WDATA"]
    pub mod VDDSOC_AI_WDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC_AI_RDATA_REGISTER"]
pub mod VDDSOC_AI_RDATA {
    #[doc = "VDDSOC_AI_RDATA"]
    pub mod VDDSOC_AI_RDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC2PLL_AI_CTRL_1G_REGISTER"]
pub mod VDDSOC2PLL_AI_CTRL_1G {
    #[doc = "VDDSOC2PLL_AIADDR_1G"]
    pub mod VDDSOC2PLL_AIADDR_1G {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC2PLL_AITOGGLE_1G"]
    pub mod VDDSOC2PLL_AITOGGLE_1G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC2PLL_AITOGGLE_DONE_1G"]
    pub mod VDDSOC2PLL_AITOGGLE_DONE_1G {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC2PLL_AIRWB_1G"]
    pub mod VDDSOC2PLL_AIRWB_1G {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC2PLL_AI_WDATA_1G_REGISTER"]
pub mod VDDSOC2PLL_AI_WDATA_1G {
    #[doc = "VDDSOC2PLL_AI_WDATA_1G"]
    pub mod VDDSOC2PLL_AI_WDATA_1G {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC2PLL_AI_RDATA_1G_REGISTER"]
pub mod VDDSOC2PLL_AI_RDATA_1G {
    #[doc = "VDDSOC2PLL_AI_RDATA_1G"]
    pub mod VDDSOC2PLL_AI_RDATA_1G {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC_AI_CTRL_AUDIO_REGISTER"]
pub mod VDDSOC2PLL_AI_CTRL_AUDIO {
    #[doc = "VDDSOC2PLL_AI_ADDR_AUDIO"]
    pub mod VDDSOC2PLL_AI_ADDR_AUDIO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC2PLL_AITOGGLE_AUDIO"]
    pub mod VDDSOC2PLL_AITOGGLE_AUDIO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC2PLL_AITOGGLE_DONE_AUDIO"]
    pub mod VDDSOC2PLL_AITOGGLE_DONE_AUDIO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC_AIRWB"]
    pub mod VDDSOC2PLL_AIRWB_AUDIO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC_AI_WDATA_AUDIO_REGISTER"]
pub mod VDDSOC2PLL_AI_WDATA_AUDIO {
    #[doc = "VDDSOC2PLL_AI_WDATA_AUDIO"]
    pub mod VDDSOC2PLL_AI_WDATA_AUDIO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC2PLL_AI_RDATA_REGISTER"]
pub mod VDDSOC2PLL_AI_RDATA_AUDIO {
    #[doc = "VDDSOC2PLL_AI_RDATA_AUDIO"]
    pub mod VDDSOC2PLL_AI_RDATA_AUDIO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC2PLL_AI_CTRL_VIDEO_REGISTER"]
pub mod VDDSOC2PLL_AI_CTRL_VIDEO {
    #[doc = "VDDSOC2PLL_AIADDR_VIDEO"]
    pub mod VDDSOC2PLL_AIADDR_VIDEO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC2PLL_AITOGGLE_VIDEO"]
    pub mod VDDSOC2PLL_AITOGGLE_VIDEO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC2PLL_AITOGGLE_DONE_VIDEO"]
    pub mod VDDSOC2PLL_AITOGGLE_DONE_VIDEO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDSOC2PLL_AIRWB_VIDEO"]
    pub mod VDDSOC2PLL_AIRWB_VIDEO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC2PLL_AI_WDATA_VIDEO_REGISTER"]
pub mod VDDSOC2PLL_AI_WDATA_VIDEO {
    #[doc = "VDDSOC2PLL_AI_WDATA_VIDEO"]
    pub mod VDDSOC2PLL_AI_WDATA_VIDEO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC2PLL_AI_RDATA_VIDEO_REGISTER"]
pub mod VDDSOC2PLL_AI_RDATA_VIDEO {
    #[doc = "VDDSOC2PLL_AI_RDATA_VIDEO"]
    pub mod VDDSOC2PLL_AI_RDATA_VIDEO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDSOC_AI_CTRL_REGISTER"]
pub mod VDDLPSR_AI_CTRL {
    #[doc = "VDDLPSR_AI_ADDR"]
    pub mod VDDLPSR_AI_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDLPSR_AIRWB"]
    pub mod VDDLPSR_AIRWB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDLPSR_AI_WDATA_REGISTER"]
pub mod VDDLPSR_AI_WDATA {
    #[doc = "VDD_LPSR_AI_WDATA"]
    pub mod VDDLPSR_AI_WDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDLPSR_AI_RDATA_REFTOP_REGISTER"]
pub mod VDDLPSR_AI_RDATA_REFTOP {
    #[doc = "VDDLPSR_AI_RDATA_REFTOP"]
    pub mod VDDLPSR_AI_RDATA_REFTOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDLPSR_AI_RDATA_TMPSNS_REGISTER"]
pub mod VDDLPSR_AI_RDATA_TMPSNS {
    #[doc = "VDDLPSR_AI_RDATA_TMPSNS"]
    pub mod VDDLPSR_AI_RDATA_TMPSNS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDLPSR_AI400M_CTRL_REGISTER"]
pub mod VDDLPSR_AI400M_CTRL {
    #[doc = "VDDLPSR_AI400M_ADDR"]
    pub mod VDDLPSR_AI400M_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDLPSR_AITOGGLE_400M"]
    pub mod VDDLPSR_AITOGGLE_400M {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDLPSR_AITOGGLE_DONE_400M"]
    pub mod VDDLPSR_AITOGGLE_DONE_400M {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDDLPSR_AI400M_RWB"]
    pub mod VDDLPSR_AI400M_RWB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDLPSR_AI400M_WDATA_REGISTER"]
pub mod VDDLPSR_AI400M_WDATA {
    #[doc = "VDDLPSR_AI400M_WDATA"]
    pub mod VDDLPSR_AI400M_WDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDDLPSR_AI400M_RDATA_REGISTER"]
pub mod VDDLPSR_AI400M_RDATA {
    #[doc = "VDDLPSR_AI400M_RDATA"]
    pub mod VDDLPSR_AI400M_RDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
