#[doc = "MUB"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Processor B Transmit Register 0"]
    pub TR0: crate::RWRegister<u32>,
    #[doc = "Processor B Transmit Register 1"]
    pub TR1: crate::RWRegister<u32>,
    #[doc = "Processor B Transmit Register 2"]
    pub TR2: crate::RWRegister<u32>,
    #[doc = "Processor B Transmit Register 3"]
    pub TR3: crate::RWRegister<u32>,
    #[doc = "Processor B Receive Register 0"]
    pub RR0: crate::RORegister<u32>,
    #[doc = "Processor B Receive Register 1"]
    pub RR1: crate::RORegister<u32>,
    #[doc = "Processor B Receive Register 2"]
    pub RR2: crate::RORegister<u32>,
    #[doc = "Processor B Receive Register 3"]
    pub RR3: crate::RORegister<u32>,
    #[doc = "Processor B Status Register"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Processor B Control Register"]
    pub CR: crate::RWRegister<u32>,
}
#[doc = "Processor B Transmit Register 0"]
pub mod TR0 {
    #[doc = "TR0"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor B Transmit Register 1"]
pub mod TR1 {
    #[doc = "TR1"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor B Transmit Register 2"]
pub mod TR2 {
    #[doc = "TR2"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor B Transmit Register 3"]
pub mod TR3 {
    #[doc = "TR3"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor B Receive Register 0"]
pub mod RR0 {
    #[doc = "RR0"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor B Receive Register 1"]
pub mod RR1 {
    #[doc = "RR1"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor B Receive Register 2"]
pub mod RR2 {
    #[doc = "RR2"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor B Receive Register 3"]
pub mod RR3 {
    #[doc = "RR3"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Processor B Status Register"]
pub mod SR {
    #[doc = "Fn"]
    pub mod FN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ABFn bit in MUA.CR register is written 0 (default)."]
            pub const ZERO: u32 = 0;
            #[doc = "ABFn bit in MUA.CR register is written 1."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "EP"]
    pub mod EP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Processor B-side event is not pending (default)."]
            pub const NOT_PENDING: u32 = 0;
            #[doc = "The Processor B-side event is pending."]
            pub const PENDING: u32 = 0x01;
        }
    }
    #[doc = "RS"]
    pub mod RS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The Processor A or the Processor A-side of the MU is not in reset."]
            pub const NOT_RESET: u32 = 0;
            #[doc = "The Processor A or the Processor A-side of the MU is in reset."]
            pub const RESET: u32 = 0x01;
        }
    }
    #[doc = "FUP"]
    pub mod FUP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flags updated, initiated by the Processor B, in progress (default)"]
            pub const NO_UPDATE: u32 = 0;
            #[doc = "Processor B initiated flags update, processing"]
            pub const UPDATE: u32 = 0x01;
        }
    }
    #[doc = "TEn"]
    pub mod TEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MUB.TRn register is not empty."]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "MUB.TRn register is empty (default)."]
            pub const EMPTY: u32 = 0x01;
        }
    }
    #[doc = "RFn"]
    pub mod RFN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MUB.RRn register is not full (default)."]
            pub const NOT_FULL: u32 = 0;
            #[doc = "MUB.RRn register has received data from MUA.TRn register and is ready to be read by the Processor B."]
            pub const FULL: u32 = 0x01;
        }
    }
    #[doc = "GIPn"]
    pub mod GIPN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor B general purpose interrupt n is not pending. (default)"]
            pub const NOT_PENDING: u32 = 0;
            #[doc = "Processor B general purpose interrupt n is pending."]
            pub const PENDING: u32 = 0x01;
        }
    }
}
#[doc = "Processor B Control Register"]
pub mod CR {
    #[doc = "Fn"]
    pub mod FN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clears the Fn bit in the MUA.SR register."]
            pub const CLEAR: u32 = 0;
            #[doc = "Sets the Fn bit in the MUA.SR register."]
            pub const SET: u32 = 0x01;
        }
    }
    #[doc = "GIRn"]
    pub mod GIRN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor B General Interrupt n is not requested to the Processor A (default)."]
            pub const NOT_REQUESTED: u32 = 0;
            #[doc = "Processor B General Interrupt n is requested to the Processor A."]
            pub const REQUESTED: u32 = 0x01;
        }
    }
    #[doc = "TIEn"]
    pub mod TIEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables Processor B Transmit Interrupt n. (default)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables Processor B Transmit Interrupt n."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "RIEn"]
    pub mod RIEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables Processor B Receive Interrupt n. (default)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables Processor B Receive Interrupt n."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GIEn"]
    pub mod GIEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables Processor B General Interrupt n. (default)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables Processor B General Interrupt n."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
