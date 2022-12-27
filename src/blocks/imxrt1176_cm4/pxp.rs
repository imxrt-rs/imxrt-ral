#[doc = "PXP v2.0 Register Reference Index"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register 0"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Control Register 0"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "Control Register 0"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "Control Register 0"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STAT_SET: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STAT_CLR: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STAT_TOG: crate::RWRegister<u32>,
    #[doc = "Output Buffer Control Register"]
    pub OUT_CTRL: crate::RWRegister<u32>,
    #[doc = "Output Buffer Control Register"]
    pub OUT_CTRL_SET: crate::RWRegister<u32>,
    #[doc = "Output Buffer Control Register"]
    pub OUT_CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "Output Buffer Control Register"]
    pub OUT_CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "Output Frame Buffer Pointer"]
    pub OUT_BUF: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "Output Frame Buffer Pointer #2"]
    pub OUT_BUF2: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "Output Buffer Pitch"]
    pub OUT_PITCH: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "Output Surface Lower Right Coordinate"]
    pub OUT_LRC: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "Processed Surface Upper Left Coordinate"]
    pub OUT_PS_ULC: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Processed Surface Lower Right Coordinate"]
    pub OUT_PS_LRC: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Alpha Surface Upper Left Coordinate"]
    pub OUT_AS_ULC: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "Alpha Surface Lower Right Coordinate"]
    pub OUT_AS_LRC: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "Processed Surface (PS) Control Register"]
    pub PS_CTRL: crate::RWRegister<u32>,
    #[doc = "Processed Surface (PS) Control Register"]
    pub PS_CTRL_SET: crate::RWRegister<u32>,
    #[doc = "Processed Surface (PS) Control Register"]
    pub PS_CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "Processed Surface (PS) Control Register"]
    pub PS_CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "PS Input Buffer Address"]
    pub PS_BUF: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "PS U/Cb or 2 Plane UV Input Buffer Address"]
    pub PS_UBUF: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "PS V/Cr Input Buffer Address"]
    pub PS_VBUF: crate::RWRegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "Processed Surface Pitch"]
    pub PS_PITCH: crate::RWRegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "PS Background Color"]
    pub PS_BACKGROUND: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "PS Scale Factor Register"]
    pub PS_SCALE: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "PS Scale Offset Register"]
    pub PS_OFFSET: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "PS Color Key Low"]
    pub PS_CLRKEYLOW: crate::RWRegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "PS Color Key High"]
    pub PS_CLRKEYHIGH: crate::RWRegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "Alpha Surface Control"]
    pub AS_CTRL: crate::RWRegister<u32>,
    _reserved17: [u8; 0x0c],
    #[doc = "Alpha Surface Buffer Pointer"]
    pub AS_BUF: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0c],
    #[doc = "Alpha Surface Pitch"]
    pub AS_PITCH: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0c],
    #[doc = "Overlay Color Key Low"]
    pub AS_CLRKEYLOW: crate::RWRegister<u32>,
    _reserved20: [u8; 0x0c],
    #[doc = "Overlay Color Key High"]
    pub AS_CLRKEYHIGH: crate::RWRegister<u32>,
    _reserved21: [u8; 0x0c],
    #[doc = "Color Space Conversion Coefficient Register 0"]
    pub CSC1_COEF0: crate::RWRegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "Color Space Conversion Coefficient Register 1"]
    pub CSC1_COEF1: crate::RWRegister<u32>,
    _reserved23: [u8; 0x0c],
    #[doc = "Color Space Conversion Coefficient Register 2"]
    pub CSC1_COEF2: crate::RWRegister<u32>,
    _reserved24: [u8; 0x015c],
    #[doc = "PXP Power Control Register"]
    pub POWER: crate::RWRegister<u32>,
    _reserved25: [u8; 0xdc],
    #[doc = "Next Frame Pointer"]
    pub NEXT: crate::RWRegister<u32>,
    _reserved26: [u8; 0x3c],
    #[doc = "PXP Alpha Engine A Control Register."]
    pub PORTER_DUFF_CTRL: crate::RWRegister<u32>,
}
#[doc = "Control Register 0"]
pub mod CTRL {
    #[doc = "Enables PXP operation with specified parameters"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "PXP is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    pub mod IRQ_ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP interrupt is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "PXP interrupt is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Next command interrupt enable"]
    pub mod NEXT_IRQ_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Enable handshake with LCD controller"]
    pub mod ENABLE_LCD_HANDSHAKE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    pub mod ROTATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROT_0"]
            pub const ROT_0: u32 = 0;
            #[doc = "ROT_90"]
            pub const ROT_90: u32 = 0x01;
            #[doc = "ROT_180"]
            pub const ROT_180: u32 = 0x02;
            #[doc = "ROT_270"]
            pub const ROT_270: u32 = 0x03;
        }
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    pub mod HFLIP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Horizontal Flip is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Horizontal Flip is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    pub mod VFLIP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vertical Flip is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Vertical Flip is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    pub mod ROT_POS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select the block size to process."]
    pub mod BLOCK_SIZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Process 8x8 pixel blocks."]
            pub const _8X8: u32 = 0;
            #[doc = "Process 16x16 pixel blocks."]
            pub const _16X16: u32 = 0x01;
        }
    }
    #[doc = "Enable the PXP to run continuously"]
    pub mod EN_REPEAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP will complete the process and enter the idle state ready to accept the next frame to be processed"]
            pub const COMPLETE: u32 = 0;
            #[doc = "PXP will repeat based on the current configuration register settings"]
            pub const REPEAT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "All clocks to PXP is gated-off"]
            pub const GATED: u32 = 0x01;
        }
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal PXP operation is enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Clocking with PXP is disabled and held in its reset (lowest power) state. This is the default value."]
            pub const DISABLED: u32 = 0x01;
        }
    }
}
#[doc = "Control Register 0"]
pub mod CTRL_SET {
    #[doc = "Enables PXP operation with specified parameters"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "PXP is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    pub mod IRQ_ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP interrupt is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "PXP interrupt is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Next command interrupt enable"]
    pub mod NEXT_IRQ_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Enable handshake with LCD controller"]
    pub mod ENABLE_LCD_HANDSHAKE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    pub mod ROTATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROT_0"]
            pub const ROT_0: u32 = 0;
            #[doc = "ROT_90"]
            pub const ROT_90: u32 = 0x01;
            #[doc = "ROT_180"]
            pub const ROT_180: u32 = 0x02;
            #[doc = "ROT_270"]
            pub const ROT_270: u32 = 0x03;
        }
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    pub mod HFLIP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Horizontal Flip is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Horizontal Flip is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    pub mod VFLIP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vertical Flip is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Vertical Flip is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    pub mod ROT_POS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select the block size to process."]
    pub mod BLOCK_SIZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Process 8x8 pixel blocks."]
            pub const _8X8: u32 = 0;
            #[doc = "Process 16x16 pixel blocks."]
            pub const _16X16: u32 = 0x01;
        }
    }
    #[doc = "Enable the PXP to run continuously"]
    pub mod EN_REPEAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP will complete the process and enter the idle state ready to accept the next frame to be processed"]
            pub const COMPLETE: u32 = 0;
            #[doc = "PXP will repeat based on the current configuration register settings"]
            pub const REPEAT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "All clocks to PXP is gated-off"]
            pub const GATED: u32 = 0x01;
        }
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal PXP operation is enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Clocking with PXP is disabled and held in its reset (lowest power) state. This is the default value."]
            pub const DISABLED: u32 = 0x01;
        }
    }
}
#[doc = "Control Register 0"]
pub mod CTRL_CLR {
    #[doc = "Enables PXP operation with specified parameters"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "PXP is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    pub mod IRQ_ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP interrupt is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "PXP interrupt is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Next command interrupt enable"]
    pub mod NEXT_IRQ_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Enable handshake with LCD controller"]
    pub mod ENABLE_LCD_HANDSHAKE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    pub mod ROTATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROT_0"]
            pub const ROT_0: u32 = 0;
            #[doc = "ROT_90"]
            pub const ROT_90: u32 = 0x01;
            #[doc = "ROT_180"]
            pub const ROT_180: u32 = 0x02;
            #[doc = "ROT_270"]
            pub const ROT_270: u32 = 0x03;
        }
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    pub mod HFLIP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Horizontal Flip is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Horizontal Flip is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    pub mod VFLIP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vertical Flip is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Vertical Flip is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    pub mod ROT_POS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select the block size to process."]
    pub mod BLOCK_SIZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Process 8x8 pixel blocks."]
            pub const _8X8: u32 = 0;
            #[doc = "Process 16x16 pixel blocks."]
            pub const _16X16: u32 = 0x01;
        }
    }
    #[doc = "Enable the PXP to run continuously"]
    pub mod EN_REPEAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP will complete the process and enter the idle state ready to accept the next frame to be processed"]
            pub const COMPLETE: u32 = 0;
            #[doc = "PXP will repeat based on the current configuration register settings"]
            pub const REPEAT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "All clocks to PXP is gated-off"]
            pub const GATED: u32 = 0x01;
        }
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal PXP operation is enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Clocking with PXP is disabled and held in its reset (lowest power) state. This is the default value."]
            pub const DISABLED: u32 = 0x01;
        }
    }
}
#[doc = "Control Register 0"]
pub mod CTRL_TOG {
    #[doc = "Enables PXP operation with specified parameters"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "PXP is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    pub mod IRQ_ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP interrupt is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "PXP interrupt is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Next command interrupt enable"]
    pub mod NEXT_IRQ_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Enable handshake with LCD controller"]
    pub mod ENABLE_LCD_HANDSHAKE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    pub mod ROTATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROT_0"]
            pub const ROT_0: u32 = 0;
            #[doc = "ROT_90"]
            pub const ROT_90: u32 = 0x01;
            #[doc = "ROT_180"]
            pub const ROT_180: u32 = 0x02;
            #[doc = "ROT_270"]
            pub const ROT_270: u32 = 0x03;
        }
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    pub mod HFLIP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Horizontal Flip is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Horizontal Flip is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    pub mod VFLIP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vertical Flip is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Vertical Flip is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    pub mod ROT_POS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select the block size to process."]
    pub mod BLOCK_SIZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Process 8x8 pixel blocks."]
            pub const _8X8: u32 = 0;
            #[doc = "Process 16x16 pixel blocks."]
            pub const _16X16: u32 = 0x01;
        }
    }
    #[doc = "Enable the PXP to run continuously"]
    pub mod EN_REPEAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PXP will complete the process and enter the idle state ready to accept the next frame to be processed"]
            pub const COMPLETE: u32 = 0;
            #[doc = "PXP will repeat based on the current configuration register settings"]
            pub const REPEAT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "All clocks to PXP is gated-off"]
            pub const GATED: u32 = 0x01;
        }
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal PXP operation is enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Clocking with PXP is disabled and held in its reset (lowest power) state. This is the default value."]
            pub const DISABLED: u32 = 0x01;
        }
    }
}
#[doc = "Status Register"]
pub mod STAT {
    #[doc = "Indicates current PXP interrupt status"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const IRQ_0: u32 = 0;
            #[doc = "Interrupt generated"]
            pub const IRQ_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    pub mod AXI_WRITE_ERROR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI write is normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "AXI write error has occurred"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    pub mod AXI_READ_ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI read is normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "AXI read error has occurred"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    pub mod NEXT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    pub mod AXI_ERROR_ID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    pub mod LUT_DMA_LOAD_DONE_IRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT DMA LOAD transfer is active"]
            pub const ACTIVE: u32 = 0;
            #[doc = "LUT DMA LOAD transfer is complete"]
            pub const COMPLETE: u32 = 0x01;
        }
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    pub mod BLOCKY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    pub mod BLOCKX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STAT_SET {
    #[doc = "Indicates current PXP interrupt status"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const IRQ_0: u32 = 0;
            #[doc = "Interrupt generated"]
            pub const IRQ_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    pub mod AXI_WRITE_ERROR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI write is normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "AXI write error has occurred"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    pub mod AXI_READ_ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI read is normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "AXI read error has occurred"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    pub mod NEXT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    pub mod AXI_ERROR_ID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    pub mod LUT_DMA_LOAD_DONE_IRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT DMA LOAD transfer is active"]
            pub const ACTIVE: u32 = 0;
            #[doc = "LUT DMA LOAD transfer is complete"]
            pub const COMPLETE: u32 = 0x01;
        }
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    pub mod BLOCKY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    pub mod BLOCKX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STAT_CLR {
    #[doc = "Indicates current PXP interrupt status"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const IRQ_0: u32 = 0;
            #[doc = "Interrupt generated"]
            pub const IRQ_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    pub mod AXI_WRITE_ERROR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI write is normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "AXI write error has occurred"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    pub mod AXI_READ_ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI read is normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "AXI read error has occurred"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    pub mod NEXT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    pub mod AXI_ERROR_ID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    pub mod LUT_DMA_LOAD_DONE_IRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT DMA LOAD transfer is active"]
            pub const ACTIVE: u32 = 0;
            #[doc = "LUT DMA LOAD transfer is complete"]
            pub const COMPLETE: u32 = 0x01;
        }
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    pub mod BLOCKY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    pub mod BLOCKX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STAT_TOG {
    #[doc = "Indicates current PXP interrupt status"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const IRQ_0: u32 = 0;
            #[doc = "Interrupt generated"]
            pub const IRQ_1: u32 = 0x01;
        }
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    pub mod AXI_WRITE_ERROR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI write is normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "AXI write error has occurred"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    pub mod AXI_READ_ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI read is normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "AXI read error has occurred"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    pub mod NEXT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    pub mod AXI_ERROR_ID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    pub mod LUT_DMA_LOAD_DONE_IRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT DMA LOAD transfer is active"]
            pub const ACTIVE: u32 = 0;
            #[doc = "LUT DMA LOAD transfer is complete"]
            pub const COMPLETE: u32 = 0x01;
        }
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    pub mod BLOCKY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    pub mod BLOCKX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Buffer Control Register"]
pub mod OUT_CTRL {
    #[doc = "Output framebuffer format"]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels"]
            pub const ARGB8888: u32 = 0;
            #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
            pub const RGB888: u32 = 0x04;
            #[doc = "24-bit pixels (packed 24-bit format)"]
            pub const RGB888P: u32 = 0x05;
            #[doc = "16-bit pixels"]
            pub const ARGB1555: u32 = 0x08;
            #[doc = "16-bit pixels"]
            pub const ARGB4444: u32 = 0x09;
            #[doc = "16-bit pixels"]
            pub const RGB555: u32 = 0x0c;
            #[doc = "16-bit pixels"]
            pub const RGB444: u32 = 0x0d;
            #[doc = "16-bit pixels"]
            pub const RGB565: u32 = 0x0e;
            #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
            pub const YUV1P444: u32 = 0x10;
            #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
            pub const UYVY1P422: u32 = 0x12;
            #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
            pub const VYUY1P422: u32 = 0x13;
            #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
            pub const Y8: u32 = 0x14;
            #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
            pub const Y4: u32 = 0x15;
            #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
            pub const YUV2P422: u32 = 0x18;
            #[doc = "16-bit pixels (2-plane UV)"]
            pub const YUV2P420: u32 = 0x19;
            #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
            pub const YVU2P422: u32 = 0x1a;
            #[doc = "16-bit pixels (2-plane VU)"]
            pub const YVU2P420: u32 = 0x1b;
        }
    }
    #[doc = "Determines how the PXP writes it's output data"]
    pub mod INTERLACED_OUTPUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All data written in progressive format to the OUTBUF Pointer."]
            pub const PROGRESSIVE: u32 = 0;
            #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
            pub const FIELD0: u32 = 0x01;
            #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
            pub const FIELD1: u32 = 0x02;
            #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
            pub const INTERLACED: u32 = 0x03;
        }
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    pub mod ALPHA_OUTPUT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Retain"]
            pub const RETAIN: u32 = 0;
            #[doc = "Overwritten"]
            pub const OVERWRITTEN: u32 = 0x01;
        }
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    pub mod ALPHA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Buffer Control Register"]
pub mod OUT_CTRL_SET {
    #[doc = "Output framebuffer format"]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels"]
            pub const ARGB8888: u32 = 0;
            #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
            pub const RGB888: u32 = 0x04;
            #[doc = "24-bit pixels (packed 24-bit format)"]
            pub const RGB888P: u32 = 0x05;
            #[doc = "16-bit pixels"]
            pub const ARGB1555: u32 = 0x08;
            #[doc = "16-bit pixels"]
            pub const ARGB4444: u32 = 0x09;
            #[doc = "16-bit pixels"]
            pub const RGB555: u32 = 0x0c;
            #[doc = "16-bit pixels"]
            pub const RGB444: u32 = 0x0d;
            #[doc = "16-bit pixels"]
            pub const RGB565: u32 = 0x0e;
            #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
            pub const YUV1P444: u32 = 0x10;
            #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
            pub const UYVY1P422: u32 = 0x12;
            #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
            pub const VYUY1P422: u32 = 0x13;
            #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
            pub const Y8: u32 = 0x14;
            #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
            pub const Y4: u32 = 0x15;
            #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
            pub const YUV2P422: u32 = 0x18;
            #[doc = "16-bit pixels (2-plane UV)"]
            pub const YUV2P420: u32 = 0x19;
            #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
            pub const YVU2P422: u32 = 0x1a;
            #[doc = "16-bit pixels (2-plane VU)"]
            pub const YVU2P420: u32 = 0x1b;
        }
    }
    #[doc = "Determines how the PXP writes it's output data"]
    pub mod INTERLACED_OUTPUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All data written in progressive format to the OUTBUF Pointer."]
            pub const PROGRESSIVE: u32 = 0;
            #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
            pub const FIELD0: u32 = 0x01;
            #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
            pub const FIELD1: u32 = 0x02;
            #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
            pub const INTERLACED: u32 = 0x03;
        }
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    pub mod ALPHA_OUTPUT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Retain"]
            pub const RETAIN: u32 = 0;
            #[doc = "Overwritten"]
            pub const OVERWRITTEN: u32 = 0x01;
        }
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    pub mod ALPHA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Buffer Control Register"]
pub mod OUT_CTRL_CLR {
    #[doc = "Output framebuffer format"]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels"]
            pub const ARGB8888: u32 = 0;
            #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
            pub const RGB888: u32 = 0x04;
            #[doc = "24-bit pixels (packed 24-bit format)"]
            pub const RGB888P: u32 = 0x05;
            #[doc = "16-bit pixels"]
            pub const ARGB1555: u32 = 0x08;
            #[doc = "16-bit pixels"]
            pub const ARGB4444: u32 = 0x09;
            #[doc = "16-bit pixels"]
            pub const RGB555: u32 = 0x0c;
            #[doc = "16-bit pixels"]
            pub const RGB444: u32 = 0x0d;
            #[doc = "16-bit pixels"]
            pub const RGB565: u32 = 0x0e;
            #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
            pub const YUV1P444: u32 = 0x10;
            #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
            pub const UYVY1P422: u32 = 0x12;
            #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
            pub const VYUY1P422: u32 = 0x13;
            #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
            pub const Y8: u32 = 0x14;
            #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
            pub const Y4: u32 = 0x15;
            #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
            pub const YUV2P422: u32 = 0x18;
            #[doc = "16-bit pixels (2-plane UV)"]
            pub const YUV2P420: u32 = 0x19;
            #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
            pub const YVU2P422: u32 = 0x1a;
            #[doc = "16-bit pixels (2-plane VU)"]
            pub const YVU2P420: u32 = 0x1b;
        }
    }
    #[doc = "Determines how the PXP writes it's output data"]
    pub mod INTERLACED_OUTPUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All data written in progressive format to the OUTBUF Pointer."]
            pub const PROGRESSIVE: u32 = 0;
            #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
            pub const FIELD0: u32 = 0x01;
            #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
            pub const FIELD1: u32 = 0x02;
            #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
            pub const INTERLACED: u32 = 0x03;
        }
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    pub mod ALPHA_OUTPUT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Retain"]
            pub const RETAIN: u32 = 0;
            #[doc = "Overwritten"]
            pub const OVERWRITTEN: u32 = 0x01;
        }
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    pub mod ALPHA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Buffer Control Register"]
pub mod OUT_CTRL_TOG {
    #[doc = "Output framebuffer format"]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels"]
            pub const ARGB8888: u32 = 0;
            #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
            pub const RGB888: u32 = 0x04;
            #[doc = "24-bit pixels (packed 24-bit format)"]
            pub const RGB888P: u32 = 0x05;
            #[doc = "16-bit pixels"]
            pub const ARGB1555: u32 = 0x08;
            #[doc = "16-bit pixels"]
            pub const ARGB4444: u32 = 0x09;
            #[doc = "16-bit pixels"]
            pub const RGB555: u32 = 0x0c;
            #[doc = "16-bit pixels"]
            pub const RGB444: u32 = 0x0d;
            #[doc = "16-bit pixels"]
            pub const RGB565: u32 = 0x0e;
            #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
            pub const YUV1P444: u32 = 0x10;
            #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
            pub const UYVY1P422: u32 = 0x12;
            #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
            pub const VYUY1P422: u32 = 0x13;
            #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
            pub const Y8: u32 = 0x14;
            #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
            pub const Y4: u32 = 0x15;
            #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
            pub const YUV2P422: u32 = 0x18;
            #[doc = "16-bit pixels (2-plane UV)"]
            pub const YUV2P420: u32 = 0x19;
            #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
            pub const YVU2P422: u32 = 0x1a;
            #[doc = "16-bit pixels (2-plane VU)"]
            pub const YVU2P420: u32 = 0x1b;
        }
    }
    #[doc = "Determines how the PXP writes it's output data"]
    pub mod INTERLACED_OUTPUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All data written in progressive format to the OUTBUF Pointer."]
            pub const PROGRESSIVE: u32 = 0;
            #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
            pub const FIELD0: u32 = 0x01;
            #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
            pub const FIELD1: u32 = 0x02;
            #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
            pub const INTERLACED: u32 = 0x03;
        }
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    pub mod ALPHA_OUTPUT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Retain"]
            pub const RETAIN: u32 = 0;
            #[doc = "Overwritten"]
            pub const OVERWRITTEN: u32 = 0x01;
        }
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    pub mod ALPHA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Frame Buffer Pointer"]
pub mod OUT_BUF {
    #[doc = "Current address pointer for the output frame buffer"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Frame Buffer Pointer #2"]
pub mod OUT_BUF2 {
    #[doc = "Current address pointer for the output frame buffer"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Buffer Pitch"]
pub mod OUT_PITCH {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Surface Lower Right Coordinate"]
pub mod OUT_LRC {
    #[doc = "Indicates the number of vertical PIXELS in the output surface (non-rotated)"]
    pub mod Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates number of horizontal PIXELS in the output surface (non-rotated)"]
    pub mod X {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processed Surface Upper Left Coordinate"]
pub mod OUT_PS_ULC {
    #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the processed surface in the output buffer"]
    pub mod Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the upper left X-coordinate (in pixels) of the processed surface (PS) in the output buffer"]
    pub mod X {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processed Surface Lower Right Coordinate"]
pub mod OUT_PS_LRC {
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer"]
    pub mod Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the processed surface (PS) in the output frame buffer"]
    pub mod X {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alpha Surface Upper Left Coordinate"]
pub mod OUT_AS_ULC {
    #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the alpha surface in the output frame buffer"]
    pub mod Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the upper left X-coordinate (in pixels) of the alpha surface (AS) in the output frame buffer"]
    pub mod X {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alpha Surface Lower Right Coordinate"]
pub mod OUT_AS_LRC {
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the alpha surface in the output frame buffer"]
    pub mod Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the alpha surface (AS) in the output frame buffer"]
    pub mod X {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processed Surface (PS) Control Register"]
pub mod PS_CTRL {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels (unpacked 24-bit format with/without alpha at high 8bits)"]
            pub const RGB888_ARGB8888: u32 = 0x04;
            #[doc = "16-bit pixels with/without alpha at high 1bit"]
            pub const RGB555_ARGB1555: u32 = 0x0c;
            #[doc = "16-bit pixels with/without alpha at high 4 bits"]
            pub const RGB444_ARGB4444: u32 = 0x0d;
            #[doc = "16-bit pixels"]
            pub const RGB565: u32 = 0x0e;
            #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
            pub const YUV1P444: u32 = 0x10;
            #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
            pub const UYVY1P422: u32 = 0x12;
            #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
            pub const VYUY1P422: u32 = 0x13;
            #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
            pub const Y8: u32 = 0x14;
            #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
            pub const Y4: u32 = 0x15;
            #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
            pub const YUV2P422: u32 = 0x18;
            #[doc = "16-bit pixels (2-plane UV)"]
            pub const YUV2P420: u32 = 0x19;
            #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
            pub const YVU2P422: u32 = 0x1a;
            #[doc = "16-bit pixels (2-plane VU)"]
            pub const YVU2P420: u32 = 0x1b;
            #[doc = "16-bit pixels (3-plane format)"]
            pub const YUV422: u32 = 0x1e;
            #[doc = "16-bit pixels (3-plane format)"]
            pub const YUV420: u32 = 0x1f;
            #[doc = "2-bit pixels with alpha at the low 8 bits"]
            pub const RGBA8888: u32 = 0x24;
            #[doc = "16-bit pixels with alpha at the low 1bits"]
            pub const RGBA5551: u32 = 0x2c;
            #[doc = "16-bit pixels with alpha at the low 4 bits"]
            pub const RGBA4444: u32 = 0x2d;
        }
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    pub mod WB_SWAP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte swap is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Byte swap is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Verticle pre decimation filter control."]
    pub mod DECY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable pre-decimation filter."]
            pub const DISABLE: u32 = 0;
            #[doc = "Decimate PS by 2."]
            pub const DECY2: u32 = 0x01;
            #[doc = "Decimate PS by 4."]
            pub const DECY4: u32 = 0x02;
            #[doc = "Decimate PS by 8."]
            pub const DECY8: u32 = 0x03;
        }
    }
    #[doc = "Horizontal pre decimation filter control."]
    pub mod DECX {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable pre-decimation filter."]
            pub const DISABLE: u32 = 0;
            #[doc = "Decimate PS by 2."]
            pub const DECX2: u32 = 0x01;
            #[doc = "Decimate PS by 4."]
            pub const DECX4: u32 = 0x02;
            #[doc = "Decimate PS by 8."]
            pub const DECX8: u32 = 0x03;
        }
    }
}
#[doc = "Processed Surface (PS) Control Register"]
pub mod PS_CTRL_SET {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels (unpacked 24-bit format with/without alpha at high 8bits)"]
            pub const RGB888_ARGB8888: u32 = 0x04;
            #[doc = "16-bit pixels with/without alpha at high 1bit"]
            pub const RGB555_ARGB1555: u32 = 0x0c;
            #[doc = "16-bit pixels with/without alpha at high 4 bits"]
            pub const RGB444_ARGB4444: u32 = 0x0d;
            #[doc = "16-bit pixels"]
            pub const RGB565: u32 = 0x0e;
            #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
            pub const YUV1P444: u32 = 0x10;
            #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
            pub const UYVY1P422: u32 = 0x12;
            #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
            pub const VYUY1P422: u32 = 0x13;
            #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
            pub const Y8: u32 = 0x14;
            #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
            pub const Y4: u32 = 0x15;
            #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
            pub const YUV2P422: u32 = 0x18;
            #[doc = "16-bit pixels (2-plane UV)"]
            pub const YUV2P420: u32 = 0x19;
            #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
            pub const YVU2P422: u32 = 0x1a;
            #[doc = "16-bit pixels (2-plane VU)"]
            pub const YVU2P420: u32 = 0x1b;
            #[doc = "16-bit pixels (3-plane format)"]
            pub const YUV422: u32 = 0x1e;
            #[doc = "16-bit pixels (3-plane format)"]
            pub const YUV420: u32 = 0x1f;
            #[doc = "2-bit pixels with alpha at the low 8 bits"]
            pub const RGBA8888: u32 = 0x24;
            #[doc = "16-bit pixels with alpha at the low 1bits"]
            pub const RGBA5551: u32 = 0x2c;
            #[doc = "16-bit pixels with alpha at the low 4 bits"]
            pub const RGBA4444: u32 = 0x2d;
        }
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    pub mod WB_SWAP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte swap is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Byte swap is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Verticle pre decimation filter control."]
    pub mod DECY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable pre-decimation filter."]
            pub const DISABLE: u32 = 0;
            #[doc = "Decimate PS by 2."]
            pub const DECY2: u32 = 0x01;
            #[doc = "Decimate PS by 4."]
            pub const DECY4: u32 = 0x02;
            #[doc = "Decimate PS by 8."]
            pub const DECY8: u32 = 0x03;
        }
    }
    #[doc = "Horizontal pre decimation filter control."]
    pub mod DECX {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable pre-decimation filter."]
            pub const DISABLE: u32 = 0;
            #[doc = "Decimate PS by 2."]
            pub const DECX2: u32 = 0x01;
            #[doc = "Decimate PS by 4."]
            pub const DECX4: u32 = 0x02;
            #[doc = "Decimate PS by 8."]
            pub const DECX8: u32 = 0x03;
        }
    }
}
#[doc = "Processed Surface (PS) Control Register"]
pub mod PS_CTRL_CLR {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels (unpacked 24-bit format with/without alpha at high 8bits)"]
            pub const RGB888_ARGB8888: u32 = 0x04;
            #[doc = "16-bit pixels with/without alpha at high 1bit"]
            pub const RGB555_ARGB1555: u32 = 0x0c;
            #[doc = "16-bit pixels with/without alpha at high 4 bits"]
            pub const RGB444_ARGB4444: u32 = 0x0d;
            #[doc = "16-bit pixels"]
            pub const RGB565: u32 = 0x0e;
            #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
            pub const YUV1P444: u32 = 0x10;
            #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
            pub const UYVY1P422: u32 = 0x12;
            #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
            pub const VYUY1P422: u32 = 0x13;
            #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
            pub const Y8: u32 = 0x14;
            #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
            pub const Y4: u32 = 0x15;
            #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
            pub const YUV2P422: u32 = 0x18;
            #[doc = "16-bit pixels (2-plane UV)"]
            pub const YUV2P420: u32 = 0x19;
            #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
            pub const YVU2P422: u32 = 0x1a;
            #[doc = "16-bit pixels (2-plane VU)"]
            pub const YVU2P420: u32 = 0x1b;
            #[doc = "16-bit pixels (3-plane format)"]
            pub const YUV422: u32 = 0x1e;
            #[doc = "16-bit pixels (3-plane format)"]
            pub const YUV420: u32 = 0x1f;
            #[doc = "2-bit pixels with alpha at the low 8 bits"]
            pub const RGBA8888: u32 = 0x24;
            #[doc = "16-bit pixels with alpha at the low 1bits"]
            pub const RGBA5551: u32 = 0x2c;
            #[doc = "16-bit pixels with alpha at the low 4 bits"]
            pub const RGBA4444: u32 = 0x2d;
        }
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    pub mod WB_SWAP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte swap is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Byte swap is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Verticle pre decimation filter control."]
    pub mod DECY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable pre-decimation filter."]
            pub const DISABLE: u32 = 0;
            #[doc = "Decimate PS by 2."]
            pub const DECY2: u32 = 0x01;
            #[doc = "Decimate PS by 4."]
            pub const DECY4: u32 = 0x02;
            #[doc = "Decimate PS by 8."]
            pub const DECY8: u32 = 0x03;
        }
    }
    #[doc = "Horizontal pre decimation filter control."]
    pub mod DECX {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable pre-decimation filter."]
            pub const DISABLE: u32 = 0;
            #[doc = "Decimate PS by 2."]
            pub const DECX2: u32 = 0x01;
            #[doc = "Decimate PS by 4."]
            pub const DECX4: u32 = 0x02;
            #[doc = "Decimate PS by 8."]
            pub const DECX8: u32 = 0x03;
        }
    }
}
#[doc = "Processed Surface (PS) Control Register"]
pub mod PS_CTRL_TOG {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels (unpacked 24-bit format with/without alpha at high 8bits)"]
            pub const RGB888_ARGB8888: u32 = 0x04;
            #[doc = "16-bit pixels with/without alpha at high 1bit"]
            pub const RGB555_ARGB1555: u32 = 0x0c;
            #[doc = "16-bit pixels with/without alpha at high 4 bits"]
            pub const RGB444_ARGB4444: u32 = 0x0d;
            #[doc = "16-bit pixels"]
            pub const RGB565: u32 = 0x0e;
            #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
            pub const YUV1P444: u32 = 0x10;
            #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
            pub const UYVY1P422: u32 = 0x12;
            #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
            pub const VYUY1P422: u32 = 0x13;
            #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
            pub const Y8: u32 = 0x14;
            #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
            pub const Y4: u32 = 0x15;
            #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
            pub const YUV2P422: u32 = 0x18;
            #[doc = "16-bit pixels (2-plane UV)"]
            pub const YUV2P420: u32 = 0x19;
            #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
            pub const YVU2P422: u32 = 0x1a;
            #[doc = "16-bit pixels (2-plane VU)"]
            pub const YVU2P420: u32 = 0x1b;
            #[doc = "16-bit pixels (3-plane format)"]
            pub const YUV422: u32 = 0x1e;
            #[doc = "16-bit pixels (3-plane format)"]
            pub const YUV420: u32 = 0x1f;
            #[doc = "2-bit pixels with alpha at the low 8 bits"]
            pub const RGBA8888: u32 = 0x24;
            #[doc = "16-bit pixels with alpha at the low 1bits"]
            pub const RGBA5551: u32 = 0x2c;
            #[doc = "16-bit pixels with alpha at the low 4 bits"]
            pub const RGBA4444: u32 = 0x2d;
        }
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    pub mod WB_SWAP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte swap is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Byte swap is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Verticle pre decimation filter control."]
    pub mod DECY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable pre-decimation filter."]
            pub const DISABLE: u32 = 0;
            #[doc = "Decimate PS by 2."]
            pub const DECY2: u32 = 0x01;
            #[doc = "Decimate PS by 4."]
            pub const DECY4: u32 = 0x02;
            #[doc = "Decimate PS by 8."]
            pub const DECY8: u32 = 0x03;
        }
    }
    #[doc = "Horizontal pre decimation filter control."]
    pub mod DECX {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable pre-decimation filter."]
            pub const DISABLE: u32 = 0;
            #[doc = "Decimate PS by 2."]
            pub const DECX2: u32 = 0x01;
            #[doc = "Decimate PS by 4."]
            pub const DECX4: u32 = 0x02;
            #[doc = "Decimate PS by 8."]
            pub const DECX8: u32 = 0x03;
        }
    }
}
#[doc = "PS Input Buffer Address"]
pub mod PS_BUF {
    #[doc = "Address pointer for the PS RGB or Y (luma) input buffer."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PS U/Cb or 2 Plane UV Input Buffer Address"]
pub mod PS_UBUF {
    #[doc = "Address pointer for the PS U/Cb or 2 plane UV Chroma input buffer."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PS V/Cr Input Buffer Address"]
pub mod PS_VBUF {
    #[doc = "Address pointer for the PS V/Cr Chroma input buffer."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processed Surface Pitch"]
pub mod PS_PITCH {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PS Background Color"]
pub mod PS_BACKGROUND {
    #[doc = "Background color (in 24bpp format) for any pixels not within the buffer range specified by the PS ULC/LRC"]
    pub mod COLOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PS Scale Factor Register"]
pub mod PS_SCALE {
    #[doc = "This is a two bit integer and 12 bit fractional representation (##"]
    pub mod XSCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is a two bit integer and 12 bit fractional representation (##"]
    pub mod YSCALE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PS Scale Offset Register"]
pub mod PS_OFFSET {
    #[doc = "This is a 12 bit fractional representation (0"]
    pub mod XOFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is a 12 bit fractional representation (0"]
    pub mod YOFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PS Color Key Low"]
pub mod PS_CLRKEYLOW {
    #[doc = "Low range of color key applied to PS buffer"]
    pub mod PIXEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PS Color Key High"]
pub mod PS_CLRKEYHIGH {
    #[doc = "High range of color key applied to PS buffer"]
    pub mod PIXEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alpha Surface Control"]
pub mod AS_CTRL {
    #[doc = "Determines how the alpha value is constructed for this alpha surface"]
    pub mod ALPHA_CTRL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Indicates that the AS pixel alpha value will be used to blend the AS with PS. The ALPHA field is ignored."]
            pub const EMBEDDED: u32 = 0;
            #[doc = "Indicates that the value in the ALPHA field should be used instead of the alpha values present in the input pixels."]
            pub const OVERRIDE: u32 = 0x01;
            #[doc = "Indicates that the value in the ALPHA field should be used to scale all pixel alpha values. Each pixel alpha is multiplied by the value in the ALPHA field."]
            pub const MULTIPLY: u32 = 0x02;
            #[doc = "Enable ROPs. The ROP field indicates an operation to be performed on the alpha surface and PS pixels."]
            pub const ROPS: u32 = 0x03;
        }
    }
    #[doc = "Indicates that colorkey functionality is enabled for this alpha surface"]
    pub mod ENABLE_COLORKEY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Indicates the input buffer format for AS."]
    pub mod FORMAT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32-bit pixels with alpha"]
            pub const ARGB8888: u32 = 0;
            #[doc = "2-bit pixel with alpha at low 8 bits"]
            pub const RGBA888: u32 = 0x01;
            #[doc = "32-bit pixels without alpha (unpacked 24-bit format)"]
            pub const RGB888: u32 = 0x04;
            #[doc = "16-bit pixels with alpha"]
            pub const ARGB1555: u32 = 0x08;
            #[doc = "16-bit pixels with alpha"]
            pub const ARGB4444: u32 = 0x09;
            #[doc = "16-bit pixel with alpha at low 1 bit"]
            pub const RGBA5551: u32 = 0x0a;
            #[doc = "16-bit pixel with alpha at low 4 bits"]
            pub const RGBA4444: u32 = 0x0b;
            #[doc = "16-bit pixels without alpha"]
            pub const RGB555: u32 = 0x0c;
            #[doc = "16-bit pixels without alpha"]
            pub const RGB444: u32 = 0x0d;
            #[doc = "16-bit pixels without alpha"]
            pub const RGB565: u32 = 0x0e;
        }
    }
    #[doc = "Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in PXP_AS_CTRL\\[ALPHA_CTRL\\]"]
    pub mod ALPHA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates a raster operation to perform when enabled"]
    pub mod ROP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AS AND PS"]
            pub const MASKAS: u32 = 0;
            #[doc = "nAS AND PS"]
            pub const MASKNOTAS: u32 = 0x01;
            #[doc = "AS AND nPS"]
            pub const MASKASNOT: u32 = 0x02;
            #[doc = "AS OR PS"]
            pub const MERGEAS: u32 = 0x03;
            #[doc = "nAS OR PS"]
            pub const MERGENOTAS: u32 = 0x04;
            #[doc = "AS OR nPS"]
            pub const MERGEASNOT: u32 = 0x05;
            #[doc = "nAS"]
            pub const NOTCOPYAS: u32 = 0x06;
            #[doc = "nPS"]
            pub const NOT: u32 = 0x07;
            #[doc = "AS NAND PS"]
            pub const NOTMASKAS: u32 = 0x08;
            #[doc = "AS NOR PS"]
            pub const NOTMERGEAS: u32 = 0x09;
            #[doc = "AS XOR PS"]
            pub const XORAS: u32 = 0x0a;
            #[doc = "AS XNOR PS"]
            pub const NOTXORAS: u32 = 0x0b;
        }
    }
    #[doc = "Setting this bit to logic 0 will not alter the alpha value"]
    pub mod ALPHA_INVERT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const ALPHA_INVERT_0: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERTED: u32 = 0x01;
        }
    }
}
#[doc = "Alpha Surface Buffer Pointer"]
pub mod AS_BUF {
    #[doc = "Address pointer for the alpha surface 0 buffer."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alpha Surface Pitch"]
pub mod AS_PITCH {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Overlay Color Key Low"]
pub mod AS_CLRKEYLOW {
    #[doc = "Low range of RGB color key applied to AS buffer. Each overlay has an independent colorkey enable."]
    pub mod PIXEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Overlay Color Key High"]
pub mod AS_CLRKEYHIGH {
    #[doc = "High range of RGB color key applied to AS buffer. Each overlay has an independent colorkey enable."]
    pub mod PIXEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Color Space Conversion Coefficient Register 0"]
pub mod CSC1_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data"]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data"]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass the CSC unit in the scaling engine"]
    pub mod BYPASS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to 1 when performing YCbCr conversion to RGB"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "YUV to RGB"]
            pub const YCBCR_MODE_0: u32 = 0;
            #[doc = "YCbCr to RGB"]
            pub const YCBCR_MODE_1: u32 = 0x01;
        }
    }
}
#[doc = "Color Space Conversion Coefficient Register 1"]
pub mod CSC1_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)"]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)"]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Color Space Conversion Coefficient Register 2"]
pub mod CSC1_COEF2 {
    #[doc = "Two's complement Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)"]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's complement Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)"]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PXP Power Control Register"]
pub mod POWER {
    #[doc = "Select the low power state of the Rotation (ROT) memory."]
    pub mod ROT_MEM_LP_STATE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Memory is not in low power state."]
            pub const NONE: u32 = 0;
            #[doc = "Light Sleep Mode. Low leakage mode, maintain memory contents."]
            pub const LS: u32 = 0x01;
            #[doc = "Deep Sleep Mode. Low leakage mode, maintain memory contents."]
            pub const DS: u32 = 0x02;
            #[doc = "Shut Down Mode. Shut Down periphery and core, no memory retention."]
            pub const SD: u32 = 0x04;
        }
    }
}
#[doc = "Next Frame Pointer"]
pub mod NEXT {
    #[doc = "Indicates that the \"next frame\" functionality has been enabled"]
    pub mod ENABLED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A pointer to a data structure containing register values to be used when processing the next frame"]
    pub mod POINTER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PXP Alpha Engine A Control Register."]
pub mod PORTER_DUFF_CTRL {
    #[doc = "Porter-Duff Enable"]
    pub mod PORTER_DUFF_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "s0 to s1 factor mode"]
    pub mod S0_S1_FACTOR_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const S0_S1_FACTOR_MODE_0: u32 = 0;
            #[doc = "0"]
            pub const S0_S1_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Straight alpha"]
            pub const S0_S1_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Inverse alpha"]
            pub const S0_S1_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "s0 global alpha mode"]
    pub mod S0_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Global alpha"]
            pub const S0_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Local alpha"]
            pub const S0_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Scaled alpha"]
            pub const S0_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Scaled alpha"]
            pub const S0_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "s0 alpha mode (Porter-Duff alpha mode)"]
    pub mod S0_ALPHA_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode"]
            pub const S0_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inverted mode"]
            pub const S0_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "s0 color mode (Porter-Duff color mode)"]
    pub mod S0_COLOR_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Original pixel"]
            pub const S0_COLOR_MODE_0: u32 = 0;
            #[doc = "Scaled pixel"]
            pub const S0_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "s1 to s0 factor mode (Porter-Duff factor mode)"]
    pub mod S1_S0_FACTOR_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const S1_S0_FACTOR_MODE_0: u32 = 0;
            #[doc = "0"]
            pub const S1_S0_FACTOR_MODE_1: u32 = 0x01;
            #[doc = "Straight alpha"]
            pub const S1_S0_FACTOR_MODE_2: u32 = 0x02;
            #[doc = "Inverse alpha"]
            pub const S1_S0_FACTOR_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "s1 global alpha mode (Porter-Duff Global Alpha mode)"]
    pub mod S1_GLOBAL_ALPHA_MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Global alpha"]
            pub const S1_GLOBAL_ALPHA_MODE_0: u32 = 0;
            #[doc = "Local alpha"]
            pub const S1_GLOBAL_ALPHA_MODE_1: u32 = 0x01;
            #[doc = "Scaled alpha"]
            pub const S1_GLOBAL_ALPHA_MODE_2: u32 = 0x02;
            #[doc = "Scaled alpha"]
            pub const S1_GLOBAL_ALPHA_MODE_3: u32 = 0x03;
        }
    }
    #[doc = "s1 alpha mode (Porter-Duff Alpha mode)"]
    pub mod S1_ALPHA_MODE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Straight mode"]
            pub const S1_ALPHA_MODE_0: u32 = 0;
            #[doc = "Inverted mode"]
            pub const S1_ALPHA_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "s1 color mode"]
    pub mod S1_COLOR_MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Original pixel"]
            pub const S1_COLOR_MODE_0: u32 = 0;
            #[doc = "Scaled pixel"]
            pub const S1_COLOR_MODE_1: u32 = 0x01;
        }
    }
    #[doc = "s0 global alpha"]
    pub mod S0_GLOBAL_ALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "s1 global alpha"]
    pub mod S1_GLOBAL_ALPHA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
