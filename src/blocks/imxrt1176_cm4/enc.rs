#[doc = "QDC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u16>,
    #[doc = "Input Filter Register"]
    pub FILT: crate::RWRegister<u16>,
    #[doc = "Watchdog Timeout Register"]
    pub WTR: crate::RWRegister<u16>,
    #[doc = "Position Difference Counter Register"]
    pub POSD: crate::RWRegister<u16>,
    #[doc = "Position Difference Hold Register"]
    pub POSDH: crate::RORegister<u16>,
    #[doc = "Revolution Counter Register"]
    pub REV: crate::RWRegister<u16>,
    #[doc = "Revolution Hold Register"]
    pub REVH: crate::RORegister<u16>,
    #[doc = "Upper Position Counter Register"]
    pub UPOS: crate::RWRegister<u16>,
    #[doc = "Lower Position Counter Register"]
    pub LPOS: crate::RWRegister<u16>,
    #[doc = "Upper Position Hold Register"]
    pub UPOSH: crate::RORegister<u16>,
    #[doc = "Lower Position Hold Register"]
    pub LPOSH: crate::RORegister<u16>,
    #[doc = "Upper Initialization Register"]
    pub UINIT: crate::RWRegister<u16>,
    #[doc = "Lower Initialization Register"]
    pub LINIT: crate::RWRegister<u16>,
    #[doc = "Input Monitor Register"]
    pub IMR: crate::RORegister<u16>,
    #[doc = "Test Register"]
    pub TST: crate::RWRegister<u16>,
    #[doc = "Control 2 Register"]
    pub CTRL2: crate::RWRegister<u16>,
    #[doc = "Upper Modulus Register"]
    pub UMOD: crate::RWRegister<u16>,
    #[doc = "Lower Modulus Register"]
    pub LMOD: crate::RWRegister<u16>,
    #[doc = "Upper Position Compare Register"]
    pub UCOMP: crate::RWRegister<u16>,
    #[doc = "Lower Position Compare Register"]
    pub LCOMP: crate::RWRegister<u16>,
    #[doc = "Last Edge Time Register"]
    pub LASTEDGE: crate::RORegister<u16>,
    #[doc = "Last Edge Time Hold Register"]
    pub LASTEDGEH: crate::RORegister<u16>,
    #[doc = "Position Difference Period Counter Register"]
    pub POSDPER: crate::RORegister<u16>,
    #[doc = "Position Difference Period Buffer Register"]
    pub POSDPERBFR: crate::RORegister<u16>,
    #[doc = "Position Difference Period Hold Register"]
    pub POSDPERH: crate::RORegister<u16>,
    #[doc = "Control 3 Register"]
    pub CTRL3: crate::RWRegister<u16>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "Compare Interrupt Enable"]
    pub mod CMPIE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare interrupt is disabled"]
            pub const CMPIE_0: u16 = 0;
            #[doc = "Compare interrupt is enabled"]
            pub const CMPIE_1: u16 = 0x01;
        }
    }
    #[doc = "Compare Interrupt Request"]
    pub mod CMPIRQ {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No match has occurred"]
            pub const CMPIRQ_0: u16 = 0;
            #[doc = "COMP match has occurred"]
            pub const CMPIRQ_1: u16 = 0x01;
        }
    }
    #[doc = "Watchdog Enable"]
    pub mod WDE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog timer is disabled"]
            pub const WDE_0: u16 = 0;
            #[doc = "Watchdog timer is enabled"]
            pub const WDE_1: u16 = 0x01;
        }
    }
    #[doc = "Watchdog Timeout Interrupt Enable"]
    pub mod DIE {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog timer interrupt is disabled"]
            pub const DIE_0: u16 = 0;
            #[doc = "Watchdog timer interrupt is enabled"]
            pub const DIE_1: u16 = 0x01;
        }
    }
    #[doc = "Watchdog Timeout Interrupt Request"]
    pub mod DIRQ {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt has occurred"]
            pub const DIRQ_0: u16 = 0;
            #[doc = "Watchdog timeout interrupt has occurred"]
            pub const DIRQ_1: u16 = 0x01;
        }
    }
    #[doc = "Use Negative Edge of INDEX Pulse"]
    pub mod XNE {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use positive transition edge of INDEX pulse"]
            pub const XNE_0: u16 = 0;
            #[doc = "Use negative transition edge of INDEX pulse"]
            pub const XNE_1: u16 = 0x01;
        }
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    pub mod XIP {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const XIP_0: u16 = 0;
            #[doc = "INDEX pulse initializes the position counter"]
            pub const XIP_1: u16 = 0x01;
        }
    }
    #[doc = "INDEX Pulse Interrupt Enable"]
    pub mod XIE {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "INDEX pulse interrupt is disabled"]
            pub const XIE_0: u16 = 0;
            #[doc = "INDEX pulse interrupt is enabled"]
            pub const XIE_1: u16 = 0x01;
        }
    }
    #[doc = "INDEX Pulse Interrupt Request"]
    pub mod XIRQ {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt has occurred"]
            pub const XIRQ_0: u16 = 0;
            #[doc = "INDEX pulse interrupt has occurred"]
            pub const XIRQ_1: u16 = 0x01;
        }
    }
    #[doc = "Enable Signal Phase Count Mode"]
    pub mod PH1 {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use standard quadrature decoder where PHASEA and PHASEB represent a two phase quadrature signal."]
            pub const PH1_0: u16 = 0;
            #[doc = "Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction. If CTRL\\[REV\\] = 0, PHASEB = 0, then count up If CTRL\\[REV\\] = 0, PHASEB = 1, then count down If CTRL\\[REV\\] = 1, PHASEB = 0, then count down If CTRL\\[REV\\] = 1, PHASEB = 1, then count up"]
            pub const PH1_1: u16 = 0x01;
        }
    }
    #[doc = "Enable Reverse Direction Counting"]
    pub mod REV {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count normally"]
            pub const REV_0: u16 = 0;
            #[doc = "Count in the reverse direction"]
            pub const REV_1: u16 = 0x01;
        }
    }
    #[doc = "Software Triggered Initialization of Position Counters UPOS and LPOS"]
    pub mod SWIP {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const SWIP_0: u16 = 0;
            #[doc = "Initialize position counter"]
            pub const SWIP_1: u16 = 0x01;
        }
    }
    #[doc = "Use Negative Edge of HOME Input"]
    pub mod HNE {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use positive going edge-to-trigger initialization of position counters UPOS and LPOS"]
            pub const HNE_0: u16 = 0;
            #[doc = "Use negative going edge-to-trigger initialization of position counters UPOS and LPOS"]
            pub const HNE_1: u16 = 0x01;
        }
    }
    #[doc = "Enable HOME to Initialize Position Counters UPOS and LPOS"]
    pub mod HIP {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const HIP_0: u16 = 0;
            #[doc = "HOME signal initializes the position counter"]
            pub const HIP_1: u16 = 0x01;
        }
    }
    #[doc = "HOME Interrupt Enable"]
    pub mod HIE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable HOME interrupts"]
            pub const HIE_0: u16 = 0;
            #[doc = "Enable HOME interrupts"]
            pub const HIE_1: u16 = 0x01;
        }
    }
    #[doc = "HOME Signal Transition Interrupt Request"]
    pub mod HIRQ {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const HIRQ_0: u16 = 0;
            #[doc = "HOME signal transition interrupt request"]
            pub const HIRQ_1: u16 = 0x01;
        }
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
    #[doc = "Clock prescaler value"]
    pub mod FILT_PRSC {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog Timeout Register"]
pub mod WTR {
    #[doc = "WDOG\\[15:0\\] is a binary representation of the number of clock cycles plus one that the watchdog timer counts before timing out and optionally generating an interrupt"]
    pub mod WDOG {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Counter Register"]
pub mod POSD {
    #[doc = "This read/write register contains the position change in value occurring between each read of the position register"]
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
    #[doc = "This read-only register contains a snapshot of the value of the POSD register"]
    pub mod POSDH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Revolution Counter Register"]
pub mod REV {
    #[doc = "This read/write register contains the current value of the revolution counter."]
    pub mod REV {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Revolution Hold Register"]
pub mod REVH {
    #[doc = "This read-only register contains a snapshot of the value of the REV register."]
    pub mod REVH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Counter Register"]
pub mod UPOS {
    #[doc = "This read/write register contains the upper (most significant) half of the position counter"]
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
    #[doc = "This read/write register contains the lower (least significant) half of the position counter"]
    pub mod POS {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Hold Register"]
pub mod UPOSH {
    #[doc = "This read-only register contains a snapshot of the UPOS register."]
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
    #[doc = "This read-only register contains a snapshot of the LPOS register."]
    pub mod POSH {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Initialization Register"]
pub mod UINIT {
    #[doc = "This read/write register contains the value to be used to initialize the upper half of the position counter (UPOS)"]
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
    #[doc = "This read/write register contains the value to be used to initialize the lower half of the position counter (LPOS)"]
    pub mod INIT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Monitor Register"]
pub mod IMR {
    #[doc = "This is the raw HOME input."]
    pub mod HOME {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the raw INDEX input."]
    pub mod INDEX {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the raw PHASEB input."]
    pub mod PHB {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the raw PHASEA input."]
    pub mod PHA {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the filtered version of HOME input."]
    pub mod FHOM {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the filtered version of INDEX input."]
    pub mod FIND {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the filtered version of PHASEB input."]
    pub mod FPHB {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the filtered version of PHASEA input."]
    pub mod FPHA {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Test Register"]
pub mod TST {
    #[doc = "These bits hold the number of quadrature advances to generate."]
    pub mod TEST_COUNT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits hold the period of quadrature phase in IPBus clock cycles."]
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
            #[doc = "Leaves quadrature decoder signal in a positive direction"]
            pub const QDN_0: u16 = 0;
            #[doc = "Generates a negative quadrature decoder signal"]
            pub const QDN_1: u16 = 0x01;
        }
    }
    #[doc = "Test Counter Enable"]
    pub mod TCE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Test count is not enabled"]
            pub const TCE_0: u16 = 0;
            #[doc = "Test count is enabled"]
            pub const TCE_1: u16 = 0x01;
        }
    }
    #[doc = "Test Mode Enable"]
    pub mod TEN {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Test module is not enabled"]
            pub const TEN_0: u16 = 0;
            #[doc = "Test module is enabled"]
            pub const TEN_1: u16 = 0x01;
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
        pub mod RW {
            #[doc = "Disable updates of hold registers on rising edge of TRIGGER"]
            pub const UPDHLD_0: u16 = 0;
            #[doc = "Enable updates of hold registers on rising edge of TRIGGER"]
            pub const UPDHLD_1: u16 = 0x01;
        }
    }
    #[doc = "Update Position Registers"]
    pub mod UPDPOS {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action for POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
            pub const UPDPOS_0: u16 = 0;
            #[doc = "Clear POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
            pub const UPDPOS_1: u16 = 0x01;
        }
    }
    #[doc = "Enable Modulo Counting"]
    pub mod MOD {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable modulo counting"]
            pub const MOD_0: u16 = 0;
            #[doc = "Enable modulo counting"]
            pub const MOD_1: u16 = 0x01;
        }
    }
    #[doc = "Count Direction Flag"]
    pub mod DIR {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last count was in the down direction"]
            pub const DIR_0: u16 = 0;
            #[doc = "Last count was in the up direction"]
            pub const DIR_1: u16 = 0x01;
        }
    }
    #[doc = "Roll-under Interrupt Enable"]
    pub mod RUIE {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Roll-under interrupt is disabled"]
            pub const RUIE_0: u16 = 0;
            #[doc = "Roll-under interrupt is enabled"]
            pub const RUIE_1: u16 = 0x01;
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
            pub const RUIRQ_0: u16 = 0;
            #[doc = "Roll-under has occurred"]
            pub const RUIRQ_1: u16 = 0x01;
        }
    }
    #[doc = "Roll-over Interrupt Enable"]
    pub mod ROIE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Roll-over interrupt is disabled"]
            pub const ROIE_0: u16 = 0;
            #[doc = "Roll-over interrupt is enabled"]
            pub const ROIE_1: u16 = 0x01;
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
            pub const ROIRQ_0: u16 = 0;
            #[doc = "Roll-over has occurred"]
            pub const ROIRQ_1: u16 = 0x01;
        }
    }
    #[doc = "Revolution Counter Modulus Enable"]
    pub mod REVMOD {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use INDEX pulse to increment/decrement revolution counter (REV)."]
            pub const REVMOD_0: u16 = 0;
            #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)."]
            pub const REVMOD_1: u16 = 0x01;
        }
    }
    #[doc = "Output Control"]
    pub mod OUTCTL {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the compare value (COMP)."]
            pub const OUTCTL_0: u16 = 0;
            #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read."]
            pub const OUTCTL_1: u16 = 0x01;
        }
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    pub mod SABIE {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Simultaneous PHASEA and PHASEB change interrupt disabled."]
            pub const SABIE_0: u16 = 0;
            #[doc = "Simultaneous PHASEA and PHASEB change interrupt enabled."]
            pub const SABIE_1: u16 = 0x01;
        }
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    pub mod SABIRQ {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No simultaneous change of PHASEA and PHASEB has occurred."]
            pub const SABIRQ_0: u16 = 0;
            #[doc = "A simultaneous change of PHASEA and PHASEB has occurred."]
            pub const SABIRQ_1: u16 = 0x01;
        }
    }
}
#[doc = "Upper Modulus Register"]
pub mod UMOD {
    #[doc = "This read/write register contains the upper (most significant) half of the modulus register"]
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
    #[doc = "This read/write register contains the lower (least significant) half of the modulus register"]
    pub mod MOD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Compare Register"]
pub mod UCOMP {
    #[doc = "This read/write register contains the upper (most significant) half of the position compare register"]
    pub mod COMP {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Compare Register"]
pub mod LCOMP {
    #[doc = "This read/write register contains the lower (least significant) half of the position compare register"]
    pub mod COMP {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
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
#[doc = "Control 3 Register"]
pub mod CTRL3 {
    #[doc = "Period measurement function enable"]
    pub mod PMEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Period measurement functions are not used. POSD is loaded to POSDH and then cleared whenever POSD, UPOS, LPOS, or REV is read."]
            pub const PMEN_0: u16 = 0;
            #[doc = "Period measurement functions are used. POSD is loaded to POSDH and then cleared only when POSD is read."]
            pub const PMEN_1: u16 = 0x01;
        }
    }
    #[doc = "Prescaler"]
    pub mod PRSC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
