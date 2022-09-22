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
}
#[doc = "DCDC Register 0"]
pub mod REG0 {
    #[doc = "power down the zero cross detection function for discontinuous conductor mode"]
    pub mod PWD_ZCD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable automatic clock switch from internal osc to xtal clock."]
    pub mod DISABLE_AUTO_CLK_SWITCH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select 24 MHz Crystal clock for DCDC, when dcdc_disable_auto_clk_switch is set."]
    pub mod SEL_CLK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down internal osc. Only set this bit, when 24 MHz crystal osc is available"]
    pub mod PWD_OSC_INT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The power down signal of the current detector."]
    pub mod PWD_CUR_SNS_CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the threshold of current detector, if the peak current of the inductor exceeds the threshold, the current detector will assert"]
    pub mod CUR_SNS_THRSH {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power down overcurrent detection comparator"]
    pub mod PWD_OVERCUR_DET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The threshold of over current detection in run mode and power save mode: run mode power save mode 0x0 1 A 0"]
    pub mod OVERCUR_TRIG_ADJ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to \"1\" to power down the low voltage detection comparator"]
    pub mod PWD_CMP_BATT_DET {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "adjust value to poslimit_buck register"]
    pub mod ADJ_POSLIMIT_BUCK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable the overload detection in power save mode, if current is larger than the overloading threshold (typical value is 50 mA), DCDC will switch to the run mode automatically"]
    pub mod EN_LP_OVERLOAD_SNS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power down overvoltage detection comparator"]
    pub mod PWD_HIGH_VOLT_DET {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the threshold of the counting number of charging times during the period that lp_overload_freq_sel sets in power save mode"]
    pub mod LP_OVERLOAD_THRSH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the period of counting the charging times in power save mode 0: eight 32k cycle 1: sixteen 32k cycle"]
    pub mod LP_OVERLOAD_FREQ_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Adjust hysteretic value in low power from 12.5mV to 25mV"]
    pub mod LP_HIGH_HYS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power down output range comparator"]
    pub mod PWD_CMP_OFFSET {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'b1: Disable xtalok detection circuit 1'b0: Enable xtalok detection circuit"]
    pub mod XTALOK_DISABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset current alert signal"]
    pub mod CURRENT_ALERT_RESET {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to switch internal ring osc to xtal 24M"]
    pub mod XTAL_24M_OK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status register to indicate DCDC status. 1'b1: DCDC already settled 1'b0: DCDC is settling"]
    pub mod STS_DC_OK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 1"]
pub mod REG1 {
    #[doc = "select the feedback point of the internal regulator"]
    pub mod REG_FBK_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "control the load resistor of the internal regulator of DCDC, the load resistor is connected as default \"1\", and need set to \"0\" to disconnect the load resistor"]
    pub mod REG_RLOAD_SW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set the current bias of low power comparator 0x0: 50 nA 0x1: 100 nA 0x2: 200 nA 0x3: 400 nA"]
    pub mod LP_CMP_ISRC_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "increase the threshold detection for common mode analog comparator"]
    pub mod LOOPCTRL_HST_THRESH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable hysteresis in switching converter common mode analog comparators"]
    pub mod LOOPCTRL_EN_HYST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "trim bandgap voltage"]
    pub mod VBG_TRIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
    #[doc = "Enable analog circuit of DC-DC converter to respond faster under transient load conditions."]
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
    #[doc = "Invert the sign of the hysteresis in DC-DC analog comparators."]
    pub mod LOOPCTRL_HYST_SIGN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enables the DC-DC to improve efficiency and minimize ripple using the information from the BATT_VAL field"]
    pub mod BATTMONITOR_EN_BATADJ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set to \"0\" : stop charging if the duty cycle is lower than what set by dcdc_neglimit_in"]
    pub mod DISABLE_PULSE_SKIP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set high to improve the transition from heavy load to light load"]
    pub mod DCM_SET_CTRL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Register 3"]
pub mod REG3 {
    #[doc = "Target value of VDD_SOC, 25 mV each step 0x0: 0.8V 0xE: 1.15V 0x1F:1.575V"]
    pub mod TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target value of standby (low power) mode 0x0: 0"]
    pub mod TARGET_LP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set DCDC clock to half freqeuncy for continuous mode"]
    pub mod MINPWR_DC_HALFCLK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ajust delay to reduce ground noise"]
    pub mod MISC_DELAY_TIMING {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved"]
    pub mod MISC_DISABLEFET_LOGIC {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable stepping for the output VDD_SOC of DCDC"]
    pub mod DISABLE_STEP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
