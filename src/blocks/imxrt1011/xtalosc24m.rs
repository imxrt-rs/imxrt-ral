#[doc = "XTALOSC24M"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0150],
    #[doc = "Miscellaneous Register 0"]
    pub MISC0: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_SET: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_CLR: crate::RWRegister<u32>,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_TOG: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0110],
    #[doc = "XTAL OSC (LP) Control Register"]
    pub LOWPWR_CTRL: crate::RWRegister<u32>,
    #[doc = "XTAL OSC (LP) Control Register"]
    pub LOWPWR_CTRL_SET: crate::RWRegister<u32>,
    #[doc = "XTAL OSC (LP) Control Register"]
    pub LOWPWR_CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "XTAL OSC (LP) Control Register"]
    pub LOWPWR_CTRL_TOG: crate::RWRegister<u32>,
    _reserved2: [u8; 0x20],
    #[doc = "XTAL OSC Configuration 0 Register"]
    pub OSC_CONFIG0: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 0 Register"]
    pub OSC_CONFIG0_SET: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 0 Register"]
    pub OSC_CONFIG0_CLR: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 0 Register"]
    pub OSC_CONFIG0_TOG: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 1 Register"]
    pub OSC_CONFIG1: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 1 Register"]
    pub OSC_CONFIG1_SET: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 1 Register"]
    pub OSC_CONFIG1_CLR: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 1 Register"]
    pub OSC_CONFIG1_TOG: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 2 Register"]
    pub OSC_CONFIG2: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 2 Register"]
    pub OSC_CONFIG2_SET: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 2 Register"]
    pub OSC_CONFIG2_CLR: crate::RWRegister<u32>,
    #[doc = "XTAL OSC Configuration 2 Register"]
    pub OSC_CONFIG2_TOG: crate::RWRegister<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0 {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Not related to oscillator."]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure the analog behavior in stop mode.Not related to oscillator."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    pub mod VID_PLL_PREDIV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const VID_PLL_PREDIV_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const VID_PLL_PREDIV_1: u32 = 0x01;
        }
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_SET {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Not related to oscillator."]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure the analog behavior in stop mode.Not related to oscillator."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    pub mod VID_PLL_PREDIV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const VID_PLL_PREDIV_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const VID_PLL_PREDIV_1: u32 = 0x01;
        }
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_CLR {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Not related to oscillator."]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure the analog behavior in stop mode.Not related to oscillator."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    pub mod VID_PLL_PREDIV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const VID_PLL_PREDIV_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const VID_PLL_PREDIV_1: u32 = 0x01;
        }
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_TOG {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
    }
    #[doc = "Not related to oscillator."]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configure the analog behavior in stop mode.Not related to oscillator."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    pub mod VID_PLL_PREDIV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const VID_PLL_PREDIV_0: u32 = 0;
            #[doc = "Divide by 2"]
            pub const VID_PLL_PREDIV_1: u32 = 0x01;
        }
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod LOWPWR_CTRL {
    #[doc = "RC Osc. enable control."]
    pub mod RC_OSC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use XTAL OSC to source the 24MHz clock"]
            pub const RC_OSC_EN_0: u32 = 0;
            #[doc = "Use RC OSC"]
            pub const RC_OSC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Select the source for the 24MHz clock."]
    pub mod OSC_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XTAL OSC"]
            pub const OSC_SEL_0: u32 = 0;
            #[doc = "RC OSC"]
            pub const OSC_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    pub mod LPBG_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal power bandgap"]
            pub const LPBG_SEL_0: u32 = 0;
            #[doc = "Low power bandgap"]
            pub const LPBG_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    pub mod LPBG_TEST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    pub mod REFTOP_IBIAS_OFF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    pub mod L1_PWRGATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    pub mod L2_PWRGATE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    pub mod CPU_PWRGATE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    pub mod DISPLAY_PWRGATE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For debug purposes only"]
    pub mod RCOSC_CG_OVERRIDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    pub mod XTALOSC_PWRUP_DELAY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.25ms"]
            pub const XTALOSC_PWRUP_DELAY_0: u32 = 0;
            #[doc = "0.5ms"]
            pub const XTALOSC_PWRUP_DELAY_1: u32 = 0x01;
            #[doc = "1ms"]
            pub const XTALOSC_PWRUP_DELAY_2: u32 = 0x02;
            #[doc = "2ms"]
            pub const XTALOSC_PWRUP_DELAY_3: u32 = 0x03;
        }
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    pub mod XTALOSC_PWRUP_STAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not stable"]
            pub const XTALOSC_PWRUP_STAT_0: u32 = 0;
            #[doc = "Stable and ready to use"]
            pub const XTALOSC_PWRUP_STAT_1: u32 = 0x01;
        }
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    pub mod MIX_PWRGATE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    pub mod GPU_PWRGATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod LOWPWR_CTRL_SET {
    #[doc = "RC Osc. enable control."]
    pub mod RC_OSC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use XTAL OSC to source the 24MHz clock"]
            pub const RC_OSC_EN_0: u32 = 0;
            #[doc = "Use RC OSC"]
            pub const RC_OSC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Select the source for the 24MHz clock."]
    pub mod OSC_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XTAL OSC"]
            pub const OSC_SEL_0: u32 = 0;
            #[doc = "RC OSC"]
            pub const OSC_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    pub mod LPBG_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal power bandgap"]
            pub const LPBG_SEL_0: u32 = 0;
            #[doc = "Low power bandgap"]
            pub const LPBG_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    pub mod LPBG_TEST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    pub mod REFTOP_IBIAS_OFF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    pub mod L1_PWRGATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    pub mod L2_PWRGATE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    pub mod CPU_PWRGATE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    pub mod DISPLAY_PWRGATE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For debug purposes only"]
    pub mod RCOSC_CG_OVERRIDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    pub mod XTALOSC_PWRUP_DELAY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.25ms"]
            pub const XTALOSC_PWRUP_DELAY_0: u32 = 0;
            #[doc = "0.5ms"]
            pub const XTALOSC_PWRUP_DELAY_1: u32 = 0x01;
            #[doc = "1ms"]
            pub const XTALOSC_PWRUP_DELAY_2: u32 = 0x02;
            #[doc = "2ms"]
            pub const XTALOSC_PWRUP_DELAY_3: u32 = 0x03;
        }
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    pub mod XTALOSC_PWRUP_STAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not stable"]
            pub const XTALOSC_PWRUP_STAT_0: u32 = 0;
            #[doc = "Stable and ready to use"]
            pub const XTALOSC_PWRUP_STAT_1: u32 = 0x01;
        }
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    pub mod MIX_PWRGATE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    pub mod GPU_PWRGATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod LOWPWR_CTRL_CLR {
    #[doc = "RC Osc. enable control."]
    pub mod RC_OSC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use XTAL OSC to source the 24MHz clock"]
            pub const RC_OSC_EN_0: u32 = 0;
            #[doc = "Use RC OSC"]
            pub const RC_OSC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Select the source for the 24MHz clock."]
    pub mod OSC_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XTAL OSC"]
            pub const OSC_SEL_0: u32 = 0;
            #[doc = "RC OSC"]
            pub const OSC_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    pub mod LPBG_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal power bandgap"]
            pub const LPBG_SEL_0: u32 = 0;
            #[doc = "Low power bandgap"]
            pub const LPBG_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    pub mod LPBG_TEST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    pub mod REFTOP_IBIAS_OFF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    pub mod L1_PWRGATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    pub mod L2_PWRGATE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    pub mod CPU_PWRGATE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    pub mod DISPLAY_PWRGATE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For debug purposes only"]
    pub mod RCOSC_CG_OVERRIDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    pub mod XTALOSC_PWRUP_DELAY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.25ms"]
            pub const XTALOSC_PWRUP_DELAY_0: u32 = 0;
            #[doc = "0.5ms"]
            pub const XTALOSC_PWRUP_DELAY_1: u32 = 0x01;
            #[doc = "1ms"]
            pub const XTALOSC_PWRUP_DELAY_2: u32 = 0x02;
            #[doc = "2ms"]
            pub const XTALOSC_PWRUP_DELAY_3: u32 = 0x03;
        }
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    pub mod XTALOSC_PWRUP_STAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not stable"]
            pub const XTALOSC_PWRUP_STAT_0: u32 = 0;
            #[doc = "Stable and ready to use"]
            pub const XTALOSC_PWRUP_STAT_1: u32 = 0x01;
        }
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    pub mod MIX_PWRGATE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    pub mod GPU_PWRGATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod LOWPWR_CTRL_TOG {
    #[doc = "RC Osc. enable control."]
    pub mod RC_OSC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use XTAL OSC to source the 24MHz clock"]
            pub const RC_OSC_EN_0: u32 = 0;
            #[doc = "Use RC OSC"]
            pub const RC_OSC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Select the source for the 24MHz clock."]
    pub mod OSC_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "XTAL OSC"]
            pub const OSC_SEL_0: u32 = 0;
            #[doc = "RC OSC"]
            pub const OSC_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    pub mod LPBG_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal power bandgap"]
            pub const LPBG_SEL_0: u32 = 0;
            #[doc = "Low power bandgap"]
            pub const LPBG_SEL_1: u32 = 0x01;
        }
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    pub mod LPBG_TEST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    pub mod REFTOP_IBIAS_OFF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    pub mod L1_PWRGATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    pub mod L2_PWRGATE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    pub mod CPU_PWRGATE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    pub mod DISPLAY_PWRGATE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For debug purposes only"]
    pub mod RCOSC_CG_OVERRIDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    pub mod XTALOSC_PWRUP_DELAY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.25ms"]
            pub const XTALOSC_PWRUP_DELAY_0: u32 = 0;
            #[doc = "0.5ms"]
            pub const XTALOSC_PWRUP_DELAY_1: u32 = 0x01;
            #[doc = "1ms"]
            pub const XTALOSC_PWRUP_DELAY_2: u32 = 0x02;
            #[doc = "2ms"]
            pub const XTALOSC_PWRUP_DELAY_3: u32 = 0x03;
        }
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    pub mod XTALOSC_PWRUP_STAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not stable"]
            pub const XTALOSC_PWRUP_STAT_0: u32 = 0;
            #[doc = "Stable and ready to use"]
            pub const XTALOSC_PWRUP_STAT_1: u32 = 0x01;
        }
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    pub mod MIX_PWRGATE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    pub mod GPU_PWRGATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod OSC_CONFIG0 {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    pub mod ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    pub mod BYPASS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    pub mod INVERT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RC osc. tuning values."]
    pub mod RC_OSC_PROG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive hysteresis value"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Negative hysteresis value"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current tuning value in use."]
    pub mod RC_OSC_PROG_CUR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod OSC_CONFIG0_SET {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    pub mod ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    pub mod BYPASS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    pub mod INVERT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RC osc. tuning values."]
    pub mod RC_OSC_PROG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive hysteresis value"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Negative hysteresis value"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current tuning value in use."]
    pub mod RC_OSC_PROG_CUR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod OSC_CONFIG0_CLR {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    pub mod ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    pub mod BYPASS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    pub mod INVERT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RC osc. tuning values."]
    pub mod RC_OSC_PROG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive hysteresis value"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Negative hysteresis value"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current tuning value in use."]
    pub mod RC_OSC_PROG_CUR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod OSC_CONFIG0_TOG {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    pub mod ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    pub mod BYPASS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    pub mod INVERT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RC osc. tuning values."]
    pub mod RC_OSC_PROG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive hysteresis value"]
    pub mod HYST_PLUS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Negative hysteresis value"]
    pub mod HYST_MINUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current tuning value in use."]
    pub mod RC_OSC_PROG_CUR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod OSC_CONFIG1 {
    #[doc = "The target count used to tune the RC OSC frequency"]
    pub mod COUNT_RC_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current tuning value in use."]
    pub mod COUNT_RC_CUR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod OSC_CONFIG1_SET {
    #[doc = "The target count used to tune the RC OSC frequency"]
    pub mod COUNT_RC_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current tuning value in use."]
    pub mod COUNT_RC_CUR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod OSC_CONFIG1_CLR {
    #[doc = "The target count used to tune the RC OSC frequency"]
    pub mod COUNT_RC_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current tuning value in use."]
    pub mod COUNT_RC_CUR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod OSC_CONFIG1_TOG {
    #[doc = "The target count used to tune the RC OSC frequency"]
    pub mod COUNT_RC_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current tuning value in use."]
    pub mod COUNT_RC_CUR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod OSC_CONFIG2 {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    pub mod COUNT_1M_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    pub mod ENABLE_1M {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    pub mod MUX_1M {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    pub mod CLK_1M_ERR_FL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod OSC_CONFIG2_SET {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    pub mod COUNT_1M_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    pub mod ENABLE_1M {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    pub mod MUX_1M {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    pub mod CLK_1M_ERR_FL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod OSC_CONFIG2_CLR {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    pub mod COUNT_1M_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    pub mod ENABLE_1M {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    pub mod MUX_1M {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    pub mod CLK_1M_ERR_FL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod OSC_CONFIG2_TOG {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    pub mod COUNT_1M_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    pub mod ENABLE_1M {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    pub mod MUX_1M {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    pub mod CLK_1M_ERR_FL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
