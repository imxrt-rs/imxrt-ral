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
    #[doc = "SNVS_LP Tamper Glitch Filters Configuration Register"]
    pub LPTGFCR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Tamper Detect Configuration Register"]
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
    pub LPSMCMR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
    pub LPSMCLR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Digital Low-Voltage Detector Register"]
    pub LPLVDR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
    pub LPGPR0_LEGACY_ALIAS: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Zeroizable Master Key Register"]
    pub LPZMKR: [crate::RWRegister<u32>; 8usize],
    _reserved0: [u8; 0x04],
    #[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
    pub LPGPR_ALIAS: [crate::RWRegister<u32>; 4usize],
    #[doc = "SNVS_LP Tamper Detectors Config 2 Register"]
    pub LPTDC2R: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Tamper Detectors Status Register"]
    pub LPTDSR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Tamper Glitch Filter 1 Configuration Register"]
    pub LPTGF1CR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Tamper Glitch Filter 2 Configuration Register"]
    pub LPTGF2CR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "SNVS_LP Active Tamper 1 Configuration Register"]
    pub LPAT1CR: crate::WORegister<u32>,
    #[doc = "SNVS_LP Active Tamper 2 Configuration Register"]
    pub LPAT2CR: crate::WORegister<u32>,
    #[doc = "SNVS_LP Active Tamper 3 Configuration Register"]
    pub LPAT3CR: crate::WORegister<u32>,
    #[doc = "SNVS_LP Active Tamper 4 Configuration Register"]
    pub LPAT4CR: crate::WORegister<u32>,
    #[doc = "SNVS_LP Active Tamper 5 Configuration Register"]
    pub LPAT5CR: crate::WORegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "SNVS_LP Active Tamper Control Register"]
    pub LPATCTLR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Active Tamper Clock Control Register"]
    pub LPATCLKR: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Active Tamper Routing Control 1 Register"]
    pub LPATRC1R: crate::RWRegister<u32>,
    #[doc = "SNVS_LP Active Tamper Routing Control 2 Register"]
    pub LPATRC2R: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
    pub LPGPR: [crate::RWRegister<u32>; 4usize],
    _reserved4: [u8; 0x0ae8],
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
    #[doc = "LP Tamper Glitch Filter Configuration Register Soft Lock When set, prevents any writes to the LPTGFCR"]
    pub mod LPTGFCR_SL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
    pub mod LPSECR_SL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed"]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed"]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
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
    #[doc = "Active Tamper 1 Soft Lock When set, prevents any writes to the Active Tamper 1 registers"]
    pub mod AT1_SL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 2 Soft Lock When set, prevents any writes to the Active Tamper 2 registers"]
    pub mod AT2_SL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 3 Soft Lock When set, prevents any writes to the Active Tamper 3 registers"]
    pub mod AT3_SL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 4 Soft Lock When set, prevents any writes to the Active Tamper 4 registers"]
    pub mod AT4_SL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 5 Soft Lock When set, prevents any writes to the Active Tamper 5 registers"]
    pub mod AT5_SL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
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
    #[doc = "CAAM Security Violation Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the CAAM Security Violation security violation"]
    pub mod CAAM_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAAM Security Violation Interrupt is Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "CAAM Security Violation Interrupt is Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "JTAG Active Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the JTAG Active security violation"]
    pub mod JTAGC_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "JTAG Active Interrupt is Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "JTAG Active Interrupt is Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Watchdog 2 Reset Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Watchdog 2 Reset security violation"]
    pub mod WDOG2_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog 2 Reset Interrupt is Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Watchdog 2 Reset Interrupt is Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Internal Boot Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Internal Boot security violation"]
    pub mod SRC_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal Boot Interrupt is Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Internal Boot Interrupt is Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "OCOTP attack error Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the OCOTP attack error security violation"]
    pub mod OCOTP_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCOTP attack error Interrupt is Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "OCOTP attack error Interrupt is Enabled"]
            pub const ENABLED: u32 = 0x01;
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
    #[doc = "CAAM Security Violation Security Violation Configuration This field configures the CAAM Security Violation Security Violation Input"]
    pub mod CAAM_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAAM Security Violation is a non-fatal violation"]
            pub const NON_FATAL: u32 = 0;
            #[doc = "CAAM Security Violation is a fatal violation"]
            pub const FATAL: u32 = 0x01;
        }
    }
    #[doc = "JTAG Active Security Violation Configuration This field configures the JTAG Active Security Violation Input"]
    pub mod JTAGC_CFG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "JTAG Active is a non-fatal violation"]
            pub const NON_FATAL: u32 = 0;
            #[doc = "JTAG Active is a fatal violation"]
            pub const FATAL: u32 = 0x01;
        }
    }
    #[doc = "Watchdog 2 Reset Security Violation Configuration This field configures the Watchdog 2 Reset Security Violation Input"]
    pub mod WDOG2_CFG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog 2 Reset is a non-fatal violation"]
            pub const NON_FATAL: u32 = 0;
            #[doc = "Watchdog 2 Reset is a fatal violation"]
            pub const FATAL: u32 = 0x01;
        }
    }
    #[doc = "Internal Boot Security Violation Configuration This field configures the Internal Boot Security Violation Input"]
    pub mod SRC_CFG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal Boot is a non-fatal violation"]
            pub const NON_FATAL: u32 = 0;
            #[doc = "Internal Boot is a fatal violation"]
            pub const FATAL: u32 = 0x01;
        }
    }
    #[doc = "OCOTP attack error Security Violation Configuration This field configures the OCOTP attack error Security Violation Input"]
    pub mod OCOTP_CFG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCOTP attack error is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "OCOTP attack error is a non-fatal violation"]
            pub const NON_FATAL: u32 = 0x01;
            #[doc = "OCOTP attack error is a fatal violation"]
            pub const FATAL: u32 = 0x02;
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
    #[doc = "System Security Configuration This field reflects the three security configuration inputs to SNVS"]
    pub mod SYS_SECURITY_CFG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fab Configuration - the default configuration of newly fabricated chips"]
            pub const FAB_CONFIG: u32 = 0;
            #[doc = "Open Configuration - the configuration after NXP-programmable fuses have been blown"]
            pub const OPEN_CONFIG: u32 = 0x01;
            #[doc = "Closed Configuration - the configuration after OEM-programmable fuses have been blown"]
            pub const CLOSED_CONFIG: u32 = 0x03;
            #[doc = "Field Return Configuration - the configuration of chips that are returned to NXP for analysis"]
            pub const FIELD_RETURN_CONFIG: u32 = 0x07;
        }
    }
    #[doc = "System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM"]
    pub mod SYS_SECURE_BOOT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
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
    #[doc = "CAAM Security Violation security violation was detected."]
    pub mod CAAM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No CAAM Security Violation security violation was detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "CAAM Security Violation security violation was detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "JTAG Active security violation was detected."]
    pub mod JTAGC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No JTAG Active security violation was detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "JTAG Active security violation was detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "Watchdog 2 Reset security violation was detected."]
    pub mod WDOG2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Watchdog 2 Reset security violation was detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "Watchdog 2 Reset security violation was detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "Internal Boot security violation was detected."]
    pub mod SRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Internal Boot security violation was detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "Internal Boot security violation was detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "OCOTP attack error security violation was detected."]
    pub mod OCOTP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No OCOTP attack error security violation was detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "OCOTP attack error security violation was detected."]
            pub const REPORTED: u32 = 0x01;
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
    #[doc = "LP Security Violation A security volation was detected in the SNVS low power section"]
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
    #[doc = "LP Tamper Glitch Filter Configuration Register Hard Lock When set, prevents any writes to the LPTGFCR"]
    pub mod LPTGFCR_HL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
    pub mod LPSECR_HL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
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
    #[doc = "Active Tamper 1 Hard Lock When set, prevents any writes to the Active Tamper 1 registers"]
    pub mod AT1_HL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 2 Hard Lock When set, prevents any writes to the Active Tamper 2 registers"]
    pub mod AT2_HL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 3 Hard Lock When set, prevents any writes to the Active Tamper 3 registers"]
    pub mod AT3_HL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 4 Hard Lock When set, prevents any writes to the Active Tamper 4 registers"]
    pub mod AT4_HL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 5 Hard Lock When set, prevents any writes to the Active Tamper 5 registers"]
    pub mod AT5_HL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is allowed."]
            pub const WRITE_ACCESS_ALLOWED: u32 = 0;
            #[doc = "Write access is not allowed."]
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0x01;
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
    #[doc = "Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
    #[deprecated(since = "0.5.1", note = "Use LVD_EN")]
    pub mod PWR_GLITCH_EN {
        pub use super::LVD_EN::*;
    }
    #[doc = "Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
    pub mod LVD_EN {
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
    #[doc = "CAAM Security Violation Enable This bit enables CAAM Security Violation Input"]
    pub mod CAAM_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAAM Security Violation is disabled in the LP domain."]
            pub const DISABLED: u32 = 0;
            #[doc = "CAAM Security Violation is enabled in the LP domain."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "JTAG Active Enable This bit enables JTAG Active Input"]
    pub mod JTAGC_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "JTAG Active is disabled in the LP domain."]
            pub const DISABLED: u32 = 0;
            #[doc = "JTAG Active is enabled in the LP domain."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Watchdog 2 Reset Enable This bit enables Watchdog 2 Reset Input"]
    pub mod WDOG2_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog 2 Reset is disabled in the LP domain."]
            pub const DISABLED: u32 = 0;
            #[doc = "Watchdog 2 Reset is enabled in the LP domain."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Internal Boot Enable This bit enables Internal Boot Input"]
    pub mod SRC_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal Boot is disabled in the LP domain."]
            pub const DISABLED: u32 = 0;
            #[doc = "Internal Boot is enabled in the LP domain."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "OCOTP attack error Enable This bit enables OCOTP attack error Input"]
    pub mod OCOTP_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OCOTP attack error is disabled in the LP domain."]
            pub const DISABLED: u32 = 0;
            #[doc = "OCOTP attack error is enabled in the LP domain."]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Tamper Glitch Filters Configuration Register"]
pub mod LPTGFCR {
    #[doc = "Wire-Mesh Tamper Glitch Filter Configures the length of the digital glitch filter for the wire-mesh tamper 1 and 2 pins between 1 and 63 SRTC clock cycles"]
    pub mod WMTGF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wire-Mesh Tamper Glitch Filter Enable When set, enables the wire-mesh tamper glitch filter"]
    pub mod WMTGF_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wire-mesh tamper glitch filter is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "Wire-mesh tamper glitch filter is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tamper Glitch Filter 1 Configures the length of the digital glitch filter for the external tamper 1 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 1 Enable When set, enables the external tamper glitch filter 1."]
    pub mod ETGF1_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 1 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 1 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tamper Glitch Filter 2 Configures the length of the digital glitch filter for the external tamper 2 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF2 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 2 Enable When set, enables the external tamper glitch filter 2."]
    pub mod ETGF2_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 2 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 2 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Tamper Detect Configuration Register"]
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
    #[doc = "Clock Tamper Enable When set, a clock monitor tamper generates an LP security violation."]
    pub mod CT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock tamper is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Clock tamper is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Temperature Tamper Enable When set, a temperature monitor tamper generates an LP security violation"]
    pub mod TT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Temperature tamper is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Temperature tamper is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Voltage Tamper Enable Voltage Tamper Enable should be enabled 500 us after setting SCSC_SOSC_CTR \\[VOLT_TEMP_TAMPER_EN\\]"]
    pub mod VT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Voltage tamper is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Voltage tamper is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Wire-Mesh Tampering 1 Enable When set, wire-mesh tampering 1 detection generates an LP security violation"]
    pub mod WMT1_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wire-mesh tamper 1 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Wire-mesh tamper 1 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Wire-Mesh Tampering 2 Enable When set, wire-mesh tampering 2 detection generates an LP security violation"]
    pub mod WMT2_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wire-mesh tamper 2 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Wire-mesh tamper 2 is enabled."]
            pub const ENABLED: u32 = 0x01;
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
    #[doc = "External Tampering 2 Enable When set, external tampering 2 detection generates an LP security violation"]
    pub mod ET2_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 2 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 2 is enabled."]
            pub const ENABLED: u32 = 0x01;
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
    #[doc = "External Tampering 2 Polarity This bit is used to determine the polarity of external tamper 2."]
    pub mod ET2P {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 2 is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 2 is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
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
    #[doc = "Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
    pub mod LTDC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
    pub mod HTDC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage Reference Configuration These configuration bits are wired as an output of the module."]
    pub mod VRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
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
    #[doc = "Digital Low Voltage Event Detected"]
    pub mod LVD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No low voltage event detected."]
            pub const NOLOWVOLT: u32 = 0;
            #[doc = "Low voltage event is detected."]
            pub const LOWVOLTDETECTED: u32 = 0x01;
        }
    }
    #[doc = "Clock Tampering Detected"]
    pub mod CTD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock tamper."]
            pub const NOREPORT: u32 = 0;
            #[doc = "Clock tamper is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "Temperature Tamper Detected"]
    pub mod TTD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No temperature tamper."]
            pub const NOREPORT: u32 = 0;
            #[doc = "Temperature tamper is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "Voltage Tampering Detected"]
    pub mod VTD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Voltage tampering not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "Voltage tampering detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "Wire-Mesh Tampering 1 Detected"]
    pub mod WMT1D {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wire-mesh tampering 1 not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "Wire-mesh tampering 1 detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "Wire-Mesh Tampering 2 Detected"]
    pub mod WMT2D {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wire-mesh tampering 2 not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "Wire-mesh tampering 2 detected."]
            pub const REPORTED: u32 = 0x01;
        }
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
    #[doc = "External Tampering 2 Detected"]
    pub mod ET2D {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tampering 2 not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tampering 2 detected."]
            pub const REPORTED: u32 = 0x01;
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
    pub mod SPOF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Set Power Off was not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "Set Power Off was detected."]
            pub const REPORTED: u32 = 0x01;
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
#[doc = "SNVS_LP Digital Low-Voltage Detector Register"]
pub mod LPLVDR {
    #[doc = "Low-Voltage Detector Value"]
    pub mod LVD {
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
#[doc = "SNVS_LP Tamper Detectors Config 2 Register"]
pub mod LPTDC2R {
    #[doc = "External Tampering 3 Enable When set, external tampering 3 detection generates an LP security violation"]
    pub mod ET3_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 3 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 3 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 4 Enable When set, external tampering 4 detection generates an LP security violation"]
    pub mod ET4_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 4 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 4 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 5 Enable When set, external tampering 5 detection generates an LP security violation"]
    pub mod ET5_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 5 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 5 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 6 Enable When set, external tampering 6 detection generates an LP security violation"]
    pub mod ET6_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 6 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 6 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 7 Enable When set, external tampering 7 detection generates an LP security violation"]
    pub mod ET7_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 7 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 7 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 8 Enable When set, external tampering 8 detection generates an LP security violation"]
    pub mod ET8_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 8 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 8 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 9 Enable When set, external tampering 9 detection generates an LP security violation"]
    pub mod ET9_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 9 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 9 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 10 Enable When set, external tampering 10 detection generates an LP security violation"]
    pub mod ET10_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 10 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "External tamper 10 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 3 Polarity This bit is used to determine the polarity of external tamper 3."]
    pub mod ET3P {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 3 active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 3 active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 4 Polarity This bit is used to determine the polarity of external tamper 4."]
    pub mod ET4P {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 4 is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 4 is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 5 Polarity This bit is used to determine the polarity of external tamper 5."]
    pub mod ET5P {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 5 is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 5 is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 6 Polarity This bit is used to determine the polarity of external tamper 6."]
    pub mod ET6P {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 6 is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 6 is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 7 Polarity This bit is used to determine the polarity of external tamper 7."]
    pub mod ET7P {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 7 is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 7 is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 8 Polarity This bit is used to determine the polarity of external tamper 8."]
    pub mod ET8P {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 8 is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 8 is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 9 Polarity This bit is used to determine the polarity of external tamper 9."]
    pub mod ET9P {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 9 is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 9 is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 10 Polarity This bit is used to determine the polarity of external tamper 10."]
    pub mod ET10P {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 10 is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "External tamper 10 is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Tamper Detectors Status Register"]
pub mod LPTDSR {
    #[doc = "External Tampering 3 Detected"]
    pub mod ET3D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 3 is not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tamper 3 is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 4 Detected"]
    pub mod ET4D {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 4 is not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tamper 4 is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 5 Detected"]
    pub mod ET5D {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 5 is not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tamper 5 is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 6 Detected"]
    pub mod ET6D {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 6 is not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tamper 6 is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 7 Detected"]
    pub mod ET7D {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 7 is not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tamper 7 is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 8 Detected"]
    pub mod ET8D {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 8 is not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tamper 8 is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 9 Enable When set, external tampering 9 detection generates an LP security violation"]
    pub mod ET9D {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 9 is not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tamper 9 is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
    #[doc = "External Tampering 10 Detected"]
    pub mod ET10D {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper 10 is not detected."]
            pub const NOREPORT: u32 = 0;
            #[doc = "External tamper 10 is detected."]
            pub const REPORTED: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Tamper Glitch Filter 1 Configuration Register"]
pub mod LPTGF1CR {
    #[doc = "External Tamper Glitch Filter 3 Configures the length of the digital glitch filter for the external tamper 3 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 3 Enable When set, enables the external tamper glitch filter 3."]
    pub mod ETGF3_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 3 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 3 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tamper Glitch Filter 4 Configures the length of the digital glitch filter for the external tamper 4 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 4 Enable When set, enables the external tamper glitch filter 4."]
    pub mod ETGF4_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 4 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 4 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tamper Glitch Filter 5 Configures the length of the digital glitch filter for the external tamper 5 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 5 Enable When set, enables the external tamper glitch filter 5."]
    pub mod ETGF5_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 5 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 5 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tamper Glitch Filter 6 Configures the length of the digital glitch filter for the external tamper 6 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 6 Enable When set, enables the external tamper glitch filter 6."]
    pub mod ETGF6_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 6 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 6 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Tamper Glitch Filter 2 Configuration Register"]
pub mod LPTGF2CR {
    #[doc = "External Tamper Glitch Filter 7 Configures the length of the digital glitch filter for the external tamper 7 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 7 Enable When set, enables the external tamper glitch filter 7."]
    pub mod ETGF7_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 7 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 7 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tamper Glitch Filter 8 Configures the length of the digital glitch filter for the external tamper 8 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 8 Enable When set, enables the external tamper glitch filter 8."]
    pub mod ETGF8_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 8 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 8 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tamper Glitch Filter 9 Configures the length of the digital glitch filter for the external tamper 9 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF9 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 9 Enable When set, enables the external tamper glitch filter 9."]
    pub mod ETGF9_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 9 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 9 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "External Tamper Glitch Filter 10 Configures the length of the digital glitch filter for the external tamper 10 pin between 128 and 32640 SRTC clock cycles"]
    pub mod ETGF10 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper Glitch Filter 10 Enable When set, enables the external tamper glitch filter 10."]
    pub mod ETGF10_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External tamper glitch filter 10 is bypassed."]
            pub const BYPASSED: u32 = 0;
            #[doc = "External tamper glitch filter 10 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Active Tamper 1 Configuration Register"]
pub mod LPAT1CR {
    #[doc = "Active Tamper 1 Initial Seed Default Seed is 1111h."]
    pub mod SEED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 1 Polynomial Default Polynomial is 8400h."]
    pub mod POLYNOMIAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Active Tamper 2 Configuration Register"]
pub mod LPAT2CR {
    #[doc = "Active Tamper 2 Initial Seed Default Seed is 2222h."]
    pub mod SEED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 2 Polynomial Default Polynomial is 9C00h."]
    pub mod POLYNOMIAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Active Tamper 3 Configuration Register"]
pub mod LPAT3CR {
    #[doc = "Active Tamper 3 Initial Seed Default Seed is 3333h."]
    pub mod SEED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 3 Polynomial Default Polynomial is CA00h."]
    pub mod POLYNOMIAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Active Tamper 4 Configuration Register"]
pub mod LPAT4CR {
    #[doc = "Active Tamper 4 Initial Seed Default Seed is 4444h."]
    pub mod SEED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 4 Polynomial Default Polynomial is 8580h."]
    pub mod POLYNOMIAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Active Tamper 5 Configuration Register"]
pub mod LPAT5CR {
    #[doc = "Active Tamper 5 Initial Seed Default Seed is 5555h."]
    pub mod SEED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 5 Polynomial Default Polynomial is A840h."]
    pub mod POLYNOMIAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Active Tamper Control Register"]
pub mod LPATCTLR {
    #[doc = "Active Tamper 1 Enable When set, enables the Active Tamper 1 LFSR."]
    pub mod AT1_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 1 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 1 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 2 Enable When set, enables the Active Tamper 2 LFSR."]
    pub mod AT2_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 2 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 2 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 3 Enable When set, enables the Active Tamper 3 LFSR."]
    pub mod AT3_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 3 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 3 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 4 Enable When set, enables the Active Tamper 4 LFSR."]
    pub mod AT4_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 4 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 4 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 5 Enable When set, enables the Active Tamper 5 LFSR."]
    pub mod AT5_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 5 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 5 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 1 Pad Out Enable When set, enables the Active Tamper 1 external pad."]
    pub mod AT1_PAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 1 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 1 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 2 Pad Out Enable When set, enables the Active Tamper 2 external pad."]
    pub mod AT2_PAD_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 2 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 2 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 3 Pad Out Enable When set, enables the Active Tamper 3 external pad."]
    pub mod AT3_PAD_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 3 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 3 is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 4 Pad Out Enable When set, enables the Active Tamper 4 external pad."]
    pub mod AT4_PAD_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 4 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 4 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Active Tamper 5 Pad Out Enable When set, enables the Active Tamper 5 external pad."]
    pub mod AT5_PAD_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active Tamper 5 is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active Tamper 5 is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "SNVS_LP Active Tamper Clock Control Register"]
pub mod LPATCLKR {
    #[doc = "Active Tamper 1 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz"]
    pub mod AT1_CLK_CTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 2 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz"]
    pub mod AT2_CLK_CTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 3 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz"]
    pub mod AT3_CLK_CTL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 4 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz"]
    pub mod AT4_CLK_CTL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper 5 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz"]
    pub mod AT5_CLK_CTL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Active Tamper Routing Control 1 Register"]
pub mod LPATRC1R {
    #[doc = "External Tamper 1 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET1RCTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper 2 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET2RCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper 3 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET3RCTL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper 4 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET4RCTL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper 5 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET5RCTL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper 6 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET6RCTL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper 7 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET7RCTL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper 8 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET8RCTL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP Active Tamper Routing Control 2 Register"]
pub mod LPATRC2R {
    #[doc = "External Tamper 9 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET9RCTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Tamper 10 Routing Control Any undefined selection will be routed to passive"]
    pub mod ET10RCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
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
    #[doc = "SNVS ECO Revision The engineering change order revision number for this release of SNVS."]
    pub mod ECO_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5 06h - Era 6"]
    pub mod IP_ERA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
