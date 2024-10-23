#[doc = "FLEXIO"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "FLEXIO Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Pin State"]
    pub PIN: crate::RORegister<u32>,
    #[doc = "Shifter Status"]
    pub SHIFTSTAT: crate::RWRegister<u32>,
    #[doc = "Shifter Error"]
    pub SHIFTERR: crate::RWRegister<u32>,
    #[doc = "Timer Status"]
    pub TIMSTAT: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Shifter Status Interrupt Enable"]
    pub SHIFTSIEN: crate::RWRegister<u32>,
    #[doc = "Shifter Error Interrupt Enable"]
    pub SHIFTEIEN: crate::RWRegister<u32>,
    #[doc = "Timer Interrupt Enable"]
    pub TIMIEN: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Shifter Status DMA Enable"]
    pub SHIFTSDEN: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Timer Status DMA Enable"]
    pub TIMERSDEN: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Shifter State"]
    pub SHIFTSTATE: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Trigger Status"]
    pub TRGSTAT: crate::RWRegister<u32>,
    #[doc = "External Trigger Interrupt Enable"]
    pub TRIGIEN: crate::RWRegister<u32>,
    #[doc = "Pin Status"]
    pub PINSTAT: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Enable"]
    pub PINIEN: crate::RWRegister<u32>,
    #[doc = "Pin Rising Edge Enable"]
    pub PINREN: crate::RWRegister<u32>,
    #[doc = "Pin Falling Edge Enable"]
    pub PINFEN: crate::RWRegister<u32>,
    #[doc = "Pin Output Data"]
    pub PINOUTD: crate::RWRegister<u32>,
    #[doc = "Pin Output Enable"]
    pub PINOUTE: crate::RWRegister<u32>,
    #[doc = "Pin Output Disable"]
    pub PINOUTDIS: crate::RWRegister<u32>,
    #[doc = "Pin Output Clear"]
    pub PINOUTCLR: crate::RWRegister<u32>,
    #[doc = "Pin Output Set"]
    pub PINOUTSET: crate::RWRegister<u32>,
    #[doc = "Pin Output Toggle"]
    pub PINOUTTOG: crate::RWRegister<u32>,
    _reserved5: [u8; 0x08],
    #[doc = "Shifter Control N"]
    pub SHIFTCTL: [crate::RWRegister<u32>; 8usize],
    _reserved6: [u8; 0x60],
    #[doc = "Shifter Configuration N"]
    pub SHIFTCFG: [crate::RWRegister<u32>; 8usize],
    _reserved7: [u8; 0xe0],
    #[doc = "Shifter Buffer N"]
    pub SHIFTBUF: [crate::RWRegister<u32>; 8usize],
    _reserved8: [u8; 0x60],
    #[doc = "Shifter Buffer N Bit Swapped"]
    pub SHIFTBUFBIS: [crate::RWRegister<u32>; 8usize],
    _reserved9: [u8; 0x60],
    #[doc = "Shifter Buffer N Byte Swapped"]
    pub SHIFTBUFBYS: [crate::RWRegister<u32>; 8usize],
    _reserved10: [u8; 0x60],
    #[doc = "Shifter Buffer N Bit Byte Swapped"]
    pub SHIFTBUFBBS: [crate::RWRegister<u32>; 8usize],
    _reserved11: [u8; 0x60],
    #[doc = "Timer Control N"]
    pub TIMCTL: [crate::RWRegister<u32>; 8usize],
    _reserved12: [u8; 0x60],
    #[doc = "Timer Configuration N"]
    pub TIMCFG: [crate::RWRegister<u32>; 8usize],
    _reserved13: [u8; 0x60],
    #[doc = "Timer Compare N"]
    pub TIMCMP: [crate::RWRegister<u32>; 8usize],
    _reserved14: [u8; 0x0160],
    #[doc = "Shifter Buffer N Nibble Byte Swapped"]
    pub SHIFTBUFNBS: [crate::RWRegister<u32>; 8usize],
    _reserved15: [u8; 0x60],
    #[doc = "Shifter Buffer N Halfword Swapped"]
    pub SHIFTBUFHWS: [crate::RWRegister<u32>; 8usize],
    _reserved16: [u8; 0x60],
    #[doc = "Shifter Buffer N Nibble Swapped"]
    pub SHIFTBUFNIS: [crate::RWRegister<u32>; 8usize],
    _reserved17: [u8; 0x60],
    #[doc = "Shifter Buffer N Odd Even Swapped"]
    pub SHIFTBUFOES: [crate::RWRegister<u32>; 8usize],
    _reserved18: [u8; 0x60],
    #[doc = "Shifter Buffer N Even Odd Swapped"]
    pub SHIFTBUFEOS: [crate::RWRegister<u32>; 8usize],
    _reserved19: [u8; 0x60],
    #[doc = "Shifter Buffer N Halfword Byte Swapped"]
    pub SHIFTBUFHBS: [crate::RWRegister<u32>; 8usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard features implemented."]
            pub const STANDARD: u32 = 0;
            #[doc = "Supports state, logic and parallel modes."]
            pub const STATE_LOGIC_PARALLEL: u32 = 0x01;
            #[doc = "Supports pin control registers."]
            pub const PINCTRL: u32 = 0x02;
            #[doc = "Supports state, logic and parallel modes; plus pin control registers."]
            pub const STATE_LOGIC_PARALLEL_PINCTRL: u32 = 0x03;
        }
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "Shifter Number"]
    pub mod SHIFTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Number"]
    pub mod TIMER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pin Number"]
    pub mod PIN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Number"]
    pub mod TRIGGER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FLEXIO Control"]
pub mod CTRL {
    #[doc = "FlexIO Enable"]
    pub mod FLEXEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexIO module is disabled."]
            pub const FLEXEN_0: u32 = 0;
            #[doc = "FlexIO module is enabled."]
            pub const FLEXEN_1: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod SWRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software reset is disabled"]
            pub const SWRST_0: u32 = 0;
            #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
            pub const SWRST_1: u32 = 0x01;
        }
    }
    #[doc = "Fast Access"]
    pub mod FASTACC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configures for normal register accesses to FlexIO"]
            pub const FASTACC_0: u32 = 0;
            #[doc = "Configures for fast register accesses to FlexIO"]
            pub const FASTACC_1: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexIO is disabled in debug modes."]
            pub const DBGE_0: u32 = 0;
            #[doc = "FlexIO is enabled in debug modes"]
            pub const DBGE_1: u32 = 0x01;
        }
    }
    #[doc = "Doze Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexIO enabled in Doze modes."]
            pub const DOZEN_0: u32 = 0;
            #[doc = "FlexIO disabled in Doze modes."]
            pub const DOZEN_1: u32 = 0x01;
        }
    }
}
#[doc = "Pin State"]
pub mod PIN {
    #[doc = "Pin Data Input"]
    pub mod PDI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Status"]
pub mod SHIFTSTAT {
    #[doc = "Shifter Status Flag"]
    pub mod SSF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Error"]
pub mod SHIFTERR {
    #[doc = "Shifter Error Flags"]
    pub mod SEF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Status"]
pub mod TIMSTAT {
    #[doc = "Timer Status Flags"]
    pub mod TSF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Status Interrupt Enable"]
pub mod SHIFTSIEN {
    #[doc = "Shifter Status Interrupt Enable"]
    pub mod SSIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Error Interrupt Enable"]
pub mod SHIFTEIEN {
    #[doc = "Shifter Error Interrupt Enable"]
    pub mod SEIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Interrupt Enable"]
pub mod TIMIEN {
    #[doc = "Timer Status Interrupt Enable"]
    pub mod TEIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Status DMA Enable"]
pub mod SHIFTSDEN {
    #[doc = "Shifter Status DMA Enable"]
    pub mod SSDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Status DMA Enable"]
pub mod TIMERSDEN {
    #[doc = "Timer Status DMA Enable"]
    pub mod TSDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter State"]
pub mod SHIFTSTATE {
    #[doc = "Current State Pointer"]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger Status"]
pub mod TRGSTAT {
    #[doc = "External Trigger Status Flags"]
    pub mod ETSF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External Trigger Interrupt Enable"]
pub mod TRIGIEN {
    #[doc = "External Trigger Interrupt Enable"]
    pub mod TRIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Status"]
pub mod PINSTAT {
    #[doc = "Pin Status Flags"]
    pub mod PSF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Interrupt Enable"]
pub mod PINIEN {
    #[doc = "Pin Status Interrupt Enable"]
    pub mod PSIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Rising Edge Enable"]
pub mod PINREN {
    #[doc = "Pin Rising Edge"]
    pub mod PRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Falling Edge Enable"]
pub mod PINFEN {
    #[doc = "Pin Falling Edge"]
    pub mod PFE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Data"]
pub mod PINOUTD {
    #[doc = "Output Data"]
    pub mod OUTD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Enable"]
pub mod PINOUTE {
    #[doc = "Output Enable"]
    pub mod OUTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Disable"]
pub mod PINOUTDIS {
    #[doc = "Output Disable"]
    pub mod OUTDIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Clear"]
pub mod PINOUTCLR {
    #[doc = "Output Clear"]
    pub mod OUTCLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Set"]
pub mod PINOUTSET {
    #[doc = "Output Set"]
    pub mod OUTSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Toggle"]
pub mod PINOUTTOG {
    #[doc = "Output Toggle"]
    pub mod OUTTOG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Control N"]
pub mod SHIFTCTL {
    #[doc = "Shifter Mode"]
    pub mod SMOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const SMOD_0: u32 = 0;
            #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
            pub const SMOD_1: u32 = 0x01;
            #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
            pub const SMOD_2: u32 = 0x02;
            #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
            pub const SMOD_4: u32 = 0x04;
            #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
            pub const SMOD_5: u32 = 0x05;
            #[doc = "State mode. SHIFTBUF contents are used for storing programmable state attributes."]
            pub const SMOD_6: u32 = 0x06;
            #[doc = "Logic mode. SHIFTBUF contents are used for implementing programmable logic look up table."]
            pub const SMOD_7: u32 = 0x07;
        }
    }
    #[doc = "Shifter Pin Polarity"]
    pub mod PINPOL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is active high"]
            pub const PINPOL_0: u32 = 0;
            #[doc = "Pin is active low"]
            pub const PINPOL_1: u32 = 0x01;
        }
    }
    #[doc = "Shifter Pin Select"]
    pub mod PINSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shifter Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shifter pin output disabled"]
            pub const PINCFG_0: u32 = 0;
            #[doc = "Shifter pin open drain or bidirectional output enable"]
            pub const PINCFG_1: u32 = 0x01;
            #[doc = "Shifter pin bidirectional output data"]
            pub const PINCFG_2: u32 = 0x02;
            #[doc = "Shifter pin output"]
            pub const PINCFG_3: u32 = 0x03;
        }
    }
    #[doc = "Timer Polarity"]
    pub mod TIMPOL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shift on posedge of Shift clock"]
            pub const TIMPOL_0: u32 = 0;
            #[doc = "Shift on negedge of Shift clock"]
            pub const TIMPOL_1: u32 = 0x01;
        }
    }
    #[doc = "Timer Select"]
    pub mod TIMSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Configuration N"]
pub mod SHIFTCFG {
    #[doc = "Shifter Start Bit"]
    pub mod SSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
            pub const SSTART_0: u32 = 0;
            #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
            pub const SSTART_1: u32 = 0x01;
            #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
            pub const SSTART_2: u32 = 0x02;
            #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
            pub const SSTART_3: u32 = 0x03;
        }
    }
    #[doc = "Shifter Stop bit"]
    pub mod SSTOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop bit disabled for transmitter/receiver/match store"]
            pub const VALUE00: u32 = 0;
            #[doc = "Stop bit disabled for transmitter/receiver/match store. When timer is in stop condition, receiver/match store stores receive data on the configured shift edge."]
            pub const VALUE01: u32 = 0x01;
            #[doc = "Transmitter outputs stop bit value 0 on store. If stop bit is not 0, receiver/match store sets error flag. When timer is in stop condition, receiver/match stores also store receive data on the configured shift edge."]
            pub const VALUE10: u32 = 0x02;
            #[doc = "Transmitter outputs stop bit value 1 on store. If stop bit is not 1, receiver/match store sets error flag. When timer is in stop condition, receiver/match store also stores receive data on the configured shift edge."]
            pub const VALUE11: u32 = 0x03;
        }
    }
    #[doc = "Input Source"]
    pub mod INSRC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin"]
            pub const INSRC_0: u32 = 0;
            #[doc = "Shifter N+1 Output"]
            pub const INSRC_1: u32 = 0x01;
        }
    }
    #[doc = "Late Store"]
    pub mod LATST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shift register stores the pre-shift register state."]
            pub const PRESHIFT: u32 = 0;
            #[doc = "Shift register stores the post-shift register state."]
            pub const POSTSHIFT: u32 = 0x01;
        }
    }
    #[doc = "Shifter Size"]
    pub mod SSIZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shift register is 32-bit."]
            pub const WIDTH32: u32 = 0;
            #[doc = "Shift register is 24-bit."]
            pub const WIDTH24: u32 = 0x01;
        }
    }
    #[doc = "Parallel Width"]
    pub mod PWIDTH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N"]
pub mod SHIFTBUF {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Bit Swapped"]
pub mod SHIFTBUFBIS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFBIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Byte Swapped"]
pub mod SHIFTBUFBYS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFBYS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Bit Byte Swapped"]
pub mod SHIFTBUFBBS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFBBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Control N"]
pub mod TIMCTL {
    #[doc = "Timer Mode"]
    pub mod TIMOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer Disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Dual 8-bit counters baud mode."]
            pub const DUAL8BIT_BAUD: u32 = 0x01;
            #[doc = "Dual 8-bit counters PWM high mode."]
            pub const DUAL8BIT_PWM_H: u32 = 0x02;
            #[doc = "Single 16-bit counter mode."]
            pub const SINGLE16BIT: u32 = 0x03;
            #[doc = "Single 16-bit counter disable mode."]
            pub const SINGLE16BIT_DISABLE: u32 = 0x04;
            #[doc = "Dual 8-bit counters word mode."]
            pub const DUAL8BIT_WORD: u32 = 0x05;
            #[doc = "Dual 8-bit counters PWM low mode."]
            pub const DUAL8BIT_PWM_L: u32 = 0x06;
            #[doc = "Single 16-bit input capture mode."]
            pub const SINGLE16BIT_IN_CAPTURE: u32 = 0x07;
        }
    }
    #[doc = "Timer One Time Operation"]
    pub mod ONETIM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The timer enable event is generated as normal."]
            pub const NOT_BLOCKED: u32 = 0;
            #[doc = "The timer enable event is blocked unless timer status flag is clear."]
            pub const BLOCKED: u32 = 0x01;
        }
    }
    #[doc = "Timer Pin Input Select"]
    pub mod PININS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer pin input and output are selected by PINSEL."]
            pub const PINSEL: u32 = 0;
            #[doc = "Timer pin input is selected by PINSEL+1, timer pin output remains selected by PINSEL."]
            pub const PINSELPLUS1: u32 = 0x01;
        }
    }
    #[doc = "Timer Pin Polarity"]
    pub mod PINPOL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is active high"]
            pub const PINPOL_0: u32 = 0;
            #[doc = "Pin is active low"]
            pub const PINPOL_1: u32 = 0x01;
        }
    }
    #[doc = "Timer Pin Select"]
    pub mod PINSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer pin output disabled"]
            pub const PINCFG_0: u32 = 0;
            #[doc = "Timer pin open drain or bidirectional output enable"]
            pub const PINCFG_1: u32 = 0x01;
            #[doc = "Timer pin bidirectional output data"]
            pub const PINCFG_2: u32 = 0x02;
            #[doc = "Timer pin output"]
            pub const PINCFG_3: u32 = 0x03;
        }
    }
    #[doc = "Trigger Source"]
    pub mod TRGSRC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External trigger selected"]
            pub const TRGSRC_0: u32 = 0;
            #[doc = "Internal trigger selected"]
            pub const TRGSRC_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger Polarity"]
    pub mod TRGPOL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger active high"]
            pub const TRGPOL_0: u32 = 0;
            #[doc = "Trigger active low"]
            pub const TRGPOL_1: u32 = 0x01;
        }
    }
    #[doc = "Trigger Select"]
    pub mod TRGSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Configuration N"]
pub mod TIMCFG {
    #[doc = "Timer Start Bit"]
    pub mod TSTART {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start bit disabled"]
            pub const TSTART_0: u32 = 0;
            #[doc = "Start bit enabled"]
            pub const TSTART_1: u32 = 0x01;
        }
    }
    #[doc = "Timer Stop Bit"]
    pub mod TSTOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop bit disabled"]
            pub const TSTOP_0: u32 = 0;
            #[doc = "Stop bit is enabled on timer compare"]
            pub const TSTOP_1: u32 = 0x01;
            #[doc = "Stop bit is enabled on timer disable"]
            pub const TSTOP_2: u32 = 0x02;
            #[doc = "Stop bit is enabled on timer compare and timer disable"]
            pub const TSTOP_3: u32 = 0x03;
        }
    }
    #[doc = "Timer Enable"]
    pub mod TIMENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer always enabled"]
            pub const TIMENA_0: u32 = 0;
            #[doc = "Timer enabled on Timer N-1 enable"]
            pub const TIMENA_1: u32 = 0x01;
            #[doc = "Timer enabled on Trigger high"]
            pub const TIMENA_2: u32 = 0x02;
            #[doc = "Timer enabled on Trigger high and Pin high"]
            pub const TIMENA_3: u32 = 0x03;
            #[doc = "Timer enabled on Pin rising edge"]
            pub const TIMENA_4: u32 = 0x04;
            #[doc = "Timer enabled on Pin rising edge and Trigger high"]
            pub const TIMENA_5: u32 = 0x05;
            #[doc = "Timer enabled on Trigger rising edge"]
            pub const TIMENA_6: u32 = 0x06;
            #[doc = "Timer enabled on Trigger rising or falling edge"]
            pub const TIMENA_7: u32 = 0x07;
        }
    }
    #[doc = "Timer Disable"]
    pub mod TIMDIS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer never disabled"]
            pub const TIMDIS_0: u32 = 0;
            #[doc = "Timer disabled on Timer N-1 disable"]
            pub const TIMDIS_1: u32 = 0x01;
            #[doc = "Timer disabled on Timer compare (upper 8-bits match and decrement)"]
            pub const TIMDIS_2: u32 = 0x02;
            #[doc = "Timer disabled on Timer compare (upper 8-bits match and decrement) and Trigger Low"]
            pub const TIMDIS_3: u32 = 0x03;
            #[doc = "Timer disabled on Pin rising or falling edge"]
            pub const TIMDIS_4: u32 = 0x04;
            #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
            pub const TIMDIS_5: u32 = 0x05;
            #[doc = "Timer disabled on Trigger falling edge"]
            pub const TIMDIS_6: u32 = 0x06;
        }
    }
    #[doc = "Timer Reset"]
    pub mod TIMRST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer never reset"]
            pub const NEVER: u32 = 0;
            #[doc = "Timer reset on Timer Output high."]
            pub const TMR_OUT_HI: u32 = 0x01;
            #[doc = "Timer reset on Timer Pin equal to Timer Output"]
            pub const PIN_EQ_TMR_OUT: u32 = 0x02;
            #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
            pub const TRIG_EQ_TMR_OUT: u32 = 0x03;
            #[doc = "Timer reset on Timer Pin rising edge"]
            pub const PIN_RISE_EDGE: u32 = 0x04;
            #[doc = "Timer reset on Trigger rising edge"]
            pub const TRIG_RISE_EDGE: u32 = 0x06;
            #[doc = "Timer reset on Trigger rising or falling edge"]
            pub const TRIG_EDGE: u32 = 0x07;
        }
    }
    #[doc = "Timer Decrement"]
    pub mod TIMDEC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
            pub const FLEXIO_CLK_SHIFTCLK_TMR_OUT: u32 = 0;
            #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
            pub const TRIG_EDGE_SHIFTCLK_TMR_OUT: u32 = 0x01;
            #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
            pub const PIN_EDGE_SHIFTCLK_TMR_OUT: u32 = 0x02;
            #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
            pub const TRIG_EDGE_SHIFTCLK_TRIG_IN: u32 = 0x03;
            #[doc = "Decrement counter on FlexIO clock divided by 16, Shift clock equals Timer output."]
            pub const FLEXIO_CLK_DIV16_SHIFTCLK_TMR_OUT: u32 = 0x04;
            #[doc = "Decrement counter on FlexIO clock divided by 256, Shift clock equals Timer output."]
            pub const FLEXIO_CLK_DIV256_SHIFTCLK_TMR_OUT: u32 = 0x05;
            #[doc = "Decrement counter on Pin input (rising edge), Shift clock equals Pin input."]
            pub const PIN_RISE_SHIFTCLK_PIN_IN: u32 = 0x06;
            #[doc = "Decrement counter on Trigger input (rising edge), Shift clock equals Trigger input."]
            pub const TRIG_RISE_SHIFTCLK_TRIG_IN: u32 = 0x07;
        }
    }
    #[doc = "Timer Output"]
    pub mod TIMOUT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
            pub const TIMOUT_0: u32 = 0;
            #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
            pub const TIMOUT_1: u32 = 0x01;
            #[doc = "Timer output is logic one when enabled and on timer reset"]
            pub const TIMOUT_2: u32 = 0x02;
            #[doc = "Timer output is logic zero when enabled and on timer reset"]
            pub const TIMOUT_3: u32 = 0x03;
        }
    }
}
#[doc = "Timer Compare N"]
pub mod TIMCMP {
    #[doc = "Timer Compare Value"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Nibble Byte Swapped"]
pub mod SHIFTBUFNBS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFNBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Halfword Swapped"]
pub mod SHIFTBUFHWS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFHWS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Nibble Swapped"]
pub mod SHIFTBUFNIS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFNIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Odd Even Swapped"]
pub mod SHIFTBUFOES {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFOES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Even Odd Swapped"]
pub mod SHIFTBUFEOS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFEOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Halfword Byte Swapped"]
pub mod SHIFTBUFHBS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFHBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
