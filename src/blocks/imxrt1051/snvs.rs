#[doc = "SNVS"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SNVS_HP Lock Register"]
    pub HPLR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Command Register"]
    pub HPCOMR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Control Register"]
    pub HPCR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Security Interrupt Control Register"]
    pub HPSICR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Security Violation Control Register"]
    pub HPSVCR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Status Register"]
    pub HPSR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Security Violation Status Register"]
    pub HPSVSR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP High Assurance Counter IV Register"]
    pub HPHACIVR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP High Assurance Counter Register"]
    pub HPHACR: crate::RORegister<u32>,
    #[doc = "SNVS_HP Real Time Counter MSB Register"]
    pub HPRTCMR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Real Time Counter LSB Register"]
    pub HPRTCLR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Time Alarm MSB Register"]
    pub HPTAMR: crate::RWRegister<u32>,
    #[doc = "SNVS_HP Time Alarm LSB Register"]
    pub HPTALR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Lock Register"]
    pub LPLR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Control Register"]
    pub LPCR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Master Key Control Register"]
    pub LPMKCR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Security Violation Control Register"]
    pub LPSVCR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "SNVS_LP Tamper Detectors Configuration Register"]
    pub LPTDCR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Status Register"]
    pub LPSR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
    pub LPSRTCMR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
    pub LPSRTCLR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Time Alarm Register"]
    pub LPTAR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
    pub LPSMCMR: crate::RORegister<u32>,
    #[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
    pub LPSMCLR: crate::RORegister<u32>,
    #[doc = "SNVS_LP Power Glitch Detector Register"]
    pub LPPGDR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
    pub LPGPR0_LEGACY_ALIAS: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Zeroizable Master Key Register"]
    pub LPZMKR: [crate::RWRegister<u32>; 8usize],
    _reserved1: [u8; 0x04],
    #[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
    pub LPGPR_ALIAS: [crate::RWRegister<u32>; 4usize],
    _reserved2: [u8; 0x60],
    #[doc = "SNVS_LP General Purpose Registers 0 .. 7"]
    pub LPGPR: [crate::RWRegister<u32>; 8usize],
    _reserved3: [u8; 0x0ad8],
    #[doc = "SNVS_HP Version ID Register 1"]
    pub HPVIDR1: crate::RORegister<u32>,
    #[doc = "SNVS_HP Version ID Register 2"]
    pub HPVIDR2: crate::RORegister<u32>,
}
#[doc = "SNVS_HP Lock Register"]
pub mod HPLR {
    #[doc = "Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    pub mod ZMK_WSL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const ZMK_WSL_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const ZMK_WSL_1: u32 = 0x01;
        }
    }
    #[doc = "Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    pub mod ZMK_RSL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is allowed (only in software Programming mode)"]
            pub const ZMK_RSL_0: u32 = 0;
            #[doc = "Read access is not allowed"]
            pub const ZMK_RSL_1: u32 = 0x01;
        }
    }
    #[doc = "Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    pub mod SRTC_SL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const SRTC_SL_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const SRTC_SL_1: u32 = 0x01;
        }
    }
    #[doc = "LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    pub mod LPCALB_SL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const LPCALB_SL_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const LPCALB_SL_1: u32 = 0x01;
        }
    }
    #[doc = "Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    pub mod MC_SL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access (increment) is allowed"]
            pub const MC_SL_0: u32 = 0;
            #[doc = "Write access (increment) is not allowed"]
            pub const MC_SL_1: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    pub mod GPR_SL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const GPR_SL_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const GPR_SL_1: u32 = 0x01;
        }
    }
    #[doc = "LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    pub mod LPSVCR_SL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const LPSVCR_SL_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const LPSVCR_SL_1: u32 = 0x01;
        }
    }
    #[doc = "LP Tamper Detectors Configuration Register Soft Lock When set, prevents any writes to the LPTDCR"]
    pub mod LPTDCR_SL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const LPTDCR_SL_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const LPTDCR_SL_1: u32 = 0x01;
        }
    }
    #[doc = "Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    pub mod MKS_SL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const MKS_SL_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const MKS_SL_1: u32 = 0x01;
        }
    }
    #[doc = "HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    pub mod HPSVCR_L {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const HPSVCR_L_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const HPSVCR_L_1: u32 = 0x01;
        }
    }
    #[doc = "HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    pub mod HPSICR_L {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const HPSICR_L_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const HPSICR_L_1: u32 = 0x01;
        }
    }
    #[doc = "High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    pub mod HAC_L {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const HAC_L_0: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const HAC_L_1: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_HP Command Register"]
pub mod HPCOMR {
    #[doc = "SSM State Transition Transition state of the system security monitor"]
    pub mod SSM_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    pub mod SSM_ST_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure to Trusted State transition is enabled"]
            pub const SSM_ST_DIS_0: u32 = 0;
            #[doc = "Secure to Trusted State transition is disabled"]
            pub const SSM_ST_DIS_1: u32 = 0x01;
        }
    }
    #[doc = "SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    pub mod SSM_SFNS_DIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Soft Fail to Non-Secure State transition is enabled"]
            pub const SSM_SFNS_DIS_0: u32 = 0;
            #[doc = "Soft Fail to Non-Secure State transition is disabled"]
            pub const SSM_SFNS_DIS_1: u32 = 0x01;
        }
    }
    #[doc = "LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set"]
    pub mod LP_SWR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Action"]
            pub const LP_SWR_0: u32 = 0;
            #[doc = "Reset LP section"]
            pub const LP_SWR_1: u32 = 0x01;
        }
    }
    #[doc = "LP Software Reset Disable When set, disables the LP software reset"]
    pub mod LP_SWR_DIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP software reset is enabled"]
            pub const LP_SWR_DIS_0: u32 = 0;
            #[doc = "LP software reset is disabled"]
            pub const LP_SWR_DIS_1: u32 = 0x01;
        }
    }
    #[doc = "Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    pub mod SW_SV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    pub mod SW_FSV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    pub mod SW_LPSV {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
    pub mod PROG_ZMK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Action"]
            pub const PROG_ZMK_0: u32 = 0;
            #[doc = "Activate hardware key programming mechanism"]
            pub const PROG_ZMK_1: u32 = 0x01;
        }
    }
    #[doc = "Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    pub mod MKS_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OTP master key is selected as an SNVS master key"]
            pub const MKS_EN_0: u32 = 0;
            #[doc = "SNVS master key is selected according to the setting of the MASTER_KEY_SEL field of LPMKCR"]
            pub const MKS_EN_1: u32 = 0x01;
        }
    }
    #[doc = "High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    pub mod HAC_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "High Assurance Counter is disabled"]
            pub const HAC_EN_0: u32 = 0;
            #[doc = "High Assurance Counter is enabled"]
            pub const HAC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
    pub mod HAC_LOAD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Action"]
            pub const HAC_LOAD_0: u32 = 0;
            #[doc = "Load the HAC"]
            pub const HAC_LOAD_1: u32 = 0x01;
        }
    }
    #[doc = "High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
    pub mod HAC_CLEAR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Action"]
            pub const HAC_CLEAR_0: u32 = 0;
            #[doc = "Clear the HAC"]
            pub const HAC_CLEAR_1: u32 = 0x01;
        }
    }
    #[doc = "High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    pub mod HAC_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    pub mod NPSWA_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP Control Register"]
pub mod HPCR {
    #[doc = "HP Real Time Counter Enable"]
    pub mod RTC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RTC is disabled"]
            pub const RTC_EN_0: u32 = 0;
            #[doc = "RTC is enabled"]
            pub const RTC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    pub mod HPTA_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HP Time Alarm Interrupt is disabled"]
            pub const HPTA_EN_0: u32 = 0;
            #[doc = "HP Time Alarm Interrupt is enabled"]
            pub const HPTA_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Disable periodic interrupt in the functional interrupt"]
    pub mod DIS_PI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Periodic interrupt will trigger a functional interrupt"]
            pub const DIS_PI_0: u32 = 0;
            #[doc = "Disable periodic interrupt in the function interrupt"]
            pub const DIS_PI_1: u32 = 0x01;
        }
    }
    #[doc = "HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    pub mod PI_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HP Periodic Interrupt is disabled"]
            pub const PI_EN_0: u32 = 0;
            #[doc = "HP Periodic Interrupt is enabled"]
            pub const PI_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    pub mod PI_FREQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "- bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_0: u32 = 0;
            #[doc = "- bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_1: u32 = 0x01;
            #[doc = "- bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_2: u32 = 0x02;
            #[doc = "- bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_3: u32 = 0x03;
            #[doc = "- bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_4: u32 = 0x04;
            #[doc = "- bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_5: u32 = 0x05;
            #[doc = "- bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_6: u32 = 0x06;
            #[doc = "- bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_7: u32 = 0x07;
            #[doc = "- bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_8: u32 = 0x08;
            #[doc = "- bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_9: u32 = 0x09;
            #[doc = "- bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_10: u32 = 0x0a;
            #[doc = "- bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_11: u32 = 0x0b;
            #[doc = "- bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_12: u32 = 0x0c;
            #[doc = "- bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_13: u32 = 0x0d;
            #[doc = "- bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_14: u32 = 0x0e;
            #[doc = "- bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
            pub const PI_FREQ_15: u32 = 0x0f;
        }
    }
    #[doc = "HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    pub mod HPCALB_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HP Timer calibration disabled"]
            pub const HPCALB_EN_0: u32 = 0;
            #[doc = "HP Timer calibration enabled"]
            pub const HPCALB_EN_1: u32 = 0x01;
        }
    }
    #[doc = "HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    pub mod HPCALB_VAL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "+0 counts per each 32768 ticks of the counter"]
            pub const HPCALB_VAL_0: u32 = 0;
            #[doc = "+1 counts per each 32768 ticks of the counter"]
            pub const HPCALB_VAL_1: u32 = 0x01;
            #[doc = "+2 counts per each 32768 ticks of the counter"]
            pub const HPCALB_VAL_2: u32 = 0x02;
            #[doc = "+15 counts per each 32768 ticks of the counter"]
            pub const HPCALB_VAL_15: u32 = 0x0f;
            #[doc = "-16 counts per each 32768 ticks of the counter"]
            pub const HPCALB_VAL_16: u32 = 0x10;
            #[doc = "-15 counts per each 32768 ticks of the counter"]
            pub const HPCALB_VAL_17: u32 = 0x11;
            #[doc = "-2 counts per each 32768 ticks of the counter"]
            pub const HPCALB_VAL_30: u32 = 0x1e;
            #[doc = "-1 counts per each 32768 ticks of the counter"]
            pub const HPCALB_VAL_31: u32 = 0x1f;
        }
    }
    #[doc = "HP Time Synchronize"]
    pub mod HP_TS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Action"]
            pub const HP_TS_0: u32 = 0;
            #[doc = "Synchronize the HP Time Counter to the LP Time Counter"]
            pub const HP_TS_1: u32 = 0x01;
        }
    }
    #[doc = "Button Configuration"]
    pub mod BTN_CONFIG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Button interrupt mask"]
    pub mod BTN_MASK {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP Security Interrupt Control Register"]
pub mod HPSICR {
    #[doc = "Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    pub mod SV0_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 0 Interrupt is Disabled"]
            pub const SV0_EN_0: u32 = 0;
            #[doc = "Security Violation 0 Interrupt is Enabled"]
            pub const SV0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    pub mod SV1_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 1 Interrupt is Disabled"]
            pub const SV1_EN_0: u32 = 0;
            #[doc = "Security Violation 1 Interrupt is Enabled"]
            pub const SV1_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    pub mod SV2_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 2 Interrupt is Disabled"]
            pub const SV2_EN_0: u32 = 0;
            #[doc = "Security Violation 2 Interrupt is Enabled"]
            pub const SV2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    pub mod SV3_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 3 Interrupt is Disabled"]
            pub const SV3_EN_0: u32 = 0;
            #[doc = "Security Violation 3 Interrupt is Enabled"]
            pub const SV3_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    pub mod SV4_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 4 Interrupt is Disabled"]
            pub const SV4_EN_0: u32 = 0;
            #[doc = "Security Violation 4 Interrupt is Enabled"]
            pub const SV4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    pub mod SV5_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 5 Interrupt is Disabled"]
            pub const SV5_EN_0: u32 = 0;
            #[doc = "Security Violation 5 Interrupt is Enabled"]
            pub const SV5_EN_1: u32 = 0x01;
        }
    }
    #[doc = "LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    pub mod LPSVI_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP Security Violation Interrupt is Disabled"]
            pub const LPSVI_EN_0: u32 = 0;
            #[doc = "LP Security Violation Interrupt is Enabled"]
            pub const LPSVI_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_HP Security Violation Control Register"]
pub mod HPSVCR {
    #[doc = "Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    pub mod SV0_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 0 is a non-fatal violation"]
            pub const SV0_CFG_0: u32 = 0;
            #[doc = "Security Violation 0 is a fatal violation"]
            pub const SV0_CFG_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    pub mod SV1_CFG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 1 is a non-fatal violation"]
            pub const SV1_CFG_0: u32 = 0;
            #[doc = "Security Violation 1 is a fatal violation"]
            pub const SV1_CFG_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    pub mod SV2_CFG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 2 is a non-fatal violation"]
            pub const SV2_CFG_0: u32 = 0;
            #[doc = "Security Violation 2 is a fatal violation"]
            pub const SV2_CFG_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    pub mod SV3_CFG {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 3 is a non-fatal violation"]
            pub const SV3_CFG_0: u32 = 0;
            #[doc = "Security Violation 3 is a fatal violation"]
            pub const SV3_CFG_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    pub mod SV4_CFG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 4 is a non-fatal violation"]
            pub const SV4_CFG_0: u32 = 0;
            #[doc = "Security Violation 4 is a fatal violation"]
            pub const SV4_CFG_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    pub mod SV5_CFG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 5 is disabled"]
            pub const SV5_CFG_0: u32 = 0;
            #[doc = "Security Violation 5 is a non-fatal violation"]
            pub const SV5_CFG_1: u32 = 0x01;
            #[doc = "Security Violation 5 is a fatal violation"]
            pub const SV5_CFG_2: u32 = 0x02;
        }
    }
    #[doc = "LP Security Violation Configuration This field configures the LP security violation source."]
    pub mod LPSV_CFG {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP security violation is disabled"]
            pub const LPSV_CFG_0: u32 = 0;
            #[doc = "LP security violation is a non-fatal violation"]
            pub const LPSV_CFG_1: u32 = 0x01;
            #[doc = "LP security violation is a fatal violation"]
            pub const LPSV_CFG_2: u32 = 0x02;
        }
    }
}
#[doc = "SNVS_HP Status Register"]
pub mod HPSR {
    #[doc = "HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    pub mod HPTA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No time alarm interrupt occurred."]
            pub const HPTA_0: u32 = 0;
            #[doc = "A time alarm interrupt occurred."]
            pub const HPTA_1: u32 = 0x01;
        }
    }
    #[doc = "Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    pub mod PI {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No periodic interrupt occurred."]
            pub const PI_0: u32 = 0;
            #[doc = "A periodic interrupt occurred."]
            pub const PI_1: u32 = 0x01;
        }
    }
    #[doc = "Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
    pub mod LPDIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Button Value of the BTN input"]
    pub mod BTN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    pub mod BI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Security Monitor State This field contains the encoded state of the SSM's state machine"]
    pub mod SSM_STATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Init"]
            pub const SSM_STATE_0: u32 = 0;
            #[doc = "Hard Fail"]
            pub const SSM_STATE_1: u32 = 0x01;
            #[doc = "Soft Fail"]
            pub const SSM_STATE_3: u32 = 0x03;
            #[doc = "Init Intermediate (transition state between Init and Check - SSM stays in this state only one clock cycle)"]
            pub const SSM_STATE_8: u32 = 0x08;
            #[doc = "Check"]
            pub const SSM_STATE_9: u32 = 0x09;
            #[doc = "Non-Secure"]
            pub const SSM_STATE_11: u32 = 0x0b;
            #[doc = "Trusted"]
            pub const SSM_STATE_13: u32 = 0x0d;
            #[doc = "Secure"]
            pub const SSM_STATE_15: u32 = 0x0f;
        }
    }
    #[doc = "Security Configuration This field reflects the settings of the sys_secure_boot input and the three security configuration inputs to SNVS"]
    pub mod SECURITY_CONFIG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {
            #[doc = "FAB Configuration"]
            pub const FAB_CONFIG: u32 = 0;
            #[doc = "OPEN Configuration"]
            pub const OPEN_CONFIG: u32 = 0x01;
            #[doc = "CLOSED Configuration"]
            pub const CLOSED_CONFIG: u32 = 0x09;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
    pub mod OTPMK_SYNDROME {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "One Time Programmable Master Key is Equal to Zero"]
    pub mod OTPMK_ZERO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The OTPMK is not zero."]
            pub const OTPMK_ZERO_0: u32 = 0;
            #[doc = "The OTPMK is zero."]
            pub const OTPMK_ZERO_1: u32 = 0x01;
        }
    }
    #[doc = "Zeroizable Master Key is Equal to Zero"]
    pub mod ZMK_ZERO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The ZMK is not zero."]
            pub const ZMK_ZERO_0: u32 = 0;
            #[doc = "The ZMK is zero."]
            pub const ZMK_ZERO_1: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_HP Security Violation Status Register"]
pub mod HPSVSR {
    #[doc = "Security Violation 0 security violation was detected."]
    pub mod SV0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Security Violation 0 security violation was detected."]
            pub const SV0_0: u32 = 0;
            #[doc = "Security Violation 0 security violation was detected."]
            pub const SV0_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 1 security violation was detected."]
    pub mod SV1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Security Violation 1 security violation was detected."]
            pub const SV1_0: u32 = 0;
            #[doc = "Security Violation 1 security violation was detected."]
            pub const SV1_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 2 security violation was detected."]
    pub mod SV2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Security Violation 2 security violation was detected."]
            pub const SV2_0: u32 = 0;
            #[doc = "Security Violation 2 security violation was detected."]
            pub const SV2_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 3 security violation was detected."]
    pub mod SV3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Security Violation 3 security violation was detected."]
            pub const SV3_0: u32 = 0;
            #[doc = "Security Violation 3 security violation was detected."]
            pub const SV3_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 4 security violation was detected."]
    pub mod SV4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Security Violation 4 security violation was detected."]
            pub const SV4_0: u32 = 0;
            #[doc = "Security Violation 4 security violation was detected."]
            pub const SV4_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 5 security violation was detected."]
    pub mod SV5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Security Violation 5 security violation was detected."]
            pub const SV5_0: u32 = 0;
            #[doc = "Security Violation 5 security violation was detected."]
            pub const SV5_1: u32 = 0x01;
        }
    }
    #[doc = "Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
    pub mod SW_SV {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
    pub mod SW_FSV {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
    pub mod SW_LPSV {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
    pub mod ZMK_SYNDROME {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    pub mod ZMK_ECC_FAIL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ZMK ECC Failure was not detected."]
            pub const ZMK_ECC_FAIL_0: u32 = 0;
            #[doc = "ZMK ECC Failure was detected."]
            pub const ZMK_ECC_FAIL_1: u32 = 0x01;
        }
    }
    #[doc = "LP Security Violation A security volation was detected in the SNVS low power section."]
    pub mod LP_SEC_VIO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP High Assurance Counter IV Register"]
pub mod HPHACIVR {
    #[doc = "High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
    pub mod HAC_COUNTER_IV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP High Assurance Counter Register"]
pub mod HPHACR {
    #[doc = "High Assurance Counter When the HAC_EN bit is set and the SSM is in the soft fail state, this counter starts to count down with the system clock"]
    pub mod HAC_COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP Real Time Counter MSB Register"]
pub mod HPRTCMR {
    #[doc = "HP Real Time Counter The most-significant 15 bits of the RTC"]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP Real Time Counter LSB Register"]
pub mod HPRTCLR {
    #[doc = "HP Real Time Counter least-significant 32 bits"]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP Time Alarm MSB Register"]
pub mod HPTAMR {
    #[doc = "HP Time Alarm, most-significant 15 bits"]
    pub mod HPTA_MS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP Time Alarm LSB Register"]
pub mod HPTALR {
    #[doc = "HP Time Alarm, 32 least-significant bits"]
    pub mod HPTA_LS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Lock Register"]
pub mod LPLR {
    #[doc = "Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    pub mod ZMK_WHL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const ZMK_WHL_0: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const ZMK_WHL_1: u32 = 0x01;
        }
    }
    #[doc = "Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    pub mod ZMK_RHL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is allowed (only in software programming mode)."]
            pub const ZMK_RHL_0: u32 = 0;
            #[doc = "Read access is not allowed."]
            pub const ZMK_RHL_1: u32 = 0x01;
        }
    }
    #[doc = "Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    pub mod SRTC_HL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const SRTC_HL_0: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const SRTC_HL_1: u32 = 0x01;
        }
    }
    #[doc = "LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    pub mod LPCALB_HL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const LPCALB_HL_0: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const LPCALB_HL_1: u32 = 0x01;
        }
    }
    #[doc = "Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    pub mod MC_HL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access (increment) is allowed."]
            pub const MC_HL_0: u32 = 0;
            #[doc = "Write access (increment) is not allowed."]
            pub const MC_HL_1: u32 = 0x01;
        }
    }
    #[doc = "General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    pub mod GPR_HL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const GPR_HL_0: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const GPR_HL_1: u32 = 0x01;
        }
    }
    #[doc = "LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    pub mod LPSVCR_HL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const LPSVCR_HL_0: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const LPSVCR_HL_1: u32 = 0x01;
        }
    }
    #[doc = "LP Tamper Detectors Configuration Register Hard Lock When set, prevents any writes to the LPTDCR"]
    pub mod LPTDCR_HL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const LPTDCR_HL_0: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const LPTDCR_HL_1: u32 = 0x01;
        }
    }
    #[doc = "Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    pub mod MKS_HL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const MKS_HL_0: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const MKS_HL_1: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Control Register"]
pub mod LPCR {
    #[doc = "Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    pub mod SRTC_ENV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SRTC is disabled or invalid."]
            pub const SRTC_ENV_0: u32 = 0;
            #[doc = "SRTC is enabled and valid."]
            pub const SRTC_ENV_1: u32 = 0x01;
        }
    }
    #[doc = "LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    pub mod LPTA_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP time alarm interrupt is disabled."]
            pub const LPTA_EN_0: u32 = 0;
            #[doc = "LP time alarm interrupt is enabled."]
            pub const LPTA_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    pub mod MC_ENV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MC is disabled or invalid."]
            pub const MC_ENV_0: u32 = 0;
            #[doc = "MC is enabled and valid."]
            pub const MC_ENV_1: u32 = 0x01;
        }
    }
    #[doc = "LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    pub mod LPWUI_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    pub mod SRTC_INV_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SRTC stays valid in the case of security violation."]
            pub const SRTC_INV_EN_0: u32 = 0;
            #[doc = "SRTC is invalidated in the case of security violation."]
            pub const SRTC_INV_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Dumb PMIC Enabled When set, software can control the system power"]
    pub mod DP_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Smart PMIC enabled."]
            pub const DP_EN_0: u32 = 0;
            #[doc = "Dumb PMIC enabled."]
            pub const DP_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    pub mod TOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Leave system power on."]
            pub const TOP_0: u32 = 0;
            #[doc = "Turn off system power."]
            pub const TOP_1: u32 = 0x01;
        }
    }
    #[doc = "Power Glitch Enable By default the detection of a power glitch does not cause the pmic_en_b signal to be asserted"]
    pub mod PWR_GLITCH_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    pub mod LPCALB_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SRTC Time calibration is disabled."]
            pub const LPCALB_EN_0: u32 = 0;
            #[doc = "SRTC Time calibration is enabled."]
            pub const LPCALB_EN_1: u32 = 0x01;
        }
    }
    #[doc = "LP Calibration Value Defines signed calibration value for SRTC"]
    pub mod LPCALB_VAL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "+0 counts per each 32768 ticks of the counter clock"]
            pub const LPCALB_VAL_0: u32 = 0;
            #[doc = "+1 counts per each 32768 ticks of the counter clock"]
            pub const LPCALB_VAL_1: u32 = 0x01;
            #[doc = "+2 counts per each 32768 ticks of the counter clock"]
            pub const LPCALB_VAL_2: u32 = 0x02;
            #[doc = "+15 counts per each 32768 ticks of the counter clock"]
            pub const LPCALB_VAL_15: u32 = 0x0f;
            #[doc = "-16 counts per each 32768 ticks of the counter clock"]
            pub const LPCALB_VAL_16: u32 = 0x10;
            #[doc = "-15 counts per each 32768 ticks of the counter clock"]
            pub const LPCALB_VAL_17: u32 = 0x11;
            #[doc = "-2 counts per each 32768 ticks of the counter clock"]
            pub const LPCALB_VAL_30: u32 = 0x1e;
            #[doc = "-1 counts per each 32768 ticks of the counter clock"]
            pub const LPCALB_VAL_31: u32 = 0x1f;
        }
    }
    #[doc = "This field configures the button press time out values for the PMIC Logic"]
    pub mod BTN_PRESS_TIME {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field configures the amount of debounce time for the BTN input signal"]
    pub mod DEBOUNCE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    pub mod ON_TIME {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    pub mod PK_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    pub mod PK_OVERRIDE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Registers Zeroization Disable"]
    pub mod GPR_Z_DIS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Master Key Control Register"]
pub mod LPMKCR {
    #[doc = "Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    pub mod MASTER_KEY_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select one time programmable master key."]
            pub const MASTER_KEY_SEL_0: u32 = 0;
            #[doc = "Select zeroizable master key when MKS_EN bit is set ."]
            pub const MASTER_KEY_SEL_2: u32 = 0x02;
            #[doc = "Select combined master key when MKS_EN bit is set ."]
            pub const MASTER_KEY_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    pub mod ZMK_HWP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ZMK is in the software programming mode."]
            pub const ZMK_HWP_0: u32 = 0;
            #[doc = "ZMK is in the hardware programming mode."]
            pub const ZMK_HWP_1: u32 = 0x01;
        }
    }
    #[doc = "Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    pub mod ZMK_VAL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ZMK is not valid."]
            pub const ZMK_VAL_0: u32 = 0;
            #[doc = "ZMK is valid."]
            pub const ZMK_VAL_1: u32 = 0x01;
        }
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    pub mod ZMK_ECC_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ZMK ECC check is disabled."]
            pub const ZMK_ECC_EN_0: u32 = 0;
            #[doc = "ZMK ECC check is enabled."]
            pub const ZMK_ECC_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
    pub mod ZMK_ECC_VALUE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Security Violation Control Register"]
pub mod LPSVCR {
    #[doc = "Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    pub mod SV0_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 0 is disabled in the LP domain."]
            pub const SV0_EN_0: u32 = 0;
            #[doc = "Security Violation 0 is enabled in the LP domain."]
            pub const SV0_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    pub mod SV1_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 1 is disabled in the LP domain."]
            pub const SV1_EN_0: u32 = 0;
            #[doc = "Security Violation 1 is enabled in the LP domain."]
            pub const SV1_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    pub mod SV2_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 2 is disabled in the LP domain."]
            pub const SV2_EN_0: u32 = 0;
            #[doc = "Security Violation 2 is enabled in the LP domain."]
            pub const SV2_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    pub mod SV3_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 3 is disabled in the LP domain."]
            pub const SV3_EN_0: u32 = 0;
            #[doc = "Security Violation 3 is enabled in the LP domain."]
            pub const SV3_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    pub mod SV4_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 4 is disabled in the LP domain."]
            pub const SV4_EN_0: u32 = 0;
            #[doc = "Security Violation 4 is enabled in the LP domain."]
            pub const SV4_EN_1: u32 = 0x01;
        }
    }
    #[doc = "Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    pub mod SV5_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security Violation 5 is disabled in the LP domain."]
            pub const SV5_EN_0: u32 = 0;
            #[doc = "Security Violation 5 is enabled in the LP domain."]
            pub const SV5_EN_1: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Tamper Detectors Configuration Register"]
pub mod LPTDCR {
    #[doc = "SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    pub mod SRTCR_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SRTC rollover is disabled."]
            pub const SRTCR_EN_0: u32 = 0;
            #[doc = "SRTC rollover is enabled."]
            pub const SRTCR_EN_1: u32 = 0x01;
        }
    }
    #[doc = "MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    pub mod MCR_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MC rollover is disabled."]
            pub const MCR_EN_0: u32 = 0;
            #[doc = "MC rollover is enabled."]
            pub const MCR_EN_1: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    pub mod ET1_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 1 is disabled."]
            pub const ET1_EN_0: u32 = 0;
            #[doc = "External tamper 1 is enabled."]
            pub const ET1_EN_1: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    pub mod ET1P {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 1 is active low."]
            pub const ET1P_0: u32 = 0;
            #[doc = "External tamper 1 is active high."]
            pub const ET1P_1: u32 = 0x01;
        }
    }
    #[doc = "System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    pub mod PFD_OBSERV {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    pub mod POR_OBSERV {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    pub mod OSCB {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal SRTC clock oscillator not bypassed."]
            pub const OSCB_0: u32 = 0;
            #[doc = "Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source."]
            pub const OSCB_1: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Status Register"]
pub mod LPSR {
    #[doc = "LP Time Alarm"]
    pub mod LPTA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No time alarm interrupt occurred."]
            pub const LPTA_0: u32 = 0;
            #[doc = "A time alarm interrupt occurred."]
            pub const LPTA_1: u32 = 0x01;
        }
    }
    #[doc = "Secure Real Time Counter Rollover"]
    pub mod SRTCR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SRTC has not reached its maximum value."]
            pub const SRTCR_0: u32 = 0;
            #[doc = "SRTC has reached its maximum value."]
            pub const SRTCR_1: u32 = 0x01;
        }
    }
    #[doc = "Monotonic Counter Rollover"]
    pub mod MCR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MC has not reached its maximum value."]
            pub const MCR_0: u32 = 0;
            #[doc = "MC has reached its maximum value."]
            pub const MCR_1: u32 = 0x01;
        }
    }
    #[doc = "Power Supply Glitch Detected 0 No power supply glitch. 1 Power supply glitch is detected."]
    pub mod PGD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tampering 1 Detected"]
    pub mod ET1D {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tampering 1 not detected."]
            pub const ET1D_0: u32 = 0;
            #[doc = "External tampering 1 detected."]
            pub const ET1D_1: u32 = 0x01;
        }
    }
    #[doc = "External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    pub mod ESVD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No external security violation."]
            pub const ESVD_0: u32 = 0;
            #[doc = "External security violation is detected."]
            pub const ESVD_1: u32 = 0x01;
        }
    }
    #[doc = "Emergency Off This bit is set when a power off is requested."]
    pub mod EO {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Emergency off was not detected."]
            pub const EO_0: u32 = 0;
            #[doc = "Emergency off was detected."]
            pub const EO_1: u32 = 0x01;
        }
    }
    #[doc = "Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    pub mod SPO {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Set Power Off was not detected."]
            pub const SPO_0: u32 = 0;
            #[doc = "Set Power Off was detected."]
            pub const SPO_1: u32 = 0x01;
        }
    }
    #[doc = "Scan Exit Detected"]
    pub mod SED {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Scan exit was not detected."]
            pub const SED_0: u32 = 0;
            #[doc = "Scan exit was detected."]
            pub const SED_1: u32 = 0x01;
        }
    }
    #[doc = "LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
    pub mod LPNS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP section was not programmed in the non-secure state."]
            pub const LPNS_0: u32 = 0;
            #[doc = "LP section was programmed in the non-secure state."]
            pub const LPNS_1: u32 = 0x01;
        }
    }
    #[doc = "LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
    pub mod LPS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP section was not programmed in secure or trusted state."]
            pub const LPS_0: u32 = 0;
            #[doc = "LP section was programmed in secure or trusted state."]
            pub const LPS_1: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
pub mod LPSRTCMR {
    #[doc = "LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    pub mod SRTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
pub mod LPSRTCLR {
    #[doc = "LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
    pub mod SRTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Time Alarm Register"]
pub mod LPTAR {
    #[doc = "LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)"]
    pub mod LPTA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
pub mod LPSMCMR {
    #[doc = "Monotonic Counter most-significant 16 Bits The MC is incremented by one when: A write transaction to the LPSMCMR or LPSMCLR register is detected"]
    pub mod MON_COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses"]
    pub mod MC_ERA_BITS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
pub mod LPSMCLR {
    #[doc = "Monotonic Counter bits The MC is incremented by one when: A write transaction to the LPSMCMR or LPSMCLR Register is detected"]
    pub mod MON_COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Power Glitch Detector Register"]
pub mod LPPGDR {
    #[doc = "Power Glitch Detector Value"]
    pub mod PGD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
pub mod LPGPR0_LEGACY_ALIAS {
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Zeroizable Master Key Register"]
pub mod LPZMKR {
    #[doc = "Zeroizable Master Key Each of these registers contains 32 bits of the 256-bit ZMK value"]
    pub mod ZMK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
pub mod LPGPR_ALIAS {
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP General Purpose Registers 0 .. 7"]
pub mod LPGPR {
    #[doc = "General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP Version ID Register 1"]
pub mod HPVIDR1 {
    #[doc = "SNVS block minor version number"]
    pub mod MINOR_REV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SNVS block major version number"]
    pub mod MAJOR_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SNVS block ID"]
    pub mod IP_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_HP Version ID Register 2"]
pub mod HPVIDR2 {
    #[doc = "SNVS Configuration Options"]
    pub mod CONFIG_OPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SNVS ECO Revision"]
    pub mod ECO_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SNVS Integration Options"]
    pub mod INTG_OPT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5"]
    pub mod IP_ERA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
