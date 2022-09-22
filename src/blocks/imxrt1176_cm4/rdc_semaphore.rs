#[doc = "SEMA42"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Gate Register"]
    pub GATE: [crate::RWRegister<u8>; 64usize],
    _reserved0: [u8; 0x02],
    #[doc = "Reset Gate Read"]
    pub RSTGT_R: crate::RWRegister<u16>,
}
#[doc = "Gate Register"]
pub mod GATE {
    #[doc = "Gate Finite State Machine."]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const GTFSM_0: u8 = 0;
            #[doc = "The gate has been locked by processor with master_index = 0."]
            pub const GTFSM_1: u8 = 0x01;
            #[doc = "The gate has been locked by processor with master_index = 1."]
            pub const GTFSM_2: u8 = 0x02;
            #[doc = "The gate has been locked by processor with master_index = 2."]
            pub const GTFSM_3: u8 = 0x03;
            #[doc = "The gate has been locked by processor with master_index = 3."]
            pub const GTFSM_4: u8 = 0x04;
            #[doc = "The gate has been locked by processor with master_index = 4."]
            pub const GTFSM_5: u8 = 0x05;
            #[doc = "The gate has been locked by processor with master_index = 5."]
            pub const GTFSM_6: u8 = 0x06;
            #[doc = "The gate has been locked by processor with master_index = 6."]
            pub const GTFSM_7: u8 = 0x07;
            #[doc = "The gate has been locked by processor with master_index = 7."]
            pub const GTFSM_8: u8 = 0x08;
            #[doc = "The gate has been locked by processor with master_index = 8."]
            pub const GTFSM_9: u8 = 0x09;
            #[doc = "The gate has been locked by processor with master_index = 9."]
            pub const GTFSM_10: u8 = 0x0a;
            #[doc = "The gate has been locked by processor with master_index = 10."]
            pub const GTFSM_11: u8 = 0x0b;
            #[doc = "The gate has been locked by processor with master_index = 11."]
            pub const GTFSM_12: u8 = 0x0c;
            #[doc = "The gate has been locked by processor with master_index = 12."]
            pub const GTFSM_13: u8 = 0x0d;
            #[doc = "The gate has been locked by processor with master_index = 13."]
            pub const GTFSM_14: u8 = 0x0e;
            #[doc = "The gate has been locked by processor with master_index = 14."]
            pub const GTFSM_15: u8 = 0x0f;
        }
    }
    #[doc = "Read-only bits. They indicate which domain had currently locked the gate."]
    pub mod LDOM {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is locked by domain 0. (True if the field GTFSM does not equal to 0000.)"]
            pub const LDOM_0: u8 = 0;
            #[doc = "The gate has been locked by domain 1."]
            pub const LDOM_1: u8 = 0x01;
        }
    }
}
#[doc = "Reset Gate Read"]
pub mod RSTGT_R {
    #[doc = "Reset Gate Bus Master"]
    pub mod RSTGMS {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Gate Finite State Machine"]
    pub mod RSTGSM {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle, waiting for the first data pattern write."]
            pub const RSTGSM_0: u16 = 0;
            #[doc = "Waiting for the second data pattern write."]
            pub const RSTGSM_1: u16 = 0x01;
            #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state. The \"01\" state persists for only one clock cycle. Software will never be able to observe this state."]
            pub const RSTGSM_2: u16 = 0x02;
            #[doc = "This state encoding is never used and therefore reserved."]
            pub const RSTGSM_3: u16 = 0x03;
        }
    }
    #[doc = "Reset Gate Number"]
    pub mod RSTGTN {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
