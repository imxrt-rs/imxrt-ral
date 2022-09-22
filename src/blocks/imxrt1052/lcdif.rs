#[doc = "LCDIF Register Reference Index"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "LCDIF General Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control Register"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control Register"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control Register"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control1 Register"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control1 Register"]
    pub CTRL1_SET: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control1 Register"]
    pub CTRL1_CLR: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control1 Register"]
    pub CTRL1_TOG: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control2 Register"]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control2 Register"]
    pub CTRL2_SET: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control2 Register"]
    pub CTRL2_CLR: crate::RWRegister<u32>,
    #[doc = "LCDIF General Control2 Register"]
    pub CTRL2_TOG: crate::RWRegister<u32>,
    #[doc = "LCDIF Horizontal and Vertical Valid Data Count Register"]
    pub TRANSFER_COUNT: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "LCD Interface Current Buffer Address Register"]
    pub CUR_BUF: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "LCD Interface Next Buffer Address Register"]
    pub NEXT_BUF: crate::RWRegister<u32>,
    _reserved2: [u8; 0x1c],
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub VDCTRL0: crate::RWRegister<u32>,
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub VDCTRL0_SET: crate::RWRegister<u32>,
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub VDCTRL0_CLR: crate::RWRegister<u32>,
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub VDCTRL0_TOG: crate::RWRegister<u32>,
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register1"]
    pub VDCTRL1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
    pub VDCTRL2: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
    pub VDCTRL3: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
    pub VDCTRL4: crate::RWRegister<u32>,
    _reserved6: [u8; 0xdc],
    #[doc = "Bus Master Error Status Register"]
    pub BM_ERROR_STAT: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "CRC Status Register"]
    pub CRC_STAT: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "LCD Interface Status Register"]
    pub STAT: crate::RORegister<u32>,
    _reserved9: [u8; 0x4c],
    #[doc = "LCDIF Threshold Register"]
    pub THRES: crate::RWRegister<u32>,
    _reserved10: [u8; 0x017c],
    #[doc = "LCDIF Pigeon Mode Control0 Register"]
    pub PIGEONCTRL0: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control0 Register"]
    pub PIGEONCTRL0_SET: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control0 Register"]
    pub PIGEONCTRL0_CLR: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control0 Register"]
    pub PIGEONCTRL0_TOG: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control1 Register"]
    pub PIGEONCTRL1: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control1 Register"]
    pub PIGEONCTRL1_SET: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control1 Register"]
    pub PIGEONCTRL1_CLR: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control1 Register"]
    pub PIGEONCTRL1_TOG: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control2 Register"]
    pub PIGEONCTRL2: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control2 Register"]
    pub PIGEONCTRL2_SET: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control2 Register"]
    pub PIGEONCTRL2_CLR: crate::RWRegister<u32>,
    #[doc = "LCDIF Pigeon Mode Control2 Register"]
    pub PIGEONCTRL2_TOG: crate::RWRegister<u32>,
    _reserved11: [u8; 0x0450],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_0_0: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_0_1: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_0_2: crate::RWRegister<u32>,
    _reserved14: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_1_0: crate::RWRegister<u32>,
    _reserved15: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_1_1: crate::RWRegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_1_2: crate::RWRegister<u32>,
    _reserved17: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_2_0: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_2_1: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_2_2: crate::RWRegister<u32>,
    _reserved20: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_3_0: crate::RWRegister<u32>,
    _reserved21: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_3_1: crate::RWRegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_3_2: crate::RWRegister<u32>,
    _reserved23: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_4_0: crate::RWRegister<u32>,
    _reserved24: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_4_1: crate::RWRegister<u32>,
    _reserved25: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_4_2: crate::RWRegister<u32>,
    _reserved26: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_5_0: crate::RWRegister<u32>,
    _reserved27: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_5_1: crate::RWRegister<u32>,
    _reserved28: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_5_2: crate::RWRegister<u32>,
    _reserved29: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_6_0: crate::RWRegister<u32>,
    _reserved30: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_6_1: crate::RWRegister<u32>,
    _reserved31: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_6_2: crate::RWRegister<u32>,
    _reserved32: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_7_0: crate::RWRegister<u32>,
    _reserved33: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_7_1: crate::RWRegister<u32>,
    _reserved34: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_7_2: crate::RWRegister<u32>,
    _reserved35: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_8_0: crate::RWRegister<u32>,
    _reserved36: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_8_1: crate::RWRegister<u32>,
    _reserved37: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_8_2: crate::RWRegister<u32>,
    _reserved38: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_9_0: crate::RWRegister<u32>,
    _reserved39: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_9_1: crate::RWRegister<u32>,
    _reserved40: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_9_2: crate::RWRegister<u32>,
    _reserved41: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_10_0: crate::RWRegister<u32>,
    _reserved42: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_10_1: crate::RWRegister<u32>,
    _reserved43: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_10_2: crate::RWRegister<u32>,
    _reserved44: [u8; 0x1c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_11_0: crate::RWRegister<u32>,
    _reserved45: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_11_1: crate::RWRegister<u32>,
    _reserved46: [u8; 0x0c],
    #[doc = "Panel Interface Signal Generator Register"]
    pub PIGEON_11_2: crate::RWRegister<u32>,
    _reserved47: [u8; 0x1c],
    #[doc = "Lookup Table Data Register."]
    pub LUT_CTRL: crate::RWRegister<u32>,
    _reserved48: [u8; 0x0c],
    #[doc = "Lookup Table Control Register."]
    pub LUT0_ADDR: crate::RWRegister<u32>,
    _reserved49: [u8; 0x0c],
    #[doc = "Lookup Table Data Register."]
    pub LUT0_DATA: crate::RWRegister<u32>,
    _reserved50: [u8; 0x0c],
    #[doc = "Lookup Table Control Register."]
    pub LUT1_ADDR: crate::RWRegister<u32>,
    _reserved51: [u8; 0x0c],
    #[doc = "Lookup Table Data Register."]
    pub LUT1_DATA: crate::RWRegister<u32>,
}
#[doc = "LCDIF General Control Register"]
pub mod CTRL {
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    pub mod RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    pub mod DATA_FORMAT_24_BIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
            pub const ALL_24_BITS_VALID: u32 = 0;
            #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
            pub const DROP_UPPER_2_BITS_PER_BYTE: u32 = 0x01;
        }
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    pub mod DATA_FORMAT_18_BIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
            pub const LOWER_18_BITS_VALID: u32 = 0;
            #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
            pub const UPPER_18_BITS_VALID: u32 = 0x01;
        }
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    pub mod DATA_FORMAT_16_BIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    pub mod MASTER {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    pub mod ENABLE_PXP_HANDSHAKE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input data format."]
    pub mod WORD_LENGTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input data is 16 bits per pixel."]
            pub const _16_BIT: u32 = 0;
            #[doc = "Input data is 8 bits wide."]
            pub const _8_BIT: u32 = 0x01;
            #[doc = "Input data is 18 bits per pixel."]
            pub const _18_BIT: u32 = 0x02;
            #[doc = "Input data is 24 bits per pixel."]
            pub const _24_BIT: u32 = 0x03;
        }
    }
    #[doc = "LCD Data bus transfer width."]
    pub mod LCD_DATABUS_WIDTH {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16-bit data bus mode."]
            pub const _16_BIT: u32 = 0;
            #[doc = "8-bit data bus mode."]
            pub const _8_BIT: u32 = 0x01;
            #[doc = "18-bit data bus mode."]
            pub const _18_BIT: u32 = 0x02;
            #[doc = "24-bit data bus mode."]
            pub const _24_BIT: u32 = 0x03;
        }
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    pub mod CSC_DATA_SWIZZLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No byte swapping.(Little endian)"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
            pub const BIG_ENDIAN_SWAP: u32 = 0x01;
            #[doc = "Swap half-words."]
            pub const HWD_SWAP: u32 = 0x02;
            #[doc = "Swap bytes within each half-word."]
            pub const HWD_BYTE_SWAP: u32 = 0x03;
        }
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    pub mod INPUT_DATA_SWIZZLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No byte swapping.(Little endian)"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
            pub const BIG_ENDIAN_SWAP: u32 = 0x01;
            #[doc = "Swap half-words."]
            pub const HWD_SWAP: u32 = 0x02;
            #[doc = "Swap bytes within each half-word."]
            pub const HWD_BYTE_SWAP: u32 = 0x03;
        }
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    pub mod DOTCLK_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    pub mod BYPASS_COUNT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    pub mod SHIFT_NUM_BITS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    pub mod DATA_SHIFT_DIR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
            pub const TXDATA_SHIFT_LEFT: u32 = 0;
            #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
            pub const TXDATA_SHIFT_RIGHT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF General Control Register"]
pub mod CTRL_SET {
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    pub mod RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    pub mod DATA_FORMAT_24_BIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
            pub const ALL_24_BITS_VALID: u32 = 0;
            #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
            pub const DROP_UPPER_2_BITS_PER_BYTE: u32 = 0x01;
        }
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    pub mod DATA_FORMAT_18_BIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
            pub const LOWER_18_BITS_VALID: u32 = 0;
            #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
            pub const UPPER_18_BITS_VALID: u32 = 0x01;
        }
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    pub mod DATA_FORMAT_16_BIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    pub mod MASTER {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    pub mod ENABLE_PXP_HANDSHAKE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input data format."]
    pub mod WORD_LENGTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input data is 16 bits per pixel."]
            pub const _16_BIT: u32 = 0;
            #[doc = "Input data is 8 bits wide."]
            pub const _8_BIT: u32 = 0x01;
            #[doc = "Input data is 18 bits per pixel."]
            pub const _18_BIT: u32 = 0x02;
            #[doc = "Input data is 24 bits per pixel."]
            pub const _24_BIT: u32 = 0x03;
        }
    }
    #[doc = "LCD Data bus transfer width."]
    pub mod LCD_DATABUS_WIDTH {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16-bit data bus mode."]
            pub const _16_BIT: u32 = 0;
            #[doc = "8-bit data bus mode."]
            pub const _8_BIT: u32 = 0x01;
            #[doc = "18-bit data bus mode."]
            pub const _18_BIT: u32 = 0x02;
            #[doc = "24-bit data bus mode."]
            pub const _24_BIT: u32 = 0x03;
        }
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    pub mod CSC_DATA_SWIZZLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No byte swapping.(Little endian)"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
            pub const BIG_ENDIAN_SWAP: u32 = 0x01;
            #[doc = "Swap half-words."]
            pub const HWD_SWAP: u32 = 0x02;
            #[doc = "Swap bytes within each half-word."]
            pub const HWD_BYTE_SWAP: u32 = 0x03;
        }
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    pub mod INPUT_DATA_SWIZZLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No byte swapping.(Little endian)"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
            pub const BIG_ENDIAN_SWAP: u32 = 0x01;
            #[doc = "Swap half-words."]
            pub const HWD_SWAP: u32 = 0x02;
            #[doc = "Swap bytes within each half-word."]
            pub const HWD_BYTE_SWAP: u32 = 0x03;
        }
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    pub mod DOTCLK_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    pub mod BYPASS_COUNT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    pub mod SHIFT_NUM_BITS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    pub mod DATA_SHIFT_DIR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
            pub const TXDATA_SHIFT_LEFT: u32 = 0;
            #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
            pub const TXDATA_SHIFT_RIGHT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF General Control Register"]
pub mod CTRL_CLR {
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    pub mod RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    pub mod DATA_FORMAT_24_BIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
            pub const ALL_24_BITS_VALID: u32 = 0;
            #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
            pub const DROP_UPPER_2_BITS_PER_BYTE: u32 = 0x01;
        }
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    pub mod DATA_FORMAT_18_BIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
            pub const LOWER_18_BITS_VALID: u32 = 0;
            #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
            pub const UPPER_18_BITS_VALID: u32 = 0x01;
        }
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    pub mod DATA_FORMAT_16_BIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    pub mod MASTER {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    pub mod ENABLE_PXP_HANDSHAKE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input data format."]
    pub mod WORD_LENGTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input data is 16 bits per pixel."]
            pub const _16_BIT: u32 = 0;
            #[doc = "Input data is 8 bits wide."]
            pub const _8_BIT: u32 = 0x01;
            #[doc = "Input data is 18 bits per pixel."]
            pub const _18_BIT: u32 = 0x02;
            #[doc = "Input data is 24 bits per pixel."]
            pub const _24_BIT: u32 = 0x03;
        }
    }
    #[doc = "LCD Data bus transfer width."]
    pub mod LCD_DATABUS_WIDTH {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16-bit data bus mode."]
            pub const _16_BIT: u32 = 0;
            #[doc = "8-bit data bus mode."]
            pub const _8_BIT: u32 = 0x01;
            #[doc = "18-bit data bus mode."]
            pub const _18_BIT: u32 = 0x02;
            #[doc = "24-bit data bus mode."]
            pub const _24_BIT: u32 = 0x03;
        }
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    pub mod CSC_DATA_SWIZZLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No byte swapping.(Little endian)"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
            pub const BIG_ENDIAN_SWAP: u32 = 0x01;
            #[doc = "Swap half-words."]
            pub const HWD_SWAP: u32 = 0x02;
            #[doc = "Swap bytes within each half-word."]
            pub const HWD_BYTE_SWAP: u32 = 0x03;
        }
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    pub mod INPUT_DATA_SWIZZLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No byte swapping.(Little endian)"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
            pub const BIG_ENDIAN_SWAP: u32 = 0x01;
            #[doc = "Swap half-words."]
            pub const HWD_SWAP: u32 = 0x02;
            #[doc = "Swap bytes within each half-word."]
            pub const HWD_BYTE_SWAP: u32 = 0x03;
        }
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    pub mod DOTCLK_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    pub mod BYPASS_COUNT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    pub mod SHIFT_NUM_BITS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    pub mod DATA_SHIFT_DIR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
            pub const TXDATA_SHIFT_LEFT: u32 = 0;
            #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
            pub const TXDATA_SHIFT_RIGHT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF General Control Register"]
pub mod CTRL_TOG {
    #[doc = "When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    pub mod RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Used only when WORD_LENGTH = 3, i"]
    pub mod DATA_FORMAT_24_BIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
            pub const ALL_24_BITS_VALID: u32 = 0;
            #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
            pub const DROP_UPPER_2_BITS_PER_BYTE: u32 = 0x01;
        }
    }
    #[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    pub mod DATA_FORMAT_18_BIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
            pub const LOWER_18_BITS_VALID: u32 = 0;
            #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
            pub const UPPER_18_BITS_VALID: u32 = 0x01;
        }
    }
    #[doc = "When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    pub mod DATA_FORMAT_16_BIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to make the LCDIF act as a bus master"]
    pub mod MASTER {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    pub mod ENABLE_PXP_HANDSHAKE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input data format."]
    pub mod WORD_LENGTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input data is 16 bits per pixel."]
            pub const _16_BIT: u32 = 0;
            #[doc = "Input data is 8 bits wide."]
            pub const _8_BIT: u32 = 0x01;
            #[doc = "Input data is 18 bits per pixel."]
            pub const _18_BIT: u32 = 0x02;
            #[doc = "Input data is 24 bits per pixel."]
            pub const _24_BIT: u32 = 0x03;
        }
    }
    #[doc = "LCD Data bus transfer width."]
    pub mod LCD_DATABUS_WIDTH {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16-bit data bus mode."]
            pub const _16_BIT: u32 = 0;
            #[doc = "8-bit data bus mode."]
            pub const _8_BIT: u32 = 0x01;
            #[doc = "18-bit data bus mode."]
            pub const _18_BIT: u32 = 0x02;
            #[doc = "24-bit data bus mode."]
            pub const _24_BIT: u32 = 0x03;
        }
    }
    #[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    pub mod CSC_DATA_SWIZZLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No byte swapping.(Little endian)"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
            pub const BIG_ENDIAN_SWAP: u32 = 0x01;
            #[doc = "Swap half-words."]
            pub const HWD_SWAP: u32 = 0x02;
            #[doc = "Swap bytes within each half-word."]
            pub const HWD_BYTE_SWAP: u32 = 0x03;
        }
    }
    #[doc = "This field specifies how to swap the bytes fetched by the bus master interface"]
    pub mod INPUT_DATA_SWIZZLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No byte swapping.(Little endian)"]
            pub const NO_SWAP: u32 = 0;
            #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
            pub const BIG_ENDIAN_SWAP: u32 = 0x01;
            #[doc = "Swap half-words."]
            pub const HWD_SWAP: u32 = 0x02;
            #[doc = "Swap bytes within each half-word."]
            pub const HWD_BYTE_SWAP: u32 = 0x03;
        }
    }
    #[doc = "Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    pub mod DOTCLK_MODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    pub mod BYPASS_COUNT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The data to be transmitted is shifted left or right by this number of bits."]
    pub mod SHIFT_NUM_BITS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use this bit to determine the direction of shift of transmit data."]
    pub mod DATA_SHIFT_DIR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
            pub const TXDATA_SHIFT_LEFT: u32 = 0;
            #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
            pub const TXDATA_SHIFT_RIGHT: u32 = 0x01;
        }
    }
    #[doc = "This bit must be set to zero for normal operation"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit must be set to zero to enable normal operation of the LCDIF"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF General Control1 Register"]
pub mod CTRL1 {
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod VSYNC_EDGE_IRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod CUR_FRAME_DONE_IRQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod UNDERFLOW_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod OVERFLOW_IRQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    pub mod VSYNC_EDGE_IRQ_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    pub mod CUR_FRAME_DONE_IRQ_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    pub mod UNDERFLOW_IRQ_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    pub mod OVERFLOW_IRQ_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    pub mod BYTE_PACKING_FORMAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    pub mod IRQ_ON_ALTERNATE_FIELDS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    pub mod FIFO_CLEAR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    pub mod START_INTERLACE_FROM_SECOND_FIELD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    pub mod INTERLACE_FIELDS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    pub mod RECOVER_ON_UNDERFLOW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod BM_ERROR_IRQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    pub mod BM_ERROR_IRQ_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    pub mod CS_OUT_SELECT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Mode MIPI image data select bit"]
    pub mod IMAGE_DATA_SELECT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF General Control1 Register"]
pub mod CTRL1_SET {
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod VSYNC_EDGE_IRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod CUR_FRAME_DONE_IRQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod UNDERFLOW_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod OVERFLOW_IRQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    pub mod VSYNC_EDGE_IRQ_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    pub mod CUR_FRAME_DONE_IRQ_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    pub mod UNDERFLOW_IRQ_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    pub mod OVERFLOW_IRQ_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    pub mod BYTE_PACKING_FORMAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    pub mod IRQ_ON_ALTERNATE_FIELDS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    pub mod FIFO_CLEAR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    pub mod START_INTERLACE_FROM_SECOND_FIELD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    pub mod INTERLACE_FIELDS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    pub mod RECOVER_ON_UNDERFLOW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod BM_ERROR_IRQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    pub mod BM_ERROR_IRQ_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    pub mod CS_OUT_SELECT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Mode MIPI image data select bit"]
    pub mod IMAGE_DATA_SELECT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF General Control1 Register"]
pub mod CTRL1_CLR {
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod VSYNC_EDGE_IRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod CUR_FRAME_DONE_IRQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod UNDERFLOW_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod OVERFLOW_IRQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    pub mod VSYNC_EDGE_IRQ_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    pub mod CUR_FRAME_DONE_IRQ_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    pub mod UNDERFLOW_IRQ_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    pub mod OVERFLOW_IRQ_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    pub mod BYTE_PACKING_FORMAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    pub mod IRQ_ON_ALTERNATE_FIELDS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    pub mod FIFO_CLEAR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    pub mod START_INTERLACE_FROM_SECOND_FIELD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    pub mod INTERLACE_FIELDS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    pub mod RECOVER_ON_UNDERFLOW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod BM_ERROR_IRQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    pub mod BM_ERROR_IRQ_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    pub mod CS_OUT_SELECT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Mode MIPI image data select bit"]
    pub mod IMAGE_DATA_SELECT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF General Control1 Register"]
pub mod CTRL1_TOG {
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod VSYNC_EDGE_IRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod CUR_FRAME_DONE_IRQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod UNDERFLOW_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod OVERFLOW_IRQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    pub mod VSYNC_EDGE_IRQ_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    pub mod CUR_FRAME_DONE_IRQ_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    pub mod UNDERFLOW_IRQ_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    pub mod OVERFLOW_IRQ_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield is used to show which data bytes in a 32-bit word are valid"]
    pub mod BYTE_PACKING_FORMAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    pub mod IRQ_ON_ALTERNATE_FIELDS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    pub mod FIFO_CLEAR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The default is to grab the odd lines first and then the even lines"]
    pub mod START_INTERLACE_FROM_SECOND_FIELD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    pub mod INTERLACE_FIELDS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    pub mod RECOVER_ON_UNDERFLOW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    pub mod BM_ERROR_IRQ {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Interrupt Request Pending."]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Interrupt Request Pending."]
            pub const REQUEST: u32 = 0x01;
        }
    }
    #[doc = "This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    pub mod BM_ERROR_IRQ_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is CS0/CS1 valid select signals"]
    pub mod CS_OUT_SELECT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Mode MIPI image data select bit"]
    pub mod IMAGE_DATA_SELECT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF General Control2 Register"]
pub mod CTRL2 {
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    pub mod EVEN_LINE_PATTERN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const RGB: u32 = 0;
            #[doc = "RBG"]
            pub const RBG: u32 = 0x01;
            #[doc = "GBR"]
            pub const GBR: u32 = 0x02;
            #[doc = "GRB"]
            pub const GRB: u32 = 0x03;
            #[doc = "BRG"]
            pub const BRG: u32 = 0x04;
            #[doc = "BGR"]
            pub const BGR: u32 = 0x05;
        }
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    pub mod ODD_LINE_PATTERN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const RGB: u32 = 0;
            #[doc = "RBG"]
            pub const RBG: u32 = 0x01;
            #[doc = "GBR"]
            pub const GBR: u32 = 0x02;
            #[doc = "GRB"]
            pub const GRB: u32 = 0x03;
            #[doc = "BRG"]
            pub const BRG: u32 = 0x04;
            #[doc = "BGR"]
            pub const BGR: u32 = 0x05;
        }
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    pub mod BURST_LEN_8 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    pub mod OUTSTANDING_REQS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "REQ_1"]
            pub const REQ_1: u32 = 0;
            #[doc = "REQ_2"]
            pub const REQ_2: u32 = 0x01;
            #[doc = "REQ_4"]
            pub const REQ_4: u32 = 0x02;
            #[doc = "REQ_8"]
            pub const REQ_8: u32 = 0x03;
            #[doc = "REQ_16"]
            pub const REQ_16: u32 = 0x04;
        }
    }
}
#[doc = "LCDIF General Control2 Register"]
pub mod CTRL2_SET {
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    pub mod EVEN_LINE_PATTERN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const RGB: u32 = 0;
            #[doc = "RBG"]
            pub const RBG: u32 = 0x01;
            #[doc = "GBR"]
            pub const GBR: u32 = 0x02;
            #[doc = "GRB"]
            pub const GRB: u32 = 0x03;
            #[doc = "BRG"]
            pub const BRG: u32 = 0x04;
            #[doc = "BGR"]
            pub const BGR: u32 = 0x05;
        }
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    pub mod ODD_LINE_PATTERN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const RGB: u32 = 0;
            #[doc = "RBG"]
            pub const RBG: u32 = 0x01;
            #[doc = "GBR"]
            pub const GBR: u32 = 0x02;
            #[doc = "GRB"]
            pub const GRB: u32 = 0x03;
            #[doc = "BRG"]
            pub const BRG: u32 = 0x04;
            #[doc = "BGR"]
            pub const BGR: u32 = 0x05;
        }
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    pub mod BURST_LEN_8 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    pub mod OUTSTANDING_REQS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "REQ_1"]
            pub const REQ_1: u32 = 0;
            #[doc = "REQ_2"]
            pub const REQ_2: u32 = 0x01;
            #[doc = "REQ_4"]
            pub const REQ_4: u32 = 0x02;
            #[doc = "REQ_8"]
            pub const REQ_8: u32 = 0x03;
            #[doc = "REQ_16"]
            pub const REQ_16: u32 = 0x04;
        }
    }
}
#[doc = "LCDIF General Control2 Register"]
pub mod CTRL2_CLR {
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    pub mod EVEN_LINE_PATTERN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const RGB: u32 = 0;
            #[doc = "RBG"]
            pub const RBG: u32 = 0x01;
            #[doc = "GBR"]
            pub const GBR: u32 = 0x02;
            #[doc = "GRB"]
            pub const GRB: u32 = 0x03;
            #[doc = "BRG"]
            pub const BRG: u32 = 0x04;
            #[doc = "BGR"]
            pub const BGR: u32 = 0x05;
        }
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    pub mod ODD_LINE_PATTERN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const RGB: u32 = 0;
            #[doc = "RBG"]
            pub const RBG: u32 = 0x01;
            #[doc = "GBR"]
            pub const GBR: u32 = 0x02;
            #[doc = "GRB"]
            pub const GRB: u32 = 0x03;
            #[doc = "BRG"]
            pub const BRG: u32 = 0x04;
            #[doc = "BGR"]
            pub const BGR: u32 = 0x05;
        }
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    pub mod BURST_LEN_8 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    pub mod OUTSTANDING_REQS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "REQ_1"]
            pub const REQ_1: u32 = 0;
            #[doc = "REQ_2"]
            pub const REQ_2: u32 = 0x01;
            #[doc = "REQ_4"]
            pub const REQ_4: u32 = 0x02;
            #[doc = "REQ_8"]
            pub const REQ_8: u32 = 0x03;
            #[doc = "REQ_16"]
            pub const REQ_16: u32 = 0x04;
        }
    }
}
#[doc = "LCDIF General Control2 Register"]
pub mod CTRL2_TOG {
    #[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    pub mod EVEN_LINE_PATTERN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const RGB: u32 = 0;
            #[doc = "RBG"]
            pub const RBG: u32 = 0x01;
            #[doc = "GBR"]
            pub const GBR: u32 = 0x02;
            #[doc = "GRB"]
            pub const GRB: u32 = 0x03;
            #[doc = "BRG"]
            pub const BRG: u32 = 0x04;
            #[doc = "BGR"]
            pub const BGR: u32 = 0x05;
        }
    }
    #[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    pub mod ODD_LINE_PATTERN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGB"]
            pub const RGB: u32 = 0;
            #[doc = "RBG"]
            pub const RBG: u32 = 0x01;
            #[doc = "GBR"]
            pub const GBR: u32 = 0x02;
            #[doc = "GRB"]
            pub const GRB: u32 = 0x03;
            #[doc = "BRG"]
            pub const BRG: u32 = 0x04;
            #[doc = "BGR"]
            pub const BGR: u32 = 0x05;
        }
    }
    #[doc = "By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    pub mod BURST_LEN_8 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    pub mod OUTSTANDING_REQS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "REQ_1"]
            pub const REQ_1: u32 = 0;
            #[doc = "REQ_2"]
            pub const REQ_2: u32 = 0x01;
            #[doc = "REQ_4"]
            pub const REQ_4: u32 = 0x02;
            #[doc = "REQ_8"]
            pub const REQ_8: u32 = 0x03;
            #[doc = "REQ_16"]
            pub const REQ_16: u32 = 0x04;
        }
    }
}
#[doc = "LCDIF Horizontal and Vertical Valid Data Count Register"]
pub mod TRANSFER_COUNT {
    #[doc = "Total valid data (pixels) in each horizontal line"]
    pub mod H_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of horizontal lines per frame which contain valid data"]
    pub mod V_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCD Interface Current Buffer Address Register"]
pub mod CUR_BUF {
    #[doc = "Address of the current frame being transmitted by LCDIF."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCD Interface Next Buffer Address Register"]
pub mod NEXT_BUF {
    #[doc = "Address of the next frame that will be transmitted by LCDIF."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod VDCTRL0 {
    #[doc = "Number of units for which VSYNC signal is active"]
    pub mod VSYNC_PULSE_WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    pub mod HALF_LINE_MODE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    pub mod HALF_LINE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    pub mod VSYNC_PULSE_WIDTH_UNIT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    pub mod VSYNC_PERIOD_UNIT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    pub mod ENABLE_POL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    pub mod DOTCLK_POL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    pub mod HSYNC_POL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    pub mod VSYNC_POL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    pub mod ENABLE_PRESENT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod VDCTRL0_SET {
    #[doc = "Number of units for which VSYNC signal is active"]
    pub mod VSYNC_PULSE_WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    pub mod HALF_LINE_MODE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    pub mod HALF_LINE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    pub mod VSYNC_PULSE_WIDTH_UNIT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    pub mod VSYNC_PERIOD_UNIT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    pub mod ENABLE_POL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    pub mod DOTCLK_POL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    pub mod HSYNC_POL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    pub mod VSYNC_POL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    pub mod ENABLE_PRESENT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod VDCTRL0_CLR {
    #[doc = "Number of units for which VSYNC signal is active"]
    pub mod VSYNC_PULSE_WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    pub mod HALF_LINE_MODE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    pub mod HALF_LINE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    pub mod VSYNC_PULSE_WIDTH_UNIT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    pub mod VSYNC_PERIOD_UNIT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    pub mod ENABLE_POL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    pub mod DOTCLK_POL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    pub mod HSYNC_POL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    pub mod VSYNC_POL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    pub mod ENABLE_PRESENT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod VDCTRL0_TOG {
    #[doc = "Number of units for which VSYNC signal is active"]
    pub mod VSYNC_PULSE_WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    pub mod HALF_LINE_MODE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    pub mod HALF_LINE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    pub mod VSYNC_PULSE_WIDTH_UNIT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    pub mod VSYNC_PERIOD_UNIT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during valid data transfer on each horizontal line."]
    pub mod ENABLE_POL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    pub mod DOTCLK_POL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    pub mod HSYNC_POL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    pub mod VSYNC_POL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    pub mod ENABLE_PRESENT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register1"]
pub mod VDCTRL1 {
    #[doc = "Total number of units between two positive or two negative edges of the VSYNC signal"]
    pub mod VSYNC_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
pub mod VDCTRL2 {
    #[doc = "Total number of DISPLAY CLOCK (pix_clk) cycles between two positive or two negative edges of the HSYNC signal"]
    pub mod HSYNC_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of DISPLAY CLOCK (pix_clk) cycles for which HSYNC signal is active."]
    pub mod HSYNC_PULSE_WIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
pub mod VDCTRL3 {
    #[doc = "In the VSYNC interface mode, wait for this number of DISPLAY CLOCK (pix_clk) cycles from the falling VSYNC edge (or rising if VSYNC_POL is 1) before starting LCD transactions and is applicable only if WAIT_FOR_VSYNC_EDGE is set"]
    pub mod VERTICAL_WAIT_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the DOTCLK mode, wait for this number of clocks from falling edge (or rising if HSYNC_POL is 1) of HSYNC signal to account for horizontal back porch plus the number of DOTCLKs before the moving picture information begins"]
    pub mod HORIZONTAL_WAIT_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit must be set to 1 in the VSYNC mode of operation, and 0 in the DOTCLK mode of operation."]
    pub mod VSYNC_ONLY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When this bit is set, the LCDIF block will internally mux HSYNC with LCD_D14, DOTCLK with LCD_D13 and ENABLE with LCD_D12, otherwise these signals will go out on separate pins"]
    pub mod MUX_SYNC_SIGNALS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
pub mod VDCTRL4 {
    #[doc = "Total number of DISPLAY CLOCK (pix_clk) cycles on each horizontal line that carry valid data in DOTCLK mode"]
    pub mod DOTCLK_H_VALID_DATA_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this field to 1 if the LCD controller requires that the VSYNC or VSYNC/HSYNC/DOTCLK control signals should be active at least one frame before the data transfers actually start and remain active at least one frame after the data transfers end"]
    pub mod SYNC_SIGNALS_ON {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield selects the amount of time by which the DOTCLK signal should be delayed before coming out of the LCD_DOTCK pin"]
    pub mod DOTCLK_DLY_SEL {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bus Master Error Status Register"]
pub mod BM_ERROR_STAT {
    #[doc = "Virtual address at which bus master error occurred."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CRC Status Register"]
pub mod CRC_STAT {
    #[doc = "Calculated CRC value."]
    pub mod CRC_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCD Interface Status Register"]
pub mod STAT {
    #[doc = "Read only view of the current count in Latency buffer (LFIFO)."]
    pub mod LFIFO_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read only view of the signals that indicates LCD TXFIFO is empty."]
    pub mod TXFIFO_EMPTY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read only view of the signals that indicates LCD TXFIFO is full."]
    pub mod TXFIFO_FULL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read only view of the signals that indicates LCD LFIFO is empty."]
    pub mod LFIFO_EMPTY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read only view of the signals that indicates LCD LFIFO is full."]
    pub mod LFIFO_FULL {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reflects the current state of the DMA Request line for the LCDIF"]
    pub mod DMA_REQ {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: LCDIF not present on this product 1: LCDIF is present."]
    pub mod PRESENT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Threshold Register"]
pub mod THRES {
    #[doc = "This value should be set to a value of pixels from 0 to 511"]
    pub mod PANIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value should be set to a value of pixels, from 0 to 511"]
    pub mod FASTCLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod PIGEONCTRL0 {
    #[doc = "Period of line counter during FD phase"]
    pub mod FD_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Period of pclk counter during LD phase"]
    pub mod LD_PERIOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod PIGEONCTRL0_SET {
    #[doc = "Period of line counter during FD phase"]
    pub mod FD_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Period of pclk counter during LD phase"]
    pub mod LD_PERIOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod PIGEONCTRL0_CLR {
    #[doc = "Period of line counter during FD phase"]
    pub mod FD_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Period of pclk counter during LD phase"]
    pub mod LD_PERIOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod PIGEONCTRL0_TOG {
    #[doc = "Period of line counter during FD phase"]
    pub mod FD_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Period of pclk counter during LD phase"]
    pub mod LD_PERIOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod PIGEONCTRL1 {
    #[doc = "Period of frame counter"]
    pub mod FRAME_CNT_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Max cycles of frame counter"]
    pub mod FRAME_CNT_CYCLES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod PIGEONCTRL1_SET {
    #[doc = "Period of frame counter"]
    pub mod FRAME_CNT_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Max cycles of frame counter"]
    pub mod FRAME_CNT_CYCLES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod PIGEONCTRL1_CLR {
    #[doc = "Period of frame counter"]
    pub mod FRAME_CNT_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Max cycles of frame counter"]
    pub mod FRAME_CNT_CYCLES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod PIGEONCTRL1_TOG {
    #[doc = "Period of frame counter"]
    pub mod FRAME_CNT_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Max cycles of frame counter"]
    pub mod FRAME_CNT_CYCLES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod PIGEONCTRL2 {
    #[doc = "Pigeon mode data enable"]
    pub mod PIGEON_DATA_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    pub mod PIGEON_CLK_GATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod PIGEONCTRL2_SET {
    #[doc = "Pigeon mode data enable"]
    pub mod PIGEON_DATA_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    pub mod PIGEON_CLK_GATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod PIGEONCTRL2_CLR {
    #[doc = "Pigeon mode data enable"]
    pub mod PIGEON_DATA_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    pub mod PIGEON_CLK_GATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod PIGEONCTRL2_TOG {
    #[doc = "Pigeon mode data enable"]
    pub mod PIGEON_DATA_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pigeon mode dot clock gate enable"]
    pub mod PIGEON_CLK_GATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_0_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_0_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_0_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_1_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_1_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_1_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_2_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_2_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_2_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_3_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_3_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_3_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_4_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_4_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_4_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_5_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_5_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_5_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_6_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_6_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_6_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_7_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_7_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_7_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_8_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_8_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_8_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_9_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_9_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_9_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_10_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_10_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_10_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_11_0 {
    #[doc = "Enable pigeon Mode on this signal"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of signal output"]
    pub mod POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Signal (Active high)"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Inverted signal (Active low)"]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
    }
    #[doc = "Event to incrment local counter"]
    pub mod INC_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk"]
            pub const PCLK: u32 = 0;
            #[doc = "Line start pulse"]
            pub const LINE: u32 = 0x01;
            #[doc = "Frame start pulse"]
            pub const FRAME: u32 = 0x02;
            #[doc = "Use another signal as tick event"]
            pub const SIG_ANOTHER: u32 = 0x03;
        }
    }
    #[doc = "offset on pclk unit"]
    pub mod OFFSET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select global counters as mask condition, use together with MASK_CNT"]
    pub mod MASK_CNT_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "pclk counter within one hscan state"]
            pub const HSTATE_CNT: u32 = 0;
            #[doc = "pclk cycle within one hscan state"]
            pub const HSTATE_CYCLE: u32 = 0x01;
            #[doc = "line counter within one vscan state"]
            pub const VSTATE_CNT: u32 = 0x02;
            #[doc = "line cycle within one vscan state"]
            pub const VSTATE_CYCLE: u32 = 0x03;
            #[doc = "frame counter"]
            pub const FRAME_CNT: u32 = 0x04;
            #[doc = "frame cycle"]
            pub const FRAME_CYCLE: u32 = 0x05;
            #[doc = "horizontal counter (pclk counter within one line )"]
            pub const HCNT: u32 = 0x06;
            #[doc = "vertical counter (line counter within one frame)"]
            pub const VCNT: u32 = 0x07;
        }
    }
    #[doc = "When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    pub mod MASK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    pub mod STATE_MASK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRAME SYNC"]
            pub const FS: u32 = 0x01;
            #[doc = "FRAME BEGIN"]
            pub const FB: u32 = 0x02;
            #[doc = "FRAME DATA"]
            pub const FD: u32 = 0x04;
            #[doc = "FRAME END"]
            pub const FE: u32 = 0x08;
            #[doc = "LINE SYNC"]
            pub const LS: u32 = 0x10;
            #[doc = "LINE BEGIN"]
            pub const LB: u32 = 0x20;
            #[doc = "LINE DATA"]
            pub const LD: u32 = 0x40;
            #[doc = "LINE END"]
            pub const LE: u32 = 0x80;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_11_1 {
    #[doc = "Assert signal output when counter match this value"]
    pub mod SET_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start as active"]
            pub const START_ACTIVE: u32 = 0;
        }
    }
    #[doc = "Deassert signal output when counter match this value"]
    pub mod CLR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod PIGEON_11_2 {
    #[doc = "Logic operation with another signal: DIS/AND/OR/COND"]
    pub mod SIG_LOGIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No logic operation"]
            pub const DIS: u32 = 0;
            #[doc = "sigout = sig_another AND this_sig"]
            pub const AND: u32 = 0x01;
            #[doc = "sigout = sig_another OR this_sig"]
            pub const OR: u32 = 0x02;
            #[doc = "mask = sig_another AND other_masks"]
            pub const MASK: u32 = 0x03;
        }
    }
    #[doc = "Select another signal for logic operation or as mask or counter tick event"]
    pub mod SIG_ANOTHER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep active until mask off"]
            pub const CLEAR_USING_MASK: u32 = 0;
        }
    }
}
#[doc = "Lookup Table Data Register."]
pub mod LUT_CTRL {
    #[doc = "Setting this bit will bypass the LUT memory resource completely"]
    pub mod LUT_BYPASS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lookup Table Control Register."]
pub mod LUT0_ADDR {
    #[doc = "LUT indexed address pointer"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lookup Table Data Register."]
pub mod LUT0_DATA {
    #[doc = "Writing this field will load 4 bytes, aligned to four byte boundaries, of data indexed by the ADDR field of the REG_LUT_CTRL register"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lookup Table Control Register."]
pub mod LUT1_ADDR {
    #[doc = "LUT indexed address pointer"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lookup Table Data Register."]
pub mod LUT1_DATA {
    #[doc = "Writing this field will load 4 bytes, aligned to four byte boundaries, of data indexed by the ADDR field of the REG_LUT_CTRL register"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
