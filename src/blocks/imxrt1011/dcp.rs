#[doc = "DCP register reference index"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DCP control register 0"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "DCP control register 0"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "DCP control register 0"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "DCP control register 0"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "DCP status register"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "DCP status register"]
    pub STAT_SET: crate::RWRegister<u32>,
    #[doc = "DCP status register"]
    pub STAT_CLR: crate::RWRegister<u32>,
    #[doc = "DCP status register"]
    pub STAT_TOG: crate::RWRegister<u32>,
    #[doc = "DCP channel control register"]
    pub CHANNELCTRL: crate::RWRegister<u32>,
    #[doc = "DCP channel control register"]
    pub CHANNELCTRL_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel control register"]
    pub CHANNELCTRL_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel control register"]
    pub CHANNELCTRL_TOG: crate::RWRegister<u32>,
    #[doc = "DCP capability 0 register"]
    pub CAPABILITY0: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "DCP capability 1 register"]
    pub CAPABILITY1: crate::RORegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "DCP context buffer pointer"]
    pub CONTEXT: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "DCP key index"]
    pub KEY: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "DCP key data"]
    pub KEYDATA: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "DCP work packet 0 status register"]
    pub PACKET0: crate::RORegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "DCP work packet 1 status register"]
    pub PACKET1: crate::RORegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "DCP work packet 2 status register"]
    pub PACKET2: crate::RORegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "DCP work packet 3 status register"]
    pub PACKET3: crate::RORegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "DCP work packet 4 status register"]
    pub PACKET4: crate::RORegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "DCP work packet 5 status register"]
    pub PACKET5: crate::RORegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "DCP work packet 6 status register"]
    pub PACKET6: crate::RORegister<u32>,
    _reserved11: [u8; 0x1c],
    #[doc = "DCP channel 0 command pointer address register"]
    pub CH0CMDPTR: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "DCP channel 0 semaphore register"]
    pub CH0SEMA: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "DCP channel 0 status register"]
    pub CH0STAT: crate::RWRegister<u32>,
    #[doc = "DCP channel 0 status register"]
    pub CH0STAT_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel 0 status register"]
    pub CH0STAT_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel 0 status register"]
    pub CH0STAT_TOG: crate::RWRegister<u32>,
    #[doc = "DCP channel 0 options register"]
    pub CH0OPTS: crate::RWRegister<u32>,
    #[doc = "DCP channel 0 options register"]
    pub CH0OPTS_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel 0 options register"]
    pub CH0OPTS_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel 0 options register"]
    pub CH0OPTS_TOG: crate::RWRegister<u32>,
    #[doc = "DCP channel 1 command pointer address register"]
    pub CH1CMDPTR: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "DCP channel 1 semaphore register"]
    pub CH1SEMA: crate::RWRegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "DCP channel 1 status register"]
    pub CH1STAT: crate::RWRegister<u32>,
    #[doc = "DCP channel 1 status register"]
    pub CH1STAT_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel 1 status register"]
    pub CH1STAT_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel 1 status register"]
    pub CH1STAT_TOG: crate::RWRegister<u32>,
    #[doc = "DCP channel 1 options register"]
    pub CH1OPTS: crate::RWRegister<u32>,
    #[doc = "DCP channel 1 options register"]
    pub CH1OPTS_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel 1 options register"]
    pub CH1OPTS_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel 1 options register"]
    pub CH1OPTS_TOG: crate::RWRegister<u32>,
    #[doc = "DCP channel 2 command pointer address register"]
    pub CH2CMDPTR: crate::RWRegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "DCP channel 2 semaphore register"]
    pub CH2SEMA: crate::RWRegister<u32>,
    _reserved17: [u8; 0x0c],
    #[doc = "DCP channel 2 status register"]
    pub CH2STAT: crate::RWRegister<u32>,
    #[doc = "DCP channel 2 status register"]
    pub CH2STAT_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel 2 status register"]
    pub CH2STAT_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel 2 status register"]
    pub CH2STAT_TOG: crate::RWRegister<u32>,
    #[doc = "DCP channel 2 options register"]
    pub CH2OPTS: crate::RWRegister<u32>,
    #[doc = "DCP channel 2 options register"]
    pub CH2OPTS_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel 2 options register"]
    pub CH2OPTS_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel 2 options register"]
    pub CH2OPTS_TOG: crate::RWRegister<u32>,
    #[doc = "DCP channel 3 command pointer address register"]
    pub CH3CMDPTR: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0c],
    #[doc = "DCP channel 3 semaphore register"]
    pub CH3SEMA: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0c],
    #[doc = "DCP channel 3 status register"]
    pub CH3STAT: crate::RWRegister<u32>,
    #[doc = "DCP channel 3 status register"]
    pub CH3STAT_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel 3 status register"]
    pub CH3STAT_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel 3 status register"]
    pub CH3STAT_TOG: crate::RWRegister<u32>,
    #[doc = "DCP channel 3 options register"]
    pub CH3OPTS: crate::RWRegister<u32>,
    #[doc = "DCP channel 3 options register"]
    pub CH3OPTS_SET: crate::RWRegister<u32>,
    #[doc = "DCP channel 3 options register"]
    pub CH3OPTS_CLR: crate::RWRegister<u32>,
    #[doc = "DCP channel 3 options register"]
    pub CH3OPTS_TOG: crate::RWRegister<u32>,
    _reserved20: [u8; 0x0200],
    #[doc = "DCP debug select register"]
    pub DBGSELECT: crate::RWRegister<u32>,
    _reserved21: [u8; 0x0c],
    #[doc = "DCP debug data register"]
    pub DBGDATA: crate::RORegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "DCP page table register"]
    pub PAGETABLE: crate::RWRegister<u32>,
    _reserved23: [u8; 0x0c],
    #[doc = "DCP version register"]
    pub VERSION: crate::RORegister<u32>,
}
#[doc = "DCP control register 0"]
pub mod CTRL {
    #[doc = "Per-channel interrupt enable bit"]
    pub mod CHANNEL_INTERRUPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Enable automatic context switching for the channels"]
    pub mod ENABLE_CONTEXT_SWITCHING {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    pub mod ENABLE_CONTEXT_CACHING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    pub mod GATHER_RESIDUAL_WRITES {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    pub mod PRESENT_SHA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Absent"]
            pub const ABSENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    pub mod PRESENT_CRYPTO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Absent"]
            pub const ABSENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP control register 0"]
pub mod CTRL_SET {
    #[doc = "Per-channel interrupt enable bit"]
    pub mod CHANNEL_INTERRUPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Enable automatic context switching for the channels"]
    pub mod ENABLE_CONTEXT_SWITCHING {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    pub mod ENABLE_CONTEXT_CACHING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    pub mod GATHER_RESIDUAL_WRITES {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    pub mod PRESENT_SHA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Absent"]
            pub const ABSENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    pub mod PRESENT_CRYPTO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Absent"]
            pub const ABSENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP control register 0"]
pub mod CTRL_CLR {
    #[doc = "Per-channel interrupt enable bit"]
    pub mod CHANNEL_INTERRUPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Enable automatic context switching for the channels"]
    pub mod ENABLE_CONTEXT_SWITCHING {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    pub mod ENABLE_CONTEXT_CACHING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    pub mod GATHER_RESIDUAL_WRITES {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    pub mod PRESENT_SHA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Absent"]
            pub const ABSENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    pub mod PRESENT_CRYPTO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Absent"]
            pub const ABSENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP control register 0"]
pub mod CTRL_TOG {
    #[doc = "Per-channel interrupt enable bit"]
    pub mod CHANNEL_INTERRUPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Enable automatic context switching for the channels"]
    pub mod ENABLE_CONTEXT_SWITCHING {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The software must set this bit to enable the caching of contexts between the operations"]
    pub mod ENABLE_CONTEXT_CACHING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    pub mod GATHER_RESIDUAL_WRITES {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the SHA1/SHA2 functions are present."]
    pub mod PRESENT_SHA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Absent"]
            pub const ABSENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Indicates whether the crypto (cipher/hash) functions are present."]
    pub mod PRESENT_CRYPTO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Absent"]
            pub const ABSENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for a normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to zero to enable a normal DCP operation"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP status register"]
pub mod STAT {
    #[doc = "Indicates which channels have pending interrupt requests"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    pub mod READY_CHANNELS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Current (active) channel (encoded)"]
    pub mod CUR_CHANNEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None"]
            pub const NONE: u32 = 0;
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x03;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x04;
        }
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    pub mod OTP_KEY_READY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP status register"]
pub mod STAT_SET {
    #[doc = "Indicates which channels have pending interrupt requests"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    pub mod READY_CHANNELS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Current (active) channel (encoded)"]
    pub mod CUR_CHANNEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None"]
            pub const NONE: u32 = 0;
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x03;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x04;
        }
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    pub mod OTP_KEY_READY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP status register"]
pub mod STAT_CLR {
    #[doc = "Indicates which channels have pending interrupt requests"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    pub mod READY_CHANNELS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Current (active) channel (encoded)"]
    pub mod CUR_CHANNEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None"]
            pub const NONE: u32 = 0;
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x03;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x04;
        }
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    pub mod OTP_KEY_READY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP status register"]
pub mod STAT_TOG {
    #[doc = "Indicates which channels have pending interrupt requests"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    pub mod READY_CHANNELS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Current (active) channel (encoded)"]
    pub mod CUR_CHANNEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "None"]
            pub const NONE: u32 = 0;
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x03;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x04;
        }
    }
    #[doc = "When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    pub mod OTP_KEY_READY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel control register"]
pub mod CHANNELCTRL {
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    pub mod ENABLE_CHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    pub mod HIGH_PRIORITY_CHANNEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    pub mod CH0_IRQ_MERGED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel control register"]
pub mod CHANNELCTRL_SET {
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    pub mod ENABLE_CHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    pub mod HIGH_PRIORITY_CHANNEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    pub mod CH0_IRQ_MERGED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel control register"]
pub mod CHANNELCTRL_CLR {
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    pub mod ENABLE_CHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    pub mod HIGH_PRIORITY_CHANNEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    pub mod CH0_IRQ_MERGED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel control register"]
pub mod CHANNELCTRL_TOG {
    #[doc = "Setting a bit in this field enables the DMA channel associated with it"]
    pub mod ENABLE_CHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    pub mod HIGH_PRIORITY_CHANNEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0"]
            pub const CH0: u32 = 0x01;
            #[doc = "CH1"]
            pub const CH1: u32 = 0x02;
            #[doc = "CH2"]
            pub const CH2: u32 = 0x04;
            #[doc = "CH3"]
            pub const CH3: u32 = 0x08;
        }
    }
    #[doc = "Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    pub mod CH0_IRQ_MERGED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP capability 0 register"]
pub mod CAPABILITY0 {
    #[doc = "Encoded value indicating the number of key-storage locations implemented in the design"]
    pub mod NUM_KEYS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoded value indicating the number of channels implemented in the design"]
    pub mod NUM_CHANNELS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write to a 1 to disable the per-device unique key"]
    pub mod DISABLE_UNIQUE_KEY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write to 1 to disable the decryption"]
    pub mod DISABLE_DECRYPT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP capability 1 register"]
pub mod CAPABILITY1 {
    #[doc = "One-hot field indicating which cipher algorithms are available"]
    pub mod CIPHER_ALGORITHMS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AES128"]
            pub const AES128: u32 = 0x01;
        }
    }
    #[doc = "One-hot field indicating which hashing features are implemented in the hardware"]
    pub mod HASH_ALGORITHMS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SHA1"]
            pub const SHA1: u32 = 0x01;
            #[doc = "CRC32"]
            pub const CRC32: u32 = 0x02;
            #[doc = "SHA256"]
            pub const SHA256: u32 = 0x04;
        }
    }
}
#[doc = "DCP context buffer pointer"]
pub mod CONTEXT {
    #[doc = "Context pointer address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP key index"]
pub mod KEY {
    #[doc = "Key subword pointer"]
    pub mod SUBWORD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key index pointer. The valid indices are 0-\\[number_keys\\]."]
    pub mod INDEX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP key data"]
pub mod KEYDATA {
    #[doc = "Word 0 data for the key. This is the least-significant word."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP work packet 0 status register"]
pub mod PACKET0 {
    #[doc = "Next pointer register"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP work packet 1 status register"]
pub mod PACKET1 {
    #[doc = "Reflects whether the channel must issue an interrupt upon the completion of the packet."]
    pub mod INTERRUPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the channel's semaphore must be decremented at the end of the current operation"]
    pub mod DECR_SEMAPHORE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the next command pointer register must be loaded into the channel's current descriptor pointer"]
    pub mod CHAIN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the next packet's address is located following this packet's payload."]
    pub mod CHAIN_CONTIGUOUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the selected hashing function should be enabled for this operation."]
    pub mod ENABLE_MEMCOPY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the selected cipher function must be enabled for this operation."]
    pub mod ENABLE_CIPHER {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the selected hashing function must be enabled for this operation."]
    pub mod ENABLE_HASH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the DCP must perform a blit operation"]
    pub mod ENABLE_BLIT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the cipher block is enabled, this bit indicates whether the operation is encryption or decryption"]
    pub mod CIPHER_ENCRYPT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DECRYPT"]
            pub const DECRYPT: u32 = 0;
            #[doc = "ENCRYPT"]
            pub const ENCRYPT: u32 = 0x01;
        }
    }
    #[doc = "Reflects whether the cipher block must load the initialization vector from the payload for this operation"]
    pub mod CIPHER_INIT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether a hardware-based key must be used"]
    pub mod OTP_KEY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, it indicates the payload contains the key"]
    pub mod PAYLOAD_KEY {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the current hashing block is the initial block in the hashing operation, so the hash registers must be initialized before the operation"]
    pub mod HASH_INIT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the current hashing block is the final block in the hashing operation, so the hash padding must be applied by the hardware"]
    pub mod HASH_TERM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the calculated hash value must be compared to the hash provided in the payload."]
    pub mod CHECK_HASH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the hashing is enabled, this bit controls whether the input or output data is hashed."]
    pub mod HASH_OUTPUT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "INPUT"]
            pub const INPUT: u32 = 0;
            #[doc = "OUTPUT"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "When this bit is set (MEMCOPY and BLIT modes only), the DCP simply fills the destination buffer with the value found in the source address field"]
    pub mod CONSTANT_FILL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is used to test the channel semaphore transition to 0. FOR TEST USE ONLY!"]
    pub mod TEST_SEMA_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the DCP engine swaps the key bytes (big-endian key)."]
    pub mod KEY_BYTESWAP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the DCP engine swaps the key words (big-endian key)."]
    pub mod KEY_WORDSWAP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the DCP engine byteswaps the input data (big-endian data)."]
    pub mod INPUT_BYTESWAP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the DCP engine wordswaps the input data (big-endian data)."]
    pub mod INPUT_WORDSWAP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the DCP engine byteswaps the output data (big-endian data)."]
    pub mod OUTPUT_BYTESWAP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects whether the DCP engine wordswaps the output data (big-endian data)."]
    pub mod OUTPUT_WORDSWAP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Packet Tag"]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP work packet 2 status register"]
pub mod PACKET2 {
    #[doc = "Cipher selection field"]
    pub mod CIPHER_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AES128"]
            pub const AES128: u32 = 0;
        }
    }
    #[doc = "Cipher mode selection field. Reflects the mode of operation for the cipher operations."]
    pub mod CIPHER_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECB"]
            pub const ECB: u32 = 0;
            #[doc = "CBC"]
            pub const CBC: u32 = 0x01;
        }
    }
    #[doc = "Key selection field"]
    pub mod KEY_SELECT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "KEY0"]
            pub const KEY0: u32 = 0;
            #[doc = "KEY1"]
            pub const KEY1: u32 = 0x01;
            #[doc = "KEY2"]
            pub const KEY2: u32 = 0x02;
            #[doc = "KEY3"]
            pub const KEY3: u32 = 0x03;
            #[doc = "UNIQUE_KEY"]
            pub const UNIQUE_KEY: u32 = 0xfe;
            #[doc = "OTP_KEY"]
            pub const OTP_KEY: u32 = 0xff;
        }
    }
    #[doc = "Hash Selection Field"]
    pub mod HASH_SELECT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SHA1"]
            pub const SHA1: u32 = 0;
            #[doc = "CRC32"]
            pub const CRC32: u32 = 0x01;
            #[doc = "SHA256"]
            pub const SHA256: u32 = 0x02;
        }
    }
    #[doc = "Cipher configuration bits. Optional configuration bits are required for the ciphers."]
    pub mod CIPHER_CFG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP work packet 3 status register"]
pub mod PACKET3 {
    #[doc = "Source buffer address pointer"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP work packet 4 status register"]
pub mod PACKET4 {
    #[doc = "Destination buffer address pointer"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP work packet 5 status register"]
pub mod PACKET5 {
    #[doc = "Byte count register. This value is the working value and updates as the operation proceeds."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP work packet 6 status register"]
pub mod PACKET6 {
    #[doc = "This regiser reflects the payload pointer for the current control packet."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 command pointer address register"]
pub mod CH0CMDPTR {
    #[doc = "Pointer to the descriptor structure to be processed for channel 0."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 semaphore register"]
pub mod CH0SEMA {
    #[doc = "The value written to this field is added to the semaphore count in an atomic way such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    pub mod INCREMENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    pub mod VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 status register"]
pub mod CH0STAT {
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error signalled because the next pointer is 0x00000000"]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error signalled because an error is reported reading/writing the payload"]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 status register"]
pub mod CH0STAT_SET {
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error signalled because the next pointer is 0x00000000"]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error signalled because an error is reported reading/writing the payload"]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 status register"]
pub mod CH0STAT_CLR {
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error signalled because the next pointer is 0x00000000"]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error signalled because an error is reported reading/writing the payload"]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 status register"]
pub mod CH0STAT_TOG {
    #[doc = "This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the additional error codes for some of the error conditions"]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error signalled because the next pointer is 0x00000000"]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error signalled because an error is reported reading/writing the payload"]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure"]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 options register"]
pub mod CH0OPTS {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 options register"]
pub mod CH0OPTS_SET {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 options register"]
pub mod CH0OPTS_CLR {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 0 options register"]
pub mod CH0OPTS_TOG {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 command pointer address register"]
pub mod CH1CMDPTR {
    #[doc = "Pointer to the descriptor structure to be processed for channel 1."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 semaphore register"]
pub mod CH1SEMA {
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and the DCP hardware substracts happening on the same clock are protected"]
    pub mod INCREMENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    pub mod VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 status register"]
pub mod CH1STAT {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 status register"]
pub mod CH1STAT_SET {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 status register"]
pub mod CH1STAT_CLR {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 status register"]
pub mod CH1STAT_TOG {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported when reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported when reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 options register"]
pub mod CH1OPTS {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 options register"]
pub mod CH1OPTS_SET {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 options register"]
pub mod CH1OPTS_CLR {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 1 options register"]
pub mod CH1OPTS_TOG {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 command pointer address register"]
pub mod CH2CMDPTR {
    #[doc = "Pointer to the descriptor structure to be processed for channel 2."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 semaphore register"]
pub mod CH2SEMA {
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    pub mod INCREMENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    pub mod VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 status register"]
pub mod CH2STAT {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 status register"]
pub mod CH2STAT_SET {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 status register"]
pub mod CH2STAT_CLR {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 status register"]
pub mod CH2STAT_TOG {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 options register"]
pub mod CH2OPTS {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 options register"]
pub mod CH2OPTS_SET {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 options register"]
pub mod CH2OPTS_CLR {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 2 options register"]
pub mod CH2OPTS_TOG {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 command pointer address register"]
pub mod CH3CMDPTR {
    #[doc = "Pointer to the descriptor structure to be processed for channel 3."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 semaphore register"]
pub mod CH3SEMA {
    #[doc = "The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    pub mod INCREMENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the semaphore counter."]
    pub mod VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 status register"]
pub mod CH3STAT {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 status register"]
pub mod CH3STAT_SET {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 status register"]
pub mod CH3STAT_CLR {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 status register"]
pub mod CH3STAT_TOG {
    #[doc = "This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    pub mod HASH_MISMATCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    pub mod ERROR_SETUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod"]
    pub mod ERROR_PACKET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when reading from the source buffer"]
    pub mod ERROR_SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a bus error occurred when storing to the destination buffer"]
    pub mod ERROR_DST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    pub mod ERROR_PAGEFAULT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates additional error codes for some of the error conditions."]
    pub mod ERROR_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error is signalled because the next pointer is 0x00000000."]
            pub const NEXT_CHAIN_IS_0: u32 = 0x01;
            #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
            pub const NO_CHAIN: u32 = 0x02;
            #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
            pub const CONTEXT_ERROR: u32 = 0x03;
            #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
            pub const PAYLOAD_ERROR: u32 = 0x04;
            #[doc = "Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash)."]
            pub const INVALID_MODE: u32 = 0x05;
        }
    }
    #[doc = "Indicates the tag from the last completed packet in the command structure."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 options register"]
pub mod CH3OPTS {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 options register"]
pub mod CH3OPTS_SET {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 options register"]
pub mod CH3OPTS_CLR {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP channel 3 options register"]
pub mod CH3OPTS_TOG {
    #[doc = "This field indicates the recovery time for the channel"]
    pub mod RECOVERY_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP debug select register"]
pub mod DBGSELECT {
    #[doc = "Selects a value to read via the debug data register."]
    pub mod INDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CONTROL"]
            pub const CONTROL: u32 = 0x01;
            #[doc = "OTPKEY0"]
            pub const OTPKEY0: u32 = 0x10;
            #[doc = "OTPKEY1"]
            pub const OTPKEY1: u32 = 0x11;
            #[doc = "OTPKEY2"]
            pub const OTPKEY2: u32 = 0x12;
            #[doc = "OTPKEY3"]
            pub const OTPKEY3: u32 = 0x13;
        }
    }
}
#[doc = "DCP debug data register"]
pub mod DBGDATA {
    #[doc = "Debug data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP page table register"]
pub mod PAGETABLE {
    #[doc = "Page table enable control"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Page table flush control. To flush the TLB, write this bit to 1 and then back to 0."]
    pub mod FLUSH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Page table base address"]
    pub mod BASE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCP version register"]
pub mod VERSION {
    #[doc = "Fixed read-only value reflecting the stepping of the version of the design implementation."]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fixed read-only value reflecting the MINOR version of the design implementation."]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fixed read-only value reflecting the MAJOR version of the design implementation."]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
