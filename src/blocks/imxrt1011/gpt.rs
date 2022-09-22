#[doc = "GPT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPT Control Register"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "GPT Prescaler Register"]
    pub PR: crate::RWRegister<u32>,
    #[doc = "GPT Status Register"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "GPT Interrupt Register"]
    pub IR: crate::RWRegister<u32>,
    #[doc = "GPT Output Compare Register 1"]
    pub OCR: [crate::RWRegister<u32>; 3usize],
    #[doc = "GPT Input Capture Register 1"]
    pub ICR: [crate::RORegister<u32>; 2usize],
    #[doc = "GPT Counter Register"]
    pub CNT: crate::RORegister<u32>,
}
#[doc = "GPT Control Register"]
pub mod CR {
    #[doc = "GPT Enable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT is disabled."]
            pub const EN_0: u32 = 0;
            #[doc = "GPT is enabled."]
            pub const EN_1: u32 = 0x01;
        }
    }
    #[doc = "GPT Enable mode"]
    pub mod ENMOD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT counter will retain its value when it is disabled."]
            pub const ENMOD_0: u32 = 0;
            #[doc = "GPT counter value is reset to 0 when it is disabled."]
            pub const ENMOD_1: u32 = 0x01;
        }
    }
    #[doc = "GPT debug mode enable"]
    pub mod DBGEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT is disabled in debug mode."]
            pub const DBGEN_0: u32 = 0;
            #[doc = "GPT is enabled in debug mode."]
            pub const DBGEN_1: u32 = 0x01;
        }
    }
    #[doc = "GPT Wait Mode enable"]
    pub mod WAITEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT is disabled in wait mode."]
            pub const WAITEN_0: u32 = 0;
            #[doc = "GPT is enabled in wait mode."]
            pub const WAITEN_1: u32 = 0x01;
        }
    }
    #[doc = "GPT Doze Mode Enable"]
    pub mod DOZEEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT is disabled in doze mode."]
            pub const DOZEEN_0: u32 = 0;
            #[doc = "GPT is enabled in doze mode."]
            pub const DOZEEN_1: u32 = 0x01;
        }
    }
    #[doc = "GPT Stop Mode enable"]
    pub mod STOPEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT is disabled in Stop mode."]
            pub const STOPEN_0: u32 = 0;
            #[doc = "GPT is enabled in Stop mode."]
            pub const STOPEN_1: u32 = 0x01;
        }
    }
    #[doc = "Clock Source select"]
    pub mod CLKSRC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const CLKSRC_0: u32 = 0;
            #[doc = "Peripheral Clock (ipg_clk)"]
            pub const CLKSRC_1: u32 = 0x01;
            #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)"]
            pub const CLKSRC_2: u32 = 0x02;
            #[doc = "External Clock"]
            pub const CLKSRC_3: u32 = 0x03;
            #[doc = "Low Frequency Reference Clock (ipg_clk_32k)"]
            pub const CLKSRC_4: u32 = 0x04;
            #[doc = "Crystal oscillator as Reference Clock (ipg_clk_24M)"]
            pub const CLKSRC_5: u32 = 0x05;
        }
    }
    #[doc = "Free-Run or Restart mode"]
    pub mod FRR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Restart mode"]
            pub const FRR_0: u32 = 0;
            #[doc = "Free-Run mode"]
            pub const FRR_1: u32 = 0x01;
        }
    }
    #[doc = "Enable 24 MHz clock input from crystal"]
    pub mod EN_24M {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "24M clock disabled"]
            pub const EN_24M_0: u32 = 0;
            #[doc = "24M clock enabled"]
            pub const EN_24M_1: u32 = 0x01;
        }
    }
    #[doc = "Software reset"]
    pub mod SWR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT is not in reset state"]
            pub const SWR_0: u32 = 0;
            #[doc = "GPT is in reset state"]
            pub const SWR_1: u32 = 0x01;
        }
    }
    #[doc = "See IM2"]
    pub mod IM1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    pub mod IM2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See OM3"]
    pub mod OM1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See OM3"]
    pub mod OM2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    pub mod OM3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See F03"]
    pub mod FO1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See F03"]
    pub mod FO2 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    pub mod FO3 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPT Prescaler Register"]
pub mod PR {
    #[doc = "Prescaler bits"]
    pub mod PRESCALER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const PRESCALER_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const PRESCALER_1: u32 = 0x01;
            #[doc = "Divide by 4096"]
            pub const PRESCALER_4095: u32 = 0x0fff;
        }
    }
    #[doc = "Prescaler bits"]
    pub mod PRESCALER24M {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const PRESCALER24M_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const PRESCALER24M_1: u32 = 0x01;
            #[doc = "Divide by 16"]
            pub const PRESCALER24M_15: u32 = 0x0f;
        }
    }
}
#[doc = "GPT Status Register"]
pub mod SR {
    #[doc = "See OF3"]
    pub mod OF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See OF3"]
    pub mod OF2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    pub mod OF3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See IF2"]
    pub mod IF1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    pub mod IF2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rollover Flag"]
    pub mod ROV {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rollover has not occurred."]
            pub const ROV_0: u32 = 0;
            #[doc = "Rollover has occurred."]
            pub const ROV_1: u32 = 0x01;
        }
    }
}
#[doc = "GPT Interrupt Register"]
pub mod IR {
    #[doc = "See OF3IE"]
    pub mod OF1IE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See OF3IE"]
    pub mod OF2IE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    pub mod OF3IE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "See IF2IE"]
    pub mod IF1IE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    pub mod IF2IE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    pub mod ROVIE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rollover interrupt is disabled."]
            pub const ROVIE_0: u32 = 0;
            #[doc = "Rollover interrupt enabled."]
            pub const ROVIE_1: u32 = 0x01;
        }
    }
}
#[doc = "GPT Output Compare Register 1"]
pub mod OCR {
    #[doc = "Compare Value"]
    pub mod COMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPT Input Capture Register 1"]
pub mod ICR {
    #[doc = "Capture Value"]
    pub mod CAPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPT Counter Register"]
pub mod CNT {
    #[doc = "Counter Value. The COUNT bits show the current count value of the GPT counter."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
