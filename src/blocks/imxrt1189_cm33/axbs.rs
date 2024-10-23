#[doc = "AXBS"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Priority Slave Registers"]
    pub PRS0: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CRS0: crate::RWRegister<u32>,
    _reserved1: [u8; 0xec],
    #[doc = "Priority Slave Registers"]
    pub PRS1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CRS1: crate::RWRegister<u32>,
    _reserved3: [u8; 0xec],
    #[doc = "Priority Slave Registers"]
    pub PRS2: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CRS2: crate::RWRegister<u32>,
    _reserved5: [u8; 0xec],
    #[doc = "Priority Slave Registers"]
    pub PRS3: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CRS3: crate::RWRegister<u32>,
    _reserved7: [u8; 0xec],
    #[doc = "Priority Slave Registers"]
    pub PRS4: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CRS4: crate::RWRegister<u32>,
    _reserved9: [u8; 0xec],
    #[doc = "Priority Slave Registers"]
    pub PRS5: crate::RWRegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CRS5: crate::RWRegister<u32>,
    _reserved11: [u8; 0xec],
    #[doc = "Priority Slave Registers"]
    pub PRS6: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CRS6: crate::RWRegister<u32>,
    _reserved13: [u8; 0xec],
    #[doc = "Priority Slave Registers"]
    pub PRS7: crate::RWRegister<u32>,
    _reserved14: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CRS7: crate::RWRegister<u32>,
}
#[doc = "Priority Slave Registers"]
pub mod PRS0 {
    #[doc = "Master 0 Priority"]
    pub mod M0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 1 Priority"]
    pub mod M1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 2 Priority"]
    pub mod M2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 3 Priority"]
    pub mod M3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 4 Priority"]
    pub mod M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 5 Priority"]
    pub mod M5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
}
#[doc = "Control Register"]
pub mod CRS0 {
    #[doc = "Park"]
    pub mod PARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Park on master port M0"]
            pub const MASTER_PORT_0: u32 = 0;
            #[doc = "Park on master port M1"]
            pub const MASTER_PORT_1: u32 = 0x01;
            #[doc = "Park on master port M2"]
            pub const MASTER_PORT_2: u32 = 0x02;
            #[doc = "Park on master port M3"]
            pub const MASTER_PORT_3: u32 = 0x03;
            #[doc = "Park on master port M4"]
            pub const MASTER_PORT_4: u32 = 0x04;
            #[doc = "Park on master port M5"]
            pub const MASTER_PORT_5: u32 = 0x05;
            #[doc = "Park on master port M6"]
            pub const MASTER_PORT_6: u32 = 0x06;
            #[doc = "Park on master port M7"]
            pub const MASTER_PORT_7: u32 = 0x07;
        }
    }
    #[doc = "Parking Control"]
    pub mod PCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
            pub const PARK: u32 = 0;
            #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
            pub const SLAVE_PORT: u32 = 0x01;
            #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
            pub const SAFE_STATE: u32 = 0x02;
        }
    }
    #[doc = "Arbitration Mode"]
    pub mod ARB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Round-robin (rotating) priority"]
            pub const RR: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 0"]
    pub mod HPE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
            pub const M0: u32 = 0;
            #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
            pub const M1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 1"]
    pub mod HPE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
            pub const HPE1: u32 = 0;
            #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
            pub const HPE1_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 2"]
    pub mod HPE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
            pub const HPE2: u32 = 0;
            #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
            pub const HPE2_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 3"]
    pub mod HPE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
            pub const HPE3: u32 = 0;
            #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
            pub const HPE3_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 4"]
    pub mod HPE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
            pub const HPE4: u32 = 0;
            #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
            pub const HPE4_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 5"]
    pub mod HPE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
            pub const HPE5: u32 = 0;
            #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
            pub const HPE5_1: u32 = 0x01;
        }
    }
    #[doc = "Read Only"]
    pub mod RO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRSn and PRSn registers are writeable"]
            pub const CRS_PRS_Y: u32 = 0;
            #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
            pub const CRS_PRS_N: u32 = 0x01;
        }
    }
}
#[doc = "Priority Slave Registers"]
pub mod PRS1 {
    #[doc = "Master 0 Priority"]
    pub mod M0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 1 Priority"]
    pub mod M1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 2 Priority"]
    pub mod M2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 3 Priority"]
    pub mod M3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 4 Priority"]
    pub mod M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 5 Priority"]
    pub mod M5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
}
#[doc = "Control Register"]
pub mod CRS1 {
    #[doc = "Park"]
    pub mod PARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Park on master port M0"]
            pub const MASTER_PORT_0: u32 = 0;
            #[doc = "Park on master port M1"]
            pub const MASTER_PORT_1: u32 = 0x01;
            #[doc = "Park on master port M2"]
            pub const MASTER_PORT_2: u32 = 0x02;
            #[doc = "Park on master port M3"]
            pub const MASTER_PORT_3: u32 = 0x03;
            #[doc = "Park on master port M4"]
            pub const MASTER_PORT_4: u32 = 0x04;
            #[doc = "Park on master port M5"]
            pub const MASTER_PORT_5: u32 = 0x05;
            #[doc = "Park on master port M6"]
            pub const MASTER_PORT_6: u32 = 0x06;
            #[doc = "Park on master port M7"]
            pub const MASTER_PORT_7: u32 = 0x07;
        }
    }
    #[doc = "Parking Control"]
    pub mod PCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
            pub const PARK: u32 = 0;
            #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
            pub const SLAVE_PORT: u32 = 0x01;
            #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
            pub const SAFE_STATE: u32 = 0x02;
        }
    }
    #[doc = "Arbitration Mode"]
    pub mod ARB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Round-robin (rotating) priority"]
            pub const RR: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 0"]
    pub mod HPE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
            pub const M0: u32 = 0;
            #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
            pub const M1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 1"]
    pub mod HPE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
            pub const HPE1: u32 = 0;
            #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
            pub const HPE1_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 2"]
    pub mod HPE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
            pub const HPE2: u32 = 0;
            #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
            pub const HPE2_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 3"]
    pub mod HPE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
            pub const HPE3: u32 = 0;
            #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
            pub const HPE3_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 4"]
    pub mod HPE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
            pub const HPE4: u32 = 0;
            #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
            pub const HPE4_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 5"]
    pub mod HPE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
            pub const HPE5: u32 = 0;
            #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
            pub const HPE5_1: u32 = 0x01;
        }
    }
    #[doc = "Read Only"]
    pub mod RO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRSn and PRSn registers are writeable"]
            pub const CRS_PRS_Y: u32 = 0;
            #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
            pub const CRS_PRS_N: u32 = 0x01;
        }
    }
}
#[doc = "Priority Slave Registers"]
pub mod PRS2 {
    #[doc = "Master 0 Priority"]
    pub mod M0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 1 Priority"]
    pub mod M1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 2 Priority"]
    pub mod M2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 3 Priority"]
    pub mod M3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 4 Priority"]
    pub mod M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 5 Priority"]
    pub mod M5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
}
#[doc = "Control Register"]
pub mod CRS2 {
    #[doc = "Park"]
    pub mod PARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Park on master port M0"]
            pub const MASTER_PORT_0: u32 = 0;
            #[doc = "Park on master port M1"]
            pub const MASTER_PORT_1: u32 = 0x01;
            #[doc = "Park on master port M2"]
            pub const MASTER_PORT_2: u32 = 0x02;
            #[doc = "Park on master port M3"]
            pub const MASTER_PORT_3: u32 = 0x03;
            #[doc = "Park on master port M4"]
            pub const MASTER_PORT_4: u32 = 0x04;
            #[doc = "Park on master port M5"]
            pub const MASTER_PORT_5: u32 = 0x05;
            #[doc = "Park on master port M6"]
            pub const MASTER_PORT_6: u32 = 0x06;
            #[doc = "Park on master port M7"]
            pub const MASTER_PORT_7: u32 = 0x07;
        }
    }
    #[doc = "Parking Control"]
    pub mod PCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
            pub const PARK: u32 = 0;
            #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
            pub const SLAVE_PORT: u32 = 0x01;
            #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
            pub const SAFE_STATE: u32 = 0x02;
        }
    }
    #[doc = "Arbitration Mode"]
    pub mod ARB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Round-robin (rotating) priority"]
            pub const RR: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 0"]
    pub mod HPE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
            pub const M0: u32 = 0;
            #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
            pub const M1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 1"]
    pub mod HPE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
            pub const HPE1: u32 = 0;
            #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
            pub const HPE1_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 2"]
    pub mod HPE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
            pub const HPE2: u32 = 0;
            #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
            pub const HPE2_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 3"]
    pub mod HPE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
            pub const HPE3: u32 = 0;
            #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
            pub const HPE3_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 4"]
    pub mod HPE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
            pub const HPE4: u32 = 0;
            #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
            pub const HPE4_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 5"]
    pub mod HPE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
            pub const HPE5: u32 = 0;
            #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
            pub const HPE5_1: u32 = 0x01;
        }
    }
    #[doc = "Read Only"]
    pub mod RO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRSn and PRSn registers are writeable"]
            pub const CRS_PRS_Y: u32 = 0;
            #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
            pub const CRS_PRS_N: u32 = 0x01;
        }
    }
}
#[doc = "Priority Slave Registers"]
pub mod PRS3 {
    #[doc = "Master 0 Priority"]
    pub mod M0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 1 Priority"]
    pub mod M1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 2 Priority"]
    pub mod M2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 3 Priority"]
    pub mod M3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 4 Priority"]
    pub mod M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 5 Priority"]
    pub mod M5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
}
#[doc = "Control Register"]
pub mod CRS3 {
    #[doc = "Park"]
    pub mod PARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Park on master port M0"]
            pub const MASTER_PORT_0: u32 = 0;
            #[doc = "Park on master port M1"]
            pub const MASTER_PORT_1: u32 = 0x01;
            #[doc = "Park on master port M2"]
            pub const MASTER_PORT_2: u32 = 0x02;
            #[doc = "Park on master port M3"]
            pub const MASTER_PORT_3: u32 = 0x03;
            #[doc = "Park on master port M4"]
            pub const MASTER_PORT_4: u32 = 0x04;
            #[doc = "Park on master port M5"]
            pub const MASTER_PORT_5: u32 = 0x05;
            #[doc = "Park on master port M6"]
            pub const MASTER_PORT_6: u32 = 0x06;
            #[doc = "Park on master port M7"]
            pub const MASTER_PORT_7: u32 = 0x07;
        }
    }
    #[doc = "Parking Control"]
    pub mod PCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
            pub const PARK: u32 = 0;
            #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
            pub const SLAVE_PORT: u32 = 0x01;
            #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
            pub const SAFE_STATE: u32 = 0x02;
        }
    }
    #[doc = "Arbitration Mode"]
    pub mod ARB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Round-robin (rotating) priority"]
            pub const RR: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 0"]
    pub mod HPE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
            pub const M0: u32 = 0;
            #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
            pub const M1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 1"]
    pub mod HPE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
            pub const HPE1: u32 = 0;
            #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
            pub const HPE1_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 2"]
    pub mod HPE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
            pub const HPE2: u32 = 0;
            #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
            pub const HPE2_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 3"]
    pub mod HPE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
            pub const HPE3: u32 = 0;
            #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
            pub const HPE3_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 4"]
    pub mod HPE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
            pub const HPE4: u32 = 0;
            #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
            pub const HPE4_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 5"]
    pub mod HPE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
            pub const HPE5: u32 = 0;
            #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
            pub const HPE5_1: u32 = 0x01;
        }
    }
    #[doc = "Read Only"]
    pub mod RO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRSn and PRSn registers are writeable"]
            pub const CRS_PRS_Y: u32 = 0;
            #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
            pub const CRS_PRS_N: u32 = 0x01;
        }
    }
}
#[doc = "Priority Slave Registers"]
pub mod PRS4 {
    #[doc = "Master 0 Priority"]
    pub mod M0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 1 Priority"]
    pub mod M1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 2 Priority"]
    pub mod M2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 3 Priority"]
    pub mod M3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 4 Priority"]
    pub mod M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 5 Priority"]
    pub mod M5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
}
#[doc = "Control Register"]
pub mod CRS4 {
    #[doc = "Park"]
    pub mod PARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Park on master port M0"]
            pub const MASTER_PORT_0: u32 = 0;
            #[doc = "Park on master port M1"]
            pub const MASTER_PORT_1: u32 = 0x01;
            #[doc = "Park on master port M2"]
            pub const MASTER_PORT_2: u32 = 0x02;
            #[doc = "Park on master port M3"]
            pub const MASTER_PORT_3: u32 = 0x03;
            #[doc = "Park on master port M4"]
            pub const MASTER_PORT_4: u32 = 0x04;
            #[doc = "Park on master port M5"]
            pub const MASTER_PORT_5: u32 = 0x05;
            #[doc = "Park on master port M6"]
            pub const MASTER_PORT_6: u32 = 0x06;
            #[doc = "Park on master port M7"]
            pub const MASTER_PORT_7: u32 = 0x07;
        }
    }
    #[doc = "Parking Control"]
    pub mod PCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
            pub const PARK: u32 = 0;
            #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
            pub const SLAVE_PORT: u32 = 0x01;
            #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
            pub const SAFE_STATE: u32 = 0x02;
        }
    }
    #[doc = "Arbitration Mode"]
    pub mod ARB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Round-robin (rotating) priority"]
            pub const RR: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 0"]
    pub mod HPE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
            pub const M0: u32 = 0;
            #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
            pub const M1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 1"]
    pub mod HPE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
            pub const HPE1: u32 = 0;
            #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
            pub const HPE1_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 2"]
    pub mod HPE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
            pub const HPE2: u32 = 0;
            #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
            pub const HPE2_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 3"]
    pub mod HPE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
            pub const HPE3: u32 = 0;
            #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
            pub const HPE3_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 4"]
    pub mod HPE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
            pub const HPE4: u32 = 0;
            #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
            pub const HPE4_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 5"]
    pub mod HPE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
            pub const HPE5: u32 = 0;
            #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
            pub const HPE5_1: u32 = 0x01;
        }
    }
    #[doc = "Read Only"]
    pub mod RO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRSn and PRSn registers are writeable"]
            pub const CRS_PRS_Y: u32 = 0;
            #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
            pub const CRS_PRS_N: u32 = 0x01;
        }
    }
}
#[doc = "Priority Slave Registers"]
pub mod PRS5 {
    #[doc = "Master 0 Priority"]
    pub mod M0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 1 Priority"]
    pub mod M1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 2 Priority"]
    pub mod M2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 3 Priority"]
    pub mod M3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 4 Priority"]
    pub mod M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 5 Priority"]
    pub mod M5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
}
#[doc = "Control Register"]
pub mod CRS5 {
    #[doc = "Park"]
    pub mod PARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Park on master port M0"]
            pub const MASTER_PORT_0: u32 = 0;
            #[doc = "Park on master port M1"]
            pub const MASTER_PORT_1: u32 = 0x01;
            #[doc = "Park on master port M2"]
            pub const MASTER_PORT_2: u32 = 0x02;
            #[doc = "Park on master port M3"]
            pub const MASTER_PORT_3: u32 = 0x03;
            #[doc = "Park on master port M4"]
            pub const MASTER_PORT_4: u32 = 0x04;
            #[doc = "Park on master port M5"]
            pub const MASTER_PORT_5: u32 = 0x05;
            #[doc = "Park on master port M6"]
            pub const MASTER_PORT_6: u32 = 0x06;
            #[doc = "Park on master port M7"]
            pub const MASTER_PORT_7: u32 = 0x07;
        }
    }
    #[doc = "Parking Control"]
    pub mod PCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
            pub const PARK: u32 = 0;
            #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
            pub const SLAVE_PORT: u32 = 0x01;
            #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
            pub const SAFE_STATE: u32 = 0x02;
        }
    }
    #[doc = "Arbitration Mode"]
    pub mod ARB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Round-robin (rotating) priority"]
            pub const RR: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 0"]
    pub mod HPE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
            pub const M0: u32 = 0;
            #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
            pub const M1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 1"]
    pub mod HPE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
            pub const HPE1: u32 = 0;
            #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
            pub const HPE1_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 2"]
    pub mod HPE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
            pub const HPE2: u32 = 0;
            #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
            pub const HPE2_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 3"]
    pub mod HPE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
            pub const HPE3: u32 = 0;
            #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
            pub const HPE3_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 4"]
    pub mod HPE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
            pub const HPE4: u32 = 0;
            #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
            pub const HPE4_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 5"]
    pub mod HPE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
            pub const HPE5: u32 = 0;
            #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
            pub const HPE5_1: u32 = 0x01;
        }
    }
    #[doc = "Read Only"]
    pub mod RO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRSn and PRSn registers are writeable"]
            pub const CRS_PRS_Y: u32 = 0;
            #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
            pub const CRS_PRS_N: u32 = 0x01;
        }
    }
}
#[doc = "Priority Slave Registers"]
pub mod PRS6 {
    #[doc = "Master 0 Priority"]
    pub mod M0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 1 Priority"]
    pub mod M1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 2 Priority"]
    pub mod M2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 3 Priority"]
    pub mod M3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 4 Priority"]
    pub mod M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 5 Priority"]
    pub mod M5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
}
#[doc = "Control Register"]
pub mod CRS6 {
    #[doc = "Park"]
    pub mod PARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Park on master port M0"]
            pub const MASTER_PORT_0: u32 = 0;
            #[doc = "Park on master port M1"]
            pub const MASTER_PORT_1: u32 = 0x01;
            #[doc = "Park on master port M2"]
            pub const MASTER_PORT_2: u32 = 0x02;
            #[doc = "Park on master port M3"]
            pub const MASTER_PORT_3: u32 = 0x03;
            #[doc = "Park on master port M4"]
            pub const MASTER_PORT_4: u32 = 0x04;
            #[doc = "Park on master port M5"]
            pub const MASTER_PORT_5: u32 = 0x05;
            #[doc = "Park on master port M6"]
            pub const MASTER_PORT_6: u32 = 0x06;
            #[doc = "Park on master port M7"]
            pub const MASTER_PORT_7: u32 = 0x07;
        }
    }
    #[doc = "Parking Control"]
    pub mod PCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
            pub const PARK: u32 = 0;
            #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
            pub const SLAVE_PORT: u32 = 0x01;
            #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
            pub const SAFE_STATE: u32 = 0x02;
        }
    }
    #[doc = "Arbitration Mode"]
    pub mod ARB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Round-robin (rotating) priority"]
            pub const RR: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 0"]
    pub mod HPE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
            pub const M0: u32 = 0;
            #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
            pub const M1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 1"]
    pub mod HPE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
            pub const HPE1: u32 = 0;
            #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
            pub const HPE1_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 2"]
    pub mod HPE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
            pub const HPE2: u32 = 0;
            #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
            pub const HPE2_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 3"]
    pub mod HPE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
            pub const HPE3: u32 = 0;
            #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
            pub const HPE3_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 4"]
    pub mod HPE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
            pub const HPE4: u32 = 0;
            #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
            pub const HPE4_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 5"]
    pub mod HPE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
            pub const HPE5: u32 = 0;
            #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
            pub const HPE5_1: u32 = 0x01;
        }
    }
    #[doc = "Read Only"]
    pub mod RO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRSn and PRSn registers are writeable"]
            pub const CRS_PRS_Y: u32 = 0;
            #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
            pub const CRS_PRS_N: u32 = 0x01;
        }
    }
}
#[doc = "Priority Slave Registers"]
pub mod PRS7 {
    #[doc = "Master 0 Priority"]
    pub mod M0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 1 Priority"]
    pub mod M1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 2 Priority"]
    pub mod M2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 3 Priority"]
    pub mod M3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 4 Priority"]
    pub mod M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
    #[doc = "Master 5 Priority"]
    pub mod M5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This master has level 1 or highest priority when accessing the slave port."]
            pub const SLAVE_PORT_1: u32 = 0;
            #[doc = "This master has level 2 priority when accessing the slave port."]
            pub const SLAVE_PORT_2: u32 = 0x01;
            #[doc = "This master has level 3 priority when accessing the slave port."]
            pub const SLAVE_PORT_3: u32 = 0x02;
            #[doc = "This master has level 4 priority when accessing the slave port."]
            pub const SLAVE_PORT_4: u32 = 0x03;
            #[doc = "This master has level 5 priority when accessing the slave port."]
            pub const SLAVE_PORT_5: u32 = 0x04;
            #[doc = "This master has level 6 priority when accessing the slave port."]
            pub const SLAVE_PORT_6: u32 = 0x05;
            #[doc = "This master has level 7 priority when accessing the slave port."]
            pub const SLAVE_PORT_7: u32 = 0x06;
            #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
            pub const SLAVE_PORT_8: u32 = 0x07;
        }
    }
}
#[doc = "Control Register"]
pub mod CRS7 {
    #[doc = "Park"]
    pub mod PARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Park on master port M0"]
            pub const MASTER_PORT_0: u32 = 0;
            #[doc = "Park on master port M1"]
            pub const MASTER_PORT_1: u32 = 0x01;
            #[doc = "Park on master port M2"]
            pub const MASTER_PORT_2: u32 = 0x02;
            #[doc = "Park on master port M3"]
            pub const MASTER_PORT_3: u32 = 0x03;
            #[doc = "Park on master port M4"]
            pub const MASTER_PORT_4: u32 = 0x04;
            #[doc = "Park on master port M5"]
            pub const MASTER_PORT_5: u32 = 0x05;
            #[doc = "Park on master port M6"]
            pub const MASTER_PORT_6: u32 = 0x06;
            #[doc = "Park on master port M7"]
            pub const MASTER_PORT_7: u32 = 0x07;
        }
    }
    #[doc = "Parking Control"]
    pub mod PCTL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
            pub const PARK: u32 = 0;
            #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
            pub const SLAVE_PORT: u32 = 0x01;
            #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
            pub const SAFE_STATE: u32 = 0x02;
        }
    }
    #[doc = "Arbitration Mode"]
    pub mod ARB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Round-robin (rotating) priority"]
            pub const RR: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 0"]
    pub mod HPE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
            pub const M0: u32 = 0;
            #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
            pub const M1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 1"]
    pub mod HPE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
            pub const HPE1: u32 = 0;
            #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
            pub const HPE1_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 2"]
    pub mod HPE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
            pub const HPE2: u32 = 0;
            #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
            pub const HPE2_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 3"]
    pub mod HPE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
            pub const HPE3: u32 = 0;
            #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
            pub const HPE3_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 4"]
    pub mod HPE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
            pub const HPE4: u32 = 0;
            #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
            pub const HPE4_1: u32 = 0x01;
        }
    }
    #[doc = "High Priority Elevation 5"]
    pub mod HPE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
            pub const HPE5: u32 = 0;
            #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
            pub const HPE5_1: u32 = 0x01;
        }
    }
    #[doc = "Read Only"]
    pub mod RO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CRSn and PRSn registers are writeable"]
            pub const CRS_PRS_Y: u32 = 0;
            #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
            pub const CRS_PRS_N: u32 = 0x01;
        }
    }
}
