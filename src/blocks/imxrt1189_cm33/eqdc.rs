#[doc = "Quadrature_Decoder"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u16>,
    #[doc = "Control 2 Register"]
    pub CTRL2: crate::RWRegister<u16>,
    #[doc = "Input Filter Register"]
    pub FILT: crate::RWRegister<u16>,
    #[doc = "Last Edge Time Register"]
    pub LASTEDGE: crate::RORegister<u16>,
    #[doc = "Position Difference Period Counter Register"]
    pub POSDPER: crate::RORegister<u16>,
    #[doc = "Position Difference Period Buffer Register"]
    pub POSDPERBFR: crate::RORegister<u16>,
    #[doc = "Upper Position Counter Register"]
    pub UPOS: crate::RWRegister<u16>,
    #[doc = "Lower Position Counter Register"]
    pub LPOS: crate::RWRegister<u16>,
    #[doc = "Position Difference Counter Register"]
    pub POSD: crate::RWRegister<u16>,
    #[doc = "Position Difference Hold Register"]
    pub POSDH: crate::RORegister<u16>,
    #[doc = "Upper Position Hold Register"]
    pub UPOSH: crate::RORegister<u16>,
    #[doc = "Lower Position Hold Register"]
    pub LPOSH: crate::RORegister<u16>,
    #[doc = "Last Edge Time Hold Register"]
    pub LASTEDGEH: crate::RORegister<u16>,
    #[doc = "Position Difference Period Hold Register"]
    pub POSDPERH: crate::RORegister<u16>,
    #[doc = "Revolution Hold Register"]
    pub REVH: crate::RORegister<u16>,
    #[doc = "Revolution Counter Register"]
    pub REV: crate::RWRegister<u16>,
    #[doc = "Upper Initialization Register"]
    pub UINIT: crate::RWRegister<u16>,
    #[doc = "Lower Initialization Register"]
    pub LINIT: crate::RWRegister<u16>,
    #[doc = "Upper Modulus Register"]
    pub UMOD: crate::RWRegister<u16>,
    #[doc = "Lower Modulus Register"]
    pub LMOD: crate::RWRegister<u16>,
    #[doc = "Upper Position Compare Register 0"]
    pub UCOMP0: crate::RWRegister<u16>,
    #[doc = "Lower Position Compare Register 0"]
    pub LCOMP0: crate::RWRegister<u16>,
    #[doc = "Upper Position Holder Register 1"]
    pub UPOSH1: crate::RORegister<u16>,
    #[doc = "Lower Position Holder Register 1"]
    pub LPOSH1: crate::RORegister<u16>,
    #[doc = "Upper Position Holder Register 3"]
    pub UPOSH2: crate::RORegister<u16>,
    #[doc = "Lower Position Holder Register 2"]
    pub LPOSH2: crate::RORegister<u16>,
    #[doc = "Upper Position Holder Register 3"]
    pub UPOSH3: crate::RORegister<u16>,
    #[doc = "Lower Position Holder Register 3"]
    pub LPOSH3: crate::RORegister<u16>,
    #[doc = "Interrupt Control Register"]
    pub INTCTRL: crate::RWRegister<u16>,
    #[doc = "Watchdog Timeout Register"]
    pub WTR: crate::RWRegister<u16>,
    #[doc = "Input Monitor Register"]
    pub IMR: crate::RWRegister<u16>,
    #[doc = "Test Register"]
    pub TST: crate::RWRegister<u16>,
    _reserved0: [u8; 0x10],
    #[doc = "Upper VERID"]
    pub UVERID: crate::RORegister<u16>,
    #[doc = "Lower VERID"]
    pub LVERID: crate::RORegister<u16>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "Load Okay"]
    pub mod LDOK {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No loading action taken. Users can write new values to buffered registers (writing into outer-set of these buffered registers)"]
            pub const LDOK0: u16 = 0;
            #[doc = "Outer-set values are ready to be loaded into inner-set and take effect. The loading time point depends on CTRL2\\[LDMOD\\]."]
            pub const LDOK1: u16 = 0x01;
        }
    }
    #[doc = "DMA Enable"]
    pub mod DMAEN {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA is disabled"]
            pub const DMAEN_0: u16 = 0;
            #[doc = "DMA is enabled. DMA request asserts automatically when the values in the outer-set of buffered compare registers (UCOMP0/LCOMP0;UCOMP1/LCOMP1;UCOMP2/LCOMP2;UCOMP3/LCOMP3), initial registers(UINIT/LINIT) and modulus registers (UMOD/LMOD) are loaded into the inner-set of buffer and then LDOK is cleared automatically. After the completion of this DMA transfer, LDOK is set automatically, it ensures outer-set values can be loaded into inner-set which in turn triggers DMA again."]
            pub const DMAEN_1: u16 = 0x01;
        }
    }
    #[doc = "Watchdog Enable"]
    pub mod WDE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const WDE0: u16 = 0;
            #[doc = "Enabled"]
            pub const WDE1: u16 = 0x01;
        }
    }
    #[doc = "Watchdog Timeout Interrupt Enable"]
    pub mod WDIE {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const WDIE0: u16 = 0;
            #[doc = "Enabled"]
            pub const WDIE1: u16 = 0x01;
        }
    }
    #[doc = "Watchdog Timeout Interrupt Request"]
    pub mod WDIRQ {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Watchdog timeout interrupt has occurred"]
            pub const WDIRQ0: u16 = 0;
            #[doc = "Watchdog timeout interrupt has occurred"]
            pub const WDIRQ1: u16 = 0x01;
        }
    }
    #[doc = "Select Positive/Negative Edge of INDEX/PRESET Pulse"]
    pub mod XNE {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use positive edge of INDEX/PRESET pulse"]
            pub const XNE0: u16 = 0;
            #[doc = "Use negative edge of INDEX/PRESET pulse"]
            pub const XNE1: u16 = 0x01;
        }
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    pub mod XIP {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "INDEX pulse does not initialize the position counter"]
            pub const XIP0: u16 = 0;
            #[doc = "INDEX pulse initializes the position counter"]
            pub const XIP1: u16 = 0x01;
        }
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Enable"]
    pub mod XIE {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const XIE0: u16 = 0;
            #[doc = "Enabled"]
            pub const XIE1: u16 = 0x01;
        }
    }
    #[doc = "INDEX/PRESET Pulse Interrupt Request"]
    pub mod XIRQ {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "INDEX/PRESET pulse has not occurred"]
            pub const XIRQ0: u16 = 0;
            #[doc = "INDEX/PRESET pulse has occurred"]
            pub const XIRQ1: u16 = 0x01;
        }
    }
    #[doc = "Enable Single Phase Mode"]
    pub mod PH1 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard quadrature decoder, where PHASEA and PHASEB represent a two-phase quadrature signal."]
            pub const PH10: u16 = 0;
            #[doc = "Single phase mode, bypass the quadrature decoder, refer to CTRL2\\[CMODE\\] description"]
            pub const PH11: u16 = 0x01;
        }
    }
    #[doc = "Enable Reverse Direction Counting"]
    pub mod REV {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count normally and the position counter initialization uses upper/lower initialization register UINIT/LINIT"]
            pub const REV0: u16 = 0;
            #[doc = "Count in the reverse direction and the position counter initialization uses upper/lower modulus register UMOD/LMOD"]
            pub const REV1: u16 = 0x01;
        }
    }
    #[doc = "Software-Triggered Initialization of Position Counters UPOS and LPOS"]
    pub mod SWIP {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const SWIP0: u16 = 0;
            #[doc = "Initialize position counter"]
            pub const SWIP1: u16 = 0x01;
        }
    }
    #[doc = "Use Negative Edge of HOME/ENABLE Input"]
    pub mod HNE {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When CTRL\\[OPMODE\\] = 0,use HOME positive edge to trigger initialization of position counters. When CTRL\\[OPMODE\\] = 1,use ENABLE high level to enable POS/POSD/WDG/REV counters"]
            pub const HNE0: u16 = 0;
            #[doc = "When CTRL\\[OPMODE\\] = 0,use HOME negative edge to trigger initialization of position counters. When CTRL\\[OPMODE\\] = 1,use ENABLE low level to enable POS/POSD/WDG/REV counters"]
            pub const HNE1: u16 = 0x01;
        }
    }
    #[doc = "Enable HOME to Initialize Position Counter UPOS/LPOS"]
    pub mod HIP {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const HIP0: u16 = 0;
            #[doc = "HOME signal initializes the position counter"]
            pub const HIP1: u16 = 0x01;
        }
    }
    #[doc = "HOME/ENABLE Interrupt Enable"]
    pub mod HIE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const HIE0: u16 = 0;
            #[doc = "Enabled"]
            pub const HIE1: u16 = 0x01;
        }
    }
    #[doc = "HOME/ENABLE Signal Transition Interrupt Request"]
    pub mod HIRQ {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No transition on the HOME/ENABLE signal has occurred"]
            pub const HIRQ0: u16 = 0;
            #[doc = "A transition on the HOME/ENABLE signal has occurred"]
            pub const HIRQ1: u16 = 0x01;
        }
    }
}
#[doc = "Control 2 Register"]
pub mod CTRL2 {
    #[doc = "Update Hold Registers"]
    pub mod UPDHLD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Update Position Registers"]
    pub mod UPDPOS {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Operation Mode Select"]
    pub mod OPMODE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Decode Mode: Input nodes INDEX/PRESET and HOME/ENABLE are assigned to function of INDEX and HOME."]
            pub const OPMODE0: u16 = 0;
            #[doc = "Count Mode: Input nodes INDEX/PRESET and HOME/ENABLE are assigned to functions of PRESET and ENABLE. In this mode: (1)only when ENABLE=1, all counters (position/position difference/revolution/watchdog) can run, when ENABLE=0, all counters (position/position difference/revolution/watchdog) can't run. (2) the rising edge of PRESET input can initialize position/revolution/watchdog counters (position counter initialization also need referring to bit CTRL\\[REV\\])."]
            pub const OPMODE1: u16 = 0x01;
        }
    }
    #[doc = "Buffered Register Load (Update) Mode Select"]
    pub mod LDMOD {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Buffered registers are loaded and take effect immediately upon CTRL\\[LDOK\\] is set."]
            pub const LDMOD0: u16 = 0;
            #[doc = "Buffered registers are loaded and take effect at the next roll-over or roll-under if CTRL\\[LDOK\\] is set."]
            pub const LDMOD1: u16 = 0x01;
        }
    }
    #[doc = "Revolution Counter Modulus Enable"]
    pub mod REVMOD {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use INDEX pulse to increment/decrement revolution counter (REV)"]
            pub const REVMOD0: u16 = 0;
            #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)"]
            pub const REVMOD1: u16 = 0x01;
        }
    }
    #[doc = "Output Control"]
    pub mod OUTCTL {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "POS_MATCH\\[x\\](x range is 0-3) is asserted when the Position Counter is equal to according compare value (UCOMPx/LCOMPx)(x range is 0-3), and de-asserted when the Position Counter not equal to the compare value (UCOMPx/LCOMPx)(x range is 0-3)"]
            pub const OUTCTL0: u16 = 0;
            #[doc = "All POS_MATCH\\[x\\](x range is 0-3) are asserted a pulse, when the UPOS, LPOS, REV, or POSD registers are read"]
            pub const OUTCTL1: u16 = 0x01;
        }
    }
    #[doc = "Period measurement function enable"]
    pub mod PMEN {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Period measurement functions are not used. POSD is loaded to POSDH and then cleared whenever POSD, UPOS, LPOS or REV is read."]
            pub const PMEN0: u16 = 0;
            #[doc = "Period measurement functions are used. POSD is loaded into POSDH and then cleared only when POSD is read."]
            pub const PMEN1: u16 = 0x01;
        }
    }
    #[doc = "Initial Position Register"]
    pub mod INITPOS {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Don't initialize position counter on rising edge of TRIGGER"]
            pub const INITPOS0: u16 = 0;
            #[doc = "Initialize position counter on rising edge of TRIGGER"]
            pub const INITPOS1: u16 = 0x01;
        }
    }
    #[doc = "Count Once"]
    pub mod ONCE {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Position counter counts repeatedly"]
            pub const ONCE0: u16 = 0;
            #[doc = "Position counter counts until roll-over or roll-under, then stop."]
            pub const ONCE1: u16 = 0x01;
        }
    }
    #[doc = "Counting Mode"]
    pub mod CMODE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Filter Register"]
pub mod FILT {
    #[doc = "Input Filter Sample Period"]
    pub mod FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Filter Sample Count"]
    pub mod FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Filter Clock Source selection"]
    pub mod FILT_CS {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Peripheral Clock"]
            pub const FILT_CS0: u16 = 0;
            #[doc = "Prescaled peripheral clock by PRSC"]
            pub const FILT_CS1: u16 = 0x01;
        }
    }
    #[doc = "Prescaler"]
    pub mod PRSC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Last Edge Time Register"]
pub mod LASTEDGE {
    #[doc = "Last Edge Time Counter"]
    pub mod LASTEDGE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Period Counter Register"]
pub mod POSDPER {
    #[doc = "Position difference period"]
    pub mod POSDPER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Period Buffer Register"]
pub mod POSDPERBFR {
    #[doc = "Position difference period buffer"]
    pub mod POSDPERBFR {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Counter Register"]
pub mod UPOS {
    #[doc = "POS"]
    pub mod POS {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Counter Register"]
pub mod LPOS {
    #[doc = "POS"]
    pub mod POS {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Counter Register"]
pub mod POSD {
    #[doc = "POSD"]
    pub mod POSD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Hold Register"]
pub mod POSDH {
    #[doc = "POSDH"]
    pub mod POSDH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Hold Register"]
pub mod UPOSH {
    #[doc = "POSH"]
    pub mod POSH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Hold Register"]
pub mod LPOSH {
    #[doc = "POSH"]
    pub mod LPOSH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Last Edge Time Hold Register"]
pub mod LASTEDGEH {
    #[doc = "Last Edge Time Hold"]
    pub mod LASTEDGEH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Period Hold Register"]
pub mod POSDPERH {
    #[doc = "Position difference period hold"]
    pub mod POSDPERH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Revolution Hold Register"]
pub mod REVH {
    #[doc = "REVH"]
    pub mod REVH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Revolution Counter Register"]
pub mod REV {
    #[doc = "REV"]
    pub mod REV {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Initialization Register"]
pub mod UINIT {
    #[doc = "INIT"]
    pub mod INIT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Initialization Register"]
pub mod LINIT {
    #[doc = "INIT"]
    pub mod INIT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Modulus Register"]
pub mod UMOD {
    #[doc = "MOD"]
    pub mod MOD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Modulus Register"]
pub mod LMOD {
    #[doc = "MOD"]
    pub mod MOD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Compare Register 0"]
pub mod UCOMP0 {
    #[doc = "UCOMP0"]
    pub mod UCOMP0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Compare Register 0"]
pub mod LCOMP0 {
    #[doc = "LCOMP0"]
    pub mod LCOMP0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Holder Register 1"]
pub mod UPOSH1 {
    #[doc = "UPOSH1"]
    pub mod UPOSH1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Holder Register 1"]
pub mod LPOSH1 {
    #[doc = "LPOSH1"]
    pub mod LPOSH1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Holder Register 3"]
pub mod UPOSH2 {
    #[doc = "UPOSH2"]
    pub mod UPOSH2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Holder Register 2"]
pub mod LPOSH2 {
    #[doc = "LPOSH2"]
    pub mod LPOSH2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Holder Register 3"]
pub mod UPOSH3 {
    #[doc = "UPOSH3"]
    pub mod UPOSH3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Holder Register 3"]
pub mod LPOSH3 {
    #[doc = "LPOSH3"]
    pub mod LPOSH3 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Control Register"]
pub mod INTCTRL {
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    pub mod SABIE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const SABIE0: u16 = 0;
            #[doc = "Enabled"]
            pub const SABIE1: u16 = 0x01;
        }
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    pub mod SABIRQ {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No simultaneous change of PHASEA and PHASEB has occurred"]
            pub const SABIRQ0: u16 = 0;
            #[doc = "A simultaneous change of PHASEA and PHASEB has occurred"]
            pub const SABIRQ1: u16 = 0x01;
        }
    }
    #[doc = "Count direction change interrupt enable"]
    pub mod DIRIE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DIRIE0: u16 = 0;
            #[doc = "Enabled"]
            pub const DIRIE1: u16 = 0x01;
        }
    }
    #[doc = "Count direction change interrupt"]
    pub mod DIRIRQ {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count direction unchanged"]
            pub const DIRIRQ0: u16 = 0;
            #[doc = "Count direction changed"]
            pub const DIRIRQ1: u16 = 0x01;
        }
    }
    #[doc = "Roll-under Interrupt Enable"]
    pub mod RUIE {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const RUIE0: u16 = 0;
            #[doc = "Enabled"]
            pub const RUIE1: u16 = 0x01;
        }
    }
    #[doc = "Roll-under Interrupt Request"]
    pub mod RUIRQ {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No roll-under has occurred"]
            pub const RUIRQ0: u16 = 0;
            #[doc = "Roll-under has occurred"]
            pub const RUIRQ1: u16 = 0x01;
        }
    }
    #[doc = "Roll-over Interrupt Enable"]
    pub mod ROIE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const ROIE: u16 = 0;
            #[doc = "Enabled"]
            pub const ROIE1: u16 = 0x01;
        }
    }
    #[doc = "Roll-over Interrupt Request"]
    pub mod ROIRQ {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No roll-over has occurred"]
            pub const ROIRQ0: u16 = 0;
            #[doc = "Roll-over has occurred"]
            pub const ROIRQ1: u16 = 0x01;
        }
    }
    #[doc = "Compare 0 Interrupt Request"]
    pub mod CMP0IRQ {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No match has occurred (the position counter does not match the COMP0 value)"]
            pub const CMP0IRQ0: u16 = 0;
            #[doc = "COMP match has occurred (the position counter matches the COMP0 value)"]
            pub const CMP0IRQ1: u16 = 0x01;
        }
    }
    #[doc = "Compare1 Interrupt Request"]
    pub mod CMP1IRQ {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No match has occurred (the position counter does not match the COMP1 value)"]
            pub const CMP1IRQ0: u16 = 0;
            #[doc = "COMP1 match has occurred (the position counter matches the COMP1 value)"]
            pub const CMP1IRQ1: u16 = 0x01;
        }
    }
    #[doc = "Compare2 Interrupt Request"]
    pub mod CMP2IRQ {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No match has occurred (the position counter does not match the COMP2 value)"]
            pub const CMP2IRQ0: u16 = 0;
            #[doc = "COMP2 match has occurred (the position counter matches the COMP2 value)"]
            pub const CMP2IRQ1: u16 = 0x01;
        }
    }
    #[doc = "Compare3 Interrupt Request"]
    pub mod CMP3IRQ {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No match has occurred (the position counter does not match the COMP3 value)"]
            pub const CMP3IRQ0: u16 = 0;
            #[doc = "COMP3 match has occurred (the position counter matches the COMP3 value)"]
            pub const CMP3IRQ1: u16 = 0x01;
        }
    }
}
#[doc = "Watchdog Timeout Register"]
pub mod WTR {
    #[doc = "WDOG"]
    pub mod WDOG {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Monitor Register"]
pub mod IMR {
    #[doc = "HOME_ENABLE"]
    pub mod HOME_ENABLE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "INDEX_PRESET"]
    pub mod INDEX_PRESET {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHB"]
    pub mod PHB {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHA"]
    pub mod PHA {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter operation on HOME/ENABLE input"]
    pub mod FHOM_ENA {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter operation on INDEX/PRESET input"]
    pub mod FIND_PRE {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter operation on PHASEB input"]
    pub mod FPHB {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter operation on PHASEA input"]
    pub mod FPHA {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Position Compare 0 Flag Output"]
    pub mod CMPF0 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When the position counter is less than value of COMP0 register"]
            pub const CMPF00: u16 = 0;
            #[doc = "When the position counter is greater or equal than value of COMP0 register"]
            pub const CMPF01: u16 = 0x01;
        }
    }
    #[doc = "Position Compare1 Flag Output"]
    pub mod CMP1F {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When the position counter is less than value of COMP1 register"]
            pub const CMP1F0: u16 = 0;
            #[doc = "When the position counter is greater or equal than value of COMP1 register"]
            pub const CMP1F1: u16 = 0x01;
        }
    }
    #[doc = "Position Compare2 Flag Output"]
    pub mod CMP2F {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When the position counter is less than value of COMP2 register"]
            pub const CMP2F0: u16 = 0;
            #[doc = "When the position counter is greater or equal than value of COMP2 register"]
            pub const CMP2F1: u16 = 0x01;
        }
    }
    #[doc = "Position Compare3 Flag Output"]
    pub mod CMP3F {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When the position counter value is less than value of COMP3 register"]
            pub const CMP3F0: u16 = 0;
            #[doc = "When the position counter is greater or equal than value of COMP3 register"]
            pub const CMP3F1: u16 = 0x01;
        }
    }
    #[doc = "Count Direction Flag Hold"]
    pub mod DIRH {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count Direction Flag Output"]
    pub mod DIR {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Current count was in the down direction"]
            pub const DIR0: u16 = 0;
            #[doc = "Current count was in the up direction"]
            pub const DIR1: u16 = 0x01;
        }
    }
}
#[doc = "Test Register"]
pub mod TST {
    #[doc = "TEST_COUNT"]
    pub mod TEST_COUNT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TEST_PERIOD"]
    pub mod TEST_PERIOD {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Quadrature Decoder Negative Signal"]
    pub mod QDN {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generates a positive quadrature decoder signal"]
            pub const QDN0: u16 = 0;
            #[doc = "Generates a negative quadrature decoder signal"]
            pub const QDN1: u16 = 0x01;
        }
    }
    #[doc = "Test Counter Enable"]
    pub mod TCE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TCE0: u16 = 0;
            #[doc = "Enabled"]
            pub const TCE1: u16 = 0x01;
        }
    }
    #[doc = "Test Mode Enable"]
    pub mod TEN {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const TEN0: u16 = 0;
            #[doc = "Enabled"]
            pub const TEN1: u16 = 0x01;
        }
    }
}
#[doc = "Upper VERID"]
pub mod UVERID {
    #[doc = "UVERID"]
    pub mod UVERID {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower VERID"]
pub mod LVERID {
    #[doc = "LVERID"]
    pub mod LVERID {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
