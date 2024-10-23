#[doc = "DCDC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DCDC Register 0"]
    pub REG0: crate::RWRegister<u32>,
    #[doc = "DCDC Register 1"]
    pub REG1: crate::RWRegister<u32>,
    #[doc = "DCDC Register 2"]
    pub REG2: crate::RWRegister<u32>,
    #[doc = "DCDC Register 3"]
    pub REG3: crate::RWRegister<u32>,
    #[doc = "DCDC Control Register 0"]
    pub CTRL0: crate::RWRegister<u32>,
    #[doc = "OK CNT"]
    pub OK_CNT: crate::RWRegister<u32>,
    #[doc = "CURRENT TARGET VALUE for DCDC ANALOG"]
    pub CURRENT_TRG: crate::RWRegister<u32>,
    #[doc = "FILTER CNT"]
    pub FILTER_CNT: crate::RWRegister<u32>,
    #[doc = "TRG_0 Authentication Control"]
    pub TRG_0_AUTHEN: crate::RWRegister<u32>,
    #[doc = "Target SW Control for CORE 0"]
    pub TRG_SW_0: crate::RWRegister<u32>,
    #[doc = "Target GPC Control for CORE 0"]
    pub TRG_GPC_0: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "TRG_1 Authentication Control"]
    pub TRG_1_AUTHEN: crate::RWRegister<u32>,
    #[doc = "Target SW Control for CORE 1"]
    pub TRG_SW_1: crate::RWRegister<u32>,
    #[doc = "Target GPC Control for CORE 1"]
    pub TRG_GPC_1: crate::RWRegister<u32>,
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
    #[doc = "Power down internal osc"]
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
    #[doc = "Overcurrent Trigger Adjust"]
    pub mod OVERCUR_TRIG_ADJ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "In Run Mode, 1.5 A. In LP Mode, 150 mA"]
            pub const SELECT_ZERO: u32 = 0;
            #[doc = "In Run Mode, 1.5 A. In LP Mode, 130 mA"]
            pub const SELECT_ONE: u32 = 0x01;
            #[doc = "In Run Mode, 2 A. In LP Mode, 150 mA"]
            pub const SELECT_TWO: u32 = 0x02;
            #[doc = "In Run Mode, 2 A. In LP Mode, 130 mA"]
            pub const SELECT_THREE: u32 = 0x03;
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
    #[doc = "Resistor Load of Regulator Enable"]
    pub mod RLOAD_REG_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Resistor load disconnected"]
            pub const RLOAD_DISCONNECT: u32 = 0;
            #[doc = "Resistor load connected"]
            pub const RLOAD_CONNECT: u32 = 0x01;
        }
    }
    #[doc = "Trim Bandgap Voltage Trim step is 3mV. 00000 - 0.452V 10000 - 0.5V 11111 - 0.545V"]
    pub mod VBG_TRIM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Negative duty cycle limit of DC-DC converter"]
    pub mod NEGLIMIT_IN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Increase Threshold Detection"]
    pub mod LOOPCTRL_CM_HST_THRESH {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable increase the threshold detection for common mode analog comparators."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable increase the threshold detection for common mode analog comparators."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Increase Threshold Detection"]
    pub mod LOOPCTRL_DF_HST_THRESH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable increase the threshold detection for differential mode analog comparators."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable increase the threshold detection for differential mode analog comparators."]
            pub const ENABLE: u32 = 0x01;
        }
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
    #[doc = "Ratio of integral control parameter to proportional control parameter in the switching DC-DC converter, and can be used to optimize efficiency and loop response"]
    pub mod LOOPCTRL_DC_C {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Magnitude of proportional control parameter in the switching DC-DC converter control loop."]
    pub mod LOOPCTRL_DC_R {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
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
        pub mod RW {
            #[doc = "Disable increasing the threshold detection function."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable increasing the threshold detection function."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Invert the sign of the hysteresis in DC-DC analog comparators."]
    pub mod LOOPCTRL_HYST_SIGN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the invert function."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable the invert function."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "This bit enables the DC-DC to improve efficiency and minimize ripple using the information from the BATTMONITOR_BATT_VAL field"]
    pub mod BATTMONITOR_EN_BATADJ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the improvement function."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable the improvement function."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Software should be configured to place the battery voltage in this register measured with an 8-mV LSB resolution through the ADC"]
    pub mod BATTMONITOR_BATT_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
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
        pub mod RW {
            #[doc = "Disable supply stepping to change."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable supply stepping to change."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DCDC Register 3"]
pub mod REG3 {
    #[doc = "The voltage on DCDC_IN is lower than 2.8V"]
    pub mod IN_BROWNOUT_WARN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The voltage on DCDC_IN raises up to 2.8V."]
            pub const NOT_WARNING: u32 = 0;
            #[doc = "The voltage on DCDC_IN is lower than 2.8V. Once this bit sets, the bit must be cleared by software write one clear action while the voltage on DCDC_IN raises up to 2.8V. Writing \"0\" to this bit has no effect. Writing \"1\" to this bit has no effect while it's \"0\"."]
            pub const WARNING: u32 = 0x01;
        }
    }
    #[doc = "Enable feed-forward (FF) function that can speed up transient settling."]
    pub mod ENABLE_FF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the FF function."]
            pub const ENABLE_0: u32 = 0;
            #[doc = "Disable the FF function."]
            pub const DISABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Disable Pulse Skip"]
    pub mod DISABLE_PULSE_SKIP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The DCDC will be idle when out-of-range comparator detects the output voltage is higher than the target by 25mV"]
    pub mod DISABLE_IDLE_SKIP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the idle skip function."]
            pub const ENABLE_0: u32 = 0;
            #[doc = "Disable the idle skip function."]
            pub const DISABLE_1: u32 = 0x01;
        }
    }
    #[doc = "Double the bias current of the comparator for low-voltage detector in LP (low-power) mode."]
    pub mod DOUBLE_IBIAS_CMP_LP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the function."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable the function."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Select the feedback point of the internal regulator. No need to change this field in user mode."]
    pub mod REG_FBK_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use half switch FET"]
    pub mod MINPWR_HALF_FETS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Donot use half switch FET."]
            pub const NO_HALF_S_FET: u32 = 0;
            #[doc = "Use half switch FET."]
            pub const USE_HALF_S_FET: u32 = 0x01;
        }
    }
    #[doc = "Miscellaneous Delay Timing"]
    pub mod MISC_DELAY_TIMING {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the function."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable the function."]
            pub const ENABLE: u32 = 0x01;
        }
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
}
#[doc = "DCDC Control Register 0"]
pub mod CTRL0 {
    #[doc = "Enable internal count for DCDC_OK timeout"]
    pub mod ENABLE_OK_CNT {
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
    #[doc = "IN_BROWNOUT_WARN_EN"]
    pub mod IN_BROWNOUT_WARN_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable IN_BROWNOUT_WARN int flag bit output to CORE as an interrupt resource."]
            pub const NO_BROWNOUT_WARN: u32 = 0;
            #[doc = "Enable IN_BROWNOUT_WARN int flag bit output to CORE as an interrupt resource."]
            pub const BROWNOUT_WARN: u32 = 0x01;
        }
    }
    #[doc = "Set to 0x1: To improve loading ability under heavy load."]
    pub mod DEBUG_BITS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRG_GPC_EN: used to enable TRG_GPC_* value or not."]
    pub mod TRG_GPC_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No matter there is GPC stby request or not, value in TRG_SW_* register will always be used as DCDC analog target value."]
            pub const NO_GPC_EN: u32 = 0;
            #[doc = "When there is a GPC stby request, value in TRG_GPC_* register will be used as DCDC analog target value instead of TRG_SW_*'s"]
            pub const GPC_EN: u32 = 0x01;
        }
    }
}
#[doc = "OK CNT"]
pub mod OK_CNT {
    #[doc = "OK_COUNT"]
    pub mod OK_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CURRENT TARGET VALUE for DCDC ANALOG"]
pub mod CURRENT_TRG {
    #[doc = "This value is current value used by DCDC analog"]
    pub mod VDD1P0CTRL_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value is current value used by DCDC analog"]
    pub mod VDD1P8CTRL_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Notice: Only if the time of the bit's 1 is too long(more than several seconds) and REG0\\[STS_DC_OK\\]=1, then you can write 1 to clear it"]
    pub mod DCDC_UPDATING {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last DCDC change has been done. New value can be written to TRG register to trigger a new change of DCDC voltage."]
            pub const NO_UPDATING: u32 = 0;
            #[doc = "Last DCDC change is still not done. New value should not be written to TRG register to trigger a new change of DCDC voltage."]
            pub const UPDATING: u32 = 0x01;
        }
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    pub mod VDD1P0CTRL_LP_TRG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value comes from the smaller one between TRG_SW_0 and TRG_SW_1. This bit only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    pub mod LP_EN_1P0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DCDC 1P0 works in run mode. Its output voltage is controlled by VDD1P0CTRL_TRG."]
            pub const VDD1P0_NORMAL_MODE: u32 = 0;
            #[doc = "DCDC 1P0 works in low power mode. Its output voltage is controlled by VDD1P0CTRL_LP_TRG and its output current is less than 50mA."]
            pub const VDD1P0_LP_MODE: u32 = 0x01;
        }
    }
}
#[doc = "FILTER CNT"]
pub mod FILTER_CNT {
    #[doc = "FILTER_CNT_CFG"]
    pub mod FILTER_CNT_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRG_0 Authentication Control"]
pub mod TRG_0_AUTHEN {
    #[doc = "Allow user mode write"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRG_0 registers can only be written in privilege mode."]
            pub const PRIV_ONLY: u32 = 0;
            #[doc = "TRG_0 registers can be written either in privilege mode or user mode."]
            pub const PRIV_OR_USER: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure mode access"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRG_0 registers can only be written in secure mode."]
            pub const SECURE_ONLY: u32 = 0;
            #[doc = "TRG_0 registers can be written either in secure mode or non-secure mode."]
            pub const SEC_OR_NON_SEC: u32 = 0x01;
        }
    }
    #[doc = "Lock TZ_NS and TZ_USER"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TZ_NS and TZ_USER value can be changed."]
            pub const CHANGE: u32 = 0;
            #[doc = "LOCK_TZ, TZ_NS and TZ_USER value cannot be changed."]
            pub const NO_CHANGE: u32 = 0x01;
        }
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WHITE_LIST value can be changed."]
            pub const CHANGE: u32 = 0;
            #[doc = "LOCK_LIST and WHITE_LIST value cannot be changed."]
            pub const NO_CHANGE: u32 = 0x01;
        }
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Core with domain ID=0 can write TRG_0 registers."]
            pub const DOMAIN0: u32 = 0x01;
            #[doc = "Core with domain ID=1 can write TRG_0 registers."]
            pub const DOMAIN1: u32 = 0x02;
            #[doc = "Core with domain ID=2 can write TRG_0 registers."]
            pub const DOMAIN2: u32 = 0x04;
            #[doc = "Core with domain ID=3 can write TRG_0 registers."]
            pub const DOMAIN3: u32 = 0x08;
            #[doc = "Core with domain ID=4 can write TRG_0 registers."]
            pub const DOMAIN4: u32 = 0x10;
            #[doc = "Core with domain ID=5 can write TRG_0 registers."]
            pub const DOMAIN5: u32 = 0x20;
            #[doc = "Core with domain ID=6 can write TRG_0 registers."]
            pub const DOMAIN6: u32 = 0x40;
            #[doc = "Core with domain ID=7 can write TRG_0 registers."]
            pub const DOMAIN7: u32 = 0x80;
            #[doc = "Core with domain ID=8 can write TRG_0 registers."]
            pub const DOMAIN8: u32 = 0x0100;
            #[doc = "Core with domain ID=9 can write TRG_0 registers."]
            pub const DOMAIN9: u32 = 0x0200;
            #[doc = "Core with domain ID=10 can write TRG_0 registers."]
            pub const DOMAIN10: u32 = 0x0400;
            #[doc = "Core with domain ID=11 can write TRG_0 registers."]
            pub const DOMAIN11: u32 = 0x0800;
            #[doc = "Core with domain ID=12 can write TRG_0 registers."]
            pub const DOMAIN12: u32 = 0x1000;
            #[doc = "Core with domain ID=13 can write TRG_0 registers."]
            pub const DOMAIN13: u32 = 0x2000;
            #[doc = "Core with domain ID=14 can write TRG_0 registers."]
            pub const DOMAIN14: u32 = 0x4000;
            #[doc = "Core with domain ID=15 can write TRG_0 registers."]
            pub const DOMAIN15: u32 = 0x8000;
        }
    }
}
#[doc = "Target SW Control for CORE 0"]
pub mod TRG_SW_0 {
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    pub mod VDD1P0CTRL_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    pub mod VDD1P8CTRL_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    pub mod VDD1P0CTRL_LP_TRG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    pub mod LP_EN_1P0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target GPC Control for CORE 0"]
pub mod TRG_GPC_0 {
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    pub mod VDD1P0CTRL_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    pub mod VDD1P8CTRL_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    pub mod VDD1P0CTRL_LP_TRG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    pub mod LP_EN_1P0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRG_1 Authentication Control"]
pub mod TRG_1_AUTHEN {
    #[doc = "Allow user mode write"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRG_1 registers can only be written in privilege mode."]
            pub const PRIV_ONLY: u32 = 0;
            #[doc = "TRG_1 registers can be written either in privilege mode or user mode."]
            pub const PRIV_OR_USER: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure mode access"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRG_1 registers can only be written in secure mode."]
            pub const SECURE_ONLY: u32 = 0;
            #[doc = "TRG_1 registers can be written either in secure mode or non-secure mode."]
            pub const SEC_OR_NON_SEC: u32 = 0x01;
        }
    }
    #[doc = "Lock TZ_NS and TZ_USER"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TZ_NS and TZ_USER value can be changed."]
            pub const CHANGE: u32 = 0;
            #[doc = "LOCK_TZ, TZ_NS and TZ_USER value cannot be changed."]
            pub const NO_CHANGE: u32 = 0x01;
        }
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WHITE_LIST value can be changed."]
            pub const CHANGE: u32 = 0;
            #[doc = "LOCK_LIST and WHITE_LIST value cannot be changed."]
            pub const NO_CHANGE: u32 = 0x01;
        }
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Core with domain ID=0 can write TRG_1 registers."]
            pub const DOMAIN0: u32 = 0x01;
            #[doc = "Core with domain ID=1 can write TRG_1 registers."]
            pub const DOMAIN1: u32 = 0x02;
            #[doc = "Core with domain ID=2 can write TRG_1 registers."]
            pub const DOMAIN2: u32 = 0x04;
            #[doc = "Core with domain ID=3 can write TRG_1 registers."]
            pub const DOMAIN3: u32 = 0x08;
            #[doc = "Core with domain ID=4 can write TRG_1 registers."]
            pub const DOMAIN4: u32 = 0x10;
            #[doc = "Core with domain ID=5 can write TRG_1 registers."]
            pub const DOMAIN5: u32 = 0x20;
            #[doc = "Core with domain ID=6 can write TRG_1 registers."]
            pub const DOMAIN6: u32 = 0x40;
            #[doc = "Core with domain ID=7 can write TRG_1 registers."]
            pub const DOMAIN7: u32 = 0x80;
            #[doc = "Core with domain ID=8 can write TRG_1 registers."]
            pub const DOMAIN8: u32 = 0x0100;
            #[doc = "Core with domain ID=9 can write TRG_1 registers."]
            pub const DOMAIN9: u32 = 0x0200;
            #[doc = "Core with domain ID=10 can write TRG_1 registers."]
            pub const DOMAIN10: u32 = 0x0400;
            #[doc = "Core with domain ID=11 can write TRG_1 registers."]
            pub const DOMAIN11: u32 = 0x0800;
            #[doc = "Core with domain ID=12 can write TRG_1 registers."]
            pub const DOMAIN12: u32 = 0x1000;
            #[doc = "Core with domain ID=13 can write TRG_1 registers."]
            pub const DOMAIN13: u32 = 0x2000;
            #[doc = "Core with domain ID=14 can write TRG_1 registers."]
            pub const DOMAIN14: u32 = 0x4000;
            #[doc = "Core with domain ID=15 can write TRG_1 registers."]
            pub const DOMAIN15: u32 = 0x8000;
        }
    }
}
#[doc = "Target SW Control for CORE 1"]
pub mod TRG_SW_1 {
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    pub mod VDD1P0CTRL_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    pub mod VDD1P8CTRL_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    pub mod VDD1P0CTRL_LP_TRG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    pub mod LP_EN_1P0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target GPC Control for CORE 1"]
pub mod TRG_GPC_1 {
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    pub mod VDD1P0CTRL_TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    pub mod VDD1P8CTRL_TRG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    pub mod VDD1P0CTRL_LP_TRG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    pub mod LP_EN_1P0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
