#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CMP Control Register 0"]
    pub CR0: crate::RWRegister<u8>,
    #[doc = "CMP Control Register 1"]
    pub CR1: crate::RWRegister<u8>,
    #[doc = "CMP Filter Period Register"]
    pub FPR: crate::RWRegister<u8>,
    #[doc = "CMP Status and Control Register"]
    pub SCR: crate::RWRegister<u8>,
    #[doc = "DAC Control Register"]
    pub DACCR: crate::RWRegister<u8>,
    #[doc = "MUX Control Register"]
    pub MUXCR: crate::RWRegister<u8>,
}
#[doc = "CMP Control Register 0"]
pub mod CR0 {
    #[doc = "Comparator hard block hysteresis control"]
    pub mod HYSTCTR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level 0"]
            pub const HYSTCTR_0: u8 = 0;
            #[doc = "Level 1"]
            pub const HYSTCTR_1: u8 = 0x01;
            #[doc = "Level 2"]
            pub const HYSTCTR_2: u8 = 0x02;
            #[doc = "Level 3"]
            pub const HYSTCTR_3: u8 = 0x03;
        }
    }
    #[doc = "Filter Sample Count"]
    pub mod FILTER_CNT {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
            pub const FILTER_CNT_0: u8 = 0;
            #[doc = "One sample must agree. The comparator output is simply sampled."]
            pub const FILTER_CNT_1: u8 = 0x01;
            #[doc = "2 consecutive samples must agree."]
            pub const FILTER_CNT_2: u8 = 0x02;
            #[doc = "3 consecutive samples must agree."]
            pub const FILTER_CNT_3: u8 = 0x03;
            #[doc = "4 consecutive samples must agree."]
            pub const FILTER_CNT_4: u8 = 0x04;
            #[doc = "5 consecutive samples must agree."]
            pub const FILTER_CNT_5: u8 = 0x05;
            #[doc = "6 consecutive samples must agree."]
            pub const FILTER_CNT_6: u8 = 0x06;
            #[doc = "7 consecutive samples must agree."]
            pub const FILTER_CNT_7: u8 = 0x07;
        }
    }
}
#[doc = "CMP Control Register 1"]
pub mod CR1 {
    #[doc = "Comparator Module Enable"]
    pub mod EN {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Analog Comparator is disabled."]
            pub const EN_0: u8 = 0;
            #[doc = "Analog Comparator is enabled."]
            pub const EN_1: u8 = 0x01;
        }
    }
    #[doc = "Comparator Output Pin Enable"]
    pub mod OPE {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
            pub const OPE_0: u8 = 0;
            #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
            pub const OPE_1: u8 = 0x01;
        }
    }
    #[doc = "Comparator Output Select"]
    pub mod COS {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
            pub const COS_0: u8 = 0;
            #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
            pub const COS_1: u8 = 0x01;
        }
    }
    #[doc = "Comparator INVERT"]
    pub mod INV {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not invert the comparator output."]
            pub const INV_0: u8 = 0;
            #[doc = "Inverts the comparator output."]
            pub const INV_1: u8 = 0x01;
        }
    }
    #[doc = "Power Mode Select"]
    pub mod PMODE {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
            pub const PMODE_0: u8 = 0;
            #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
            pub const PMODE_1: u8 = 0x01;
        }
    }
    #[doc = "Windowing Enable"]
    pub mod WE {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Windowing mode is not selected."]
            pub const WE_0: u8 = 0;
            #[doc = "Windowing mode is selected."]
            pub const WE_1: u8 = 0x01;
        }
    }
    #[doc = "Sample Enable"]
    pub mod SE {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sampling mode is not selected."]
            pub const SE_0: u8 = 0;
            #[doc = "Sampling mode is selected."]
            pub const SE_1: u8 = 0x01;
        }
    }
}
#[doc = "CMP Filter Period Register"]
pub mod FPR {
    #[doc = "Filter Sample Period"]
    pub mod FILT_PER {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CMP Status and Control Register"]
pub mod SCR {
    #[doc = "Analog Comparator Output"]
    pub mod COUT {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Analog Comparator Flag Falling"]
    pub mod CFF {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Falling-edge on COUT has not been detected."]
            pub const CFF_0: u8 = 0;
            #[doc = "Falling-edge on COUT has occurred."]
            pub const CFF_1: u8 = 0x01;
        }
    }
    #[doc = "Analog Comparator Flag Rising"]
    pub mod CFR {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rising-edge on COUT has not been detected."]
            pub const CFR_0: u8 = 0;
            #[doc = "Rising-edge on COUT has occurred."]
            pub const CFR_1: u8 = 0x01;
        }
    }
    #[doc = "Comparator Interrupt Enable Falling"]
    pub mod IEF {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const IEF_0: u8 = 0;
            #[doc = "Interrupt is enabled."]
            pub const IEF_1: u8 = 0x01;
        }
    }
    #[doc = "Comparator Interrupt Enable Rising"]
    pub mod IER {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const IER_0: u8 = 0;
            #[doc = "Interrupt is enabled."]
            pub const IER_1: u8 = 0x01;
        }
    }
    #[doc = "DMA Enable Control"]
    pub mod DMAEN {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA is disabled."]
            pub const DMAEN_0: u8 = 0;
            #[doc = "DMA is enabled."]
            pub const DMAEN_1: u8 = 0x01;
        }
    }
}
#[doc = "DAC Control Register"]
pub mod DACCR {
    #[doc = "DAC Output Voltage Select"]
    pub mod VOSEL {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply Voltage Reference Source Select"]
    pub mod VRSEL {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vin1 is selected as resistor ladder network supply reference."]
            pub const VRSEL_0: u8 = 0;
            #[doc = "Vin2 is selected as resistor ladder network supply reference."]
            pub const VRSEL_1: u8 = 0x01;
        }
    }
    #[doc = "DAC Enable"]
    pub mod DACEN {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DAC is disabled."]
            pub const DACEN_0: u8 = 0;
            #[doc = "DAC is enabled."]
            pub const DACEN_1: u8 = 0x01;
        }
    }
}
#[doc = "MUX Control Register"]
pub mod MUXCR {
    #[doc = "Minus Input Mux Control"]
    pub mod MSEL {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IN0"]
            pub const MSEL_0: u8 = 0;
            #[doc = "IN1"]
            pub const MSEL_1: u8 = 0x01;
            #[doc = "IN2"]
            pub const MSEL_2: u8 = 0x02;
            #[doc = "IN3"]
            pub const MSEL_3: u8 = 0x03;
            #[doc = "IN4"]
            pub const MSEL_4: u8 = 0x04;
            #[doc = "IN5"]
            pub const MSEL_5: u8 = 0x05;
            #[doc = "IN6"]
            pub const MSEL_6: u8 = 0x06;
            #[doc = "IN7"]
            pub const MSEL_7: u8 = 0x07;
        }
    }
    #[doc = "Plus Input Mux Control"]
    pub mod PSEL {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IN0"]
            pub const PSEL_0: u8 = 0;
            #[doc = "IN1"]
            pub const PSEL_1: u8 = 0x01;
            #[doc = "IN2"]
            pub const PSEL_2: u8 = 0x02;
            #[doc = "IN3"]
            pub const PSEL_3: u8 = 0x03;
            #[doc = "IN4"]
            pub const PSEL_4: u8 = 0x04;
            #[doc = "IN5"]
            pub const PSEL_5: u8 = 0x05;
            #[doc = "IN6"]
            pub const PSEL_6: u8 = 0x06;
            #[doc = "IN7"]
            pub const PSEL_7: u8 = 0x07;
        }
    }
}
