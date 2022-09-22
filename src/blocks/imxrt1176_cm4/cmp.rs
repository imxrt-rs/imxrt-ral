#[doc = "CMP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "CMP Control Register 0"]
    pub C0: crate::RWRegister<u32>,
    #[doc = "CMP Control Register 1"]
    pub C1: crate::RWRegister<u32>,
    #[doc = "CMP Control Register 2"]
    pub C2: crate::RWRegister<u32>,
    #[doc = "CMP Control Register 3"]
    pub C3: crate::RWRegister<u32>,
}
#[doc = "Version ID Register"]
pub mod VERID {
    #[doc = "Feature Specification Number. This read only filed returns the feature set number."]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minor Version Number. This read only field returns the minor version number for the module specification."]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number. This read only field returns the major version number for the module specification."]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter Register"]
pub mod PARAM {
    #[doc = "Parameter Registers. This read only filed returns the feature parameters implemented along with the Version ID register."]
    pub mod PARAM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CMP Control Register 0"]
pub mod C0 {
    #[doc = "Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    pub mod HYSTCTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The hard block output has level 0 hysteresis internally."]
            pub const HYSTCTR_0: u32 = 0;
            #[doc = "The hard block output has level 1 hysteresis internally."]
            pub const HYSTCTR_1: u32 = 0x01;
            #[doc = "The hard block output has level 2 hysteresis internally."]
            pub const HYSTCTR_2: u32 = 0x02;
            #[doc = "The hard block output has level 3 hysteresis internally."]
            pub const HYSTCTR_3: u32 = 0x03;
        }
    }
    #[doc = "Filter Sample Count"]
    pub mod FILTER_CNT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
            pub const FILTER_CNT_0: u32 = 0;
            #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
            pub const FILTER_CNT_1: u32 = 0x01;
            #[doc = "2 consecutive samples must agree."]
            pub const FILTER_CNT_2: u32 = 0x02;
            #[doc = "3 consecutive samples must agree."]
            pub const FILTER_CNT_3: u32 = 0x03;
            #[doc = "4 consecutive samples must agree."]
            pub const FILTER_CNT_4: u32 = 0x04;
            #[doc = "5 consecutive samples must agree."]
            pub const FILTER_CNT_5: u32 = 0x05;
            #[doc = "6 consecutive samples must agree."]
            pub const FILTER_CNT_6: u32 = 0x06;
            #[doc = "7 consecutive samples must agree."]
            pub const FILTER_CNT_7: u32 = 0x07;
        }
    }
    #[doc = "Comparator Module Enable"]
    pub mod EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Analog Comparator is disabled."]
            pub const EN_0: u32 = 0;
            #[doc = "Analog Comparator is enabled."]
            pub const EN_1: u32 = 0x01;
        }
    }
    #[doc = "Comparator Output Pin Enable"]
    pub mod OPE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
            pub const OPE_0: u32 = 0;
            #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
            pub const OPE_1: u32 = 0x01;
        }
    }
    #[doc = "Comparator Output Select"]
    pub mod COS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
            pub const COS_0: u32 = 0;
            #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
            pub const COS_1: u32 = 0x01;
        }
    }
    #[doc = "Comparator invert"]
    pub mod INVT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not invert the comparator output."]
            pub const INVT_0: u32 = 0;
            #[doc = "Inverts the comparator output."]
            pub const INVT_1: u32 = 0x01;
        }
    }
    #[doc = "Power Mode Select"]
    pub mod PMODE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low Speed (LS) comparison mode is selected."]
            pub const PMODE_0: u32 = 0;
            #[doc = "High Speed (HS) comparison mode is selected."]
            pub const PMODE_1: u32 = 0x01;
        }
    }
    #[doc = "Windowing Enable"]
    pub mod WE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Windowing mode is not selected."]
            pub const WE_0: u32 = 0;
            #[doc = "Windowing mode is selected."]
            pub const WE_1: u32 = 0x01;
        }
    }
    #[doc = "Sample Enable"]
    pub mod SE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sampling mode is not selected."]
            pub const SE_0: u32 = 0;
            #[doc = "Sampling mode is selected."]
            pub const SE_1: u32 = 0x01;
        }
    }
    #[doc = "Filter Sample Period"]
    pub mod FPR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Analog Comparator Output"]
    pub mod COUT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Analog Comparator Flag Falling"]
    pub mod CFF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A falling edge has not been detected on COUT."]
            pub const CFF_0: u32 = 0;
            #[doc = "A falling edge on COUT has occurred."]
            pub const CFF_1: u32 = 0x01;
        }
    }
    #[doc = "Analog Comparator Flag Rising"]
    pub mod CFR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A rising edge has not been detected on COUT."]
            pub const CFR_0: u32 = 0;
            #[doc = "A rising edge on COUT has occurred."]
            pub const CFR_1: u32 = 0x01;
        }
    }
    #[doc = "Comparator Interrupt Enable Falling"]
    pub mod IEF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const IEF_0: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const IEF_1: u32 = 0x01;
        }
    }
    #[doc = "Comparator Interrupt Enable Rising"]
    pub mod IER {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const IER_0: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const IER_1: u32 = 0x01;
        }
    }
    #[doc = "DMA Enable"]
    pub mod DMAEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA is disabled."]
            pub const DMAEN_0: u32 = 0;
            #[doc = "DMA is enabled."]
            pub const DMAEN_1: u32 = 0x01;
        }
    }
    #[doc = "CMP to DAC link enable."]
    pub mod LINKEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CMP to DAC link is disabled"]
            pub const LINKEN_0: u32 = 0;
            #[doc = "CMP to DAC link is enabled."]
            pub const LINKEN_1: u32 = 0x01;
        }
    }
}
#[doc = "CMP Control Register 1"]
pub mod C1 {
    #[doc = "DAC Output Voltage Select"]
    pub mod VOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DAC Mode Selection"]
    pub mod DMODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DAC is selected to work in low speed and low power mode."]
            pub const DMODE_0: u32 = 0;
            #[doc = "DAC is selected to work in high speed high power mode."]
            pub const DMODE_1: u32 = 0x01;
        }
    }
    #[doc = "Supply Voltage Reference Source Select"]
    pub mod VRSEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vin1 is selected as resistor ladder network supply reference Vin. Vin1 is from internal PMC."]
            pub const VRSEL_0: u32 = 0;
            #[doc = "Vin2 is selected as resistor ladder network supply reference Vin. Vin2 is from PAD."]
            pub const VRSEL_1: u32 = 0x01;
        }
    }
    #[doc = "DAC Enable"]
    pub mod DACEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DAC is disabled."]
            pub const DACEN_0: u32 = 0;
            #[doc = "DAC is enabled."]
            pub const DACEN_1: u32 = 0x01;
        }
    }
    #[doc = "Channel 0 input enable"]
    pub mod CHN0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 1 input enable"]
    pub mod CHN1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 2 input enable"]
    pub mod CHN2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 3 input enable"]
    pub mod CHN3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 4 input enable"]
    pub mod CHN4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 5 input enable"]
    pub mod CHN5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minus Input MUX Control"]
    pub mod MSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal Negative Input 0 for Minus Channel -- Internal Minus Input"]
            pub const MSEL_0: u32 = 0;
            #[doc = "External Input 1 for Minus Channel -- Reference Input 0"]
            pub const MSEL_1: u32 = 0x01;
            #[doc = "External Input 2 for Minus Channel -- Reference Input 1"]
            pub const MSEL_2: u32 = 0x02;
            #[doc = "External Input 3 for Minus Channel -- Reference Input 2"]
            pub const MSEL_3: u32 = 0x03;
            #[doc = "External Input 4 for Minus Channel -- Reference Input 3"]
            pub const MSEL_4: u32 = 0x04;
            #[doc = "External Input 5 for Minus Channel -- Reference Input 4"]
            pub const MSEL_5: u32 = 0x05;
            #[doc = "External Input 6 for Minus Channel -- Reference Input 5"]
            pub const MSEL_6: u32 = 0x06;
            #[doc = "Internal 8b DAC output"]
            pub const MSEL_7: u32 = 0x07;
        }
    }
    #[doc = "Plus Input MUX Control"]
    pub mod PSEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal Positive Input 0 for Plus Channel -- Internal Plus Input"]
            pub const PSEL_0: u32 = 0;
            #[doc = "External Input 1 for Plus Channel -- Reference Input 0"]
            pub const PSEL_1: u32 = 0x01;
            #[doc = "External Input 2 for Plus Channel -- Reference Input 1"]
            pub const PSEL_2: u32 = 0x02;
            #[doc = "External Input 3 for Plus Channel -- Reference Input 2"]
            pub const PSEL_3: u32 = 0x03;
            #[doc = "External Input 4 for Plus Channel -- Reference Input 3"]
            pub const PSEL_4: u32 = 0x04;
            #[doc = "External Input 5 for Plus Channel -- Reference Input 4"]
            pub const PSEL_5: u32 = 0x05;
            #[doc = "External Input 6 for Plus Channel -- Reference Input 5"]
            pub const PSEL_6: u32 = 0x06;
            #[doc = "Internal 8b DAC output"]
            pub const PSEL_7: u32 = 0x07;
        }
    }
}
#[doc = "CMP Control Register 2"]
pub mod C2 {
    #[doc = "ACOn"]
    pub mod ACON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator and DAC initialization delay modulus."]
    pub mod INITMOD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of sample clocks"]
    pub mod NSAM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
            pub const NSAM_0: u32 = 0;
            #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
            pub const NSAM_1: u32 = 0x01;
            #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
            pub const NSAM_2: u32 = 0x02;
            #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
            pub const NSAM_3: u32 = 0x03;
        }
    }
    #[doc = "CH0F"]
    pub mod CH0F {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CH1F"]
    pub mod CH1F {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CH2F"]
    pub mod CH2F {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CH3F"]
    pub mod CH3F {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CH4F"]
    pub mod CH4F {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CH5F"]
    pub mod CH5F {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fixed channel selection"]
    pub mod FXMXCH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External Reference Input 0 is selected as the fixed reference input for the fixed mux port."]
            pub const FXMXCH_0: u32 = 0;
            #[doc = "External Reference Input 1 is selected as the fixed reference input for the fixed mux port."]
            pub const FXMXCH_1: u32 = 0x01;
            #[doc = "External Reference Input 2 is selected as the fixed reference input for the fixed mux port."]
            pub const FXMXCH_2: u32 = 0x02;
            #[doc = "External Reference Input 3 is selected as the fixed reference input for the fixed mux port."]
            pub const FXMXCH_3: u32 = 0x03;
            #[doc = "External Reference Input 4 is selected as the fixed reference input for the fixed mux port."]
            pub const FXMXCH_4: u32 = 0x04;
            #[doc = "External Reference Input 5 is selected as the fixed reference input for the fixed mux port."]
            pub const FXMXCH_5: u32 = 0x05;
            #[doc = "The 8bit DAC is selected as the fixed reference input for the fixed mux port."]
            pub const FXMXCH_7: u32 = 0x07;
        }
    }
    #[doc = "Fixed MUX Port"]
    pub mod FXMP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
            pub const FXMP_0: u32 = 0;
            #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
            pub const FXMP_1: u32 = 0x01;
        }
    }
    #[doc = "Round-Robin interrupt enable"]
    pub mod RRIE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The round-robin interrupt is disabled."]
            pub const RRIE_0: u32 = 0;
            #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
            pub const RRIE_1: u32 = 0x01;
        }
    }
}
#[doc = "CMP Control Register 3"]
pub mod C3 {
    #[doc = "Analog Comparator Phase2 Timing Control."]
    pub mod ACPH2TC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Phase2 active time in one sampling period equals to T"]
            pub const ACPH2TC_0: u32 = 0;
            #[doc = "Phase2 active time in one sampling period equals to 2*T"]
            pub const ACPH2TC_1: u32 = 0x01;
            #[doc = "Phase2 active time in one sampling period equals to 4*T"]
            pub const ACPH2TC_2: u32 = 0x02;
            #[doc = "Phase2 active time in one sampling period equals to 8*T"]
            pub const ACPH2TC_3: u32 = 0x03;
            #[doc = "Phase2 active time in one sampling period equals to 16*T"]
            pub const ACPH2TC_4: u32 = 0x04;
            #[doc = "Phase2 active time in one sampling period equals to 32*T"]
            pub const ACPH2TC_5: u32 = 0x05;
            #[doc = "Phase2 active time in one sampling period equals to 64*T"]
            pub const ACPH2TC_6: u32 = 0x06;
            #[doc = "Phase2 active time in one sampling period equals to 16*T"]
            pub const ACPH2TC_7: u32 = 0x07;
        }
    }
    #[doc = "Analog Comparator Phase1 Timing Control."]
    pub mod ACPH1TC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Phase1 active time in one sampling period equals to T"]
            pub const ACPH1TC_0: u32 = 0;
            #[doc = "Phase1 active time in one sampling period equals to 2*T"]
            pub const ACPH1TC_1: u32 = 0x01;
            #[doc = "Phase1 active time in one sampling period equals to 4*T"]
            pub const ACPH1TC_2: u32 = 0x02;
            #[doc = "Phase1 active time in one sampling period equals to 8*T"]
            pub const ACPH1TC_3: u32 = 0x03;
            #[doc = "Phase1 active time in one sampling period equals to T"]
            pub const ACPH1TC_4: u32 = 0x04;
            #[doc = "Phase1 active time in one sampling period equals to T"]
            pub const ACPH1TC_5: u32 = 0x05;
            #[doc = "Phase1 active time in one sampling period equals to T"]
            pub const ACPH1TC_6: u32 = 0x06;
            #[doc = "Phase1 active time in one sampling period equals to 0"]
            pub const ACPH1TC_7: u32 = 0x07;
        }
    }
    #[doc = "Analog Comparator Sampling Time control."]
    pub mod ACSAT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The sampling time equals to T"]
            pub const ACSAT_0: u32 = 0;
            #[doc = "The sampling time equasl to 2*T"]
            pub const ACSAT_1: u32 = 0x01;
            #[doc = "The sampling time equasl to 4*T"]
            pub const ACSAT_2: u32 = 0x02;
            #[doc = "The sampling time equasl to 8*T"]
            pub const ACSAT_3: u32 = 0x03;
            #[doc = "The sampling time equasl to 16*T"]
            pub const ACSAT_4: u32 = 0x04;
            #[doc = "The sampling time equasl to 32*T"]
            pub const ACSAT_5: u32 = 0x05;
            #[doc = "The sampling time equasl to 64*T"]
            pub const ACSAT_6: u32 = 0x06;
            #[doc = "The sampling time equasl to 256*T"]
            pub const ACSAT_7: u32 = 0x07;
        }
    }
    #[doc = "Discrete Mode Clock Selection"]
    pub mod DMCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow clock is selected for the timing generation."]
            pub const DMCS_0: u32 = 0;
            #[doc = "Fast clock is selected for the timing generation."]
            pub const DMCS_1: u32 = 0x01;
        }
    }
    #[doc = "Resistor Divider Enable"]
    pub mod RDIVE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The resistor is not enabled even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 - 1.8v."]
            pub const RDIVE_0: u32 = 0;
            #[doc = "The resistor is enabled because the inputs are above 1.8v."]
            pub const RDIVE_1: u32 = 0x01;
        }
    }
    #[doc = "Negative Channel Continuous Mode Enable."]
    pub mod NCHCTEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Negative channel is in Discrete Mode and special timing needs to be configured."]
            pub const NCHCTEN_0: u32 = 0;
            #[doc = "Negative channel is in Continuous Mode and no special timing is requried."]
            pub const NCHCTEN_1: u32 = 0x01;
        }
    }
    #[doc = "Positive Channel Continuous Mode Enable."]
    pub mod PCHCTEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive channel is in Discrete Mode and special timing needs to be configured."]
            pub const PCHCTEN_0: u32 = 0;
            #[doc = "Positive channel is in Continuous Mode and no special timing is requried."]
            pub const PCHCTEN_1: u32 = 0x01;
        }
    }
}
