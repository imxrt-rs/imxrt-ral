#[doc = "PWM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Cluster SM%s, containing SM?CNT, SM?INIT, SM?CTRL2, SM?CTRL, SM?VAL0, SM?FRACVAL1, SM?VAL1, SM?FRACVAL2, SM?VAL2, SM?FRACVAL3, SM?VAL3, SM?FRACVAL4, SM?VAL4, SM?FRACVAL5, SM?VAL5, SM?FRCTRL, SM?OCTRL, SM?STS, SM?INTEN, SM?DMAEN, SM?TCTRL, SM?DISMAP0, SM?DISMAP1, SM?DTCNT0, SM?DTCNT1, SM?CAPTCTRLA, SM?CAPTCOMPA, SM?CAPTCTRLB, SM?CAPTCOMPB, SM?CAPTCTRLX, SM?CAPTCOMPX, SM?CVAL0, SM?CVAL0CYC, SM?CVAL1, SM?CVAL1CYC, SM?CVAL2, SM?CVAL2CYC, SM?CVAL3, SM?CVAL3CYC, SM?CVAL4, SM?CVAL4CYC, SM?CVAL5, SM?CVAL5CYC"]
    pub SM: [sm::RegisterBlock; 4usize],
    #[doc = "Output Enable Register"]
    pub OUTEN: crate::RWRegister<u16>,
    #[doc = "Mask Register"]
    pub MASK: crate::RWRegister<u16>,
    #[doc = "Software Controlled Output Register"]
    pub SWCOUT: crate::RWRegister<u16>,
    #[doc = "PWM Source Select Register"]
    pub DTSRCSEL: crate::RWRegister<u16>,
    #[doc = "Master Control Register"]
    pub MCTRL: crate::RWRegister<u16>,
    #[doc = "Master Control 2 Register"]
    pub MCTRL2: crate::RWRegister<u16>,
    #[doc = "Fault Control Register"]
    pub FCTRL0: crate::RWRegister<u16>,
    #[doc = "Fault Status Register"]
    pub FSTS0: crate::RWRegister<u16>,
    #[doc = "Fault Filter Register"]
    pub FFILT0: crate::RWRegister<u16>,
    #[doc = "Fault Test Register"]
    pub FTST0: crate::RWRegister<u16>,
    #[doc = "Fault Control 2 Register"]
    pub FCTRL20: crate::RWRegister<u16>,
}
#[doc = "Output Enable Register"]
pub mod OUTEN {
    #[doc = "PWM_X Output Enables"]
    pub mod PWMX_EN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_X output disabled."]
            pub const PWMX_EN_0: u16 = 0;
            #[doc = "PWM_X output enabled."]
            pub const PWMX_EN_1: u16 = 0x01;
        }
    }
    #[doc = "PWM_B Output Enables"]
    pub mod PWMB_EN {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_B output disabled."]
            pub const PWMB_EN_0: u16 = 0;
            #[doc = "PWM_B output enabled."]
            pub const PWMB_EN_1: u16 = 0x01;
        }
    }
    #[doc = "PWM_A Output Enables"]
    pub mod PWMA_EN {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_A output disabled."]
            pub const PWMA_EN_0: u16 = 0;
            #[doc = "PWM_A output enabled."]
            pub const PWMA_EN_1: u16 = 0x01;
        }
    }
}
#[doc = "Mask Register"]
pub mod MASK {
    #[doc = "PWM_X Masks"]
    pub mod MASKX {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_X output normal."]
            pub const MASKX_0: u16 = 0;
            #[doc = "PWM_X output masked."]
            pub const MASKX_1: u16 = 0x01;
        }
    }
    #[doc = "PWM_B Masks"]
    pub mod MASKB {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_B output normal."]
            pub const MASKB_0: u16 = 0;
            #[doc = "PWM_B output masked."]
            pub const MASKB_1: u16 = 0x01;
        }
    }
    #[doc = "PWM_A Masks"]
    pub mod MASKA {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_A output normal."]
            pub const MASKA_0: u16 = 0;
            #[doc = "PWM_A output masked."]
            pub const MASKA_1: u16 = 0x01;
        }
    }
    #[doc = "Update Mask Bits Immediately"]
    pub mod UPDATE_MASK {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule."]
            pub const UPDATE_MASK_0: u16 = 0;
            #[doc = "Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit."]
            pub const UPDATE_MASK_1: u16 = 0x01;
        }
    }
}
#[doc = "Software Controlled Output Register"]
pub mod SWCOUT {
    #[doc = "Submodule 0 Software Controlled Output 45"]
    pub mod SM0OUT45 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
            pub const SM0OUT45_0: u16 = 0;
            #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
            pub const SM0OUT45_1: u16 = 0x01;
        }
    }
    #[doc = "Submodule 0 Software Controlled Output 23"]
    pub mod SM0OUT23 {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
            pub const SM0OUT23_0: u16 = 0;
            #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
            pub const SM0OUT23_1: u16 = 0x01;
        }
    }
    #[doc = "Submodule 1 Software Controlled Output 45"]
    pub mod SM1OUT45 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
            pub const SM1OUT45_0: u16 = 0;
            #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
            pub const SM1OUT45_1: u16 = 0x01;
        }
    }
    #[doc = "Submodule 1 Software Controlled Output 23"]
    pub mod SM1OUT23 {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
            pub const SM1OUT23_0: u16 = 0;
            #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
            pub const SM1OUT23_1: u16 = 0x01;
        }
    }
    #[doc = "Submodule 2 Software Controlled Output 45"]
    pub mod SM2OUT45 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
            pub const SM2OUT45_0: u16 = 0;
            #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
            pub const SM2OUT45_1: u16 = 0x01;
        }
    }
    #[doc = "Submodule 2 Software Controlled Output 23"]
    pub mod SM2OUT23 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
            pub const SM2OUT23_0: u16 = 0;
            #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
            pub const SM2OUT23_1: u16 = 0x01;
        }
    }
    #[doc = "Submodule 3 Software Controlled Output 45"]
    pub mod SM3OUT45 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
            pub const SM3OUT45_0: u16 = 0;
            #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
            pub const SM3OUT45_1: u16 = 0x01;
        }
    }
    #[doc = "Submodule 3 Software Controlled Output 23"]
    pub mod SM3OUT23 {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
            pub const SM3OUT23_0: u16 = 0;
            #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
            pub const SM3OUT23_1: u16 = 0x01;
        }
    }
}
#[doc = "PWM Source Select Register"]
pub mod DTSRCSEL {
    #[doc = "Submodule 0 PWM45 Control Select"]
    pub mod SM0SEL45 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generated SM0PWM45 signal is used by the deadtime logic."]
            pub const SM0SEL45_0: u16 = 0;
            #[doc = "Inverted generated SM0PWM45 signal is used by the deadtime logic."]
            pub const SM0SEL45_1: u16 = 0x01;
            #[doc = "SWCOUT\\[SM0OUT45\\] is used by the deadtime logic."]
            pub const SM0SEL45_2: u16 = 0x02;
            #[doc = "PWM0_EXTB signal is used by the deadtime logic."]
            pub const SM0SEL45_3: u16 = 0x03;
        }
    }
    #[doc = "Submodule 0 PWM23 Control Select"]
    pub mod SM0SEL23 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generated SM0PWM23 signal is used by the deadtime logic."]
            pub const SM0SEL23_0: u16 = 0;
            #[doc = "Inverted generated SM0PWM23 signal is used by the deadtime logic."]
            pub const SM0SEL23_1: u16 = 0x01;
            #[doc = "SWCOUT\\[SM0OUT23\\] is used by the deadtime logic."]
            pub const SM0SEL23_2: u16 = 0x02;
            #[doc = "PWM0_EXTA signal is used by the deadtime logic."]
            pub const SM0SEL23_3: u16 = 0x03;
        }
    }
    #[doc = "Submodule 1 PWM45 Control Select"]
    pub mod SM1SEL45 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generated SM1PWM45 signal is used by the deadtime logic."]
            pub const SM1SEL45_0: u16 = 0;
            #[doc = "Inverted generated SM1PWM45 signal is used by the deadtime logic."]
            pub const SM1SEL45_1: u16 = 0x01;
            #[doc = "SWCOUT\\[SM1OUT45\\] is used by the deadtime logic."]
            pub const SM1SEL45_2: u16 = 0x02;
            #[doc = "PWM1_EXTB signal is used by the deadtime logic."]
            pub const SM1SEL45_3: u16 = 0x03;
        }
    }
    #[doc = "Submodule 1 PWM23 Control Select"]
    pub mod SM1SEL23 {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generated SM1PWM23 signal is used by the deadtime logic."]
            pub const SM1SEL23_0: u16 = 0;
            #[doc = "Inverted generated SM1PWM23 signal is used by the deadtime logic."]
            pub const SM1SEL23_1: u16 = 0x01;
            #[doc = "SWCOUT\\[SM1OUT23\\] is used by the deadtime logic."]
            pub const SM1SEL23_2: u16 = 0x02;
            #[doc = "PWM1_EXTA signal is used by the deadtime logic."]
            pub const SM1SEL23_3: u16 = 0x03;
        }
    }
    #[doc = "Submodule 2 PWM45 Control Select"]
    pub mod SM2SEL45 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generated SM2PWM45 signal is used by the deadtime logic."]
            pub const SM2SEL45_0: u16 = 0;
            #[doc = "Inverted generated SM2PWM45 signal is used by the deadtime logic."]
            pub const SM2SEL45_1: u16 = 0x01;
            #[doc = "SWCOUT\\[SM2OUT45\\] is used by the deadtime logic."]
            pub const SM2SEL45_2: u16 = 0x02;
            #[doc = "PWM2_EXTB signal is used by the deadtime logic."]
            pub const SM2SEL45_3: u16 = 0x03;
        }
    }
    #[doc = "Submodule 2 PWM23 Control Select"]
    pub mod SM2SEL23 {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generated SM2PWM23 signal is used by the deadtime logic."]
            pub const SM2SEL23_0: u16 = 0;
            #[doc = "Inverted generated SM2PWM23 signal is used by the deadtime logic."]
            pub const SM2SEL23_1: u16 = 0x01;
            #[doc = "SWCOUT\\[SM2OUT23\\] is used by the deadtime logic."]
            pub const SM2SEL23_2: u16 = 0x02;
            #[doc = "PWM2_EXTA signal is used by the deadtime logic."]
            pub const SM2SEL23_3: u16 = 0x03;
        }
    }
    #[doc = "Submodule 3 PWM45 Control Select"]
    pub mod SM3SEL45 {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generated SM3PWM45 signal is used by the deadtime logic."]
            pub const SM3SEL45_0: u16 = 0;
            #[doc = "Inverted generated SM3PWM45 signal is used by the deadtime logic."]
            pub const SM3SEL45_1: u16 = 0x01;
            #[doc = "SWCOUT\\[SM3OUT45\\] is used by the deadtime logic."]
            pub const SM3SEL45_2: u16 = 0x02;
            #[doc = "PWM3_EXTB signal is used by the deadtime logic."]
            pub const SM3SEL45_3: u16 = 0x03;
        }
    }
    #[doc = "Submodule 3 PWM23 Control Select"]
    pub mod SM3SEL23 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generated SM3PWM23 signal is used by the deadtime logic."]
            pub const SM3SEL23_0: u16 = 0;
            #[doc = "Inverted generated SM3PWM23 signal is used by the deadtime logic."]
            pub const SM3SEL23_1: u16 = 0x01;
            #[doc = "SWCOUT\\[SM3OUT23\\] is used by the deadtime logic."]
            pub const SM3SEL23_2: u16 = 0x02;
            #[doc = "PWM3_EXTA signal is used by the deadtime logic."]
            pub const SM3SEL23_3: u16 = 0x03;
        }
    }
}
#[doc = "Master Control Register"]
pub mod MCTRL {
    #[doc = "Load Okay"]
    pub mod LDOK {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not load new values."]
            pub const LDOK_0: u16 = 0;
            #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
            pub const LDOK_1: u16 = 0x01;
        }
    }
    #[doc = "Clear Load Okay"]
    pub mod CLDOK {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run"]
    pub mod RUN {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM generator is disabled in the corresponding submodule."]
            pub const RUN_0: u16 = 0;
            #[doc = "PWM generator is enabled in the corresponding submodule."]
            pub const RUN_1: u16 = 0x01;
        }
    }
    #[doc = "Current Polarity"]
    pub mod IPOL {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
            pub const IPOL_0: u16 = 0;
            #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
            pub const IPOL_1: u16 = 0x01;
        }
    }
}
#[doc = "Master Control 2 Register"]
pub mod MCTRL2 {
    #[doc = "Monitor PLL State"]
    pub mod MONPLL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
            pub const MONPLL_0: u16 = 0;
            #[doc = "Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
            pub const MONPLL_1: u16 = 0x01;
            #[doc = "Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
            pub const MONPLL_2: u16 = 0x02;
            #[doc = "Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
            pub const MONPLL_3: u16 = 0x03;
        }
    }
}
#[doc = "Fault Control Register"]
pub mod FCTRL0 {
    #[doc = "Fault Interrupt Enables"]
    pub mod FIE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FAULTx CPU interrupt requests disabled."]
            pub const FIE_0: u16 = 0;
            #[doc = "FAULTx CPU interrupt requests enabled."]
            pub const FIE_1: u16 = 0x01;
        }
    }
    #[doc = "Fault Safety Mode"]
    pub mod FSAFE {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFPINx\\]. If neither FHALF nor FFULL is setm then the fault condition cannot be cleared. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
            pub const FSAFE_0: u16 = 0;
            #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear and FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FHLAF nor FFULL is set, then the fault condition cannot be cleared."]
            pub const FSAFE_1: u16 = 0x01;
        }
    }
    #[doc = "Automatic Fault Clearing"]
    pub mod FAUTO {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared. This is further controlled by FCTRL\\[FSAFE\\]."]
            pub const FAUTO_0: u16 = 0;
            #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFLAGx\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared."]
            pub const FAUTO_1: u16 = 0x01;
        }
    }
    #[doc = "Fault Level"]
    pub mod FLVL {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A logic 0 on the fault input indicates a fault condition."]
            pub const FLVL_0: u16 = 0;
            #[doc = "A logic 1 on the fault input indicates a fault condition."]
            pub const FLVL_1: u16 = 0x01;
        }
    }
}
#[doc = "Fault Status Register"]
pub mod FSTS0 {
    #[doc = "Fault Flags"]
    pub mod FFLAG {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No fault on the FAULTx pin."]
            pub const FFLAG_0: u16 = 0;
            #[doc = "Fault on the FAULTx pin."]
            pub const FFLAG_1: u16 = 0x01;
        }
    }
    #[doc = "Full Cycle"]
    pub mod FFULL {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM outputs are not re-enabled at the start of a full cycle"]
            pub const FFULL_0: u16 = 0;
            #[doc = "PWM outputs are re-enabled at the start of a full cycle"]
            pub const FFULL_1: u16 = 0x01;
        }
    }
    #[doc = "Filtered Fault Pins"]
    pub mod FFPIN {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Cycle Fault Recovery"]
    pub mod FHALF {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
            pub const FHALF_0: u16 = 0;
            #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
            pub const FHALF_1: u16 = 0x01;
        }
    }
}
#[doc = "Fault Filter Register"]
pub mod FFILT0 {
    #[doc = "Fault Filter Period"]
    pub mod FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fault Filter Count"]
    pub mod FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fault Glitch Stretch Enable"]
    pub mod GSTR {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fault input glitch stretching is disabled."]
            pub const GSTR_0: u16 = 0;
            #[doc = "Input fault signals will be stretched to at least 2 IPBus clock cycles."]
            pub const GSTR_1: u16 = 0x01;
        }
    }
}
#[doc = "Fault Test Register"]
pub mod FTST0 {
    #[doc = "Fault Test"]
    pub mod FTEST {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No fault"]
            pub const FTEST_0: u16 = 0;
            #[doc = "Cause a simulated fault"]
            pub const FTEST_1: u16 = 0x01;
        }
    }
}
#[doc = "Fault Control 2 Register"]
pub mod FCTRL20 {
    #[doc = "No Combinational Path From Fault Input To PWM Output"]
    pub mod NOCOMB {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
            pub const NOCOMB_0: u16 = 0;
            #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
            pub const NOCOMB_1: u16 = 0x01;
        }
    }
}
pub mod sm {
    #[doc = "Cluster SM%s, containing SM?CNT, SM?INIT, SM?CTRL2, SM?CTRL, SM?VAL0, SM?FRACVAL1, SM?VAL1, SM?FRACVAL2, SM?VAL2, SM?FRACVAL3, SM?VAL3, SM?FRACVAL4, SM?VAL4, SM?FRACVAL5, SM?VAL5, SM?FRCTRL, SM?OCTRL, SM?STS, SM?INTEN, SM?DMAEN, SM?TCTRL, SM?DISMAP0, SM?DISMAP1, SM?DTCNT0, SM?DTCNT1, SM?CAPTCTRLA, SM?CAPTCOMPA, SM?CAPTCTRLB, SM?CAPTCOMPB, SM?CAPTCTRLX, SM?CAPTCOMPX, SM?CVAL0, SM?CVAL0CYC, SM?CVAL1, SM?CVAL1CYC, SM?CVAL2, SM?CVAL2CYC, SM?CVAL3, SM?CVAL3CYC, SM?CVAL4, SM?CVAL4CYC, SM?CVAL5, SM?CVAL5CYC"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Counter Register"]
        pub SMCNT: crate::RORegister<u16>,
        #[doc = "Initial Count Register"]
        pub SMINIT: crate::RWRegister<u16>,
        #[doc = "Control 2 Register"]
        pub SMCTRL2: crate::RWRegister<u16>,
        #[doc = "Control Register"]
        pub SMCTRL: crate::RWRegister<u16>,
        _reserved0: [u8; 0x02],
        #[doc = "Value Register 0"]
        pub SMVAL0: crate::RWRegister<u16>,
        #[doc = "Fractional Value Register 1"]
        pub SMFRACVAL1: crate::RWRegister<u16>,
        #[doc = "Value Register 1"]
        pub SMVAL1: crate::RWRegister<u16>,
        #[doc = "Fractional Value Register 2"]
        pub SMFRACVAL2: crate::RWRegister<u16>,
        #[doc = "Value Register 2"]
        pub SMVAL2: crate::RWRegister<u16>,
        #[doc = "Fractional Value Register 3"]
        pub SMFRACVAL3: crate::RWRegister<u16>,
        #[doc = "Value Register 3"]
        pub SMVAL3: crate::RWRegister<u16>,
        #[doc = "Fractional Value Register 4"]
        pub SMFRACVAL4: crate::RWRegister<u16>,
        #[doc = "Value Register 4"]
        pub SMVAL4: crate::RWRegister<u16>,
        #[doc = "Fractional Value Register 5"]
        pub SMFRACVAL5: crate::RWRegister<u16>,
        #[doc = "Value Register 5"]
        pub SMVAL5: crate::RWRegister<u16>,
        #[doc = "Fractional Control Register"]
        pub SMFRCTRL: crate::RWRegister<u16>,
        #[doc = "Output Control Register"]
        pub SMOCTRL: crate::RWRegister<u16>,
        #[doc = "Status Register"]
        pub SMSTS: crate::RWRegister<u16>,
        #[doc = "Interrupt Enable Register"]
        pub SMINTEN: crate::RWRegister<u16>,
        #[doc = "DMA Enable Register"]
        pub SMDMAEN: crate::RWRegister<u16>,
        #[doc = "Output Trigger Control Register"]
        pub SMTCTRL: crate::RWRegister<u16>,
        #[doc = "Fault Disable Mapping Register 0"]
        pub SMDISMAP0: crate::RWRegister<u16>,
        #[doc = "Fault Disable Mapping Register 1"]
        pub SMDISMAP1: crate::RWRegister<u16>,
        #[doc = "Deadtime Count Register 0"]
        pub SMDTCNT0: crate::RWRegister<u16>,
        #[doc = "Deadtime Count Register 1"]
        pub SMDTCNT1: crate::RWRegister<u16>,
        #[doc = "Capture Control A Register"]
        pub SMCAPTCTRLA: crate::RWRegister<u16>,
        #[doc = "Capture Compare A Register"]
        pub SMCAPTCOMPA: crate::RWRegister<u16>,
        #[doc = "Capture Control B Register"]
        pub SMCAPTCTRLB: crate::RWRegister<u16>,
        #[doc = "Capture Compare B Register"]
        pub SMCAPTCOMPB: crate::RWRegister<u16>,
        #[doc = "Capture Control X Register"]
        pub SMCAPTCTRLX: crate::RWRegister<u16>,
        #[doc = "Capture Compare X Register"]
        pub SMCAPTCOMPX: crate::RWRegister<u16>,
        #[doc = "Capture Value 0 Register"]
        pub SMCVAL0: crate::RORegister<u16>,
        #[doc = "Capture Value 0 Cycle Register"]
        pub SMCVAL0CYC: crate::RORegister<u16>,
        #[doc = "Capture Value 1 Register"]
        pub SMCVAL1: crate::RORegister<u16>,
        #[doc = "Capture Value 1 Cycle Register"]
        pub SMCVAL1CYC: crate::RORegister<u16>,
        #[doc = "Capture Value 2 Register"]
        pub SMCVAL2: crate::RORegister<u16>,
        #[doc = "Capture Value 2 Cycle Register"]
        pub SMCVAL2CYC: crate::RORegister<u16>,
        #[doc = "Capture Value 3 Register"]
        pub SMCVAL3: crate::RORegister<u16>,
        #[doc = "Capture Value 3 Cycle Register"]
        pub SMCVAL3CYC: crate::RORegister<u16>,
        #[doc = "Capture Value 4 Register"]
        pub SMCVAL4: crate::RORegister<u16>,
        #[doc = "Capture Value 4 Cycle Register"]
        pub SMCVAL4CYC: crate::RORegister<u16>,
        #[doc = "Capture Value 5 Register"]
        pub SMCVAL5: crate::RORegister<u16>,
        #[doc = "Capture Value 5 Cycle Register"]
        pub SMCVAL5CYC: crate::RORegister<u16>,
        _reserved1: [u8; 0x08],
    }
    #[doc = "Counter Register"]
    pub mod SMCNT {
        #[doc = "Counter Register Bits"]
        pub mod CNT {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Initial Count Register"]
    pub mod SMINIT {
        #[doc = "Initial Count Register Bits"]
        pub mod INIT {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Control 2 Register"]
    pub mod SMCTRL2 {
        #[doc = "Clock Source Select"]
        pub mod CLK_SEL {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
                pub const CLK_SEL_0: u16 = 0;
                #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
                pub const CLK_SEL_1: u16 = 0x01;
                #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
                pub const CLK_SEL_2: u16 = 0x02;
            }
        }
        #[doc = "Reload Source Select"]
        pub mod RELOAD_SEL {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The local RELOAD signal is used to reload registers."]
                pub const RELOAD_SEL_0: u16 = 0;
                #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0."]
                pub const RELOAD_SEL_1: u16 = 0x01;
            }
        }
        #[doc = "This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
        pub mod FORCE_SEL {
            pub const offset: u16 = 3;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
                pub const FORCE_SEL_0: u16 = 0;
                #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
                pub const FORCE_SEL_1: u16 = 0x01;
                #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
                pub const FORCE_SEL_2: u16 = 0x02;
                #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
                pub const FORCE_SEL_3: u16 = 0x03;
                #[doc = "The local sync signal from this submodule is used to force updates."]
                pub const FORCE_SEL_4: u16 = 0x04;
                #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
                pub const FORCE_SEL_5: u16 = 0x05;
                #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
                pub const FORCE_SEL_6: u16 = 0x06;
                #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
                pub const FORCE_SEL_7: u16 = 0x07;
            }
        }
        #[doc = "Force Initialization"]
        pub mod FORCE {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "FRCEN"]
        pub mod FRCEN {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Initialization from a FORCE_OUT is disabled."]
                pub const FRCEN_0: u16 = 0;
                #[doc = "Initialization from a FORCE_OUT is enabled."]
                pub const FRCEN_1: u16 = 0x01;
            }
        }
        #[doc = "Initialization Control Select"]
        pub mod INIT_SEL {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Local sync (PWM_X) causes initialization."]
                pub const INIT_SEL_0: u16 = 0;
                #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
                pub const INIT_SEL_1: u16 = 0x01;
                #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
                pub const INIT_SEL_2: u16 = 0x02;
                #[doc = "EXT_SYNC causes initialization."]
                pub const INIT_SEL_3: u16 = 0x03;
            }
        }
        #[doc = "PWM_X Initial Value"]
        pub mod PWMX_INIT {
            pub const offset: u16 = 10;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PWM45 Initial Value"]
        pub mod PWM45_INIT {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PWM23 Initial Value"]
        pub mod PWM23_INIT {
            pub const offset: u16 = 12;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Independent or Complementary Pair Operation"]
        pub mod INDEP {
            pub const offset: u16 = 13;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
                pub const INDEP_0: u16 = 0;
                #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
                pub const INDEP_1: u16 = 0x01;
            }
        }
        #[doc = "WAIT Enable"]
        pub mod WAITEN {
            pub const offset: u16 = 14;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Debug Enable"]
        pub mod DBGEN {
            pub const offset: u16 = 15;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Control Register"]
    pub mod SMCTRL {
        #[doc = "Double Switching Enable"]
        pub mod DBLEN {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Double switching disabled."]
                pub const DBLEN_0: u16 = 0;
                #[doc = "Double switching enabled."]
                pub const DBLEN_1: u16 = 0x01;
            }
        }
        #[doc = "PWMX Double Switching Enable"]
        pub mod DBLX {
            pub const offset: u16 = 1;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PWMX double pulse disabled."]
                pub const DBLX_0: u16 = 0;
                #[doc = "PWMX double pulse enabled."]
                pub const DBLX_1: u16 = 0x01;
            }
        }
        #[doc = "Load Mode Select"]
        pub mod LDMOD {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
                pub const LDMOD_0: u16 = 0;
                #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
                pub const LDMOD_1: u16 = 0x01;
            }
        }
        #[doc = "Split the DBLPWM signal to PWMA and PWMB"]
        pub mod SPLIT {
            pub const offset: u16 = 3;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "DBLPWM is not split. PWMA and PWMB each have double pulses."]
                pub const SPLIT_0: u16 = 0;
                #[doc = "DBLPWM is split to PWMA and PWMB."]
                pub const SPLIT_1: u16 = 0x01;
            }
        }
        #[doc = "Prescaler"]
        pub mod PRSC {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PWM clock frequency = fclk"]
                pub const PRSC_0: u16 = 0;
                #[doc = "PWM clock frequency = fclk/2"]
                pub const PRSC_1: u16 = 0x01;
                #[doc = "PWM clock frequency = fclk/4"]
                pub const PRSC_2: u16 = 0x02;
                #[doc = "PWM clock frequency = fclk/8"]
                pub const PRSC_3: u16 = 0x03;
                #[doc = "PWM clock frequency = fclk/16"]
                pub const PRSC_4: u16 = 0x04;
                #[doc = "PWM clock frequency = fclk/32"]
                pub const PRSC_5: u16 = 0x05;
                #[doc = "PWM clock frequency = fclk/64"]
                pub const PRSC_6: u16 = 0x06;
                #[doc = "PWM clock frequency = fclk/128"]
                pub const PRSC_7: u16 = 0x07;
            }
        }
        #[doc = "Compare Mode"]
        pub mod COMPMODE {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period."]
                pub const COMPMODE_0: u16 = 0;
                #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
                pub const COMPMODE_1: u16 = 0x01;
            }
        }
        #[doc = "Deadtime"]
        pub mod DT {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Full Cycle Reload"]
        pub mod FULL {
            pub const offset: u16 = 10;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Full-cycle reloads disabled."]
                pub const FULL_0: u16 = 0;
                #[doc = "Full-cycle reloads enabled."]
                pub const FULL_1: u16 = 0x01;
            }
        }
        #[doc = "Half Cycle Reload"]
        pub mod HALF {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Half-cycle reloads disabled."]
                pub const HALF_0: u16 = 0;
                #[doc = "Half-cycle reloads enabled."]
                pub const HALF_1: u16 = 0x01;
            }
        }
        #[doc = "Load Frequency"]
        pub mod LDFQ {
            pub const offset: u16 = 12;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Every PWM opportunity"]
                pub const LDFQ_0: u16 = 0;
                #[doc = "Every 2 PWM opportunities"]
                pub const LDFQ_1: u16 = 0x01;
                #[doc = "Every 3 PWM opportunities"]
                pub const LDFQ_2: u16 = 0x02;
                #[doc = "Every 4 PWM opportunities"]
                pub const LDFQ_3: u16 = 0x03;
                #[doc = "Every 5 PWM opportunities"]
                pub const LDFQ_4: u16 = 0x04;
                #[doc = "Every 6 PWM opportunities"]
                pub const LDFQ_5: u16 = 0x05;
                #[doc = "Every 7 PWM opportunities"]
                pub const LDFQ_6: u16 = 0x06;
                #[doc = "Every 8 PWM opportunities"]
                pub const LDFQ_7: u16 = 0x07;
                #[doc = "Every 9 PWM opportunities"]
                pub const LDFQ_8: u16 = 0x08;
                #[doc = "Every 10 PWM opportunities"]
                pub const LDFQ_9: u16 = 0x09;
                #[doc = "Every 11 PWM opportunities"]
                pub const LDFQ_10: u16 = 0x0a;
                #[doc = "Every 12 PWM opportunities"]
                pub const LDFQ_11: u16 = 0x0b;
                #[doc = "Every 13 PWM opportunities"]
                pub const LDFQ_12: u16 = 0x0c;
                #[doc = "Every 14 PWM opportunities"]
                pub const LDFQ_13: u16 = 0x0d;
                #[doc = "Every 15 PWM opportunities"]
                pub const LDFQ_14: u16 = 0x0e;
                #[doc = "Every 16 PWM opportunities"]
                pub const LDFQ_15: u16 = 0x0f;
            }
        }
    }
    #[doc = "Value Register 0"]
    pub mod SMVAL0 {
        #[doc = "Value Register 0"]
        pub mod VAL0 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Fractional Value Register 1"]
    pub mod SMFRACVAL1 {
        #[doc = "Fractional Value 1 Register"]
        pub mod FRACVAL1 {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Value Register 1"]
    pub mod SMVAL1 {
        #[doc = "Value Register 1"]
        pub mod VAL1 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Fractional Value Register 2"]
    pub mod SMFRACVAL2 {
        #[doc = "Fractional Value 2"]
        pub mod FRACVAL2 {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Value Register 2"]
    pub mod SMVAL2 {
        #[doc = "Value Register 2"]
        pub mod VAL2 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Fractional Value Register 3"]
    pub mod SMFRACVAL3 {
        #[doc = "Fractional Value 3"]
        pub mod FRACVAL3 {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Value Register 3"]
    pub mod SMVAL3 {
        #[doc = "Value Register 3"]
        pub mod VAL3 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Fractional Value Register 4"]
    pub mod SMFRACVAL4 {
        #[doc = "Fractional Value 4"]
        pub mod FRACVAL4 {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Value Register 4"]
    pub mod SMVAL4 {
        #[doc = "Value Register 4"]
        pub mod VAL4 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Fractional Value Register 5"]
    pub mod SMFRACVAL5 {
        #[doc = "Fractional Value 5"]
        pub mod FRACVAL5 {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x1f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Value Register 5"]
    pub mod SMVAL5 {
        #[doc = "Value Register 5"]
        pub mod VAL5 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Fractional Control Register"]
    pub mod SMFRCTRL {
        #[doc = "Fractional Cycle PWM Period Enable"]
        pub mod FRAC1_EN {
            pub const offset: u16 = 1;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable fractional cycle length for the PWM period."]
                pub const FRAC1_EN_0: u16 = 0;
                #[doc = "Enable fractional cycle length for the PWM period."]
                pub const FRAC1_EN_1: u16 = 0x01;
            }
        }
        #[doc = "Fractional Cycle Placement Enable for PWM_A"]
        pub mod FRAC23_EN {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable fractional cycle placement for PWM_A."]
                pub const FRAC23_EN_0: u16 = 0;
                #[doc = "Enable fractional cycle placement for PWM_A."]
                pub const FRAC23_EN_1: u16 = 0x01;
            }
        }
        #[doc = "Fractional Cycle Placement Enable for PWM_B"]
        pub mod FRAC45_EN {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable fractional cycle placement for PWM_B."]
                pub const FRAC45_EN_0: u16 = 0;
                #[doc = "Enable fractional cycle placement for PWM_B."]
                pub const FRAC45_EN_1: u16 = 0x01;
            }
        }
        #[doc = "Fractional Delay Circuit Power Up"]
        pub mod FRAC_PU {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Turn off fractional delay logic."]
                pub const FRAC_PU_0: u16 = 0;
                #[doc = "Power up fractional delay logic."]
                pub const FRAC_PU_1: u16 = 0x01;
            }
        }
        #[doc = "Test Status Bit"]
        pub mod TEST {
            pub const offset: u16 = 15;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Output Control Register"]
    pub mod SMOCTRL {
        #[doc = "PWM_X Fault State"]
        pub mod PWMXFS {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
                pub const PWMXFS_0: u16 = 0;
                #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
                pub const PWMXFS_1: u16 = 0x01;
                #[doc = "Output is tristated."]
                pub const PWMXFS_2: u16 = 0x02;
                #[doc = "Output is tristated."]
                pub const PWMXFS_3: u16 = 0x03;
            }
        }
        #[doc = "PWM_B Fault State"]
        pub mod PWMBFS {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
                pub const PWMBFS_0: u16 = 0;
                #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
                pub const PWMBFS_1: u16 = 0x01;
                #[doc = "Output is tristated."]
                pub const PWMBFS_2: u16 = 0x02;
                #[doc = "Output is tristated."]
                pub const PWMBFS_3: u16 = 0x03;
            }
        }
        #[doc = "PWM_A Fault State"]
        pub mod PWMAFS {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
                pub const PWMAFS_0: u16 = 0;
                #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
                pub const PWMAFS_1: u16 = 0x01;
                #[doc = "Output is tristated."]
                pub const PWMAFS_2: u16 = 0x02;
                #[doc = "Output is tristated."]
                pub const PWMAFS_3: u16 = 0x03;
            }
        }
        #[doc = "PWM_X Output Polarity"]
        pub mod POLX {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
                pub const POLX_0: u16 = 0;
                #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
                pub const POLX_1: u16 = 0x01;
            }
        }
        #[doc = "PWM_B Output Polarity"]
        pub mod POLB {
            pub const offset: u16 = 9;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
                pub const POLB_0: u16 = 0;
                #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
                pub const POLB_1: u16 = 0x01;
            }
        }
        #[doc = "PWM_A Output Polarity"]
        pub mod POLA {
            pub const offset: u16 = 10;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
                pub const POLA_0: u16 = 0;
                #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
                pub const POLA_1: u16 = 0x01;
            }
        }
        #[doc = "PWM_X Input"]
        pub mod PWMX_IN {
            pub const offset: u16 = 13;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PWM_B Input"]
        pub mod PWMB_IN {
            pub const offset: u16 = 14;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PWM_A Input"]
        pub mod PWMA_IN {
            pub const offset: u16 = 15;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Status Register"]
    pub mod SMSTS {
        #[doc = "Compare Flags"]
        pub mod CMPF {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x3f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No compare event has occurred for a particular VALx value."]
                pub const CMPF_0: u16 = 0;
                #[doc = "A compare event has occurred for a particular VALx value."]
                pub const CMPF_1: u16 = 0x01;
            }
        }
        #[doc = "Capture Flag X0"]
        pub mod CFX0 {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture Flag X1"]
        pub mod CFX1 {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture Flag B0"]
        pub mod CFB0 {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture Flag B1"]
        pub mod CFB1 {
            pub const offset: u16 = 9;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture Flag A0"]
        pub mod CFA0 {
            pub const offset: u16 = 10;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture Flag A1"]
        pub mod CFA1 {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Reload Flag"]
        pub mod RF {
            pub const offset: u16 = 12;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No new reload cycle since last STS\\[RF\\] clearing"]
                pub const RF_0: u16 = 0;
                #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
                pub const RF_1: u16 = 0x01;
            }
        }
        #[doc = "Reload Error Flag"]
        pub mod REF {
            pub const offset: u16 = 13;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No reload error occurred."]
                pub const REF_0: u16 = 0;
                #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
                pub const REF_1: u16 = 0x01;
            }
        }
        #[doc = "Registers Updated Flag"]
        pub mod RUF {
            pub const offset: u16 = 14;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No register update has occurred since last reload."]
                pub const RUF_0: u16 = 0;
                #[doc = "At least one of the double buffered registers has been updated since the last reload."]
                pub const RUF_1: u16 = 0x01;
            }
        }
    }
    #[doc = "Interrupt Enable Register"]
    pub mod SMINTEN {
        #[doc = "Compare Interrupt Enables"]
        pub mod CMPIE {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x3f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
                pub const CMPIE_0: u16 = 0;
                #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
                pub const CMPIE_1: u16 = 0x01;
            }
        }
        #[doc = "Capture X 0 Interrupt Enable"]
        pub mod CX0IE {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Interrupt request disabled for STS\\[CFX0\\]."]
                pub const CX0IE_0: u16 = 0;
                #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
                pub const CX0IE_1: u16 = 0x01;
            }
        }
        #[doc = "Capture X 1 Interrupt Enable"]
        pub mod CX1IE {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Interrupt request disabled for STS\\[CFX1\\]."]
                pub const CX1IE_0: u16 = 0;
                #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
                pub const CX1IE_1: u16 = 0x01;
            }
        }
        #[doc = "Capture B 0 Interrupt Enable"]
        pub mod CB0IE {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Interrupt request disabled for STS\\[CFB0\\]."]
                pub const CB0IE_0: u16 = 0;
                #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
                pub const CB0IE_1: u16 = 0x01;
            }
        }
        #[doc = "Capture B 1 Interrupt Enable"]
        pub mod CB1IE {
            pub const offset: u16 = 9;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Interrupt request disabled for STS\\[CFB1\\]."]
                pub const CB1IE_0: u16 = 0;
                #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
                pub const CB1IE_1: u16 = 0x01;
            }
        }
        #[doc = "Capture A 0 Interrupt Enable"]
        pub mod CA0IE {
            pub const offset: u16 = 10;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Interrupt request disabled for STS\\[CFA0\\]."]
                pub const CA0IE_0: u16 = 0;
                #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
                pub const CA0IE_1: u16 = 0x01;
            }
        }
        #[doc = "Capture A 1 Interrupt Enable"]
        pub mod CA1IE {
            pub const offset: u16 = 11;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Interrupt request disabled for STS\\[CFA1\\]."]
                pub const CA1IE_0: u16 = 0;
                #[doc = "Interrupt request enabled for STS\\[CFA1\\]."]
                pub const CA1IE_1: u16 = 0x01;
            }
        }
        #[doc = "Reload Interrupt Enable"]
        pub mod RIE {
            pub const offset: u16 = 12;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "STS\\[RF\\] CPU interrupt requests disabled"]
                pub const RIE_0: u16 = 0;
                #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
                pub const RIE_1: u16 = 0x01;
            }
        }
        #[doc = "Reload Error Interrupt Enable"]
        pub mod REIE {
            pub const offset: u16 = 13;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "STS\\[REF\\] CPU interrupt requests disabled"]
                pub const REIE_0: u16 = 0;
                #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
                pub const REIE_1: u16 = 0x01;
            }
        }
    }
    #[doc = "DMA Enable Register"]
    pub mod SMDMAEN {
        #[doc = "Capture X0 FIFO DMA Enable"]
        pub mod CX0DE {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture X1 FIFO DMA Enable"]
        pub mod CX1DE {
            pub const offset: u16 = 1;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture B0 FIFO DMA Enable"]
        pub mod CB0DE {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture B1 FIFO DMA Enable"]
        pub mod CB1DE {
            pub const offset: u16 = 3;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture A0 FIFO DMA Enable"]
        pub mod CA0DE {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture A1 FIFO DMA Enable"]
        pub mod CA1DE {
            pub const offset: u16 = 5;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture DMA Enable Source Select"]
        pub mod CAPTDE {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Read DMA requests disabled."]
                pub const CAPTDE_0: u16 = 0;
                #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
                pub const CAPTDE_1: u16 = 0x01;
                #[doc = "A local sync (VAL1 matches counter) sets the read DMA request."]
                pub const CAPTDE_2: u16 = 0x02;
                #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
                pub const CAPTDE_3: u16 = 0x03;
            }
        }
        #[doc = "FIFO Watermark AND Control"]
        pub mod FAND {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Selected FIFO watermarks are OR'ed together."]
                pub const FAND_0: u16 = 0;
                #[doc = "Selected FIFO watermarks are AND'ed together."]
                pub const FAND_1: u16 = 0x01;
            }
        }
        #[doc = "Value Registers DMA Enable"]
        pub mod VALDE {
            pub const offset: u16 = 9;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "DMA write requests disabled"]
                pub const VALDE_0: u16 = 0;
                #[doc = "DMA write requests for the VALx and FRACVALx registers enabled"]
                pub const VALDE_1: u16 = 0x01;
            }
        }
    }
    #[doc = "Output Trigger Control Register"]
    pub mod SMTCTRL {
        #[doc = "Output Trigger Enables"]
        pub mod OUT_TRIG_EN {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x3f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PWM_OUT_TRIGx will not set when the counter value matches the VALx value."]
                pub const OUT_TRIG_EN_0: u16 = 0;
                #[doc = "PWM_OUT_TRIGx will set when the counter value matches the VALx value."]
                pub const OUT_TRIG_EN_1: u16 = 0x01;
            }
        }
        #[doc = "Trigger frequency"]
        pub mod TRGFRQ {
            pub const offset: u16 = 12;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
                pub const TRGFRQ_0: u16 = 0;
                #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
                pub const TRGFRQ_1: u16 = 0x01;
            }
        }
        #[doc = "Output Trigger 1 Source Select"]
        pub mod PWBOT1 {
            pub const offset: u16 = 14;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port."]
                pub const PWBOT1_0: u16 = 0;
                #[doc = "Route the PWMB output to the PWM_OUT_TRIG1 port."]
                pub const PWBOT1_1: u16 = 0x01;
            }
        }
        #[doc = "Output Trigger 0 Source Select"]
        pub mod PWAOT0 {
            pub const offset: u16 = 15;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port."]
                pub const PWAOT0_0: u16 = 0;
                #[doc = "Route the PWMA output to the PWM_OUT_TRIG0 port."]
                pub const PWAOT0_1: u16 = 0x01;
            }
        }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    pub mod SMDISMAP0 {
        #[doc = "PWM_A Fault Disable Mask 0"]
        pub mod DIS0A {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PWM_B Fault Disable Mask 0"]
        pub mod DIS0B {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PWM_X Fault Disable Mask 0"]
        pub mod DIS0X {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Fault Disable Mapping Register 1"]
    pub mod SMDISMAP1 {
        #[doc = "PWM_A Fault Disable Mask 1"]
        pub mod DIS1A {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PWM_B Fault Disable Mask 1"]
        pub mod DIS1B {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PWM_X Fault Disable Mask 1"]
        pub mod DIS1X {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Deadtime Count Register 0"]
    pub mod SMDTCNT0 {
        #[doc = "DTCNT0"]
        pub mod DTCNT0 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Deadtime Count Register 1"]
    pub mod SMDTCNT1 {
        #[doc = "DTCNT1"]
        pub mod DTCNT1 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Control A Register"]
    pub mod SMCAPTCTRLA {
        #[doc = "Arm A"]
        pub mod ARMA {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Input capture operation is disabled."]
                pub const ARMA_0: u16 = 0;
                #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
                pub const ARMA_1: u16 = 0x01;
            }
        }
        #[doc = "One Shot Mode A"]
        pub mod ONESHOTA {
            pub const offset: u16 = 1;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\\[ARMA\\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
                pub const ONESHOTA_0: u16 = 0;
                #[doc = "One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\\[ARMA\\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLA\\[ARMA\\] is cleared. No further captures will be performed until CAPTCTRLA\\[ARMA\\] is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLA\\[ARMA\\] is then cleared."]
                pub const ONESHOTA_1: u16 = 0x01;
            }
        }
        #[doc = "Edge A 0"]
        pub mod EDGA0 {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const EDGA0_0: u16 = 0;
                #[doc = "Capture falling edges"]
                pub const EDGA0_1: u16 = 0x01;
                #[doc = "Capture rising edges"]
                pub const EDGA0_2: u16 = 0x02;
                #[doc = "Capture any edge"]
                pub const EDGA0_3: u16 = 0x03;
            }
        }
        #[doc = "Edge A 1"]
        pub mod EDGA1 {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const EDGA1_0: u16 = 0;
                #[doc = "Capture falling edges"]
                pub const EDGA1_1: u16 = 0x01;
                #[doc = "Capture rising edges"]
                pub const EDGA1_2: u16 = 0x02;
                #[doc = "Capture any edge"]
                pub const EDGA1_3: u16 = 0x03;
            }
        }
        #[doc = "Input Select A"]
        pub mod INP_SELA {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Raw PWM_A input signal selected as source."]
                pub const INP_SELA_0: u16 = 0;
                #[doc = "Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLA\\[EDGA0\\] and CAPTCTRLA\\[EDGA1\\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRA\\[EDGA0\\] and/or CAPTCTRLA\\[EDGA1\\] fields in order to enable one or both of the capture registers."]
                pub const INP_SELA_1: u16 = 0x01;
            }
        }
        #[doc = "Edge Counter A Enable"]
        pub mod EDGCNTA_EN {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Edge counter disabled and held in reset"]
                pub const EDGCNTA_EN_0: u16 = 0;
                #[doc = "Edge counter enabled"]
                pub const EDGCNTA_EN_1: u16 = 0x01;
            }
        }
        #[doc = "Capture A FIFOs Water Mark"]
        pub mod CFAWM {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture A0 FIFO Word Count"]
        pub mod CA0CNT {
            pub const offset: u16 = 10;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture A1 FIFO Word Count"]
        pub mod CA1CNT {
            pub const offset: u16 = 13;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Compare A Register"]
    pub mod SMCAPTCOMPA {
        #[doc = "Edge Compare A"]
        pub mod EDGCMPA {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Edge Counter A"]
        pub mod EDGCNTA {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Control B Register"]
    pub mod SMCAPTCTRLB {
        #[doc = "Arm B"]
        pub mod ARMB {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Input capture operation is disabled."]
                pub const ARMB_0: u16 = 0;
                #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
                pub const ARMB_1: u16 = 0x01;
            }
        }
        #[doc = "One Shot Mode B"]
        pub mod ONESHOTB {
            pub const offset: u16 = 1;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\\[ARMB\\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
                pub const ONESHOTB_0: u16 = 0;
                #[doc = "One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\\[ARMB\\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLB\\[ARMB\\] is cleared. No further captures will be performed until CAPTCTRLB\\[ARMB\\] is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLB\\[ARMB\\] is then cleared."]
                pub const ONESHOTB_1: u16 = 0x01;
            }
        }
        #[doc = "Edge B 0"]
        pub mod EDGB0 {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const EDGB0_0: u16 = 0;
                #[doc = "Capture falling edges"]
                pub const EDGB0_1: u16 = 0x01;
                #[doc = "Capture rising edges"]
                pub const EDGB0_2: u16 = 0x02;
                #[doc = "Capture any edge"]
                pub const EDGB0_3: u16 = 0x03;
            }
        }
        #[doc = "Edge B 1"]
        pub mod EDGB1 {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const EDGB1_0: u16 = 0;
                #[doc = "Capture falling edges"]
                pub const EDGB1_1: u16 = 0x01;
                #[doc = "Capture rising edges"]
                pub const EDGB1_2: u16 = 0x02;
                #[doc = "Capture any edge"]
                pub const EDGB1_3: u16 = 0x03;
            }
        }
        #[doc = "Input Select B"]
        pub mod INP_SELB {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Raw PWM_B input signal selected as source."]
                pub const INP_SELB_0: u16 = 0;
                #[doc = "Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLB\\[EDGB0\\] and CAPTCTRLB\\[EDGB1\\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRB\\[EDGB0\\] and/or CAPTCTRLB\\[EDGB1\\] fields in order to enable one or both of the capture registers."]
                pub const INP_SELB_1: u16 = 0x01;
            }
        }
        #[doc = "Edge Counter B Enable"]
        pub mod EDGCNTB_EN {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Edge counter disabled and held in reset"]
                pub const EDGCNTB_EN_0: u16 = 0;
                #[doc = "Edge counter enabled"]
                pub const EDGCNTB_EN_1: u16 = 0x01;
            }
        }
        #[doc = "Capture B FIFOs Water Mark"]
        pub mod CFBWM {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture B0 FIFO Word Count"]
        pub mod CB0CNT {
            pub const offset: u16 = 10;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture B1 FIFO Word Count"]
        pub mod CB1CNT {
            pub const offset: u16 = 13;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Compare B Register"]
    pub mod SMCAPTCOMPB {
        #[doc = "Edge Compare B"]
        pub mod EDGCMPB {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Edge Counter B"]
        pub mod EDGCNTB {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Control X Register"]
    pub mod SMCAPTCTRLX {
        #[doc = "Arm X"]
        pub mod ARMX {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Input capture operation is disabled."]
                pub const ARMX_0: u16 = 0;
                #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
                pub const ARMX_1: u16 = 0x01;
            }
        }
        #[doc = "One Shot Mode Aux"]
        pub mod ONESHOTX {
            pub const offset: u16 = 1;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
                pub const ONESHOTX_0: u16 = 0;
                #[doc = "One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and the ARMX bit is cleared. No further captures will be performed until the ARMX bit is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and the ARMX bit is then cleared."]
                pub const ONESHOTX_1: u16 = 0x01;
            }
        }
        #[doc = "Edge X 0"]
        pub mod EDGX0 {
            pub const offset: u16 = 2;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const EDGX0_0: u16 = 0;
                #[doc = "Capture falling edges"]
                pub const EDGX0_1: u16 = 0x01;
                #[doc = "Capture rising edges"]
                pub const EDGX0_2: u16 = 0x02;
                #[doc = "Capture any edge"]
                pub const EDGX0_3: u16 = 0x03;
            }
        }
        #[doc = "Edge X 1"]
        pub mod EDGX1 {
            pub const offset: u16 = 4;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const EDGX1_0: u16 = 0;
                #[doc = "Capture falling edges"]
                pub const EDGX1_1: u16 = 0x01;
                #[doc = "Capture rising edges"]
                pub const EDGX1_2: u16 = 0x02;
                #[doc = "Capture any edge"]
                pub const EDGX1_3: u16 = 0x03;
            }
        }
        #[doc = "Input Select X"]
        pub mod INP_SELX {
            pub const offset: u16 = 6;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Raw PWM_X input signal selected as source."]
                pub const INP_SELX_0: u16 = 0;
                #[doc = "Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLX\\[EDGX0\\] and CAPTCTRLX\\[EDGX1\\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRX\\[EDGX0\\] and/or CAPTCTRLX\\[EDGX1\\] fields in order to enable one or both of the capture registers."]
                pub const INP_SELX_1: u16 = 0x01;
            }
        }
        #[doc = "Edge Counter X Enable"]
        pub mod EDGCNTX_EN {
            pub const offset: u16 = 7;
            pub const mask: u16 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Edge counter disabled and held in reset"]
                pub const EDGCNTX_EN_0: u16 = 0;
                #[doc = "Edge counter enabled"]
                pub const EDGCNTX_EN_1: u16 = 0x01;
            }
        }
        #[doc = "Capture X FIFOs Water Mark"]
        pub mod CFXWM {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture X0 FIFO Word Count"]
        pub mod CX0CNT {
            pub const offset: u16 = 10;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Capture X1 FIFO Word Count"]
        pub mod CX1CNT {
            pub const offset: u16 = 13;
            pub const mask: u16 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Compare X Register"]
    pub mod SMCAPTCOMPX {
        #[doc = "Edge Compare X"]
        pub mod EDGCMPX {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Edge Counter X"]
        pub mod EDGCNTX {
            pub const offset: u16 = 8;
            pub const mask: u16 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 0 Register"]
    pub mod SMCVAL0 {
        #[doc = "CAPTVAL0"]
        pub mod CAPTVAL0 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    pub mod SMCVAL0CYC {
        #[doc = "CVAL0CYC"]
        pub mod CVAL0CYC {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 1 Register"]
    pub mod SMCVAL1 {
        #[doc = "CAPTVAL1"]
        pub mod CAPTVAL1 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    pub mod SMCVAL1CYC {
        #[doc = "CVAL1CYC"]
        pub mod CVAL1CYC {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 2 Register"]
    pub mod SMCVAL2 {
        #[doc = "CAPTVAL2"]
        pub mod CAPTVAL2 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 2 Cycle Register"]
    pub mod SMCVAL2CYC {
        #[doc = "CVAL2CYC"]
        pub mod CVAL2CYC {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 3 Register"]
    pub mod SMCVAL3 {
        #[doc = "CAPTVAL3"]
        pub mod CAPTVAL3 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 3 Cycle Register"]
    pub mod SMCVAL3CYC {
        #[doc = "CVAL3CYC"]
        pub mod CVAL3CYC {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 4 Register"]
    pub mod SMCVAL4 {
        #[doc = "CAPTVAL4"]
        pub mod CAPTVAL4 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 4 Cycle Register"]
    pub mod SMCVAL4CYC {
        #[doc = "CVAL4CYC"]
        pub mod CVAL4CYC {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 5 Register"]
    pub mod SMCVAL5 {
        #[doc = "CAPTVAL5"]
        pub mod CAPTVAL5 {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Capture Value 5 Cycle Register"]
    pub mod SMCVAL5CYC {
        #[doc = "CVAL5CYC"]
        pub mod CVAL5CYC {
            pub const offset: u16 = 0;
            pub const mask: u16 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
