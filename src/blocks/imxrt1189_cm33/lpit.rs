#[doc = "LPIT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "Module Control"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Module Status"]
    pub MSR: crate::RWRegister<u32>,
    #[doc = "Module Interrupt Enable"]
    pub MIER: crate::RWRegister<u32>,
    #[doc = "Set Timer Enable"]
    pub SETTEN: crate::RWRegister<u32>,
    #[doc = "Clear Timer Enable"]
    pub CLRTEN: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Array of registers: TVAL, CVAL, TCTRL"]
    pub CHANNEL: [channel::RegisterBlock; 4usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
    #[doc = "Number of Timer Channels"]
    pub mod CHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of External Trigger Inputs"]
    pub mod EXT_TRIG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Module Control"]
pub mod MCR {
    #[doc = "Module Clock Enable"]
    pub mod M_CEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod SW_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reset"]
            pub const NOT_RESET: u32 = 0;
            #[doc = "Resets"]
            pub const RESET: u32 = 0x01;
        }
    }
    #[doc = "DOZE Mode Enable"]
    pub mod DOZE_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stops timer channels"]
            pub const DISABLE: u32 = 0;
            #[doc = "Allows timer channels to continue running"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Debug Mode Enable"]
    pub mod DBG_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stops timer channels"]
            pub const DISABLE: u32 = 0;
            #[doc = "Allows timer channels to continue running"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Module Status"]
pub mod MSR {
    #[doc = "Channel 0 Timer Interrupt Flag"]
    pub mod TIF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not timed out"]
            pub const TIMEOUT_NO: u32 = 0;
            #[doc = "Timed out"]
            pub const TIMEOUT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 1 Timer Interrupt Flag"]
    pub mod TIF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not timed out"]
            pub const TIMEOUT_NO: u32 = 0;
            #[doc = "Timed out"]
            pub const TIMEOUT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 2 Timer Interrupt Flag"]
    pub mod TIF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not timed out"]
            pub const TIMEOUT_NO: u32 = 0;
            #[doc = "Timed out"]
            pub const TIMEOUT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 3 Timer Interrupt Flag"]
    pub mod TIF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not timed out"]
            pub const TIMEOUT_NO: u32 = 0;
            #[doc = "Timed out"]
            pub const TIMEOUT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Module Interrupt Enable"]
pub mod MIER {
    #[doc = "Channel 0 Timer Interrupt Enable"]
    pub mod TIE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Channel 1 Timer Interrupt Enable"]
    pub mod TIE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Channel 2 Timer Interrupt Enable"]
    pub mod TIE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Channel 3 Timer Interrupt Enable"]
    pub mod TIE3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Set Timer Enable"]
pub mod SETTEN {
    #[doc = "Set Timer 0 Enable"]
    pub mod SET_T_EN_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables timer channel 0"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Set Timer 1 Enable"]
    pub mod SET_T_EN_1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables timer channel 1"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Set Timer 2 Enable"]
    pub mod SET_T_EN_2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables timer channel 2"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Set Timer 3 Enable"]
    pub mod SET_T_EN_3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables timer channel 3"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Clear Timer Enable"]
pub mod CLRTEN {
    #[doc = "Clear Timer 0 Enable"]
    pub mod CLR_T_EN_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const DISABLE: u32 = 0;
            #[doc = "Turns TCTRL0\\[T_EN\\] = 0 for timer channel 0"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Clear Timer 1 Enable"]
    pub mod CLR_T_EN_1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const DISABLE: u32 = 0;
            #[doc = "Turns TCTRL1\\[T_EN\\] = 0 for timer channel 1"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Clear Timer 2 Enable"]
    pub mod CLR_T_EN_2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const DISABLE: u32 = 0;
            #[doc = "Turns TCTRL2\\[T_EN\\] = 0 for timer channel 2"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Clear Timer 3 Enable"]
    pub mod CLR_T_EN_3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const DISABLE: u32 = 0;
            #[doc = "Turns TCTRL3\\[T_EN\\] = 0 for timer channel 3"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
pub mod channel {
    #[doc = "Array of registers: TVAL, CVAL, TCTRL"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Timer Value"]
        pub TVAL: crate::RWRegister<u32>,
        #[doc = "Current Timer Value"]
        pub CVAL: crate::RORegister<u32>,
        #[doc = "Timer Control"]
        pub TCTRL: crate::RWRegister<u32>,
        _reserved0: [u8; 0x04],
    }
    #[doc = "Timer Value"]
    pub mod TVAL {
        #[doc = "Timer Value"]
        pub mod TMR_VAL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Invalid load value in Compare mode"]
                pub const INVALID_COMPARE_MODE_VALUE_0: u32 = 0;
                #[doc = "Invalid load value in Compare mode"]
                pub const INVALID_COMPARE_MODE_VALUE_1: u32 = 0x01;
                #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
                pub const VALUE_2: u32 = 0x02;
                #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
                pub const VALUE_3: u32 = 0x03;
                #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
                pub const VALUE_4: u32 = 0x04;
                #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
                pub const VALUE_5: u32 = 0x05;
                #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
                pub const VALUE_6: u32 = 0x06;
                #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
                pub const VALUE_7: u32 = 0x07;
                #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
                pub const VALUE_8: u32 = 0x08;
                #[doc = "In Compare mode: the value to be loaded; in Capture mode, the value of the timer"]
                pub const VALUE_9: u32 = 0x09;
            }
        }
    }
    #[doc = "Current Timer Value"]
    pub mod CVAL {
        #[doc = "Current Timer Value"]
        pub mod TMR_CUR_VAL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Timer Control"]
    pub mod TCTRL {
        #[doc = "Timer Enable"]
        pub mod T_EN {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Chain Channel"]
        pub mod CHAIN {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enabled"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Timer Operation Mode"]
        pub mod MODE {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "32-bit periodic counter"]
                pub const CTR_32BIT: u32 = 0;
                #[doc = "Dual 16-bit periodic counter"]
                pub const CTR_DUAL_16BIT: u32 = 0x01;
                #[doc = "32-bit trigger accumulator"]
                pub const TRIG_ACCUM_32BIT: u32 = 0x02;
                #[doc = "32-bit trigger input capture"]
                pub const TRIG_INPUT_32BIT: u32 = 0x03;
            }
        }
        #[doc = "Timer Start on Trigger"]
        pub mod TSOT {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Immediately"]
                pub const IMMEDIATELY: u32 = 0;
                #[doc = "When a rising edge is detected"]
                pub const RISING_EDGE: u32 = 0x01;
            }
        }
        #[doc = "Timer Stop on Interrupt"]
        pub mod TSOI {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Does not stop"]
                pub const DISABLE: u32 = 0;
                #[doc = "Stops"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Timer Reload on Trigger"]
        pub mod TROT {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Does not reload"]
                pub const DISABLE: u32 = 0;
                #[doc = "Reloads"]
                pub const ENABLE: u32 = 0x01;
            }
        }
        #[doc = "Trigger Source"]
        pub mod TRG_SRC {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "External"]
                pub const EXT_TRIG: u32 = 0;
                #[doc = "Internal"]
                pub const INT_TRIG: u32 = 0x01;
            }
        }
        #[doc = "Trigger Select"]
        pub mod TRG_SEL {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Timer channel 0-3 trigger source"]
                pub const TRIG_SOURCE_0: u32 = 0;
                #[doc = "Timer channel 0-3 trigger source"]
                pub const TRIG_SOURCE_1: u32 = 0x01;
                #[doc = "Timer channel 0-3 trigger source"]
                pub const TRIG_SOURCE_2: u32 = 0x02;
                #[doc = "Timer channel 0-3 trigger source"]
                pub const TRIG_SOURCE_3: u32 = 0x03;
            }
        }
    }
}
