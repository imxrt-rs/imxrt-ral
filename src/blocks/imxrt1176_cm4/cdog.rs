#[doc = "CDOG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CONTROL: crate::RWRegister<u32>,
    #[doc = "Instruction Timer reload"]
    pub RELOAD: crate::RWRegister<u32>,
    #[doc = "Instruction Timer"]
    pub INSTRUCTION_TIMER: crate::RWRegister<u32>,
    #[doc = "Secure Counter"]
    pub SECURE_COUNTER: crate::WORegister<u32>,
    #[doc = "Status 1"]
    pub STATUS: crate::RORegister<u32>,
    #[doc = "Status 2"]
    pub STATUS2: crate::RORegister<u32>,
    #[doc = "Flags"]
    pub FLAGS: crate::RWRegister<u32>,
    #[doc = "Persistent Data Storage"]
    pub PERSISTENT: crate::RWRegister<u32>,
    #[doc = "START Command"]
    pub START: crate::WORegister<u32>,
    #[doc = "STOP Command"]
    pub STOP: crate::WORegister<u32>,
    #[doc = "RESTART Command"]
    pub RESTART: crate::WORegister<u32>,
    #[doc = "ADD Command"]
    pub ADD: crate::WORegister<u32>,
    #[doc = "ADD1 Command"]
    pub ADD1: crate::WORegister<u32>,
    #[doc = "ADD16 Command"]
    pub ADD16: crate::WORegister<u32>,
    #[doc = "ADD256 Command"]
    pub ADD256: crate::WORegister<u32>,
    #[doc = "SUB Command"]
    pub SUB: crate::WORegister<u32>,
    #[doc = "SUB1 Command"]
    pub SUB1: crate::WORegister<u32>,
    #[doc = "SUB16 Command"]
    pub SUB16: crate::WORegister<u32>,
    #[doc = "SUB256 Command"]
    pub SUB256: crate::WORegister<u32>,
}
#[doc = "Control"]
pub mod CONTROL {
    #[doc = "Lock control"]
    pub mod LOCK_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked"]
            pub const LOCKED: u32 = 0x01;
            #[doc = "Unlocked"]
            pub const UNLOCKED: u32 = 0x02;
        }
    }
    #[doc = "TIMEOUT fault control"]
    pub mod TIMEOUT_CTRL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
    }
    #[doc = "MISCOMPARE fault control"]
    pub mod MISCOMPARE_CTRL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
    }
    #[doc = "SEQUENCE fault control"]
    pub mod SEQUENCE_CTRL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
    }
    #[doc = "CONTROL fault control"]
    pub mod CONTROL_CTRL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Disable reset"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
    }
    #[doc = "STATE fault control"]
    pub mod STATE_CTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
    }
    #[doc = "ADDRESS fault control"]
    pub mod ADDRESS_CTRL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
    }
    #[doc = "IRQ pause control"]
    pub mod IRQ_PAUSE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep the timer running"]
            pub const RUN_TIMER: u32 = 0x01;
            #[doc = "Stop the timer"]
            pub const PAUSE_TIMER: u32 = 0x02;
        }
    }
    #[doc = "DEBUG_HALT control"]
    pub mod DEBUG_HALT_CTRL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep the timer running"]
            pub const RUN_TIMER: u32 = 0x01;
            #[doc = "Stop the timer"]
            pub const PAUSE_TIMER: u32 = 0x02;
        }
    }
}
#[doc = "Instruction Timer reload"]
pub mod RELOAD {
    #[doc = "Instruction Timer reload value"]
    pub mod RLOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Instruction Timer"]
pub mod INSTRUCTION_TIMER {
    #[doc = "Current value of the Instruction Timer"]
    pub mod INSTIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Secure Counter"]
pub mod SECURE_COUNTER {
    #[doc = "Secure Counter"]
    pub mod SECCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 1"]
pub mod STATUS {
    #[doc = "Number of TIMEOUT faults since the last POR"]
    pub mod NUMTOF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of MISCOMPARE faults since the last POR"]
    pub mod NUMMISCOMPF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of SEQUENCE faults since the last POR"]
    pub mod NUMILSEQF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current State"]
    pub mod CURST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 2"]
pub mod STATUS2 {
    #[doc = "Number of CONTROL faults since the last POR"]
    pub mod NUMCNTF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of STATE faults since the last POR"]
    pub mod NUMILLSTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of ADDRESS faults since the last POR"]
    pub mod NUMILLA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flags"]
pub mod FLAGS {
    #[doc = "TIMEOUT fault flag"]
    pub mod TO_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A TIMEOUT fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A TIMEOUT fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "MISCOMPARE fault flag"]
    pub mod MISCOM_FLAG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A MISCOMPARE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A MISCOMPARE fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "SEQUENCE fault flag"]
    pub mod SEQ_FLAG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A SEQUENCE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A SEQUENCE fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "CONTROL fault flag"]
    pub mod CNT_FLAG {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A CONTROL fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A CONTROL fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "STATE fault flag"]
    pub mod STATE_FLAG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A STATE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A STATE fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "ADDRESS fault flag"]
    pub mod ADDR_FLAG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An ADDRESS fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "An ADDRESS fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "Power-on reset flag"]
    pub mod POR_FLAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A Power-on reset event has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A Power-on reset event has occurred"]
            pub const FLAG: u32 = 0x01;
        }
    }
}
#[doc = "Persistent Data Storage"]
pub mod PERSISTENT {
    #[doc = "Persistent Storage"]
    pub mod PERSIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "START Command"]
pub mod START {
    #[doc = "Start command"]
    pub mod STRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STOP Command"]
pub mod STOP {
    #[doc = "Stop command"]
    pub mod STP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RESTART Command"]
pub mod RESTART {
    #[doc = "Restart command"]
    pub mod RSTRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADD Command"]
pub mod ADD {
    #[doc = "ADD Write Value"]
    pub mod AD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADD1 Command"]
pub mod ADD1 {
    #[doc = "ADD 1"]
    pub mod AD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADD16 Command"]
pub mod ADD16 {
    #[doc = "ADD 16"]
    pub mod AD16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADD256 Command"]
pub mod ADD256 {
    #[doc = "ADD 256"]
    pub mod AD256 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SUB Command"]
pub mod SUB {
    #[doc = "Subtract Write Value"]
    pub mod S0B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SUB1 Command"]
pub mod SUB1 {
    #[doc = "Subtract 1"]
    pub mod S1B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SUB16 Command"]
pub mod SUB16 {
    #[doc = "Subtract 16"]
    pub mod SB16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SUB256 Command"]
pub mod SUB256 {
    #[doc = "Subtract 256"]
    pub mod SB256 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
