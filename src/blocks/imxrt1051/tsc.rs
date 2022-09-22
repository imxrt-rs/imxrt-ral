#[doc = "Touch Screen Controller"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "no description available"]
    pub BASIC_SETTING: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "no description available"]
    pub PRE_CHARGE_TIME: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "Flow Control"]
    pub FLOW_CONTROL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "Measure Value"]
    pub MEASEURE_VALUE: crate::RORegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "Interrupt Enable"]
    pub INT_EN: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Interrupt Signal Enable"]
    pub INT_SIG_EN: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Intterrupt Status"]
    pub INT_STATUS: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "no description available"]
    pub DEBUG_MODE: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "no description available"]
    pub DEBUG_MODE2: crate::RWRegister<u32>,
}
#[doc = "no description available"]
pub mod BASIC_SETTING {
    #[doc = "Auto Measure"]
    pub mod AUTO_MEASURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Auto Measure"]
            pub const AUTO_MEASURE_0: u32 = 0;
            #[doc = "Auto Measure"]
            pub const AUTO_MEASURE_1: u32 = 0x01;
        }
    }
    #[doc = "4/5 Wire detection"]
    pub mod _4_5_WIRE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4-Wire Detection Mode"]
            pub const _4_5_WIRE_0: u32 = 0;
            #[doc = "5-Wire Detection Mode"]
            pub const _4_5_WIRE_1: u32 = 0x01;
        }
    }
    #[doc = "Measure Delay Time"]
    pub mod MEASURE_DELAY_TIME {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "no description available"]
pub mod PRE_CHARGE_TIME {
    #[doc = "Before detection, the top screen needs some time before being pulled up to a high voltage."]
    pub mod PRE_CHARGE_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flow Control"]
pub mod FLOW_CONTROL {
    #[doc = "Soft Reset"]
    pub mod SW_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Measure"]
    pub mod START_MEASURE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not start measure for now"]
            pub const START_MEASURE_0: u32 = 0;
            #[doc = "Start measure the X/Y coordinate value"]
            pub const START_MEASURE_1: u32 = 0x01;
        }
    }
    #[doc = "Drop Measure"]
    pub mod DROP_MEASURE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not drop measure for now"]
            pub const DROP_MEASURE_0: u32 = 0;
            #[doc = "Drop the measure and controller return to idle status"]
            pub const DROP_MEASURE_1: u32 = 0x01;
        }
    }
    #[doc = "Start Sense"]
    pub mod START_SENSE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stay at idle status"]
            pub const START_SENSE_0: u32 = 0;
            #[doc = "Start sense detection and (if auto_measure set to 1) measure after detect a touch"]
            pub const START_SENSE_1: u32 = 0x01;
        }
    }
    #[doc = "This bit is for SW disable registers"]
    pub mod DISABLE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Leave HW state machine control"]
            pub const DISABLE_0: u32 = 0;
            #[doc = "SW set to idle status"]
            pub const DISABLE_1: u32 = 0x01;
        }
    }
}
#[doc = "Measure Value"]
pub mod MEASEURE_VALUE {
    #[doc = "Y Value"]
    pub mod Y_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "X Value"]
    pub mod X_VALUE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod INT_EN {
    #[doc = "Measure Interrupt Enable"]
    pub mod MEASURE_INT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable measure interrupt"]
            pub const MEASURE_INT_EN_0: u32 = 0;
            #[doc = "Enable measure interrupt"]
            pub const MEASURE_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Detect Interrupt Enable"]
    pub mod DETECT_INT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable detect interrupt"]
            pub const DETECT_INT_EN_0: u32 = 0;
            #[doc = "Enable detect interrupt"]
            pub const DETECT_INT_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Idle Software Interrupt Enable"]
    pub mod IDLE_SW_INT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable idle software interrupt"]
            pub const IDLE_SW_INT_EN_0: u32 = 0;
            #[doc = "Enable idle software interrupt"]
            pub const IDLE_SW_INT_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Signal Enable"]
pub mod INT_SIG_EN {
    #[doc = "Measure Signal Enable"]
    pub mod MEASURE_SIG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Detect Signal Enable"]
    pub mod DETECT_SIG_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable detect signal"]
            pub const DETECT_SIG_EN_0: u32 = 0;
            #[doc = "Enable detect signal"]
            pub const DETECT_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Valid Signal Enable"]
    pub mod VALID_SIG_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable valid signal"]
            pub const VALID_SIG_EN_0: u32 = 0;
            #[doc = "Enable valid signal"]
            pub const VALID_SIG_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Idle Software Signal Enable"]
    pub mod IDLE_SW_SIG_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable idle software signal"]
            pub const IDLE_SW_SIG_EN_0: u32 = 0;
            #[doc = "Enable idle software signal"]
            pub const IDLE_SW_SIG_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "Intterrupt Status"]
pub mod INT_STATUS {
    #[doc = "Measure Signal"]
    pub mod MEASURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not exist a measure signal"]
            pub const MEASURE_0: u32 = 0;
            #[doc = "Exist a measure signal"]
            pub const MEASURE_1: u32 = 0x01;
        }
    }
    #[doc = "Detect Signal"]
    pub mod DETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not exist a detect signal"]
            pub const DETECT_0: u32 = 0;
            #[doc = "Exist detect signal"]
            pub const DETECT_1: u32 = 0x01;
        }
    }
    #[doc = "Valid Signal"]
    pub mod VALID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "There is no touch detected after measurement, indicates that the measured value is not valid"]
            pub const VALID_0: u32 = 0;
            #[doc = "There is touch detection after measurement, indicates that the measure is valid"]
            pub const VALID_1: u32 = 0x01;
        }
    }
    #[doc = "Idle Software"]
    pub mod IDLE_SW {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Haven't return to idle status"]
            pub const IDLE_SW_0: u32 = 0;
            #[doc = "Already return to idle status"]
            pub const IDLE_SW_1: u32 = 0x01;
        }
    }
}
#[doc = "no description available"]
pub mod DEBUG_MODE {
    #[doc = "ADC Conversion Value"]
    pub mod ADC_CONV_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC COCO Signal"]
    pub mod ADC_COCO {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Trigger Select Signal"]
    pub mod EXT_HWTS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger"]
    pub mod TRIGGER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No hardware trigger signal"]
            pub const TRIGGER_0: u32 = 0;
            #[doc = "Hardware trigger signal, the signal must last at least 1 ips clock period"]
            pub const TRIGGER_1: u32 = 0x01;
        }
    }
    #[doc = "ADC Coco Clear"]
    pub mod ADC_COCO_CLEAR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No ADC COCO clear"]
            pub const ADC_COCO_CLEAR_0: u32 = 0;
            #[doc = "Set ADC COCO clear"]
            pub const ADC_COCO_CLEAR_1: u32 = 0x01;
        }
    }
    #[doc = "ADC COCO Clear Disable"]
    pub mod ADC_COCO_CLEAR_DISABLE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow TSC hardware generates ADC COCO clear"]
            pub const ADC_COCO_CLEAR_DISABLE_0: u32 = 0;
            #[doc = "Prevent TSC from generate ADC COCO clear signal"]
            pub const ADC_COCO_CLEAR_DISABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DEBUG_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable debug mode"]
            pub const DEBUG_EN_0: u32 = 0;
            #[doc = "Disable debug mode"]
            pub const DEBUG_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "no description available"]
pub mod DEBUG_MODE2 {
    #[doc = "XPUL Wire Pull Down Switch"]
    pub mod XPUL_PULL_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const XPUL_PULL_DOWN_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const XPUL_PULL_DOWN_1: u32 = 0x01;
        }
    }
    #[doc = "XPUL Wire Pull Up Switch"]
    pub mod XPUL_PULL_UP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const XPUL_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const XPUL_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "XPUL Wire 200K Pull Up Switch"]
    pub mod XPUL_200K_PULL_UP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const XPUL_200K_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const XPUL_200K_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "XNUR Wire Pull Down Switch"]
    pub mod XNUR_PULL_DOWN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const XNUR_PULL_DOWN_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const XNUR_PULL_DOWN_1: u32 = 0x01;
        }
    }
    #[doc = "XNUR Wire Pull Up Switch"]
    pub mod XNUR_PULL_UP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const XNUR_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const XNUR_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "XNUR Wire 200K Pull Up Switch"]
    pub mod XNUR_200K_PULL_UP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const XNUR_200K_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const XNUR_200K_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "YPLL Wire Pull Down Switch"]
    pub mod YPLL_PULL_DOWN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const YPLL_PULL_DOWN_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const YPLL_PULL_DOWN_1: u32 = 0x01;
        }
    }
    #[doc = "YPLL Wire Pull Up Switch"]
    pub mod YPLL_PULL_UP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const YPLL_PULL_UP_0: u32 = 0;
            #[doc = "Open the switch"]
            pub const YPLL_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "YPLL Wire 200K Pull Up Switch"]
    pub mod YPLL_200K_PULL_UP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const YPLL_200K_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const YPLL_200K_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "YNLR Wire Pull Down Switch"]
    pub mod YNLR_PULL_DOWN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const YNLR_PULL_DOWN_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const YNLR_PULL_DOWN_1: u32 = 0x01;
        }
    }
    #[doc = "YNLR Wire Pull Up Switch"]
    pub mod YNLR_PULL_UP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const YNLR_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const YNLR_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "YNLR Wire 200K Pull Up Switch"]
    pub mod YNLR_200K_PULL_UP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const YNLR_200K_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const YNLR_200K_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "Wiper Wire Pull Down Switch"]
    pub mod WIPER_PULL_DOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const WIPER_PULL_DOWN_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const WIPER_PULL_DOWN_1: u32 = 0x01;
        }
    }
    #[doc = "Wiper Wire Pull Up Switch"]
    pub mod WIPER_PULL_UP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const WIPER_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const WIPER_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "Wiper Wire 200K Pull Up Switch"]
    pub mod WIPER_200K_PULL_UP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Close the switch"]
            pub const WIPER_200K_PULL_UP_0: u32 = 0;
            #[doc = "Open up the switch"]
            pub const WIPER_200K_PULL_UP_1: u32 = 0x01;
        }
    }
    #[doc = "Detect Four Wire"]
    pub mod DETECT_FOUR_WIRE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No detect signal"]
            pub const DETECT_FOUR_WIRE_0: u32 = 0;
            #[doc = "Yes, there is a detect on the touch screen."]
            pub const DETECT_FOUR_WIRE_1: u32 = 0x01;
        }
    }
    #[doc = "Detect Five Wire"]
    pub mod DETECT_FIVE_WIRE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No detect signal"]
            pub const DETECT_FIVE_WIRE_0: u32 = 0;
            #[doc = "Yes, there is a detect on the touch screen."]
            pub const DETECT_FIVE_WIRE_1: u32 = 0x01;
        }
    }
    #[doc = "State Machine"]
    pub mod STATE_MACHINE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle"]
            pub const STATE_MACHINE_0: u32 = 0;
            #[doc = "Pre-charge"]
            pub const STATE_MACHINE_1: u32 = 0x01;
            #[doc = "Detect"]
            pub const STATE_MACHINE_2: u32 = 0x02;
            #[doc = "X-measure"]
            pub const STATE_MACHINE_3: u32 = 0x03;
            #[doc = "Y-measure"]
            pub const STATE_MACHINE_4: u32 = 0x04;
            #[doc = "Pre-charge"]
            pub const STATE_MACHINE_5: u32 = 0x05;
            #[doc = "Detect"]
            pub const STATE_MACHINE_6: u32 = 0x06;
        }
    }
    #[doc = "Intermediate State"]
    pub mod INTERMEDIATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in intermedia"]
            pub const INTERMEDIATE_0: u32 = 0;
            #[doc = "Intermedia"]
            pub const INTERMEDIATE_1: u32 = 0x01;
        }
    }
    #[doc = "Detect Enable Four Wire"]
    pub mod DETECT_ENABLE_FOUR_WIRE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not read four wire detect value, read default value from analogue"]
            pub const DETECT_ENABLE_FOUR_WIRE_0: u32 = 0;
            #[doc = "Read four wire detect status from analogue"]
            pub const DETECT_ENABLE_FOUR_WIRE_1: u32 = 0x01;
        }
    }
    #[doc = "Detect Enable Five Wire"]
    pub mod DETECT_ENABLE_FIVE_WIRE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not read five wire detect value, read default value from analogue"]
            pub const DETECT_ENABLE_FIVE_WIRE_0: u32 = 0;
            #[doc = "Read five wire detect status from analogue"]
            pub const DETECT_ENABLE_FIVE_WIRE_1: u32 = 0x01;
        }
    }
    #[doc = "This field indicates glitch threshold"]
    pub mod DE_GLITCH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal function: 0x1fff ipg clock cycles; Low power mode: 0x9 low power clock cycles"]
            pub const DE_GLITCH_0: u32 = 0;
            #[doc = "Normal function: 0xfff ipg clock cycles; Low power mode: :0x7 low power clock cycles"]
            pub const DE_GLITCH_1: u32 = 0x01;
            #[doc = "Normal function: 0x7ff ipg clock cycles; Low power mode:0x5 low power clock cycles"]
            pub const DE_GLITCH_2: u32 = 0x02;
            #[doc = "Normal function: 0x3 ipg clock cycles; Low power mode:0x3 low power clock cycles"]
            pub const DE_GLITCH_3: u32 = 0x03;
        }
    }
}
