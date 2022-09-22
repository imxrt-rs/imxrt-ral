#[doc = "Temperature Monitor"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0180],
    #[doc = "Tempsensor Control Register 0"]
    pub TEMPSENSE0: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 0"]
    pub TEMPSENSE0_SET: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 0"]
    pub TEMPSENSE0_CLR: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 0"]
    pub TEMPSENSE0_TOG: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 1"]
    pub TEMPSENSE1: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 1"]
    pub TEMPSENSE1_SET: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 1"]
    pub TEMPSENSE1_CLR: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 1"]
    pub TEMPSENSE1_TOG: crate::RWRegister<u32>,
    _reserved1: [u8; 0xf0],
    #[doc = "Tempsensor Control Register 2"]
    pub TEMPSENSE2: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 2"]
    pub TEMPSENSE2_SET: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 2"]
    pub TEMPSENSE2_CLR: crate::RWRegister<u32>,
    #[doc = "Tempsensor Control Register 2"]
    pub TEMPSENSE2_TOG: crate::RWRegister<u32>,
}
#[doc = "Tempsensor Control Register 0"]
pub mod TEMPSENSE0 {
    #[doc = "This bit powers down the temperature sensor."]
    pub mod POWER_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable power to the temperature sensor."]
            pub const POWER_UP: u32 = 0;
            #[doc = "Power down the temperature sensor."]
            pub const POWER_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Starts the measurement process"]
    pub mod MEASURE_TEMP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not start the measurement process."]
            pub const STOP: u32 = 0;
            #[doc = "Start the measurement process."]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Indicates that the latest temp is valid"]
    pub mod FINISHED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last measurement is not ready yet."]
            pub const INVALID: u32 = 0;
            #[doc = "Last measurement is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "This bit field contains the last measured temperature count."]
    pub mod TEMP_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    pub mod ALARM_VALUE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 0"]
pub mod TEMPSENSE0_SET {
    #[doc = "This bit powers down the temperature sensor."]
    pub mod POWER_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable power to the temperature sensor."]
            pub const POWER_UP: u32 = 0;
            #[doc = "Power down the temperature sensor."]
            pub const POWER_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Starts the measurement process"]
    pub mod MEASURE_TEMP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not start the measurement process."]
            pub const STOP: u32 = 0;
            #[doc = "Start the measurement process."]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Indicates that the latest temp is valid"]
    pub mod FINISHED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last measurement is not ready yet."]
            pub const INVALID: u32 = 0;
            #[doc = "Last measurement is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "This bit field contains the last measured temperature count."]
    pub mod TEMP_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    pub mod ALARM_VALUE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 0"]
pub mod TEMPSENSE0_CLR {
    #[doc = "This bit powers down the temperature sensor."]
    pub mod POWER_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable power to the temperature sensor."]
            pub const POWER_UP: u32 = 0;
            #[doc = "Power down the temperature sensor."]
            pub const POWER_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Starts the measurement process"]
    pub mod MEASURE_TEMP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not start the measurement process."]
            pub const STOP: u32 = 0;
            #[doc = "Start the measurement process."]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Indicates that the latest temp is valid"]
    pub mod FINISHED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last measurement is not ready yet."]
            pub const INVALID: u32 = 0;
            #[doc = "Last measurement is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "This bit field contains the last measured temperature count."]
    pub mod TEMP_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    pub mod ALARM_VALUE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 0"]
pub mod TEMPSENSE0_TOG {
    #[doc = "This bit powers down the temperature sensor."]
    pub mod POWER_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable power to the temperature sensor."]
            pub const POWER_UP: u32 = 0;
            #[doc = "Power down the temperature sensor."]
            pub const POWER_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Starts the measurement process"]
    pub mod MEASURE_TEMP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not start the measurement process."]
            pub const STOP: u32 = 0;
            #[doc = "Start the measurement process."]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Indicates that the latest temp is valid"]
    pub mod FINISHED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last measurement is not ready yet."]
            pub const INVALID: u32 = 0;
            #[doc = "Last measurement is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "This bit field contains the last measured temperature count."]
    pub mod TEMP_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    pub mod ALARM_VALUE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 1"]
pub mod TEMPSENSE1 {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    pub mod MEASURE_FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 1"]
pub mod TEMPSENSE1_SET {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    pub mod MEASURE_FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 1"]
pub mod TEMPSENSE1_CLR {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    pub mod MEASURE_FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 1"]
pub mod TEMPSENSE1_TOG {
    #[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    pub mod MEASURE_FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 2"]
pub mod TEMPSENSE2 {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    pub mod LOW_ALARM_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    pub mod PANIC_ALARM_VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 2"]
pub mod TEMPSENSE2_SET {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    pub mod LOW_ALARM_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    pub mod PANIC_ALARM_VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 2"]
pub mod TEMPSENSE2_CLR {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    pub mod LOW_ALARM_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    pub mod PANIC_ALARM_VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tempsensor Control Register 2"]
pub mod TEMPSENSE2_TOG {
    #[doc = "This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    pub mod LOW_ALARM_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    pub mod PANIC_ALARM_VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
