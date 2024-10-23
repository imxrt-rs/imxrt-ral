#[doc = "TMPSNS"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "Control 1"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "Control 1"]
    pub CTRL1_SET: crate::RWRegister<u32>,
    #[doc = "Control 1"]
    pub CTRL1_CLR: crate::RWRegister<u32>,
    #[doc = "Control 1"]
    pub CTRL1_TOG: crate::RWRegister<u32>,
    #[doc = "Range 0"]
    pub RANGE0: crate::RWRegister<u32>,
    #[doc = "Range 0"]
    pub RANGE0_SET: crate::RWRegister<u32>,
    #[doc = "Range 0"]
    pub RANGE0_CLR: crate::RWRegister<u32>,
    #[doc = "Range 0"]
    pub RANGE0_TOG: crate::RWRegister<u32>,
    #[doc = "Range 1"]
    pub RANGE1: crate::RWRegister<u32>,
    #[doc = "Range 1"]
    pub RANGE1_SET: crate::RWRegister<u32>,
    #[doc = "Range 1"]
    pub RANGE1_CLR: crate::RWRegister<u32>,
    #[doc = "Range 1"]
    pub RANGE1_TOG: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Status 0"]
    pub STATUS0: crate::RWRegister<u32>,
}
#[doc = "Control 1"]
pub mod CTRL1 {
    #[doc = "Temperature Measurement Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single Reading Mode. New reading available every time CTRL1\\[START\\] bit is set to 1 from 0."]
            pub const FREQ_0: u32 = 0;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_1: u32 = 0x01;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_2: u32 = 0x02;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_3: u32 = 0x03;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_4: u32 = 0x04;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_5: u32 = 0x05;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_6: u32 = 0x06;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_7: u32 = 0x07;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_8: u32 = 0x08;
            #[doc = "Continuous Reading Mode. Next temperature reading taken after programmed number of cycles after current reading is complete."]
            pub const FREQ_9: u32 = 0x09;
        }
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    pub mod FINISH_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const FINISH_IE_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const FINISH_IE_1: u32 = 0x01;
        }
    }
    #[doc = "Low Temperature Interrupt Enable"]
    pub mod LOW_TEMP_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const LOW_TEMP_IE_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const LOW_TEMP_IE_1: u32 = 0x01;
        }
    }
    #[doc = "High Temperature Interrupt Enable"]
    pub mod HIGH_TEMP_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const HIGH_TEMP_IE_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const HIGH_TEMP_IE_1: u32 = 0x01;
        }
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    pub mod PANIC_TEMP_IE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const PANIC_TEMP_IE_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const PANIC_TEMP_IE_1: u32 = 0x01;
        }
    }
    #[doc = "Start Temperature Measurement"]
    pub mod START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No new temperature reading taken"]
            pub const START_0: u32 = 0;
            #[doc = "Initiate a new temperature reading"]
            pub const START_1: u32 = 0x01;
        }
    }
    #[doc = "Power Down Except Bias Current"]
    pub mod PWD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sensor is active"]
            pub const PWD_0: u32 = 0;
            #[doc = "Sensor is powered down"]
            pub const PWD_1: u32 = 0x01;
        }
    }
    #[doc = "Power Down"]
    pub mod PWD_FULL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sensor is active"]
            pub const PWD_FULL_0: u32 = 0;
            #[doc = "Sensor is powered down"]
            pub const PWD_FULL_1: u32 = 0x01;
        }
    }
}
#[doc = "Control 1"]
pub mod CTRL1_SET {
    #[doc = "Temperature Measurement Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    pub mod FINISH_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low Temperature Interrupt Enable"]
    pub mod LOW_TEMP_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Interrupt Enable"]
    pub mod HIGH_TEMP_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    pub mod PANIC_TEMP_IE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Temperature Measurement"]
    pub mod START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down Except Bias Current"]
    pub mod PWD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down"]
    pub mod PWD_FULL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control 1"]
pub mod CTRL1_CLR {
    #[doc = "Temperature Measurement Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    pub mod FINISH_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low Temperature Interrupt Enable"]
    pub mod LOW_TEMP_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Interrupt Enable"]
    pub mod HIGH_TEMP_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    pub mod PANIC_TEMP_IE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Temperature Measurement"]
    pub mod START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down Except Bias Current"]
    pub mod PWD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down"]
    pub mod PWD_FULL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control 1"]
pub mod CTRL1_TOG {
    #[doc = "Temperature Measurement Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    pub mod FINISH_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low Temperature Interrupt Enable"]
    pub mod LOW_TEMP_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Interrupt Enable"]
    pub mod HIGH_TEMP_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    pub mod PANIC_TEMP_IE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Temperature Measurement"]
    pub mod START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down Except Bias Current"]
    pub mod PWD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down"]
    pub mod PWD_FULL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 0"]
pub mod RANGE0 {
    #[doc = "Low temperature threshold value"]
    pub mod LOW_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High temperature threshold value"]
    pub mod HIGH_TEMP_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 0"]
pub mod RANGE0_SET {
    #[doc = "Low temperature threshold value"]
    pub mod LOW_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High temperature threshold value"]
    pub mod HIGH_TEMP_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 0"]
pub mod RANGE0_CLR {
    #[doc = "Low temperature threshold value"]
    pub mod LOW_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High temperature threshold value"]
    pub mod HIGH_TEMP_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 0"]
pub mod RANGE0_TOG {
    #[doc = "Low temperature threshold value"]
    pub mod LOW_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High temperature threshold value"]
    pub mod HIGH_TEMP_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 1"]
pub mod RANGE1 {
    #[doc = "Panic temperature threshold value"]
    pub mod PANIC_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 1"]
pub mod RANGE1_SET {
    #[doc = "Panic temperature threshold value"]
    pub mod PANIC_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 1"]
pub mod RANGE1_CLR {
    #[doc = "Panic temperature threshold value"]
    pub mod PANIC_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 1"]
pub mod RANGE1_TOG {
    #[doc = "Panic temperature threshold value"]
    pub mod PANIC_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 0"]
pub mod STATUS0 {
    #[doc = "Measured temperature value"]
    pub mod TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Temperature measurement complete"]
    pub mod FINISH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Temperature sensor is busy (if CTRL1\\[START\\] = 1)or no new reading has been initiated (if CTRL1\\[START\\] = 0)"]
            pub const FINISH_0: u32 = 0;
            #[doc = "Temperature reading is complete and new temperature value available for reading"]
            pub const FINISH_1: u32 = 0x01;
        }
    }
    #[doc = "Low temperature alarm bit"]
    pub mod LOW_TEMP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Low temperature alert"]
            pub const LOW_TEMP_0: u32 = 0;
            #[doc = "Low temperature alert"]
            pub const LOW_TEMP_1: u32 = 0x01;
        }
    }
    #[doc = "High temperature alarm bit"]
    pub mod HIGH_TEMP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No High temperature alert"]
            pub const HIGH_TEMP_0: u32 = 0;
            #[doc = "High temperature alert"]
            pub const HIGH_TEMP_1: u32 = 0x01;
        }
    }
    #[doc = "Panic temperature alarm bit"]
    pub mod PANIC_TEMP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Panic temperature alert"]
            pub const PANIC_TEMP_0: u32 = 0;
            #[doc = "Panic temperature alert"]
            pub const PANIC_TEMP_1: u32 = 0x01;
        }
    }
}
