#[doc = "SEMA42"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Gate"]
    pub GATE3: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE2: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE1: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE0: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE7: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE6: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE5: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE4: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE11: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE10: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE9: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE8: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE15: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE14: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE13: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE12: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE19: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE18: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE17: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE16: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE23: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE22: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE21: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE20: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE27: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE26: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE25: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE24: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE31: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE30: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE29: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE28: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE35: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE34: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE33: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE32: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE39: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE38: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE37: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE36: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE43: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE42: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE41: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE40: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE47: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE46: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE45: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE44: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE51: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE50: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE49: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE48: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE55: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE54: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE53: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE52: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE59: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE58: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE57: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE56: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE63: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE62: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE61: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE60: crate::RWRegister<u8>,
    _reserved0: [u8; 0x02],
    #[doc = "Reset Gate Read"]
    pub RSTGT_R: crate::RORegister<u16>,
}
#[doc = "Gate"]
pub mod GATE3 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE2 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE1 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE0 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE7 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE6 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE5 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE4 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE11 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE10 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE9 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE8 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE15 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE14 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE13 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE12 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE19 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE18 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE17 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE16 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE23 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE22 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE21 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE20 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE27 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE26 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE25 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE24 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE31 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE30 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE29 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE28 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE35 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE34 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE33 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE32 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE39 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE38 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE37 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE36 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE43 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE42 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE41 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE40 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE47 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE46 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE45 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE44 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE51 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE50 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE49 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE48 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE55 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE54 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE53 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE52 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE59 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE58 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE57 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE56 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE63 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE62 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE61 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Gate"]
pub mod GATE60 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u8 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u8 = 0x01;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u8 = 0x02;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u8 = 0x03;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u8 = 0x04;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u8 = 0x05;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u8 = 0x06;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u8 = 0x07;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u8 = 0x08;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u8 = 0x09;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u8 = 0x0a;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u8 = 0x0b;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u8 = 0x0c;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u8 = 0x0d;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u8 = 0x0e;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u8 = 0x0f;
        }
    }
}
#[doc = "Reset Gate Read"]
pub mod RSTGT_R {
    #[doc = "Reset Gate Number"]
    pub mod RSTGTN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Gate Domain"]
    pub mod RSTGMS {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Gate Finite State Machine"]
    pub mod RSTGSM {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle, waiting for the first data pattern write."]
            pub const IDLE: u16 = 0;
            #[doc = "Waiting for the second data pattern write"]
            pub const WAITING: u16 = 0x01;
            #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state."]
            pub const TWO_WRITE_DONE: u16 = 0x02;
        }
    }
}
