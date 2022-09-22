#[doc = "DCDC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DCDC Control Register 0"]
    pub CTRL0: crate::RWRegister<u32>,
    #[doc = "DCDC Control Register 1"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "DCDC Register 0"]
    pub REG0: crate::RWRegister<u32>,
    #[doc = "DCDC Register 1"]
    pub REG1: crate::RWRegister<u32>,
    #[doc = "DCDC Register 2"]
    pub REG2: crate::RWRegister<u32>,
    #[doc = "DCDC Register 3"]
    pub REG3: crate::RWRegister<u32>,
    #[doc = "DCDC Register 4"]
    pub REG4: crate::RWRegister<u32>,
    #[doc = "DCDC Register 5"]
    pub REG5: crate::RWRegister<u32>,
    #[doc = "DCDC Register 6"]
    pub REG6: crate::RWRegister<u32>,
    #[doc = "DCDC Register 7"]
    pub REG7: crate::RWRegister<u32>,
    #[doc = "DCDC Register 7 plus"]
    pub REG7P: crate::RWRegister<u32>,
    #[doc = "DCDC Register 8"]
    pub REG8: crate::RWRegister<u32>,
    #[doc = "DCDC Register 9"]
    pub REG9: crate::RWRegister<u32>,
    #[doc = "DCDC Register 10"]
    pub REG10: crate::RWRegister<u32>,
    #[doc = "DCDC Register 11"]
    pub REG11: crate::RWRegister<u32>,
    #[doc = "DCDC Register 12"]
    pub REG12: crate::RWRegister<u32>,
    #[doc = "DCDC Register 13"]
    pub REG13: crate::RWRegister<u32>,
    #[doc = "DCDC Register 14"]
    pub REG14: crate::RWRegister<u32>,
    #[doc = "DCDC Register 15"]
    pub REG15: crate::RWRegister<u32>,
    #[doc = "DCDC Register 16"]
    pub REG16: crate::RWRegister<u32>,
    #[doc = "DCDC Register 17"]
    pub REG17: crate::RWRegister<u32>,
    #[doc = "DCDC Register 18"]
    pub REG18: crate::RWRegister<u32>,
    #[doc = "DCDC Register 19"]
    pub REG19: crate::RWRegister<u32>,
    #[doc = "DCDC Register 20"]
    pub REG20: crate::RWRegister<u32>,
    #[doc = "DCDC Register 21"]
    pub REG21: crate::RWRegister<u32>,
    #[doc = "DCDC Register 22"]
    pub REG22: crate::RWRegister<u32>,
    #[doc = "DCDC Register 23"]
    pub REG23: crate::RWRegister<u32>,
    #[doc = "DCDC Register 24"]
    pub REG24: crate::RWRegister<u32>,
}
#[doc = "DCDC Control Register 0"]
pub mod CTRL0 {
    #[doc = "DCDC Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable (Bypass)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable the DCDC_DIG switching converter output"]
    pub mod DIG_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DCDC standby mode enable"]
    pub mod STBY_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enter into standby mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DCDC low-power (LP) mode enable DCDC can't start up directly into LP mode"]
    pub mod LP_MODE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enter into low-power mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DCDC low-power mode enable by GPC standby request"]
    pub mod STBY_LP_MODE_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable DCDC entry into low-power mode from a GPC standby request"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable DCDC to enter into low-power mode from a GPC standby request"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable internal count for DCDC_OK timeout"]
    pub mod ENABLE_DCDC_CNT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wait DCDC_OK for ACK"]
            pub const WAIT: u32 = 0;
            #[doc = "Enable internal count for DCDC_OK timeout"]
            pub const ENABLE_COUNT: u32 = 0x01;
        }
    }
    #[doc = "Hold trim input"]
    pub mod TRIM_HOLD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sample trim input"]
            pub const SAMPLE: u32 = 0;
            #[doc = "Hold trim input"]
            pub const HOLD: u32 = 0x01;
        }
    }
    #[doc = "DEBUG_BITS\\[11:0\\]"]
    pub mod DEBUG_BITS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control mode"]
    pub mod CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software control mode"]
            pub const SWCTRL: u32 = 0;
            #[doc = "Hardware control mode (controlled by GPC Setpoints)"]
            pub const GPC: u32 = 0x01;
        }
    }
}
#[doc = "DCDC Control Register 1"]
pub mod CTRL1 {
    #[doc = "Target value of VDD1P8 in buck mode, 25mV each step from 0x00 to 0x1F:"]
    pub mod VDD1P8CTRL_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1.5V"]
            pub const V1P5: u32 = 0;
            #[doc = "1.8V"]
            pub const V1P8: u32 = 0x0c;
            #[doc = "2.275V"]
            pub const V2P275: u32 = 0x1f;
        }
    }
    #[doc = "Target value of VDD1P0 in buck mode, 25mV each step from 0x00 to 0x1F:"]
    pub mod VDD1P0CTRL_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.6V"]
            pub const V0P6: u32 = 0;
            #[doc = "1.0V"]
            pub const V1P0: u32 = 0x10;
            #[doc = "1.375V"]
            pub const V1P375: u32 = 0x1f;
        }
    }
    #[doc = "Target value of VDD1P8 in standby mode, 25mV each step from 0x00 to 0x1F:"]
    pub mod VDD1P8CTRL_STBY_TRG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1.525V"]
            pub const V1P525: u32 = 0;
            #[doc = "1.8V"]
            pub const V1P8: u32 = 0x0b;
            #[doc = "2.3V"]
            pub const V2P4: u32 = 0x1f;
        }
    }
    #[doc = "Target value of VDD1P0 in standby mode, 25mV each step from 0x00 to 0x1F:"]
    pub mod VDD1P0CTRL_STBY_TRG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.625V"]
            pub const V0P625: u32 = 0;
            #[doc = "1.0V"]
            pub const V1P0: u32 = 0x0f;
            #[doc = "1.4V"]
            pub const V1P4: u32 = 0x1f;
        }
    }
}
#[doc = "DCDC Register 0"]
pub mod REG0 {
    #[doc = "Power Down Zero Cross Detection"]
    pub mod PWD_ZCD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero cross detetion function powered up"]
            pub const POWERED_UP: u32 = 0;
            #[doc = "Zero cross detetion function powered down"]
            pub const POWERED_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Disable Auto Clock Switch"]
    pub mod DISABLE_AUTO_CLK_SWITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "If DISABLE_AUTO_CLK_SWITCH is set to 0 and 24M xtal is OK, the clock source will switch from internal ring oscillator to 24M xtal automatically"]
            pub const XTAL_CLK: u32 = 0;
            #[doc = "If DISABLE_AUTO_CLK_SWITCH is set to 1, SEL_CLK will determine which clock source the DCDC uses"]
            pub const SEL_CLK: u32 = 0x01;
        }
    }
    #[doc = "Select Clock"]
    pub mod SEL_CLK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DCDC uses internal ring oscillator"]
            pub const INT_RNG_OSC: u32 = 0;
            #[doc = "DCDC uses 24M xtal"]
            pub const XTAL_24M: u32 = 0x01;
        }
    }
    #[doc = "Power down internal ring oscillator"]
    pub mod PWD_OSC_INT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal ring oscillator powered up"]
            pub const POWERED_UP: u32 = 0;
            #[doc = "Internal ring oscillator powered down"]
            pub const POWERED_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Power down signal of the current detector"]
    pub mod PWD_CUR_SNS_CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Current Detector powered up"]
            pub const POWERED_UP: u32 = 0;
            #[doc = "Current Detector powered down"]
            pub const POWERED_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Current Sense (detector) Threshold"]
    pub mod CUR_SNS_THRSH {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down overcurrent detection comparator"]
    pub mod PWD_OVERCUR_DET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overcurrent detection comparator is enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Overcurrent detection comparator is disabled"]
            pub const DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Set to \"1\" to power down the low voltage detection comparator"]
    pub mod PWD_CMP_DCDC_IN_DET {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low voltage detection comparator is enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Low voltage detection comparator is disabled"]
            pub const DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Power Down High Voltage Detection for VDD1P8"]
    pub mod PWD_HIGH_VDD1P8_DET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overvoltage detection comparator for the VDD1P8 output is enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Overvoltage detection comparator for the VDD1P8 output is disabled"]
            pub const DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Power Down High Voltage Detection for VDD1P0"]
    pub mod PWD_HIGH_VDD1P0_DET {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overvoltage detection comparator for the VDD1P0 output is enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Overvoltage detection comparator for the VDD1P0 output is disabled"]
            pub const DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Low Power High Hysteric Value"]
    pub mod LP_HIGH_HYS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Adjust hysteretic value in low power to 12.5mV"]
            pub const LP_12P5MV: u32 = 0;
            #[doc = "Adjust hysteretic value in low power to 25mV"]
            pub const LP_25MV: u32 = 0x01;
        }
    }
    #[doc = "power down the out-of-range detection comparator"]
    pub mod PWD_CMP_OFFSET {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Out-of-range comparator powered up"]
            pub const POWERED_UP: u32 = 0;
            #[doc = "Out-of-range comparator powered down"]
            pub const POWERED_DOWN: u32 = 0x01;
        }
    }
    #[doc = "Disable xtalok detection circuit"]
    pub mod XTALOK_DISABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable xtalok detection circuit"]
            pub const ENABLED: u32 = 0;
            #[doc = "Disable xtalok detection circuit and always outputs OK signal \"1\""]
            pub const DISABLED: u32 = 0x01;
        }
    }
    #[doc = "24M XTAL OK"]
    pub mod XTAL_24M_OK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DCDC uses internal ring oscillator"]
            pub const INT_RNG_OSC: u32 = 0;
            #[doc = "DCDC uses xtal 24M"]
            pub const XTAL_24M: u32 = 0x01;
        }
    }
    #[doc = "DCDC Output OK"]
    pub mod STS_DC_OK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DCDC is settling"]
            pub const NOT_SETTLED: u32 = 0;
            #[doc = "DCDC already settled"]
            pub const SETTLED: u32 = 0x01;
        }
    }
}
#[doc = "DCDC Register 1"]
pub mod REG1 {
    #[doc = "DM Control"]
    pub mod DM_CTRL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change to ripple when the discontinuous current is present in DCM."]
            pub const DM_CTRL_0: u32 = 0;
            #[doc = "Improves ripple when the inductor current goes to zero in DCM."]
            pub const DM_CTRL_1: u32 = 0x01;
        }
    }
    #[doc = "Load Resistor Enable"]
    pub mod RLOAD_REG_EN_LPSR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disconnect load resistor"]
            pub const LOADR_DISCONNECT: u32 = 0;
            #[doc = "Connect load resistor"]
            pub const LOADR_CONNECT: u32 = 0x01;
        }
    }
    #[doc = "Trim Bandgap Voltage"]
    pub mod VBG_TRIM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.452V"]
            pub const MINVOLT: u32 = 0;
            #[doc = "0.5V"]
            pub const DEFAULT: u32 = 0x10;
            #[doc = "0.545V"]
            pub const MAXVOLT: u32 = 0x1f;
        }
    }
    #[doc = "Low Power Comparator Current Bias"]
    pub mod LP_CMP_ISRC_SEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "50nA"]
            pub const SEL0: u32 = 0;
            #[doc = "100nA"]
            pub const SEL1: u32 = 0x01;
            #[doc = "200nA"]
            pub const SEL2: u32 = 0x02;
            #[doc = "400nA"]
            pub const SEL3: u32 = 0x03;
        }
    }
    #[doc = "Increase Threshold Detection"]
    pub mod LOOPCTRL_CM_HST_THRESH {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Increase Threshold Detection"]
    pub mod LOOPCTRL_DF_HST_THRESH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable hysteresis in switching converter common mode analog comparators"]
    pub mod LOOPCTRL_EN_CM_HYST {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable hysteresis in switching converter common mode analog comparators"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable hysteresis in switching converter common mode analog comparators"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable hysteresis in switching converter differential mode analog comparators"]
    pub mod LOOPCTRL_EN_DF_HYST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable hysteresis in switching converter differential mode analog comparators"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable hysteresis in switching converter differential mode analog comparators"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DCDC Register 2"]
pub mod REG2 {
    #[doc = "Ratio of integral control parameter to proportional control parameter in the switching DCDC converter, and can be used to optimize efficiency and loop response"]
    pub mod LOOPCTRL_DC_C {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Magnitude of proportional control parameter in the switching DCDC converter control loop."]
    pub mod LOOPCTRL_DC_R {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's complement feed forward step in duty cycle in the switching DCDC converter"]
    pub mod LOOPCTRL_DC_FF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable RC Scale"]
    pub mod LOOPCTRL_EN_RCSCALE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Increase the threshold detection for RC scale circuit."]
    pub mod LOOPCTRL_RCSCALE_THRSH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the sign of the hysteresis in DCDC analog comparators."]
    pub mod LOOPCTRL_HYST_SIGN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the DCDC to improve efficiency and minimize ripple using the information from the BATT_VAL field"]
    pub mod BATTMONITOR_EN_BATADJ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software should be configured to place the battery voltage in this register measured with an 8-mV LSB resolution through the ADC"]
    pub mod BATTMONITOR_BATT_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCM Set Control"]
    pub mod DCM_SET_CTRL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set high to enable supply stepping to change only after the differential control loop has toggled as well"]
    pub mod LOOPCTRL_TOGGLE_DIF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 3"]
pub mod REG3 {
    #[doc = "signal \"1\" when the voltage on DCDC_IN is lower than 2.6V"]
    pub mod IN_BROWNOUT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DCDC_IN is lower than 2.6V"]
            pub const BROWNOUT: u32 = 0x01;
        }
    }
    #[doc = "signal \"1\" when overvoltage on the VDD1P8 output happens"]
    pub mod OVERVOLT_VDD1P8_DET_OUT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VDD1P8 Overvoltage"]
            pub const OVERVOLTAGE_1P8: u32 = 0x01;
        }
    }
    #[doc = "signal \"1\" when overvoltage on the VDD1P0 output happens"]
    pub mod OVERVOLT_VDD1P0_DET_OUT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VDD1P0 Overvoltage"]
            pub const OVERVOLTAGE_1P0: u32 = 0x01;
        }
    }
    #[doc = "signal \"1\" when overcurrent happens."]
    pub mod OVERCUR_DETECT_OUT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overcurrent"]
            pub const OVERCURRENT_SIGNAL: u32 = 0x01;
        }
    }
    #[doc = "no description available"]
    pub mod ENABLE_FF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable feed-forward (FF) function that can speed up transient settling."]
            pub const ENABLE_FF: u32 = 0x01;
        }
    }
    #[doc = "Disable Pulse Skip"]
    pub mod DISABLE_PULSE_SKIP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop charging if the duty cycle is lower than what is set by NEGLIMIT_IN"]
            pub const STOPCHARGE: u32 = 0;
        }
    }
    #[doc = "no description available"]
    pub mod DISABLE_IDLE_SKIP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the idle skip function. The DCDC will be idle when out-of-range comparator detects the output voltage is higher than the target by 25mV. This function requires the out-of-range comparator to be enabled (PWD_CMP_OFFSET=0)."]
            pub const ENABLE: u32 = 0;
        }
    }
    #[doc = "no description available"]
    pub mod DOUBLE_IBIAS_CMP_LP_LPSR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double the bias current of the comparator for low-voltage detector in LP (low-power) mode"]
            pub const DOUBLEBIAS: u32 = 0x01;
        }
    }
    #[doc = "Select the feedback point of the internal regulator"]
    pub mod REG_FBK_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set DCDC clock to half freqeuncy for continuous mode."]
    pub mod MINPWR_DC_HALFCLK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DCDC clock remains at full frequency for continuous mode"]
            pub const FULLFREQ: u32 = 0;
            #[doc = "DCDC clock set to half frequency for continuous mode"]
            pub const HALFFREQ: u32 = 0x01;
        }
    }
    #[doc = "Use half switch FET"]
    pub mod MINPWR_HALF_FETS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Miscellaneous Delay Timing"]
    pub mod MISC_DELAY_TIMING {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Step for VDD1P0"]
    pub mod VDD1P0CTRL_DISABLE_STEP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable stepping for VDD1P0"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable stepping for VDD1P0"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Step for VDD1P8"]
    pub mod VDD1P8CTRL_DISABLE_STEP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable stepping for VDD1P8"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable stepping for VDD1P8"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "DCDC Register 4"]
pub mod REG4 {
    #[doc = "Configures CTRL0\\[ENABLE\\] (DCDC Enable) for Setpoints 0-15"]
    pub mod ENABLE_SP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 5"]
pub mod REG5 {
    #[doc = "Configures CTRL0\\[DIG_EN\\] (DCDC_DIG Enable) for Setpoints 0-15. Always set these bits to 1."]
    pub mod DIG_EN_SP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 6"]
pub mod REG6 {
    #[doc = "Configures CTRL0\\[LP_MODE_EN\\] (LP Mode Enable) for Setpoints 0-15"]
    pub mod LP_MODE_SP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 7"]
pub mod REG7 {
    #[doc = "Configures CTRL0\\[STBY_EN\\] (Standby Enable) for Setpoints 0-15"]
    pub mod STBY_EN_SP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 7 plus"]
pub mod REG7P {
    #[doc = "Configures CTRL0\\[STBY_LP_MODE_EN\\] (LP Mode via GPC Enable) for Setpoints 0-15"]
    pub mod STBY_LP_MODE_SP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 8"]
pub mod REG8 {
    #[doc = "Configures CTRL1\\[VDD1P8CTRL_TRG\\] FOR Setpoints 0-3"]
    pub mod ANA_TRG_SP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 9"]
pub mod REG9 {
    #[doc = "Configures CTRL1\\[VDD1P8CTRL_TRG\\] FOR Setpoints 4-7"]
    pub mod ANA_TRG_SP1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 10"]
pub mod REG10 {
    #[doc = "Configures CTRL1\\[VDD1P8CTRL_TRG\\] FOR Setpoints 8-11"]
    pub mod ANA_TRG_SP2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 11"]
pub mod REG11 {
    #[doc = "Configures CTRL1\\[VDD1P8CTRL_TRG\\] FOR Setpoints 12-15"]
    pub mod ANA_TRG_SP3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 12"]
pub mod REG12 {
    #[doc = "Configures CTRL1\\[VDD1P0CTRL_TRG\\] FOR Setpoints 0-3"]
    pub mod DIG_TRG_SP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 13"]
pub mod REG13 {
    #[doc = "Configures CTRL1\\[VDD1P0CTRL_TRG\\] FOR Setpoints 4-7"]
    pub mod DIG_TRG_SP1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 14"]
pub mod REG14 {
    #[doc = "Configures CTRL1\\[VDD1P0CTRL_TRG\\] FOR Setpoints 8-11"]
    pub mod DIG_TRG_SP2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 15"]
pub mod REG15 {
    #[doc = "Configures CTRL1\\[VDD1P0CTRL_TRG\\] FOR Setpoints 12-15"]
    pub mod DIG_TRG_SP3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 16"]
pub mod REG16 {
    #[doc = "Configures CTRL1\\[VDD1P8CTRL_STBY_TRG\\] FOR Setpoints 0-3"]
    pub mod ANA_STBY_TRG_SP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 17"]
pub mod REG17 {
    #[doc = "Configures CTRL1\\[VDD1P8CTRL_STBY_TRG\\] FOR Setpoints 4-7"]
    pub mod ANA_STBY_TRG_SP1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 18"]
pub mod REG18 {
    #[doc = "Configures CTRL1\\[VDD1P8CTRL_STBY_TRG\\] FOR Setpoints 8-11"]
    pub mod ANA_STBY_TRG_SP2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 19"]
pub mod REG19 {
    #[doc = "Configures CTRL1\\[VDD1P8CTRL_STBY_TRG\\] FOR Setpoints 12-15"]
    pub mod ANA_STBY_TRG_SP3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 20"]
pub mod REG20 {
    #[doc = "Configures CTRL1\\[VDD1P0CTRL_STBY_TRG\\] FOR Setpoints 0-3"]
    pub mod DIG_STBY_TRG_SP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 21"]
pub mod REG21 {
    #[doc = "Configures CTRL1\\[VDD1P0CTRL_STBY_TRG\\] FOR Setpoints 4-7"]
    pub mod DIG_STBY_TRG_SP1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 22"]
pub mod REG22 {
    #[doc = "Configures CTRL1\\[VDD1P0CTRL_STBY_TRG\\] FOR Setpoints 8-11"]
    pub mod DIG_STBY_TRG_SP2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 23"]
pub mod REG23 {
    #[doc = "Configures CTRL1\\[VDD1P0CTRL_STBY_TRG\\] FOR Setpoints 12-15"]
    pub mod DIG_STBY_TRG_SP3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 24"]
pub mod REG24 {
    #[doc = "Internal count for dcdc_ok timeout"]
    pub mod OK_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
