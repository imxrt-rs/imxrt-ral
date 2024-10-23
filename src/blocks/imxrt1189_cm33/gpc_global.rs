#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "GPC Global Authentication Control"]
    pub AUTHEN_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "GPC CPU0 domain assignment"]
    pub GPC_CPU0_DOMAIN: crate::RWRegister<u32>,
    #[doc = "GPC CPU1 domain assignment"]
    pub GPC_CPU1_DOMAIN: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "GPC master CPU configuration"]
    pub GPC_MASTER: crate::RWRegister<u32>,
    _reserved3: [u8; 0x01d8],
    #[doc = "RCOSC control"]
    pub GPC_ROSC_CTRL: crate::RWRegister<u32>,
}
#[doc = "GPC Global Authentication Control"]
pub mod AUTHEN_CTRL {
    #[doc = "Configuration lock"]
    pub mod LOCK_CFG {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The value of low power configuration fields are not locked."]
            pub const B0: u32 = 0;
            #[doc = "The value of low power configuration fields are locked. Refer to the function field of each gpc_global registers."]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow only privilege mode to access CPU mode control registers"]
            pub const B0: u32 = 0;
            #[doc = "Allow both privilege and user mode to access CPU mode control registers"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow only secure mode to access CPU mode registers"]
            pub const B0: u32 = 0;
            #[doc = "Allow both secure and non-secure mode to access CPU mode control registers."]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NONSECURE and USER fields are not locked"]
            pub const B0: u32 = 0;
            #[doc = "NONSECURE and USER fields are locked"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WHITE_LIST is not locked"]
            pub const B0: u32 = 0;
            #[doc = "WHITE_LIST is locked"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPC CPU0 domain assignment"]
pub mod GPC_CPU0_DOMAIN {
    #[doc = "CPU0 domain assignment"]
    pub mod CPU0_DOMAIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPC CPU1 domain assignment"]
pub mod GPC_CPU1_DOMAIN {
    #[doc = "CPU1 domain assignment"]
    pub mod CPU1_DOMAIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPC master CPU configuration"]
pub mod GPC_MASTER {
    #[doc = "Setting to 1 means CPU0 is the master CPU of its domain"]
    pub mod CPU0_MASTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CPU0 is not the master CPU of its domain"]
            pub const B0: u32 = 0;
            #[doc = "CPU0 is the master CPU of its domain"]
            pub const B1: u32 = 0x01;
        }
    }
    #[doc = "Setting to 1 means CPU1 is the master CPU of its domain"]
    pub mod CPU1_MASTER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CPU1 is not the master CPU of its domain"]
            pub const B0: u32 = 0;
            #[doc = "CPU1 is the master CPU of its domain"]
            pub const B1: u32 = 0x01;
        }
    }
}
#[doc = "RCOSC control"]
pub mod GPC_ROSC_CTRL {
    #[doc = "Shut off the 24 MHz RCOSC clock when system sleep"]
    pub mod ROSC_OFF_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep 24 MHz ROSC clock running during system sleep"]
            pub const B0: u32 = 0;
            #[doc = "Shut off 24 MHz ROSC clock during system sleep"]
            pub const B1: u32 = 0x01;
        }
    }
}
