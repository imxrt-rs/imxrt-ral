#[doc = "PWM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Counter Register"]
    pub SM0CNT: crate::RORegister<u16>,
    #[doc = "Initial Count Register"]
    pub SM0INIT: crate::RWRegister<u16>,
    #[doc = "Control 2 Register"]
    pub SM0CTRL2: crate::RWRegister<u16>,
    #[doc = "Control Register"]
    pub SM0CTRL: crate::RWRegister<u16>,
    _reserved0: [u8; 0x02],
    #[doc = "Value Register 0"]
    pub SM0VAL0: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 1"]
    pub SM0FRACVAL1: crate::RWRegister<u16>,
    #[doc = "Value Register 1"]
    pub SM0VAL1: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 2"]
    pub SM0FRACVAL2: crate::RWRegister<u16>,
    #[doc = "Value Register 2"]
    pub SM0VAL2: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 3"]
    pub SM0FRACVAL3: crate::RWRegister<u16>,
    #[doc = "Value Register 3"]
    pub SM0VAL3: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 4"]
    pub SM0FRACVAL4: crate::RWRegister<u16>,
    #[doc = "Value Register 4"]
    pub SM0VAL4: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 5"]
    pub SM0FRACVAL5: crate::RWRegister<u16>,
    #[doc = "Value Register 5"]
    pub SM0VAL5: crate::RWRegister<u16>,
    #[doc = "Fractional Control Register"]
    pub SM0FRCTRL: crate::RWRegister<u16>,
    #[doc = "Output Control Register"]
    pub SM0OCTRL: crate::RWRegister<u16>,
    #[doc = "Status Register"]
    pub SM0STS: crate::RWRegister<u16>,
    #[doc = "Interrupt Enable Register"]
    pub SM0INTEN: crate::RWRegister<u16>,
    #[doc = "DMA Enable Register"]
    pub SM0DMAEN: crate::RWRegister<u16>,
    #[doc = "Output Trigger Control Register"]
    pub SM0TCTRL: crate::RWRegister<u16>,
    #[doc = "Fault Disable Mapping Register 0"]
    pub SM0DISMAP0: crate::RWRegister<u16>,
    _reserved1: [u8; 0x02],
    #[doc = "Deadtime Count Register 0"]
    pub SM0DTCNT0: crate::RWRegister<u16>,
    #[doc = "Deadtime Count Register 1"]
    pub SM0DTCNT1: crate::RWRegister<u16>,
    #[doc = "Capture Control A Register"]
    pub SM0CAPTCTRLA: crate::RWRegister<u16>,
    #[doc = "Capture Compare A Register"]
    pub SM0CAPTCOMPA: crate::RWRegister<u16>,
    #[doc = "Capture Control B Register"]
    pub SM0CAPTCTRLB: crate::RWRegister<u16>,
    #[doc = "Capture Compare B Register"]
    pub SM0CAPTCOMPB: crate::RWRegister<u16>,
    #[doc = "Capture Control X Register"]
    pub SM0CAPTCTRLX: crate::RWRegister<u16>,
    #[doc = "Capture Compare X Register"]
    pub SM0CAPTCOMPX: crate::RWRegister<u16>,
    #[doc = "Capture Value 0 Register"]
    pub SM0CVAL0: crate::RORegister<u16>,
    #[doc = "Capture Value 0 Cycle Register"]
    pub SM0CVAL0CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 1 Register"]
    pub SM0CVAL1: crate::RORegister<u16>,
    #[doc = "Capture Value 1 Cycle Register"]
    pub SM0CVAL1CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 2 Register"]
    pub SM0CVAL2: crate::RORegister<u16>,
    #[doc = "Capture Value 2 Cycle Register"]
    pub SM0CVAL2CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 3 Register"]
    pub SM0CVAL3: crate::RORegister<u16>,
    #[doc = "Capture Value 3 Cycle Register"]
    pub SM0CVAL3CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 4 Register"]
    pub SM0CVAL4: crate::RORegister<u16>,
    #[doc = "Capture Value 4 Cycle Register"]
    pub SM0CVAL4CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 5 Register"]
    pub SM0CVAL5: crate::RORegister<u16>,
    #[doc = "Capture Value 5 Cycle Register"]
    pub SM0CVAL5CYC: crate::RORegister<u16>,
    _reserved2: [u8; 0x02],
    #[doc = "Capture PWM_A Input Filter Register"]
    pub SM0CAPTFILTA: crate::RWRegister<u16>,
    #[doc = "Capture PWM_B Input Filter Register"]
    pub SM0CAPTFILTB: crate::RWRegister<u16>,
    #[doc = "Capture PWM_X Input Filter Register"]
    pub SM0CAPTFILTX: crate::RWRegister<u16>,
    #[doc = "Counter Register"]
    pub SM1CNT: crate::RORegister<u16>,
    #[doc = "Initial Count Register"]
    pub SM1INIT: crate::RWRegister<u16>,
    #[doc = "Control 2 Register"]
    pub SM1CTRL2: crate::RWRegister<u16>,
    #[doc = "Control Register"]
    pub SM1CTRL: crate::RWRegister<u16>,
    _reserved3: [u8; 0x02],
    #[doc = "Value Register 0"]
    pub SM1VAL0: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 1"]
    pub SM1FRACVAL1: crate::RWRegister<u16>,
    #[doc = "Value Register 1"]
    pub SM1VAL1: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 2"]
    pub SM1FRACVAL2: crate::RWRegister<u16>,
    #[doc = "Value Register 2"]
    pub SM1VAL2: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 3"]
    pub SM1FRACVAL3: crate::RWRegister<u16>,
    #[doc = "Value Register 3"]
    pub SM1VAL3: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 4"]
    pub SM1FRACVAL4: crate::RWRegister<u16>,
    #[doc = "Value Register 4"]
    pub SM1VAL4: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 5"]
    pub SM1FRACVAL5: crate::RWRegister<u16>,
    #[doc = "Value Register 5"]
    pub SM1VAL5: crate::RWRegister<u16>,
    #[doc = "Fractional Control Register"]
    pub SM1FRCTRL: crate::RWRegister<u16>,
    #[doc = "Output Control Register"]
    pub SM1OCTRL: crate::RWRegister<u16>,
    #[doc = "Status Register"]
    pub SM1STS: crate::RWRegister<u16>,
    #[doc = "Interrupt Enable Register"]
    pub SM1INTEN: crate::RWRegister<u16>,
    #[doc = "DMA Enable Register"]
    pub SM1DMAEN: crate::RWRegister<u16>,
    #[doc = "Output Trigger Control Register"]
    pub SM1TCTRL: crate::RWRegister<u16>,
    #[doc = "Fault Disable Mapping Register 0"]
    pub SM1DISMAP0: crate::RWRegister<u16>,
    _reserved4: [u8; 0x02],
    #[doc = "Deadtime Count Register 0"]
    pub SM1DTCNT0: crate::RWRegister<u16>,
    #[doc = "Deadtime Count Register 1"]
    pub SM1DTCNT1: crate::RWRegister<u16>,
    #[doc = "Capture Control A Register"]
    pub SM1CAPTCTRLA: crate::RWRegister<u16>,
    #[doc = "Capture Compare A Register"]
    pub SM1CAPTCOMPA: crate::RWRegister<u16>,
    #[doc = "Capture Control B Register"]
    pub SM1CAPTCTRLB: crate::RWRegister<u16>,
    #[doc = "Capture Compare B Register"]
    pub SM1CAPTCOMPB: crate::RWRegister<u16>,
    #[doc = "Capture Control X Register"]
    pub SM1CAPTCTRLX: crate::RWRegister<u16>,
    #[doc = "Capture Compare X Register"]
    pub SM1CAPTCOMPX: crate::RWRegister<u16>,
    #[doc = "Capture Value 0 Register"]
    pub SM1CVAL0: crate::RORegister<u16>,
    #[doc = "Capture Value 0 Cycle Register"]
    pub SM1CVAL0CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 1 Register"]
    pub SM1CVAL1: crate::RORegister<u16>,
    #[doc = "Capture Value 1 Cycle Register"]
    pub SM1CVAL1CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 2 Register"]
    pub SM1CVAL2: crate::RORegister<u16>,
    #[doc = "Capture Value 2 Cycle Register"]
    pub SM1CVAL2CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 3 Register"]
    pub SM1CVAL3: crate::RORegister<u16>,
    #[doc = "Capture Value 3 Cycle Register"]
    pub SM1CVAL3CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 4 Register"]
    pub SM1CVAL4: crate::RORegister<u16>,
    #[doc = "Capture Value 4 Cycle Register"]
    pub SM1CVAL4CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 5 Register"]
    pub SM1CVAL5: crate::RORegister<u16>,
    #[doc = "Capture Value 5 Cycle Register"]
    pub SM1CVAL5CYC: crate::RORegister<u16>,
    #[doc = "Phase Delay Register"]
    pub SM1PHASEDLY: crate::RWRegister<u16>,
    #[doc = "Capture PWM_A Input Filter Register"]
    pub SM1CAPTFILTA: crate::RWRegister<u16>,
    #[doc = "Capture PWM_B Input Filter Register"]
    pub SM1CAPTFILTB: crate::RWRegister<u16>,
    #[doc = "Capture PWM_X Input Filter Register"]
    pub SM1CAPTFILTX: crate::RWRegister<u16>,
    #[doc = "Counter Register"]
    pub SM2CNT: crate::RORegister<u16>,
    #[doc = "Initial Count Register"]
    pub SM2INIT: crate::RWRegister<u16>,
    #[doc = "Control 2 Register"]
    pub SM2CTRL2: crate::RWRegister<u16>,
    #[doc = "Control Register"]
    pub SM2CTRL: crate::RWRegister<u16>,
    _reserved5: [u8; 0x02],
    #[doc = "Value Register 0"]
    pub SM2VAL0: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 1"]
    pub SM2FRACVAL1: crate::RWRegister<u16>,
    #[doc = "Value Register 1"]
    pub SM2VAL1: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 2"]
    pub SM2FRACVAL2: crate::RWRegister<u16>,
    #[doc = "Value Register 2"]
    pub SM2VAL2: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 3"]
    pub SM2FRACVAL3: crate::RWRegister<u16>,
    #[doc = "Value Register 3"]
    pub SM2VAL3: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 4"]
    pub SM2FRACVAL4: crate::RWRegister<u16>,
    #[doc = "Value Register 4"]
    pub SM2VAL4: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 5"]
    pub SM2FRACVAL5: crate::RWRegister<u16>,
    #[doc = "Value Register 5"]
    pub SM2VAL5: crate::RWRegister<u16>,
    #[doc = "Fractional Control Register"]
    pub SM2FRCTRL: crate::RWRegister<u16>,
    #[doc = "Output Control Register"]
    pub SM2OCTRL: crate::RWRegister<u16>,
    #[doc = "Status Register"]
    pub SM2STS: crate::RWRegister<u16>,
    #[doc = "Interrupt Enable Register"]
    pub SM2INTEN: crate::RWRegister<u16>,
    #[doc = "DMA Enable Register"]
    pub SM2DMAEN: crate::RWRegister<u16>,
    #[doc = "Output Trigger Control Register"]
    pub SM2TCTRL: crate::RWRegister<u16>,
    #[doc = "Fault Disable Mapping Register 0"]
    pub SM2DISMAP0: crate::RWRegister<u16>,
    _reserved6: [u8; 0x02],
    #[doc = "Deadtime Count Register 0"]
    pub SM2DTCNT0: crate::RWRegister<u16>,
    #[doc = "Deadtime Count Register 1"]
    pub SM2DTCNT1: crate::RWRegister<u16>,
    #[doc = "Capture Control A Register"]
    pub SM2CAPTCTRLA: crate::RWRegister<u16>,
    #[doc = "Capture Compare A Register"]
    pub SM2CAPTCOMPA: crate::RWRegister<u16>,
    #[doc = "Capture Control B Register"]
    pub SM2CAPTCTRLB: crate::RWRegister<u16>,
    #[doc = "Capture Compare B Register"]
    pub SM2CAPTCOMPB: crate::RWRegister<u16>,
    #[doc = "Capture Control X Register"]
    pub SM2CAPTCTRLX: crate::RWRegister<u16>,
    #[doc = "Capture Compare X Register"]
    pub SM2CAPTCOMPX: crate::RWRegister<u16>,
    #[doc = "Capture Value 0 Register"]
    pub SM2CVAL0: crate::RORegister<u16>,
    #[doc = "Capture Value 0 Cycle Register"]
    pub SM2CVAL0CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 1 Register"]
    pub SM2CVAL1: crate::RORegister<u16>,
    #[doc = "Capture Value 1 Cycle Register"]
    pub SM2CVAL1CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 2 Register"]
    pub SM2CVAL2: crate::RORegister<u16>,
    #[doc = "Capture Value 2 Cycle Register"]
    pub SM2CVAL2CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 3 Register"]
    pub SM2CVAL3: crate::RORegister<u16>,
    #[doc = "Capture Value 3 Cycle Register"]
    pub SM2CVAL3CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 4 Register"]
    pub SM2CVAL4: crate::RORegister<u16>,
    #[doc = "Capture Value 4 Cycle Register"]
    pub SM2CVAL4CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 5 Register"]
    pub SM2CVAL5: crate::RORegister<u16>,
    #[doc = "Capture Value 5 Cycle Register"]
    pub SM2CVAL5CYC: crate::RORegister<u16>,
    #[doc = "Phase Delay Register"]
    pub SM2PHASEDLY: crate::RWRegister<u16>,
    #[doc = "Capture PWM_A Input Filter Register"]
    pub SM2CAPTFILTA: crate::RWRegister<u16>,
    #[doc = "Capture PWM_B Input Filter Register"]
    pub SM2CAPTFILTB: crate::RWRegister<u16>,
    #[doc = "Capture PWM_X Input Filter Register"]
    pub SM2CAPTFILTX: crate::RWRegister<u16>,
    #[doc = "Counter Register"]
    pub SM3CNT: crate::RORegister<u16>,
    #[doc = "Initial Count Register"]
    pub SM3INIT: crate::RWRegister<u16>,
    #[doc = "Control 2 Register"]
    pub SM3CTRL2: crate::RWRegister<u16>,
    #[doc = "Control Register"]
    pub SM3CTRL: crate::RWRegister<u16>,
    _reserved7: [u8; 0x02],
    #[doc = "Value Register 0"]
    pub SM3VAL0: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 1"]
    pub SM3FRACVAL1: crate::RWRegister<u16>,
    #[doc = "Value Register 1"]
    pub SM3VAL1: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 2"]
    pub SM3FRACVAL2: crate::RWRegister<u16>,
    #[doc = "Value Register 2"]
    pub SM3VAL2: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 3"]
    pub SM3FRACVAL3: crate::RWRegister<u16>,
    #[doc = "Value Register 3"]
    pub SM3VAL3: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 4"]
    pub SM3FRACVAL4: crate::RWRegister<u16>,
    #[doc = "Value Register 4"]
    pub SM3VAL4: crate::RWRegister<u16>,
    #[doc = "Fractional Value Register 5"]
    pub SM3FRACVAL5: crate::RWRegister<u16>,
    #[doc = "Value Register 5"]
    pub SM3VAL5: crate::RWRegister<u16>,
    #[doc = "Fractional Control Register"]
    pub SM3FRCTRL: crate::RWRegister<u16>,
    #[doc = "Output Control Register"]
    pub SM3OCTRL: crate::RWRegister<u16>,
    #[doc = "Status Register"]
    pub SM3STS: crate::RWRegister<u16>,
    #[doc = "Interrupt Enable Register"]
    pub SM3INTEN: crate::RWRegister<u16>,
    #[doc = "DMA Enable Register"]
    pub SM3DMAEN: crate::RWRegister<u16>,
    #[doc = "Output Trigger Control Register"]
    pub SM3TCTRL: crate::RWRegister<u16>,
    #[doc = "Fault Disable Mapping Register 0"]
    pub SM3DISMAP0: crate::RWRegister<u16>,
    _reserved8: [u8; 0x02],
    #[doc = "Deadtime Count Register 0"]
    pub SM3DTCNT0: crate::RWRegister<u16>,
    #[doc = "Deadtime Count Register 1"]
    pub SM3DTCNT1: crate::RWRegister<u16>,
    #[doc = "Capture Control A Register"]
    pub SM3CAPTCTRLA: crate::RWRegister<u16>,
    #[doc = "Capture Compare A Register"]
    pub SM3CAPTCOMPA: crate::RWRegister<u16>,
    #[doc = "Capture Control B Register"]
    pub SM3CAPTCTRLB: crate::RWRegister<u16>,
    #[doc = "Capture Compare B Register"]
    pub SM3CAPTCOMPB: crate::RWRegister<u16>,
    #[doc = "Capture Control X Register"]
    pub SM3CAPTCTRLX: crate::RWRegister<u16>,
    #[doc = "Capture Compare X Register"]
    pub SM3CAPTCOMPX: crate::RWRegister<u16>,
    #[doc = "Capture Value 0 Register"]
    pub SM3CVAL0: crate::RORegister<u16>,
    #[doc = "Capture Value 0 Cycle Register"]
    pub SM3CVAL0CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 1 Register"]
    pub SM3CVAL1: crate::RORegister<u16>,
    #[doc = "Capture Value 1 Cycle Register"]
    pub SM3CVAL1CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 2 Register"]
    pub SM3CVAL2: crate::RORegister<u16>,
    #[doc = "Capture Value 2 Cycle Register"]
    pub SM3CVAL2CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 3 Register"]
    pub SM3CVAL3: crate::RORegister<u16>,
    #[doc = "Capture Value 3 Cycle Register"]
    pub SM3CVAL3CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 4 Register"]
    pub SM3CVAL4: crate::RORegister<u16>,
    #[doc = "Capture Value 4 Cycle Register"]
    pub SM3CVAL4CYC: crate::RORegister<u16>,
    #[doc = "Capture Value 5 Register"]
    pub SM3CVAL5: crate::RORegister<u16>,
    #[doc = "Capture Value 5 Cycle Register"]
    pub SM3CVAL5CYC: crate::RORegister<u16>,
    #[doc = "Phase Delay Register"]
    pub SM3PHASEDLY: crate::RWRegister<u16>,
    #[doc = "Capture PWM_A Input Filter Register"]
    pub SM3CAPTFILTA: crate::RWRegister<u16>,
    #[doc = "Capture PWM_B Input Filter Register"]
    pub SM3CAPTFILTB: crate::RWRegister<u16>,
    #[doc = "Capture PWM_X Input Filter Register"]
    pub SM3CAPTFILTX: crate::RWRegister<u16>,
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
#[doc = "Counter Register"]
pub mod SM0CNT {
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
pub mod SM0INIT {
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
pub mod SM0CTRL2 {
    #[doc = "Clock Source Select"]
    pub mod CLK_SEL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
            pub const IPBUS: u16 = 0;
            #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
            pub const EXT_CLK: u16 = 0x01;
            #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
            pub const AUX_CLK: u16 = 0x02;
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
            pub const LOCAL: u16 = 0;
            #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
            pub const MASTER: u16 = 0x01;
        }
    }
    #[doc = "Force Select"]
    pub mod FORCE_SEL {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
            pub const LOCAL: u16 = 0;
            #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER: u16 = 0x01;
            #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
            pub const LOCAL_RELOAD: u16 = 0x02;
            #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER_RELOAD: u16 = 0x03;
            #[doc = "The local sync signal from this submodule is used to force updates."]
            pub const LOCAL_SYNC: u16 = 0x04;
            #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER_SYNC: u16 = 0x05;
            #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
            pub const EXT_FORCE: u16 = 0x06;
            #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
            pub const EXT_SYNC: u16 = 0x07;
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
    #[doc = "Force Enable"]
    pub mod FRCEN {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initialization from a FORCE_OUT is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Initialization from a FORCE_OUT is enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const PWM_X: u16 = 0;
            #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
            pub const MASTER_RELOAD: u16 = 0x01;
            #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
            pub const MASTER_SYNC: u16 = 0x02;
            #[doc = "EXT_SYNC causes initialization."]
            pub const EXT_SYNC: u16 = 0x03;
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
            pub const COMPLEMENTARY: u16 = 0;
            #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
            pub const INDEPENDENT: u16 = 0x01;
        }
    }
    #[doc = "Wait Enable"]
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
pub mod SM0CTRL {
    #[doc = "Double Switching Enable"]
    pub mod DBLEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double switching disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Double switching enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "PWM_X Double Switching Enable"]
    pub mod DBLX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_X double pulse disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "PWM_X double pulse enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const NEXT_PWM_RELOAD: u16 = 0;
            #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
            pub const MTCTRL_LDOK_SET: u16 = 0x01;
        }
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    pub mod SPLIT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DBLPWM is not split. PWM_A and PWM_B each have double pulses."]
            pub const DISABLED: u16 = 0;
            #[doc = "DBLPWM is split to PWM_A and PWM_B."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "Prescaler"]
    pub mod PRSC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Prescaler 1"]
            pub const ONE: u16 = 0;
            #[doc = "Prescaler 2"]
            pub const TWO: u16 = 0x01;
            #[doc = "Prescaler 4"]
            pub const FOUR: u16 = 0x02;
            #[doc = "Prescaler 8"]
            pub const EIGHT: u16 = 0x03;
            #[doc = "Prescaler 16"]
            pub const SIXTEEN: u16 = 0x04;
            #[doc = "Prescaler 32"]
            pub const THIRTYTWO: u16 = 0x05;
            #[doc = "Prescaler 64"]
            pub const SIXTYFOUR: u16 = 0x06;
            #[doc = "Prescaler 128"]
            pub const HUNDREDTWENTYEIGHT: u16 = 0x07;
        }
    }
    #[doc = "Compare Mode"]
    pub mod COMPMODE {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
            pub const EQUAL_TO: u16 = 0;
            #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
            pub const EQUAL_TO_OR_GREATER_THAN: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Full-cycle reloads enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Half-cycle reloads enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const EVERYPWM: u16 = 0;
            #[doc = "Every 2 PWM opportunities"]
            pub const EVERY2PWM: u16 = 0x01;
            #[doc = "Every 3 PWM opportunities"]
            pub const EVERY3PWM: u16 = 0x02;
            #[doc = "Every 4 PWM opportunities"]
            pub const EVERY4PWM: u16 = 0x03;
            #[doc = "Every 5 PWM opportunities"]
            pub const EVERY5PWM: u16 = 0x04;
            #[doc = "Every 6 PWM opportunities"]
            pub const EVERY6PWM: u16 = 0x05;
            #[doc = "Every 7 PWM opportunities"]
            pub const EVERY7PWM: u16 = 0x06;
            #[doc = "Every 8 PWM opportunities"]
            pub const EVERY8PWM: u16 = 0x07;
            #[doc = "Every 9 PWM opportunities"]
            pub const EVERY9PWM: u16 = 0x08;
            #[doc = "Every 10 PWM opportunities"]
            pub const EVERY10PWM: u16 = 0x09;
            #[doc = "Every 11 PWM opportunities"]
            pub const EVERY11PWM: u16 = 0x0a;
            #[doc = "Every 12 PWM opportunities"]
            pub const EVERY12PWM: u16 = 0x0b;
            #[doc = "Every 13 PWM opportunities"]
            pub const EVERY13PWM: u16 = 0x0c;
            #[doc = "Every 14 PWM opportunities"]
            pub const EVERY14PWM: u16 = 0x0d;
            #[doc = "Every 15 PWM opportunities"]
            pub const EVERY15PWM: u16 = 0x0e;
            #[doc = "Every 16 PWM opportunities"]
            pub const EVERY16PWM: u16 = 0x0f;
        }
    }
}
#[doc = "Value Register 0"]
pub mod SM0VAL0 {
    #[doc = "Value 0"]
    pub mod VAL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 1"]
pub mod SM0FRACVAL1 {
    #[doc = "Fractional Value 1"]
    pub mod FRACVAL1 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value Register 1"]
pub mod SM0VAL1 {
    #[doc = "Value 1"]
    pub mod VAL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 2"]
pub mod SM0FRACVAL2 {
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
pub mod SM0VAL2 {
    #[doc = "Value 2"]
    pub mod VAL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 3"]
pub mod SM0FRACVAL3 {
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
pub mod SM0VAL3 {
    #[doc = "Value 3"]
    pub mod VAL3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 4"]
pub mod SM0FRACVAL4 {
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
pub mod SM0VAL4 {
    #[doc = "Value 4"]
    pub mod VAL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 5"]
pub mod SM0FRACVAL5 {
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
pub mod SM0VAL5 {
    #[doc = "Value 5"]
    pub mod VAL5 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Control Register"]
pub mod SM0FRCTRL {
    #[doc = "Fractional Cycle PWM Period Enable"]
    pub mod FRAC1_EN {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable fractional cycle length for the PWM period."]
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle length for the PWM period."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle placement for PWM_A."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle placement for PWM_B."]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM0OCTRL {
    #[doc = "PWM_X Fault State"]
    pub mod PWMXFS {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
pub mod SM0STS {
    #[doc = "Compare Flags"]
    pub mod CMPF {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No compare event has occurred for a particular VALx value."]
            pub const NO_EVENT: u16 = 0;
            #[doc = "A compare event has occurred for a particular VALx value."]
            pub const EVENT: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
            pub const FLAG: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
            pub const FLAG: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "At least one of the double buffered registers has been updated since the last reload."]
            pub const FLAG: u16 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable Register"]
pub mod SM0INTEN {
    #[doc = "Compare Interrupt Enables"]
    pub mod CMPIE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
            pub const DISABLED: u16 = 0;
            #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    pub mod CA1IE {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt request disabled for STS\\[CFA1\\]"]
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFA1\\]"]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
            pub const ENABLED: u16 = 0x01;
        }
    }
}
#[doc = "DMA Enable Register"]
pub mod SM0DMAEN {
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
            pub const DISABLED: u16 = 0;
            #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
            pub const EXCEEDFIFO: u16 = 0x01;
            #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
            pub const LOCAL_SYNC: u16 = 0x02;
            #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
            pub const LOCAL_RELOAD: u16 = 0x03;
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
            pub const OR: u16 = 0;
            #[doc = "Selected FIFO watermarks are AND'ed together."]
            pub const AND: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u16 = 0x01;
        }
    }
}
#[doc = "Output Trigger Control Register"]
pub mod SM0TCTRL {
    #[doc = "Output Trigger Enables"]
    pub mod OUT_TRIG_EN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
            pub const VAL0: u16 = 0x01;
        }
    }
    #[doc = "Trigger Frequency"]
    pub mod TRGFRQ {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
            pub const EVERYPWM: u16 = 0;
            #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
            pub const FINALPWM: u16 = 0x01;
        }
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    pub mod PWBOT1 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
            pub const PWM_OUT_TRIG1_SIGNAL: u16 = 0;
            #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
            pub const PWMB_OUTPUT: u16 = 0x01;
        }
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    pub mod PWAOT0 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
            pub const PWM_OUT_TRIG0_SIGNAL: u16 = 0;
            #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
            pub const PWMA_OUTPUT: u16 = 0x01;
        }
    }
}
#[doc = "Fault Disable Mapping Register 0"]
pub mod SM0DISMAP0 {
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
#[doc = "Deadtime Count Register 0"]
pub mod SM0DTCNT0 {
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
pub mod SM0DTCNT1 {
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
pub mod SM0CAPTCTRLA {
    #[doc = "Arm A"]
    pub mod ARMA {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode A"]
    pub mod ONESHOTA {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_A: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM0CAPTCOMPA {
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
pub mod SM0CAPTCTRLB {
    #[doc = "Arm B"]
    pub mod ARMB {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode B"]
    pub mod ONESHOTB {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_B: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM0CAPTCOMPB {
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
pub mod SM0CAPTCTRLX {
    #[doc = "Arm X"]
    pub mod ARMX {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode Aux"]
    pub mod ONESHOTX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_X: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM0CAPTCOMPX {
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
pub mod SM0CVAL0 {
    #[doc = "Capture Value 0"]
    pub mod CAPTVAL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 0 Cycle Register"]
pub mod SM0CVAL0CYC {
    #[doc = "Capture Value 0 Cycle"]
    pub mod CVAL0CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 1 Register"]
pub mod SM0CVAL1 {
    #[doc = "Capture Value 1"]
    pub mod CAPTVAL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 1 Cycle Register"]
pub mod SM0CVAL1CYC {
    #[doc = "Capture Value 1 Cycle"]
    pub mod CVAL1CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 2 Register"]
pub mod SM0CVAL2 {
    #[doc = "Capture Value 2"]
    pub mod CAPTVAL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 2 Cycle Register"]
pub mod SM0CVAL2CYC {
    #[doc = "Capture Value 2 Cycle"]
    pub mod CVAL2CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 3 Register"]
pub mod SM0CVAL3 {
    #[doc = "Capture Value 3"]
    pub mod CAPTVAL3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 3 Cycle Register"]
pub mod SM0CVAL3CYC {
    #[doc = "Capture Value 3 Cycle"]
    pub mod CVAL3CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 4 Register"]
pub mod SM0CVAL4 {
    #[doc = "Capture Value 4"]
    pub mod CAPTVAL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 4 Cycle Register"]
pub mod SM0CVAL4CYC {
    #[doc = "Capture Value 4 Cycle"]
    pub mod CVAL4CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 5 Register"]
pub mod SM0CVAL5 {
    #[doc = "Capture Value 5"]
    pub mod CAPTVAL5 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 5 Cycle Register"]
pub mod SM0CVAL5CYC {
    #[doc = "Capture Value 5 Cycle"]
    pub mod CVAL5CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
pub mod SM0CAPTFILTA {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTA_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTA_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
pub mod SM0CAPTFILTB {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTB_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTB_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
pub mod SM0CAPTFILTX {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTX_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTX_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter Register"]
pub mod SM1CNT {
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
pub mod SM1INIT {
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
pub mod SM1CTRL2 {
    #[doc = "Clock Source Select"]
    pub mod CLK_SEL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
            pub const IPBUS: u16 = 0;
            #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
            pub const EXT_CLK: u16 = 0x01;
            #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
            pub const AUX_CLK: u16 = 0x02;
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
            pub const LOCAL: u16 = 0;
            #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
            pub const MASTER: u16 = 0x01;
        }
    }
    #[doc = "Force Select"]
    pub mod FORCE_SEL {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
            pub const LOCAL: u16 = 0;
            #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER: u16 = 0x01;
            #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
            pub const LOCAL_RELOAD: u16 = 0x02;
            #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER_RELOAD: u16 = 0x03;
            #[doc = "The local sync signal from this submodule is used to force updates."]
            pub const LOCAL_SYNC: u16 = 0x04;
            #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER_SYNC: u16 = 0x05;
            #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
            pub const EXT_FORCE: u16 = 0x06;
            #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
            pub const EXT_SYNC: u16 = 0x07;
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
    #[doc = "Force Enable"]
    pub mod FRCEN {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initialization from a FORCE_OUT is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Initialization from a FORCE_OUT is enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const PWM_X: u16 = 0;
            #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
            pub const MASTER_RELOAD: u16 = 0x01;
            #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
            pub const MASTER_SYNC: u16 = 0x02;
            #[doc = "EXT_SYNC causes initialization."]
            pub const EXT_SYNC: u16 = 0x03;
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
            pub const COMPLEMENTARY: u16 = 0;
            #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
            pub const INDEPENDENT: u16 = 0x01;
        }
    }
    #[doc = "Wait Enable"]
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
pub mod SM1CTRL {
    #[doc = "Double Switching Enable"]
    pub mod DBLEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double switching disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Double switching enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "PWM_X Double Switching Enable"]
    pub mod DBLX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_X double pulse disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "PWM_X double pulse enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const NEXT_PWM_RELOAD: u16 = 0;
            #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
            pub const MTCTRL_LDOK_SET: u16 = 0x01;
        }
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    pub mod SPLIT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DBLPWM is not split. PWM_A and PWM_B each have double pulses."]
            pub const DISABLED: u16 = 0;
            #[doc = "DBLPWM is split to PWM_A and PWM_B."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "Prescaler"]
    pub mod PRSC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Prescaler 1"]
            pub const ONE: u16 = 0;
            #[doc = "Prescaler 2"]
            pub const TWO: u16 = 0x01;
            #[doc = "Prescaler 4"]
            pub const FOUR: u16 = 0x02;
            #[doc = "Prescaler 8"]
            pub const EIGHT: u16 = 0x03;
            #[doc = "Prescaler 16"]
            pub const SIXTEEN: u16 = 0x04;
            #[doc = "Prescaler 32"]
            pub const THIRTYTWO: u16 = 0x05;
            #[doc = "Prescaler 64"]
            pub const SIXTYFOUR: u16 = 0x06;
            #[doc = "Prescaler 128"]
            pub const HUNDREDTWENTYEIGHT: u16 = 0x07;
        }
    }
    #[doc = "Compare Mode"]
    pub mod COMPMODE {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
            pub const EQUAL_TO: u16 = 0;
            #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
            pub const EQUAL_TO_OR_GREATER_THAN: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Full-cycle reloads enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Half-cycle reloads enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const EVERYPWM: u16 = 0;
            #[doc = "Every 2 PWM opportunities"]
            pub const EVERY2PWM: u16 = 0x01;
            #[doc = "Every 3 PWM opportunities"]
            pub const EVERY3PWM: u16 = 0x02;
            #[doc = "Every 4 PWM opportunities"]
            pub const EVERY4PWM: u16 = 0x03;
            #[doc = "Every 5 PWM opportunities"]
            pub const EVERY5PWM: u16 = 0x04;
            #[doc = "Every 6 PWM opportunities"]
            pub const EVERY6PWM: u16 = 0x05;
            #[doc = "Every 7 PWM opportunities"]
            pub const EVERY7PWM: u16 = 0x06;
            #[doc = "Every 8 PWM opportunities"]
            pub const EVERY8PWM: u16 = 0x07;
            #[doc = "Every 9 PWM opportunities"]
            pub const EVERY9PWM: u16 = 0x08;
            #[doc = "Every 10 PWM opportunities"]
            pub const EVERY10PWM: u16 = 0x09;
            #[doc = "Every 11 PWM opportunities"]
            pub const EVERY11PWM: u16 = 0x0a;
            #[doc = "Every 12 PWM opportunities"]
            pub const EVERY12PWM: u16 = 0x0b;
            #[doc = "Every 13 PWM opportunities"]
            pub const EVERY13PWM: u16 = 0x0c;
            #[doc = "Every 14 PWM opportunities"]
            pub const EVERY14PWM: u16 = 0x0d;
            #[doc = "Every 15 PWM opportunities"]
            pub const EVERY15PWM: u16 = 0x0e;
            #[doc = "Every 16 PWM opportunities"]
            pub const EVERY16PWM: u16 = 0x0f;
        }
    }
}
#[doc = "Value Register 0"]
pub mod SM1VAL0 {
    #[doc = "Value 0"]
    pub mod VAL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 1"]
pub mod SM1FRACVAL1 {
    #[doc = "Fractional Value 1"]
    pub mod FRACVAL1 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value Register 1"]
pub mod SM1VAL1 {
    #[doc = "Value 1"]
    pub mod VAL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 2"]
pub mod SM1FRACVAL2 {
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
pub mod SM1VAL2 {
    #[doc = "Value 2"]
    pub mod VAL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 3"]
pub mod SM1FRACVAL3 {
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
pub mod SM1VAL3 {
    #[doc = "Value 3"]
    pub mod VAL3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 4"]
pub mod SM1FRACVAL4 {
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
pub mod SM1VAL4 {
    #[doc = "Value 4"]
    pub mod VAL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 5"]
pub mod SM1FRACVAL5 {
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
pub mod SM1VAL5 {
    #[doc = "Value 5"]
    pub mod VAL5 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Control Register"]
pub mod SM1FRCTRL {
    #[doc = "Fractional Cycle PWM Period Enable"]
    pub mod FRAC1_EN {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable fractional cycle length for the PWM period."]
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle length for the PWM period."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle placement for PWM_A."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle placement for PWM_B."]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM1OCTRL {
    #[doc = "PWM_X Fault State"]
    pub mod PWMXFS {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
pub mod SM1STS {
    #[doc = "Compare Flags"]
    pub mod CMPF {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No compare event has occurred for a particular VALx value."]
            pub const NO_EVENT: u16 = 0;
            #[doc = "A compare event has occurred for a particular VALx value."]
            pub const EVENT: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
            pub const FLAG: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
            pub const FLAG: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "At least one of the double buffered registers has been updated since the last reload."]
            pub const FLAG: u16 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable Register"]
pub mod SM1INTEN {
    #[doc = "Compare Interrupt Enables"]
    pub mod CMPIE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
            pub const DISABLED: u16 = 0;
            #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    pub mod CA1IE {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt request disabled for STS\\[CFA1\\]"]
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFA1\\]"]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
            pub const ENABLED: u16 = 0x01;
        }
    }
}
#[doc = "DMA Enable Register"]
pub mod SM1DMAEN {
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
            pub const DISABLED: u16 = 0;
            #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
            pub const EXCEEDFIFO: u16 = 0x01;
            #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
            pub const LOCAL_SYNC: u16 = 0x02;
            #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
            pub const LOCAL_RELOAD: u16 = 0x03;
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
            pub const OR: u16 = 0;
            #[doc = "Selected FIFO watermarks are AND'ed together."]
            pub const AND: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u16 = 0x01;
        }
    }
}
#[doc = "Output Trigger Control Register"]
pub mod SM1TCTRL {
    #[doc = "Output Trigger Enables"]
    pub mod OUT_TRIG_EN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
            pub const VAL0: u16 = 0x01;
        }
    }
    #[doc = "Trigger Frequency"]
    pub mod TRGFRQ {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
            pub const EVERYPWM: u16 = 0;
            #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
            pub const FINALPWM: u16 = 0x01;
        }
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    pub mod PWBOT1 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
            pub const PWM_OUT_TRIG1_SIGNAL: u16 = 0;
            #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
            pub const PWMB_OUTPUT: u16 = 0x01;
        }
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    pub mod PWAOT0 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
            pub const PWM_OUT_TRIG0_SIGNAL: u16 = 0;
            #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
            pub const PWMA_OUTPUT: u16 = 0x01;
        }
    }
}
#[doc = "Fault Disable Mapping Register 0"]
pub mod SM1DISMAP0 {
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
#[doc = "Deadtime Count Register 0"]
pub mod SM1DTCNT0 {
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
pub mod SM1DTCNT1 {
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
pub mod SM1CAPTCTRLA {
    #[doc = "Arm A"]
    pub mod ARMA {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode A"]
    pub mod ONESHOTA {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_A: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM1CAPTCOMPA {
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
pub mod SM1CAPTCTRLB {
    #[doc = "Arm B"]
    pub mod ARMB {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode B"]
    pub mod ONESHOTB {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_B: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM1CAPTCOMPB {
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
pub mod SM1CAPTCTRLX {
    #[doc = "Arm X"]
    pub mod ARMX {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode Aux"]
    pub mod ONESHOTX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_X: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM1CAPTCOMPX {
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
pub mod SM1CVAL0 {
    #[doc = "Capture Value 0"]
    pub mod CAPTVAL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 0 Cycle Register"]
pub mod SM1CVAL0CYC {
    #[doc = "Capture Value 0 Cycle"]
    pub mod CVAL0CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 1 Register"]
pub mod SM1CVAL1 {
    #[doc = "Capture Value 1"]
    pub mod CAPTVAL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 1 Cycle Register"]
pub mod SM1CVAL1CYC {
    #[doc = "Capture Value 1 Cycle"]
    pub mod CVAL1CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 2 Register"]
pub mod SM1CVAL2 {
    #[doc = "Capture Value 2"]
    pub mod CAPTVAL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 2 Cycle Register"]
pub mod SM1CVAL2CYC {
    #[doc = "Capture Value 2 Cycle"]
    pub mod CVAL2CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 3 Register"]
pub mod SM1CVAL3 {
    #[doc = "Capture Value 3"]
    pub mod CAPTVAL3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 3 Cycle Register"]
pub mod SM1CVAL3CYC {
    #[doc = "Capture Value 3 Cycle"]
    pub mod CVAL3CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 4 Register"]
pub mod SM1CVAL4 {
    #[doc = "Capture Value 4"]
    pub mod CAPTVAL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 4 Cycle Register"]
pub mod SM1CVAL4CYC {
    #[doc = "Capture Value 4 Cycle"]
    pub mod CVAL4CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 5 Register"]
pub mod SM1CVAL5 {
    #[doc = "Capture Value 5"]
    pub mod CAPTVAL5 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 5 Cycle Register"]
pub mod SM1CVAL5CYC {
    #[doc = "Capture Value 5 Cycle"]
    pub mod CVAL5CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase Delay Register"]
pub mod SM1PHASEDLY {
    #[doc = "Initial Count Register Bits"]
    pub mod PHASEDLY {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
pub mod SM1CAPTFILTA {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTA_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTA_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
pub mod SM1CAPTFILTB {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTB_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTB_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
pub mod SM1CAPTFILTX {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTX_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTX_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter Register"]
pub mod SM2CNT {
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
pub mod SM2INIT {
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
pub mod SM2CTRL2 {
    #[doc = "Clock Source Select"]
    pub mod CLK_SEL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
            pub const IPBUS: u16 = 0;
            #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
            pub const EXT_CLK: u16 = 0x01;
            #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
            pub const AUX_CLK: u16 = 0x02;
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
            pub const LOCAL: u16 = 0;
            #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
            pub const MASTER: u16 = 0x01;
        }
    }
    #[doc = "Force Select"]
    pub mod FORCE_SEL {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
            pub const LOCAL: u16 = 0;
            #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER: u16 = 0x01;
            #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
            pub const LOCAL_RELOAD: u16 = 0x02;
            #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER_RELOAD: u16 = 0x03;
            #[doc = "The local sync signal from this submodule is used to force updates."]
            pub const LOCAL_SYNC: u16 = 0x04;
            #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER_SYNC: u16 = 0x05;
            #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
            pub const EXT_FORCE: u16 = 0x06;
            #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
            pub const EXT_SYNC: u16 = 0x07;
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
    #[doc = "Force Enable"]
    pub mod FRCEN {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initialization from a FORCE_OUT is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Initialization from a FORCE_OUT is enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const PWM_X: u16 = 0;
            #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
            pub const MASTER_RELOAD: u16 = 0x01;
            #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
            pub const MASTER_SYNC: u16 = 0x02;
            #[doc = "EXT_SYNC causes initialization."]
            pub const EXT_SYNC: u16 = 0x03;
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
            pub const COMPLEMENTARY: u16 = 0;
            #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
            pub const INDEPENDENT: u16 = 0x01;
        }
    }
    #[doc = "Wait Enable"]
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
pub mod SM2CTRL {
    #[doc = "Double Switching Enable"]
    pub mod DBLEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double switching disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Double switching enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "PWM_X Double Switching Enable"]
    pub mod DBLX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_X double pulse disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "PWM_X double pulse enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const NEXT_PWM_RELOAD: u16 = 0;
            #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
            pub const MTCTRL_LDOK_SET: u16 = 0x01;
        }
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    pub mod SPLIT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DBLPWM is not split. PWM_A and PWM_B each have double pulses."]
            pub const DISABLED: u16 = 0;
            #[doc = "DBLPWM is split to PWM_A and PWM_B."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "Prescaler"]
    pub mod PRSC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Prescaler 1"]
            pub const ONE: u16 = 0;
            #[doc = "Prescaler 2"]
            pub const TWO: u16 = 0x01;
            #[doc = "Prescaler 4"]
            pub const FOUR: u16 = 0x02;
            #[doc = "Prescaler 8"]
            pub const EIGHT: u16 = 0x03;
            #[doc = "Prescaler 16"]
            pub const SIXTEEN: u16 = 0x04;
            #[doc = "Prescaler 32"]
            pub const THIRTYTWO: u16 = 0x05;
            #[doc = "Prescaler 64"]
            pub const SIXTYFOUR: u16 = 0x06;
            #[doc = "Prescaler 128"]
            pub const HUNDREDTWENTYEIGHT: u16 = 0x07;
        }
    }
    #[doc = "Compare Mode"]
    pub mod COMPMODE {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
            pub const EQUAL_TO: u16 = 0;
            #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
            pub const EQUAL_TO_OR_GREATER_THAN: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Full-cycle reloads enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Half-cycle reloads enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const EVERYPWM: u16 = 0;
            #[doc = "Every 2 PWM opportunities"]
            pub const EVERY2PWM: u16 = 0x01;
            #[doc = "Every 3 PWM opportunities"]
            pub const EVERY3PWM: u16 = 0x02;
            #[doc = "Every 4 PWM opportunities"]
            pub const EVERY4PWM: u16 = 0x03;
            #[doc = "Every 5 PWM opportunities"]
            pub const EVERY5PWM: u16 = 0x04;
            #[doc = "Every 6 PWM opportunities"]
            pub const EVERY6PWM: u16 = 0x05;
            #[doc = "Every 7 PWM opportunities"]
            pub const EVERY7PWM: u16 = 0x06;
            #[doc = "Every 8 PWM opportunities"]
            pub const EVERY8PWM: u16 = 0x07;
            #[doc = "Every 9 PWM opportunities"]
            pub const EVERY9PWM: u16 = 0x08;
            #[doc = "Every 10 PWM opportunities"]
            pub const EVERY10PWM: u16 = 0x09;
            #[doc = "Every 11 PWM opportunities"]
            pub const EVERY11PWM: u16 = 0x0a;
            #[doc = "Every 12 PWM opportunities"]
            pub const EVERY12PWM: u16 = 0x0b;
            #[doc = "Every 13 PWM opportunities"]
            pub const EVERY13PWM: u16 = 0x0c;
            #[doc = "Every 14 PWM opportunities"]
            pub const EVERY14PWM: u16 = 0x0d;
            #[doc = "Every 15 PWM opportunities"]
            pub const EVERY15PWM: u16 = 0x0e;
            #[doc = "Every 16 PWM opportunities"]
            pub const EVERY16PWM: u16 = 0x0f;
        }
    }
}
#[doc = "Value Register 0"]
pub mod SM2VAL0 {
    #[doc = "Value 0"]
    pub mod VAL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 1"]
pub mod SM2FRACVAL1 {
    #[doc = "Fractional Value 1"]
    pub mod FRACVAL1 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value Register 1"]
pub mod SM2VAL1 {
    #[doc = "Value 1"]
    pub mod VAL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 2"]
pub mod SM2FRACVAL2 {
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
pub mod SM2VAL2 {
    #[doc = "Value 2"]
    pub mod VAL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 3"]
pub mod SM2FRACVAL3 {
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
pub mod SM2VAL3 {
    #[doc = "Value 3"]
    pub mod VAL3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 4"]
pub mod SM2FRACVAL4 {
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
pub mod SM2VAL4 {
    #[doc = "Value 4"]
    pub mod VAL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 5"]
pub mod SM2FRACVAL5 {
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
pub mod SM2VAL5 {
    #[doc = "Value 5"]
    pub mod VAL5 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Control Register"]
pub mod SM2FRCTRL {
    #[doc = "Fractional Cycle PWM Period Enable"]
    pub mod FRAC1_EN {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable fractional cycle length for the PWM period."]
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle length for the PWM period."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle placement for PWM_A."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle placement for PWM_B."]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM2OCTRL {
    #[doc = "PWM_X Fault State"]
    pub mod PWMXFS {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
pub mod SM2STS {
    #[doc = "Compare Flags"]
    pub mod CMPF {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No compare event has occurred for a particular VALx value."]
            pub const NO_EVENT: u16 = 0;
            #[doc = "A compare event has occurred for a particular VALx value."]
            pub const EVENT: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
            pub const FLAG: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
            pub const FLAG: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "At least one of the double buffered registers has been updated since the last reload."]
            pub const FLAG: u16 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable Register"]
pub mod SM2INTEN {
    #[doc = "Compare Interrupt Enables"]
    pub mod CMPIE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
            pub const DISABLED: u16 = 0;
            #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    pub mod CA1IE {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt request disabled for STS\\[CFA1\\]"]
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFA1\\]"]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
            pub const ENABLED: u16 = 0x01;
        }
    }
}
#[doc = "DMA Enable Register"]
pub mod SM2DMAEN {
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
            pub const DISABLED: u16 = 0;
            #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
            pub const EXCEEDFIFO: u16 = 0x01;
            #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
            pub const LOCAL_SYNC: u16 = 0x02;
            #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
            pub const LOCAL_RELOAD: u16 = 0x03;
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
            pub const OR: u16 = 0;
            #[doc = "Selected FIFO watermarks are AND'ed together."]
            pub const AND: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u16 = 0x01;
        }
    }
}
#[doc = "Output Trigger Control Register"]
pub mod SM2TCTRL {
    #[doc = "Output Trigger Enables"]
    pub mod OUT_TRIG_EN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
            pub const VAL0: u16 = 0x01;
        }
    }
    #[doc = "Trigger Frequency"]
    pub mod TRGFRQ {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
            pub const EVERYPWM: u16 = 0;
            #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
            pub const FINALPWM: u16 = 0x01;
        }
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    pub mod PWBOT1 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
            pub const PWM_OUT_TRIG1_SIGNAL: u16 = 0;
            #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
            pub const PWMB_OUTPUT: u16 = 0x01;
        }
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    pub mod PWAOT0 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
            pub const PWM_OUT_TRIG0_SIGNAL: u16 = 0;
            #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
            pub const PWMA_OUTPUT: u16 = 0x01;
        }
    }
}
#[doc = "Fault Disable Mapping Register 0"]
pub mod SM2DISMAP0 {
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
#[doc = "Deadtime Count Register 0"]
pub mod SM2DTCNT0 {
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
pub mod SM2DTCNT1 {
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
pub mod SM2CAPTCTRLA {
    #[doc = "Arm A"]
    pub mod ARMA {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode A"]
    pub mod ONESHOTA {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_A: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM2CAPTCOMPA {
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
pub mod SM2CAPTCTRLB {
    #[doc = "Arm B"]
    pub mod ARMB {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode B"]
    pub mod ONESHOTB {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_B: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM2CAPTCOMPB {
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
pub mod SM2CAPTCTRLX {
    #[doc = "Arm X"]
    pub mod ARMX {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode Aux"]
    pub mod ONESHOTX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_X: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM2CAPTCOMPX {
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
pub mod SM2CVAL0 {
    #[doc = "Capture Value 0"]
    pub mod CAPTVAL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 0 Cycle Register"]
pub mod SM2CVAL0CYC {
    #[doc = "Capture Value 0 Cycle"]
    pub mod CVAL0CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 1 Register"]
pub mod SM2CVAL1 {
    #[doc = "Capture Value 1"]
    pub mod CAPTVAL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 1 Cycle Register"]
pub mod SM2CVAL1CYC {
    #[doc = "Capture Value 1 Cycle"]
    pub mod CVAL1CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 2 Register"]
pub mod SM2CVAL2 {
    #[doc = "Capture Value 2"]
    pub mod CAPTVAL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 2 Cycle Register"]
pub mod SM2CVAL2CYC {
    #[doc = "Capture Value 2 Cycle"]
    pub mod CVAL2CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 3 Register"]
pub mod SM2CVAL3 {
    #[doc = "Capture Value 3"]
    pub mod CAPTVAL3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 3 Cycle Register"]
pub mod SM2CVAL3CYC {
    #[doc = "Capture Value 3 Cycle"]
    pub mod CVAL3CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 4 Register"]
pub mod SM2CVAL4 {
    #[doc = "Capture Value 4"]
    pub mod CAPTVAL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 4 Cycle Register"]
pub mod SM2CVAL4CYC {
    #[doc = "Capture Value 4 Cycle"]
    pub mod CVAL4CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 5 Register"]
pub mod SM2CVAL5 {
    #[doc = "Capture Value 5"]
    pub mod CAPTVAL5 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 5 Cycle Register"]
pub mod SM2CVAL5CYC {
    #[doc = "Capture Value 5 Cycle"]
    pub mod CVAL5CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase Delay Register"]
pub mod SM2PHASEDLY {
    #[doc = "Initial Count Register Bits"]
    pub mod PHASEDLY {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
pub mod SM2CAPTFILTA {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTA_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTA_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
pub mod SM2CAPTFILTB {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTB_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTB_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
pub mod SM2CAPTFILTX {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTX_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTX_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter Register"]
pub mod SM3CNT {
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
pub mod SM3INIT {
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
pub mod SM3CTRL2 {
    #[doc = "Clock Source Select"]
    pub mod CLK_SEL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
            pub const IPBUS: u16 = 0;
            #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
            pub const EXT_CLK: u16 = 0x01;
            #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
            pub const AUX_CLK: u16 = 0x02;
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
            pub const LOCAL: u16 = 0;
            #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
            pub const MASTER: u16 = 0x01;
        }
    }
    #[doc = "Force Select"]
    pub mod FORCE_SEL {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
            pub const LOCAL: u16 = 0;
            #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER: u16 = 0x01;
            #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
            pub const LOCAL_RELOAD: u16 = 0x02;
            #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER_RELOAD: u16 = 0x03;
            #[doc = "The local sync signal from this submodule is used to force updates."]
            pub const LOCAL_SYNC: u16 = 0x04;
            #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
            pub const MASTER_SYNC: u16 = 0x05;
            #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
            pub const EXT_FORCE: u16 = 0x06;
            #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
            pub const EXT_SYNC: u16 = 0x07;
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
    #[doc = "Force Enable"]
    pub mod FRCEN {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initialization from a FORCE_OUT is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Initialization from a FORCE_OUT is enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const PWM_X: u16 = 0;
            #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
            pub const MASTER_RELOAD: u16 = 0x01;
            #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
            pub const MASTER_SYNC: u16 = 0x02;
            #[doc = "EXT_SYNC causes initialization."]
            pub const EXT_SYNC: u16 = 0x03;
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
            pub const COMPLEMENTARY: u16 = 0;
            #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
            pub const INDEPENDENT: u16 = 0x01;
        }
    }
    #[doc = "Wait Enable"]
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
pub mod SM3CTRL {
    #[doc = "Double Switching Enable"]
    pub mod DBLEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double switching disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Double switching enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "PWM_X Double Switching Enable"]
    pub mod DBLX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_X double pulse disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "PWM_X double pulse enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const NEXT_PWM_RELOAD: u16 = 0;
            #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
            pub const MTCTRL_LDOK_SET: u16 = 0x01;
        }
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B"]
    pub mod SPLIT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DBLPWM is not split. PWM_A and PWM_B each have double pulses."]
            pub const DISABLED: u16 = 0;
            #[doc = "DBLPWM is split to PWM_A and PWM_B."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "Prescaler"]
    pub mod PRSC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Prescaler 1"]
            pub const ONE: u16 = 0;
            #[doc = "Prescaler 2"]
            pub const TWO: u16 = 0x01;
            #[doc = "Prescaler 4"]
            pub const FOUR: u16 = 0x02;
            #[doc = "Prescaler 8"]
            pub const EIGHT: u16 = 0x03;
            #[doc = "Prescaler 16"]
            pub const SIXTEEN: u16 = 0x04;
            #[doc = "Prescaler 32"]
            pub const THIRTYTWO: u16 = 0x05;
            #[doc = "Prescaler 64"]
            pub const SIXTYFOUR: u16 = 0x06;
            #[doc = "Prescaler 128"]
            pub const HUNDREDTWENTYEIGHT: u16 = 0x07;
        }
    }
    #[doc = "Compare Mode"]
    pub mod COMPMODE {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
            pub const EQUAL_TO: u16 = 0;
            #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
            pub const EQUAL_TO_OR_GREATER_THAN: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Full-cycle reloads enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Half-cycle reloads enabled."]
            pub const ENABLED: u16 = 0x01;
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
            pub const EVERYPWM: u16 = 0;
            #[doc = "Every 2 PWM opportunities"]
            pub const EVERY2PWM: u16 = 0x01;
            #[doc = "Every 3 PWM opportunities"]
            pub const EVERY3PWM: u16 = 0x02;
            #[doc = "Every 4 PWM opportunities"]
            pub const EVERY4PWM: u16 = 0x03;
            #[doc = "Every 5 PWM opportunities"]
            pub const EVERY5PWM: u16 = 0x04;
            #[doc = "Every 6 PWM opportunities"]
            pub const EVERY6PWM: u16 = 0x05;
            #[doc = "Every 7 PWM opportunities"]
            pub const EVERY7PWM: u16 = 0x06;
            #[doc = "Every 8 PWM opportunities"]
            pub const EVERY8PWM: u16 = 0x07;
            #[doc = "Every 9 PWM opportunities"]
            pub const EVERY9PWM: u16 = 0x08;
            #[doc = "Every 10 PWM opportunities"]
            pub const EVERY10PWM: u16 = 0x09;
            #[doc = "Every 11 PWM opportunities"]
            pub const EVERY11PWM: u16 = 0x0a;
            #[doc = "Every 12 PWM opportunities"]
            pub const EVERY12PWM: u16 = 0x0b;
            #[doc = "Every 13 PWM opportunities"]
            pub const EVERY13PWM: u16 = 0x0c;
            #[doc = "Every 14 PWM opportunities"]
            pub const EVERY14PWM: u16 = 0x0d;
            #[doc = "Every 15 PWM opportunities"]
            pub const EVERY15PWM: u16 = 0x0e;
            #[doc = "Every 16 PWM opportunities"]
            pub const EVERY16PWM: u16 = 0x0f;
        }
    }
}
#[doc = "Value Register 0"]
pub mod SM3VAL0 {
    #[doc = "Value 0"]
    pub mod VAL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 1"]
pub mod SM3FRACVAL1 {
    #[doc = "Fractional Value 1"]
    pub mod FRACVAL1 {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Value Register 1"]
pub mod SM3VAL1 {
    #[doc = "Value 1"]
    pub mod VAL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 2"]
pub mod SM3FRACVAL2 {
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
pub mod SM3VAL2 {
    #[doc = "Value 2"]
    pub mod VAL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 3"]
pub mod SM3FRACVAL3 {
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
pub mod SM3VAL3 {
    #[doc = "Value 3"]
    pub mod VAL3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 4"]
pub mod SM3FRACVAL4 {
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
pub mod SM3VAL4 {
    #[doc = "Value 4"]
    pub mod VAL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Value Register 5"]
pub mod SM3FRACVAL5 {
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
pub mod SM3VAL5 {
    #[doc = "Value 5"]
    pub mod VAL5 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Control Register"]
pub mod SM3FRCTRL {
    #[doc = "Fractional Cycle PWM Period Enable"]
    pub mod FRAC1_EN {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable fractional cycle length for the PWM period."]
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle length for the PWM period."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle placement for PWM_A."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enable fractional cycle placement for PWM_B."]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM3OCTRL {
    #[doc = "PWM_X Fault State"]
    pub mod PWMXFS {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const LOGIC_0: u16 = 0;
            #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
            pub const LOGIC_1: u16 = 0x01;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_2: u16 = 0x02;
            #[doc = "Output is put in a high-impedance state."]
            pub const TRISTATED_3: u16 = 0x03;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
            pub const NOT_INVERTED: u16 = 0;
            #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
            pub const INVERTED: u16 = 0x01;
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
pub mod SM3STS {
    #[doc = "Compare Flags"]
    pub mod CMPF {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No compare event has occurred for a particular VALx value."]
            pub const NO_EVENT: u16 = 0;
            #[doc = "A compare event has occurred for a particular VALx value."]
            pub const EVENT: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
            pub const FLAG: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
            pub const FLAG: u16 = 0x01;
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
            pub const NO_FLAG: u16 = 0;
            #[doc = "At least one of the double buffered registers has been updated since the last reload."]
            pub const FLAG: u16 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable Register"]
pub mod SM3INTEN {
    #[doc = "Compare Interrupt Enables"]
    pub mod CMPIE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
            pub const DISABLED: u16 = 0;
            #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "Capture A 1 Interrupt Enable"]
    pub mod CA1IE {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt request disabled for STS\\[CFA1\\]"]
            pub const DISABLED: u16 = 0;
            #[doc = "Interrupt request enabled for STS\\[CFA1\\]"]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
            pub const ENABLED: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
            pub const ENABLED: u16 = 0x01;
        }
    }
}
#[doc = "DMA Enable Register"]
pub mod SM3DMAEN {
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
            pub const DISABLED: u16 = 0;
            #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
            pub const EXCEEDFIFO: u16 = 0x01;
            #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
            pub const LOCAL_SYNC: u16 = 0x02;
            #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
            pub const LOCAL_RELOAD: u16 = 0x03;
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
            pub const OR: u16 = 0;
            #[doc = "Selected FIFO watermarks are AND'ed together."]
            pub const AND: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u16 = 0x01;
        }
    }
}
#[doc = "Output Trigger Control Register"]
pub mod SM3TCTRL {
    #[doc = "Output Trigger Enables"]
    pub mod OUT_TRIG_EN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
            pub const VAL0: u16 = 0x01;
        }
    }
    #[doc = "Trigger Frequency"]
    pub mod TRGFRQ {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
            pub const EVERYPWM: u16 = 0;
            #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
            pub const FINALPWM: u16 = 0x01;
        }
    }
    #[doc = "Mux Output Trigger 1 Source Select"]
    pub mod PWBOT1 {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
            pub const PWM_OUT_TRIG1_SIGNAL: u16 = 0;
            #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
            pub const PWMB_OUTPUT: u16 = 0x01;
        }
    }
    #[doc = "Mux Output Trigger 0 Source Select"]
    pub mod PWAOT0 {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
            pub const PWM_OUT_TRIG0_SIGNAL: u16 = 0;
            #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
            pub const PWMA_OUTPUT: u16 = 0x01;
        }
    }
}
#[doc = "Fault Disable Mapping Register 0"]
pub mod SM3DISMAP0 {
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
#[doc = "Deadtime Count Register 0"]
pub mod SM3DTCNT0 {
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
pub mod SM3DTCNT1 {
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
pub mod SM3CAPTCTRLA {
    #[doc = "Arm A"]
    pub mod ARMA {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode A"]
    pub mod ONESHOTA {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_A: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM3CAPTCOMPA {
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
pub mod SM3CAPTCTRLB {
    #[doc = "Arm B"]
    pub mod ARMB {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode B"]
    pub mod ONESHOTB {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_B: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM3CAPTCOMPB {
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
pub mod SM3CAPTCTRLX {
    #[doc = "Arm X"]
    pub mod ARMX {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input capture operation is disabled."]
            pub const DISABLED: u16 = 0;
            #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
            pub const ENABLED: u16 = 0x01;
        }
    }
    #[doc = "One Shot Mode Aux"]
    pub mod ONESHOTX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free Running"]
            pub const FREE_RUNNING: u16 = 0;
            #[doc = "One Shot"]
            pub const ONE_SHOT: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Capture falling edges"]
            pub const FALLING_EDGE: u16 = 0x01;
            #[doc = "Capture rising edges"]
            pub const RISING_EDGE: u16 = 0x02;
            #[doc = "Capture any edge"]
            pub const ANY_EDGE: u16 = 0x03;
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
            pub const PWM_X: u16 = 0;
            #[doc = "Edge Counter"]
            pub const EDGE_COUNTER: u16 = 0x01;
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
            pub const DISABLED: u16 = 0;
            #[doc = "Edge counter enabled"]
            pub const ENABLED: u16 = 0x01;
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
pub mod SM3CAPTCOMPX {
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
pub mod SM3CVAL0 {
    #[doc = "Capture Value 0"]
    pub mod CAPTVAL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 0 Cycle Register"]
pub mod SM3CVAL0CYC {
    #[doc = "Capture Value 0 Cycle"]
    pub mod CVAL0CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 1 Register"]
pub mod SM3CVAL1 {
    #[doc = "Capture Value 1"]
    pub mod CAPTVAL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 1 Cycle Register"]
pub mod SM3CVAL1CYC {
    #[doc = "Capture Value 1 Cycle"]
    pub mod CVAL1CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 2 Register"]
pub mod SM3CVAL2 {
    #[doc = "Capture Value 2"]
    pub mod CAPTVAL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 2 Cycle Register"]
pub mod SM3CVAL2CYC {
    #[doc = "Capture Value 2 Cycle"]
    pub mod CVAL2CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 3 Register"]
pub mod SM3CVAL3 {
    #[doc = "Capture Value 3"]
    pub mod CAPTVAL3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 3 Cycle Register"]
pub mod SM3CVAL3CYC {
    #[doc = "Capture Value 3 Cycle"]
    pub mod CVAL3CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 4 Register"]
pub mod SM3CVAL4 {
    #[doc = "Capture Value 4"]
    pub mod CAPTVAL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 4 Cycle Register"]
pub mod SM3CVAL4CYC {
    #[doc = "Capture Value 4 Cycle"]
    pub mod CVAL4CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 5 Register"]
pub mod SM3CVAL5 {
    #[doc = "Capture Value 5"]
    pub mod CAPTVAL5 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value 5 Cycle Register"]
pub mod SM3CVAL5CYC {
    #[doc = "Capture Value 5 Cycle"]
    pub mod CVAL5CYC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase Delay Register"]
pub mod SM3PHASEDLY {
    #[doc = "Initial Count Register Bits"]
    pub mod PHASEDLY {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_A Input Filter Register"]
pub mod SM3CAPTFILTA {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTA_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTA_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_B Input Filter Register"]
pub mod SM3CAPTFILTB {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTB_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTB_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture PWM_X Input Filter Register"]
pub mod SM3CAPTFILTX {
    #[doc = "Input Capture Filter Period"]
    pub mod CAPTX_FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Filter Count"]
    pub mod CAPTX_FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Enable Register"]
pub mod OUTEN {
    #[doc = "PWM_X Output Enables"]
    pub mod PWMX_EN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM_B Output Enables"]
    pub mod PWMB_EN {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM_A Output Enables"]
    pub mod PWMA_EN {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
        pub mod RW {}
    }
    #[doc = "PWM_B Masks"]
    pub mod MASKB {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM_A Masks"]
    pub mod MASKA {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
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
            #[doc = "Generated SM0PWM45 signal used by the deadtime logic."]
            pub const SM0PWM45: u16 = 0;
            #[doc = "Inverted generated SM0PWM45 signal used by the deadtime logic."]
            pub const INVERTED_SM0PWM45: u16 = 0x01;
            #[doc = "SWCOUT\\[SM0OUT45\\] used by the deadtime logic."]
            pub const SM0OUT45: u16 = 0x02;
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
            #[doc = "Generated SM1PWM45 signal used by the deadtime logic."]
            pub const SM1PWM45: u16 = 0;
            #[doc = "Inverted generated SM1PWM45 signal used by the deadtime logic."]
            pub const INVERTED_SM1PWM45: u16 = 0x01;
            #[doc = "SWCOUT\\[SM1OUT45\\] used by the deadtime logic."]
            pub const SM1OUT45: u16 = 0x02;
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
            #[doc = "Generated SM2PWM45 signal used by the deadtime logic."]
            pub const SM2PWM45: u16 = 0;
            #[doc = "Inverted generated SM2PWM45 signal used by the deadtime logic."]
            pub const INVERTED_SM2PWM45: u16 = 0x01;
            #[doc = "SWCOUT\\[SM2OUT45\\] used by the deadtime logic."]
            pub const SM2OUT45: u16 = 0x02;
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
            #[doc = "Generated SM3PWM45 signal used by the deadtime logic."]
            pub const SM3PWM45: u16 = 0;
            #[doc = "Inverted generated SM3PWM45 signal used by the deadtime logic."]
            pub const INVERTED_SM3PWM45: u16 = 0x01;
            #[doc = "SWCOUT\\[SM3OUT45\\] used by the deadtime logic."]
            pub const SM3OUT45: u16 = 0x02;
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
    #[doc = "Write protect"]
    pub mod WRPROT {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write protection off (default)."]
            pub const DISABLED: u16 = 0;
            #[doc = "Write protection on."]
            pub const ENABLED: u16 = 0x01;
            #[doc = "Write protection off and locked until chip reset."]
            pub const DISABLED_LOCKED: u16 = 0x02;
            #[doc = "Write protection on and locked until chip reset."]
            pub const ENABLED_LOCKED: u16 = 0x03;
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
