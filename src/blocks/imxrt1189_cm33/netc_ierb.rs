#[doc = "NETC Integrated Endpoint Register Block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Capability register 0"]
    pub CAPR0: crate::RORegister<u32>,
    #[doc = "Capability register 1"]
    pub CAPR1: crate::RORegister<u32>,
    #[doc = "Capability register 2"]
    pub CAPR2: crate::RORegister<u32>,
    #[doc = "Capability register 3"]
    pub CAPR3: crate::RORegister<u32>,
    _reserved0: [u8; 0x10],
    #[doc = "Common memory capability register"]
    pub CMCAPR: crate::RORegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "Ingress port filter ternary memory capability register"]
    pub IPFTMCAPR: crate::RORegister<u32>,
    _reserved2: [u8; 0x10],
    #[doc = "Time gate scheduling memory capability register"]
    pub TGSMCAPR: crate::RORegister<u32>,
    _reserved3: [u8; 0x38],
    #[doc = "Shared memory depletion threshold register"]
    pub SMDTR: crate::RWRegister<u32>,
    #[doc = "ENETC receive shared memory buffer allotment register"]
    pub ERSMBAR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x38],
    #[doc = "HTA 0 HP configuration register"]
    pub HTA0HPCR: crate::RWRegister<u32>,
    #[doc = "HTA 0 LP configuration register"]
    pub HTA0LPCR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x38],
    #[doc = "Hash bucket table memory allocation register"]
    pub HBTMAR: crate::RWRegister<u32>,
    #[doc = "Hash bucket table configuration register"]
    pub HBTCR: crate::RWRegister<u32>,
    #[doc = "Guaranteed hash table entry memory capability register"]
    pub GHTEMCAPR: crate::RORegister<u32>,
    _reserved6: [u8; 0x64],
    #[doc = "NETC FLR configuration register"]
    pub NETCFLRCR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "NETC clock period fractional register"]
    pub NETCCLKFR: crate::RWRegister<u32>,
    #[doc = "NETC clock configuration register"]
    pub NETCCLKCR: crate::RWRegister<u32>,
    #[doc = "System bus configuration register"]
    pub SBCR: crate::RWRegister<u32>,
    #[doc = "System bus outstanding transaction control register"]
    pub SBOTCR: crate::RWRegister<u32>,
    _reserved8: [u8; 0x08],
    #[doc = "Stream gating lag time for refresh register"]
    pub SGLTTR: crate::RWRegister<u32>,
    _reserved9: [u8; 0x6c],
    #[doc = "Root complex 0 binding configuration register"]
    pub R0BCR: crate::RORegister<u32>,
    _reserved10: [u8; 0x04],
    #[doc = "Root complex 0 MSI-X cache attribute register"]
    pub RC0MSICAR: crate::RWRegister<u32>,
    #[doc = "Root complex 0 MSI access management qualifier register"]
    pub RC0MSIAMQR: crate::RWRegister<u32>,
    _reserved11: [u8; 0xf0],
    #[doc = "EMDIO binding configuration register"]
    pub EMDIOBCR: crate::RORegister<u32>,
    _reserved12: [u8; 0x10],
    #[doc = "EMDIO MSI-X configuration register"]
    pub EMDIOMCR: crate::RORegister<u32>,
    _reserved13: [u8; 0x08],
    #[doc = "EMDIO config header device ID and vendor ID register"]
    pub EMDIO_CFH_DIDVID: crate::RWRegister<u32>,
    #[doc = "EMDIO config header subsystem ID and subsystem vendor ID register"]
    pub EMDIO_CFH_SIDSVID: crate::RWRegister<u32>,
    _reserved14: [u8; 0x20],
    #[doc = "EMDIO boot loader parameter register a"]
    pub EMDIOBLPR: [crate::RWRegister<u32>; 2usize],
    #[doc = "EMDIO configuration register"]
    pub EMDIO_CFG: crate::RWRegister<u32>,
    _reserved15: [u8; 0xac],
    #[doc = "Timer 0 binding configuration register"]
    pub T0BCR: crate::RORegister<u32>,
    _reserved16: [u8; 0x10],
    #[doc = "Timer 0 MSI-X configuration register"]
    pub T0MCR: crate::RWRegister<u32>,
    _reserved17: [u8; 0x08],
    #[doc = "Timer 0 config header device ID and vendor ID register"]
    pub T0_CFH_DIDVID: crate::RWRegister<u32>,
    #[doc = "Timer 0 config header subsystem ID and subsystem vendor ID register"]
    pub T0_CFH_SIDSVID: crate::RWRegister<u32>,
    _reserved18: [u8; 0x20],
    #[doc = "Timer 0 boot loader parameter register 0"]
    pub T0BLPR0: crate::RWRegister<u32>,
    #[doc = "Timer 0 boot loader parameter register 1"]
    pub T0BLPR1: crate::RWRegister<u32>,
    _reserved19: [u8; 0x0bb0],
    #[doc = "Link 0 capability register"]
    pub L0CAPR: crate::RORegister<u32>,
    #[doc = "Link 0 MAC capability register"]
    pub L0MCAPR: crate::RORegister<u32>,
    #[doc = "Link 0 I/O capability register"]
    pub L0IOCAPR: crate::RORegister<u32>,
    _reserved20: [u8; 0x04],
    #[doc = "Link 0 binding configuration register"]
    pub L0BCR: crate::RWRegister<u32>,
    #[doc = "Link 0 transmit byte credit comfort threshold register"]
    pub L0TXBCCTR: crate::RWRegister<u32>,
    _reserved21: [u8; 0x08],
    #[doc = "Link 0 end 0 MAC address register 0"]
    pub L0E0MAR0: crate::RWRegister<u32>,
    #[doc = "Link 0 end 0 MAC address register 1"]
    pub L0E0MAR1: crate::RWRegister<u32>,
    _reserved22: [u8; 0x18],
    #[doc = "Link 1 capability register"]
    pub L1CAPR: crate::RORegister<u32>,
    #[doc = "Link 1 MAC capability register"]
    pub L1MCAPR: crate::RORegister<u32>,
    #[doc = "Link 1 I/O capability register"]
    pub L1IOCAPR: crate::RORegister<u32>,
    _reserved23: [u8; 0x04],
    #[doc = "Link 1 binding configuration register"]
    pub L1BCR: crate::RWRegister<u32>,
    #[doc = "Link 1 transmit byte credit comfort threshold register"]
    pub L1TXBCCTR: crate::RWRegister<u32>,
    _reserved24: [u8; 0x08],
    #[doc = "Link 1 end 0 MAC address register 0"]
    pub L1E0MAR0: crate::RWRegister<u32>,
    #[doc = "Link 1 end 0 MAC address register 1"]
    pub L1E0MAR1: crate::RWRegister<u32>,
    _reserved25: [u8; 0x18],
    #[doc = "Link 2 capability register"]
    pub L2CAPR: crate::RORegister<u32>,
    #[doc = "Link 2 MAC capability register"]
    pub L2MCAPR: crate::RORegister<u32>,
    #[doc = "Link 2 I/O capability register"]
    pub L2IOCAPR: crate::RORegister<u32>,
    _reserved26: [u8; 0x04],
    #[doc = "Link 2 binding configuration register"]
    pub L2BCR: crate::RWRegister<u32>,
    #[doc = "Link 2 transmit byte credit comfort threshold register"]
    pub L2TXBCCTR: crate::RWRegister<u32>,
    _reserved27: [u8; 0x08],
    #[doc = "Link 2 end 0 MAC address register 0"]
    pub L2E0MAR0: crate::RWRegister<u32>,
    #[doc = "Link 2 end 0 MAC address register 1"]
    pub L2E0MAR1: crate::RWRegister<u32>,
    _reserved28: [u8; 0x18],
    #[doc = "Link 3 capability register"]
    pub L3CAPR: crate::RORegister<u32>,
    #[doc = "Link 3 MAC capability register"]
    pub L3MCAPR: crate::RORegister<u32>,
    #[doc = "Link 3 I/O capability register"]
    pub L3IOCAPR: crate::RORegister<u32>,
    _reserved29: [u8; 0x04],
    #[doc = "Link 3 binding configuration register"]
    pub L3BCR: crate::RWRegister<u32>,
    #[doc = "Link 3 transmit byte credit comfort threshold register"]
    pub L3TXBCCTR: crate::RWRegister<u32>,
    _reserved30: [u8; 0x08],
    #[doc = "Link 3 end 0 MAC address register 0"]
    pub L3E0MAR0: crate::RWRegister<u32>,
    #[doc = "Link 3 end 0 MAC address register 1"]
    pub L3E0MAR1: crate::RWRegister<u32>,
    _reserved31: [u8; 0x18],
    #[doc = "Link 4 capability register"]
    pub L4CAPR: crate::RORegister<u32>,
    #[doc = "Link 4 MAC capability register"]
    pub L4MCAPR: crate::RORegister<u32>,
    #[doc = "Link 4 I/O capability register"]
    pub L4IOCAPR: crate::RORegister<u32>,
    _reserved32: [u8; 0x04],
    #[doc = "Link 4 binding configuration register"]
    pub L4BCR: crate::RWRegister<u32>,
    #[doc = "Link 4 transmit byte credit comfort threshold register"]
    pub L4TXBCCTR: crate::RWRegister<u32>,
    _reserved33: [u8; 0x08],
    #[doc = "Link 4 end 0 MAC address register 0"]
    pub L4E0MAR0: crate::RWRegister<u32>,
    #[doc = "Link 4 end 0 MAC address register 1"]
    pub L4E0MAR1: crate::RWRegister<u32>,
    _reserved34: [u8; 0x18],
    #[doc = "Link 5 capability register"]
    pub L5CAPR: crate::RORegister<u32>,
    #[doc = "Link 5 MAC capability register"]
    pub L5MCAPR: crate::RORegister<u32>,
    _reserved35: [u8; 0x08],
    #[doc = "Link 5 binding configuration register"]
    pub L5BCR: crate::RORegister<u32>,
    #[doc = "Link 5 transmit byte credit comfort threshold register"]
    pub L5TXBCCTR: crate::RWRegister<u32>,
    _reserved36: [u8; 0x08],
    #[doc = "Link 5 end 0 MAC address register 0"]
    pub L5E0MAR0: crate::RWRegister<u32>,
    #[doc = "Link 5 end 0 MAC address register 1"]
    pub L5E0MAR1: crate::RWRegister<u32>,
    #[doc = "Link 5 end 1 MAC address register 0"]
    pub L5E1MAR0: crate::RWRegister<u32>,
    #[doc = "Link 5 end 1 MAC address register 1"]
    pub L5E1MAR1: crate::RWRegister<u32>,
    _reserved37: [u8; 0x0e90],
    #[doc = "Switch 0 binding configuration register"]
    pub S0BCR: crate::RORegister<u32>,
    _reserved38: [u8; 0x10],
    #[doc = "Switch 0 MSI-X configuration register"]
    pub S0MCR: crate::RWRegister<u32>,
    _reserved39: [u8; 0x08],
    #[doc = "Switch 0 config header device ID and vendor ID register"]
    pub S0_CFH_DIDVID: crate::RWRegister<u32>,
    #[doc = "Switch 0 config header subsystem ID and subsystem vendor ID register"]
    pub S0_CFH_SIDSVID: crate::RWRegister<u32>,
    _reserved40: [u8; 0x10],
    #[doc = "Switch 0 command cache attribute register"]
    pub S0CCAR: crate::RWRegister<u32>,
    _reserved41: [u8; 0x04],
    #[doc = "Switch 0 access management qualifier register"]
    pub S0AMQR: crate::RWRegister<u32>,
    _reserved42: [u8; 0x04],
    #[doc = "Switch 0 boot loader parameter register 0"]
    pub S0BLPR0: crate::RWRegister<u32>,
    #[doc = "Switch 0 boot loader parameter register 1"]
    pub S0BLPR1: crate::RWRegister<u32>,
    _reserved43: [u8; 0x10],
    #[doc = "Switch 0 shared memory buffer allotment register"]
    pub S0SMBAR: crate::RWRegister<u32>,
    _reserved44: [u8; 0x1c],
    #[doc = "Switch 0 hash table memory allotment register"]
    pub S0HTMAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 index table memory allocation register"]
    pub S0ITMAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 ingress port filter table memory allocation register"]
    pub S0IPFTMAR: crate::RWRegister<u32>,
    _reserved45: [u8; 0x14],
    #[doc = "Switch 0 rate policer index table memory allocation register"]
    pub S0RPITMAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 ingress stream counter index table memory allocation register"]
    pub S0ISCITMAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 ingress stream index table memory allocation register"]
    pub S0ISITMAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 ingress sequence generation index table memory allocation register"]
    pub S0ISQGITMAR: crate::RWRegister<u32>,
    _reserved46: [u8; 0x04],
    #[doc = "Switch 0 stream gate instance index table memory allocation register"]
    pub S0SGIITMAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 stream gate control list index table memory allocation register"]
    pub S0SGCLITMAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 frame modification index table memory allocation register"]
    pub S0FMITMAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 frame modification data index table memory allocation register"]
    pub S0FMDITMAR: crate::RWRegister<u32>,
    _reserved47: [u8; 0x2c],
    #[doc = "Switch 0 time gate scheduling table allocation register"]
    pub S0TGSTAR: crate::RWRegister<u32>,
    #[doc = "Switch 0 time gate scheduling lookahead register"]
    pub S0TGSLR: crate::RWRegister<u32>,
    _reserved48: [u8; 0x010c],
    #[doc = "Switch 0 management port configuration register"]
    pub S0MPCR: crate::RORegister<u32>,
    _reserved49: [u8; 0x08],
    #[doc = "Switch 0 VLAN Filter (hash) table default entry configuration registers 0"]
    pub S0VFHTDECR0: crate::RWRegister<u32>,
    #[doc = "Switch 0 VLAN filter hash table default entry configuration registers 1"]
    pub S0VFHTDECR1: crate::RWRegister<u32>,
    #[doc = "Switch 0 VLAN filter hash table default entry configuration registers 2"]
    pub S0VFHTDECR2: crate::RWRegister<u32>,
    _reserved50: [u8; 0x0de4],
    #[doc = "ENETC 0 binding configuration register 0"]
    pub E0BCR0: crate::RORegister<u32>,
    #[doc = "ENETC 0 binding configuration register 1"]
    pub E0BCR1: crate::RORegister<u32>,
    #[doc = "ENETC 0 binding configuration register 2"]
    pub E0BCR2: crate::RORegister<u32>,
    _reserved51: [u8; 0x04],
    #[doc = "ENETC 0 VSI binding configuration register"]
    pub E0VBCR: crate::RORegister<u32>,
    #[doc = "ENETC 0 MSI-X configuration register"]
    pub E0MCR: crate::RWRegister<u32>,
    _reserved52: [u8; 0x08],
    #[doc = "ENETC 0 config header device ID and vendor ID register"]
    pub E0_CFH_DIDVID: crate::RWRegister<u32>,
    #[doc = "ENETC 0 config header subsystem ID and subsystem vendor ID register"]
    pub E0_CFH_SIDSVID: crate::RWRegister<u32>,
    #[doc = "ENETC 0 config capability VF device ID register"]
    pub E0_CFC_VFDID: crate::RWRegister<u32>,
    _reserved53: [u8; 0x04],
    #[doc = "ENETC 0 buffer cache attribute register 0"]
    pub E0BCAR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 message cache attribute register"]
    pub E0MCAR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 command cache attribute register"]
    pub E0CAR: crate::RWRegister<u32>,
    _reserved54: [u8; 0x04],
    #[doc = "ENETC 0 access management qualifier register"]
    pub E0AMQR: crate::RWRegister<u32>,
    _reserved55: [u8; 0x04],
    #[doc = "ENETC 0 boot loader parameter register 0"]
    pub E0BLPR0: crate::RWRegister<u32>,
    #[doc = "ENETC 0 boot loader parameter register 1"]
    pub E0BLPR1: crate::RWRegister<u32>,
    #[doc = "ENETC 0 receive memory buffer entitlement register"]
    pub E0RXMBER: crate::RWRegister<u32>,
    #[doc = "ENETC 0 receive memory buffer limit register"]
    pub E0RXMBLR: crate::RWRegister<u32>,
    _reserved56: [u8; 0x18],
    #[doc = "ENETC 0 transmit high priority tier byte credit register"]
    pub E0TXHPTBCR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 transmit low priority tier byte credit register"]
    pub E0TXLPTBCR: crate::RWRegister<u32>,
    _reserved57: [u8; 0x08],
    #[doc = "ENETC 0 hash table memory allotment register"]
    pub E0HTMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 index table memory allocation register"]
    pub E0ITMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 ingress port filter table memory allocation register"]
    pub E0IPFTMAR: crate::RWRegister<u32>,
    _reserved58: [u8; 0x14],
    #[doc = "ENETC 0 rate policer index table memory allocation register"]
    pub E0RPITMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 ingress stream counter index table memory allocation register"]
    pub E0ISCITMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 ingress stream index table memory allocation register"]
    pub E0ISITMAR: crate::RWRegister<u32>,
    _reserved59: [u8; 0x08],
    #[doc = "ENETC 0 stream gate instance index table memory allocation register"]
    pub E0SGIITMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 stream gate control list index table memory allocation register"]
    pub E0SGCLITMAR: crate::RWRegister<u32>,
    _reserved60: [u8; 0x34],
    #[doc = "ENETC 0 time gate scheduling table allocation register"]
    pub E0TGSTAR: crate::RWRegister<u32>,
    #[doc = "ENETC 0 time gate scheduling lookahead register"]
    pub E0TGSLR: crate::RWRegister<u32>,
    _reserved61: [u8; 0x08],
    #[doc = "ENETC 1 binding configuration register 0"]
    pub E1BCR0: crate::RORegister<u32>,
    #[doc = "ENETC 1 binding configuration register 1"]
    pub E1BCR1: crate::RORegister<u32>,
    #[doc = "ENETC 1 binding configuration register 2"]
    pub E1BCR2: crate::RORegister<u32>,
    _reserved62: [u8; 0x04],
    #[doc = "ENETC 1 VSI binding configuration register"]
    pub E1VBCR: crate::RORegister<u32>,
    #[doc = "ENETC 1 MSI-X configuration register"]
    pub E1MCR: crate::RWRegister<u32>,
    _reserved63: [u8; 0x08],
    #[doc = "ENETC 1 config header device ID and vendor ID register"]
    pub E1_CFH_DIDVID: crate::RWRegister<u32>,
    #[doc = "ENETC 1 config header subsystem ID and subsystem vendor ID register"]
    pub E1_CFH_SIDSVID: crate::RWRegister<u32>,
    #[doc = "ENETC 1 config capability VF device ID register"]
    pub E1_CFC_VFDID: crate::RWRegister<u32>,
    _reserved64: [u8; 0x04],
    #[doc = "ENETC 1 buffer cache attribute register 0"]
    pub E1BCAR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 message cache attribute register"]
    pub E1MCAR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 command cache attribute register"]
    pub E1CAR: crate::RWRegister<u32>,
    _reserved65: [u8; 0x04],
    #[doc = "ENETC 1 access management qualifier register"]
    pub E1AMQR: crate::RWRegister<u32>,
    _reserved66: [u8; 0x04],
    #[doc = "ENETC 1 boot loader parameter register 0"]
    pub E1BLPR0: crate::RWRegister<u32>,
    #[doc = "ENETC 1 boot loader parameter register 1"]
    pub E1BLPR1: crate::RWRegister<u32>,
    #[doc = "ENETC 1 receive memory buffer entitlement register"]
    pub E1RXMBER: crate::RWRegister<u32>,
    #[doc = "ENETC 1 receive memory buffer limit register"]
    pub E1RXMBLR: crate::RWRegister<u32>,
    _reserved67: [u8; 0x18],
    #[doc = "ENETC 1 transmit high priority tier byte credit register"]
    pub E1TXHPTBCR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 transmit low priority tier byte credit register"]
    pub E1TXLPTBCR: crate::RWRegister<u32>,
    _reserved68: [u8; 0x08],
    #[doc = "ENETC 1 hash table memory allotment register"]
    pub E1HTMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 index table memory allocation register"]
    pub E1ITMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 ingress port filter table memory allocation register"]
    pub E1IPFTMAR: crate::RWRegister<u32>,
    _reserved69: [u8; 0x14],
    #[doc = "ENETC 1 rate policer index table memory allocation register"]
    pub E1RPITMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 ingress stream counter index table memory allocation register"]
    pub E1ISCITMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 ingress stream index table memory allocation register"]
    pub E1ISITMAR: crate::RWRegister<u32>,
    _reserved70: [u8; 0x08],
    #[doc = "ENETC 1 stream gate instance index table memory allocation register"]
    pub E1SGIITMAR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 stream gate control list index table memory allocation register"]
    pub E1SGCLITMAR: crate::RWRegister<u32>,
    _reserved71: [u8; 0x34],
    #[doc = "ENETC 1 time gate scheduling table allocation register"]
    pub E1TGSTAR: crate::RWRegister<u32>,
    #[doc = "ENETC 1 time gate scheduling lookahead register"]
    pub E1TGSLR: crate::RWRegister<u32>,
    _reserved72: [u8; 0x0e08],
    #[doc = "VSI 0 access management qualifier register"]
    pub V0AMQR: crate::RWRegister<u32>,
    _reserved73: [u8; 0x04],
    #[doc = "VSI 0 boot loader parameter register 0"]
    pub V0BLPR0: crate::RWRegister<u32>,
    #[doc = "VSI 0 boot loader parameter register 1"]
    pub V0BLPR1: crate::RWRegister<u32>,
    #[doc = "VSI 0 primary MAC address register 0"]
    pub V0PMAR0: crate::RWRegister<u32>,
    #[doc = "VSI 0 primary MAC address register 1"]
    pub V0PMAR1: crate::RWRegister<u32>,
}
#[doc = "Capability register 0"]
pub mod CAPR0 {
    #[doc = "Number of Root Complexes supported. Range: 0..15"]
    pub mod NUM_RC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of EMDIO instances supported. Range: 0..1"]
    pub mod NUM_EMDIO {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of timer instances supported. Range: 0..2"]
    pub mod NUM_TMR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the number of links supported (internal and external). Range: 0..31"]
    pub mod NUM_LINKS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of switch instances supported. Range: 0..2"]
    pub mod NUM_SW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of ENETC instances supported. Range: 0..23"]
    pub mod NUM_ENETC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Total number of ENETC VSI's instances supported. Range: 0..64"]
    pub mod NUM_VSI {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capability register 1"]
pub mod CAPR1 {
    #[doc = "Total number of receive BD rings supported by NETC. Range: 0..1023"]
    pub mod NUM_RX_BDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Total number of transmit BD rings supported by NETC. Range: 0..1023"]
    pub mod NUM_TX_BDR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capability register 2"]
pub mod CAPR2 {
    #[doc = "Number of MSI-X table entries available for allocation by NETC functions"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capability register 3"]
pub mod CAPR3 {
    #[doc = "Total number of ENETC SI MAC address filter rules supported by NETC."]
    pub mod NUM_MAC_AFTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Total number of ENETC SI VLAN filter rules supported by NETC."]
    pub mod NUM_VLAN_FTE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Common memory capability register"]
pub mod CMCAPR {
    #[doc = "Total amount of common memory in words available to NETC"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word size in bytes 0: 24B 1-3: reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress port filter ternary memory capability register"]
pub mod IPFTMCAPR {
    #[doc = "Total amount of ternary memory in words available to NETC for ingress port filtering"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word size in bits 0: 48 bits 1-3: reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time gate scheduling memory capability register"]
pub mod TGSMCAPR {
    #[doc = "Total amount of Time Gate Scheduling memory in words available to NETC"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shared memory depletion threshold register"]
pub mod SMDTR {
    #[doc = "Shared memory depletion threshold in Words"]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC receive shared memory buffer allotment register"]
pub mod ERSMBAR {
    #[doc = "Threshold in words for internal receive buffer memory used by all ENETC functions"]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HTA 0 HP configuration register"]
pub mod HTA0HPCR {
    #[doc = "HTA global high priority byte limit setting"]
    pub mod BLIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HTA global high priority frame limit"]
    pub mod FLIMIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HTA 0 LP configuration register"]
pub mod HTA0LPCR {
    #[doc = "HTA global low priority byte limit setting"]
    pub mod BLIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HTA global low priority Frame Limit"]
    pub mod FLIMIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash bucket table memory allocation register"]
pub mod HBTMAR {
    #[doc = "Number of words allocated from Common Memory"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minimum number of words required. NUM_WORDS should not be set to a value lower than MIN_WORDS."]
    pub mod MIN_WORDS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of table entries per word."]
    pub mod NEPW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash bucket table configuration register"]
pub mod HBTCR {
    #[doc = "Specifies the maximum EM/IM Hash collisions chain length allowed"]
    pub mod MAX_COL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Specifies the maximum number of Hash Entries Visited during a Search Table Management Command"]
    pub mod MAX_VISITS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Guaranteed hash table entry memory capability register"]
pub mod GHTEMCAPR {
    #[doc = "Total amount of words available to store the guaranteed hash table entries"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NETC FLR configuration register"]
pub mod NETCFLRCR {
    #[doc = "Time duration value expressed in SCALE units."]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Scale"]
    pub mod SCALE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NETC clock period fractional register"]
pub mod NETCCLKFR {
    #[doc = "NETC Clock Period's fractional nanosecond expressed as the number of fractional nanoseconds per NETC clock"]
    pub mod FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NETC clock configuration register"]
pub mod NETCCLKCR {
    #[doc = "Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Period"]
    pub mod PERIOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System bus configuration register"]
pub mod SBCR {
    #[doc = "System Bus Maximum Write Burst Size"]
    pub mod WBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Bus Maximum Read Burst Size"]
    pub mod RBS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System bus outstanding transaction control register"]
pub mod SBOTCR {
    #[doc = "This is a 32-bit read-write field, however only bits 23-16 and 7-0 have any effect on device behavior"]
    pub mod OT_LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stream gating lag time for refresh register"]
pub mod SGLTTR {
    #[doc = "Lag time is a nanosecond value = 2LAG_TIME"]
    pub mod LAG_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Root complex 0 binding configuration register"]
pub mod R0BCR {
    #[doc = "Indicates the type of root complex and routing 0: RCiEP 1: PCIe RC"]
    pub mod TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port number Indicates how to address the PCIe target based on TYPE"]
    pub mod PORT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Root complex 0 MSI-X cache attribute register"]
pub mod RC0MSICAR {
    #[doc = "MSI-X write cache type This is the cache attribute setting used when NETC generates MSI-X events"]
    pub mod MSI_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MSI-X write domain This is the domain attribute setting used when NETC generates MSI-X events"]
    pub mod MSI_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MSI-X write snoop This is the snoop attribute setting used when NETC generates MSI-X events"]
    pub mod MSI_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Root complex 0 MSI access management qualifier register"]
pub mod RC0MSIAMQR {
    #[doc = "Address Write QoS."]
    pub mod AWQOS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    pub mod BMT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EMDIO binding configuration register"]
pub mod EMDIOBCR {
    #[doc = "Root complex instance number."]
    pub mod RC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCI device function number for global EMDIO controller. For assignment of function number, see ."]
    pub mod FN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set, this EMDIO function is valid."]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EMDIO MSI-X configuration register"]
pub mod EMDIOMCR {
    #[doc = "Number of MSI-X vectors supported. Formula: NUM_MSIX+1 Range: 1"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EMDIO config header device ID and vendor ID register"]
pub mod EMDIO_CFH_DIDVID {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    pub mod VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    pub mod DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EMDIO config header subsystem ID and subsystem vendor ID register"]
pub mod EMDIO_CFH_SIDSVID {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    pub mod SUBSYSTEM_VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    pub mod SUBSYSTEM_DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EMDIO boot loader parameter register a"]
pub mod EMDIOBLPR {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EMDIO configuration register"]
pub mod EMDIO_CFG {
    #[doc = "MDIO pin mode 0 Push-pull 1 Open drain"]
    pub mod MDIO_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDC pin mode 0 Push-pull 1 Open drain"]
    pub mod MDC_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer 0 binding configuration register"]
pub mod T0BCR {
    #[doc = "Root complex instance number."]
    pub mod RC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    pub mod FN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set, this timer function is valid."]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer 0 MSI-X configuration register"]
pub mod T0MCR {
    #[doc = "Number of MSI-X vectors supported for timer function"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer 0 config header device ID and vendor ID register"]
pub mod T0_CFH_DIDVID {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    pub mod VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    pub mod DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer 0 config header subsystem ID and subsystem vendor ID register"]
pub mod T0_CFH_SIDSVID {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    pub mod SUBSYSTEM_VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    pub mod SUBSYSTEM_DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer 0 boot loader parameter register 0"]
pub mod T0BLPR0 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer 0 boot loader parameter register 1"]
pub mod T0BLPR1 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 0 capability register"]
pub mod L0CAPR {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    pub mod LINK_TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    pub mod NUM_TC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    pub mod NUM_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of congestion groups supported"]
    pub mod NUM_CG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Gate Scheduling"]
    pub mod TGS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Credit Based Shaping"]
    pub mod CBS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 0 MAC capability register"]
pub mod L0MCAPR {
    #[doc = "MAC Variant"]
    pub mod MAC_VAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress frame padding capability"]
    pub mod EFPAD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configurable preamble/IPG capability"]
    pub mod PIPG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Duplex capability"]
    pub mod HD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if frame preemption is supported"]
    pub mod FP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    pub mod MIN_MPDU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the MII protocol supported"]
    pub mod MII_PROT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 0 I/O capability register"]
pub mod L0IOCAPR {
    #[doc = "PCS protocols supported"]
    pub mod PCS_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Variants supported"]
    pub mod IO_VAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External MDIO supported."]
    pub mod EMDIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RevMII MII rate"]
    pub mod REVMII_RATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reverse Mode Device Configuration"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 0 binding configuration register"]
pub mod L0BCR {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    pub mod SW_PORT_ENETC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Primary link end's NETC Function Type"]
    pub mod NETC_FUNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value indicate an MDIO PHY address"]
    pub mod MDIO_PHYAD_PRTAD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    pub mod SPL_SW_PORT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 0 transmit byte credit comfort threshold register"]
pub mod L0TXBCCTR {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 0 end 0 MAC address register 0"]
pub mod L0E0MAR0 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 0 end 0 MAC address register 1"]
pub mod L0E0MAR1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 1 capability register"]
pub mod L1CAPR {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    pub mod LINK_TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    pub mod NUM_TC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    pub mod NUM_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of congestion groups supported"]
    pub mod NUM_CG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Gate Scheduling"]
    pub mod TGS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Credit Based Shaping"]
    pub mod CBS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 1 MAC capability register"]
pub mod L1MCAPR {
    #[doc = "MAC Variant"]
    pub mod MAC_VAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress frame padding capability"]
    pub mod EFPAD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configurable preamble/IPG capability"]
    pub mod PIPG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Duplex capability"]
    pub mod HD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if frame preemption is supported"]
    pub mod FP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    pub mod MIN_MPDU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the MII protocol supported"]
    pub mod MII_PROT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 1 I/O capability register"]
pub mod L1IOCAPR {
    #[doc = "PCS protocols supported"]
    pub mod PCS_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Variants supported"]
    pub mod IO_VAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External MDIO supported."]
    pub mod EMDIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RevMII MII rate"]
    pub mod REVMII_RATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reverse Mode Device Configuration"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 1 binding configuration register"]
pub mod L1BCR {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    pub mod SW_PORT_ENETC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Primary link end's NETC Function Type"]
    pub mod NETC_FUNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value indicate an MDIO PHY address"]
    pub mod MDIO_PHYAD_PRTAD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    pub mod SPL_SW_PORT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 1 transmit byte credit comfort threshold register"]
pub mod L1TXBCCTR {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 1 end 0 MAC address register 0"]
pub mod L1E0MAR0 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 1 end 0 MAC address register 1"]
pub mod L1E0MAR1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 2 capability register"]
pub mod L2CAPR {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    pub mod LINK_TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    pub mod NUM_TC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    pub mod NUM_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of congestion groups supported"]
    pub mod NUM_CG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Gate Scheduling"]
    pub mod TGS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Credit Based Shaping"]
    pub mod CBS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 2 MAC capability register"]
pub mod L2MCAPR {
    #[doc = "MAC Variant"]
    pub mod MAC_VAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress frame padding capability"]
    pub mod EFPAD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configurable preamble/IPG capability"]
    pub mod PIPG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Duplex capability"]
    pub mod HD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if frame preemption is supported"]
    pub mod FP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    pub mod MIN_MPDU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the MII protocol supported"]
    pub mod MII_PROT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 2 I/O capability register"]
pub mod L2IOCAPR {
    #[doc = "PCS protocols supported"]
    pub mod PCS_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Variants supported"]
    pub mod IO_VAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External MDIO supported."]
    pub mod EMDIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RevMII MII rate"]
    pub mod REVMII_RATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reverse Mode Device Configuration"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 2 binding configuration register"]
pub mod L2BCR {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    pub mod SW_PORT_ENETC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Primary link end's NETC Function Type"]
    pub mod NETC_FUNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value indicate an MDIO PHY address"]
    pub mod MDIO_PHYAD_PRTAD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    pub mod SPL_SW_PORT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 2 transmit byte credit comfort threshold register"]
pub mod L2TXBCCTR {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 2 end 0 MAC address register 0"]
pub mod L2E0MAR0 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 2 end 0 MAC address register 1"]
pub mod L2E0MAR1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 3 capability register"]
pub mod L3CAPR {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    pub mod LINK_TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    pub mod NUM_TC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    pub mod NUM_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of congestion groups supported"]
    pub mod NUM_CG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Gate Scheduling"]
    pub mod TGS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Credit Based Shaping"]
    pub mod CBS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 3 MAC capability register"]
pub mod L3MCAPR {
    #[doc = "MAC Variant"]
    pub mod MAC_VAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress frame padding capability"]
    pub mod EFPAD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configurable preamble/IPG capability"]
    pub mod PIPG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Duplex capability"]
    pub mod HD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if frame preemption is supported"]
    pub mod FP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    pub mod MIN_MPDU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the MII protocol supported"]
    pub mod MII_PROT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 3 I/O capability register"]
pub mod L3IOCAPR {
    #[doc = "PCS protocols supported"]
    pub mod PCS_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Variants supported"]
    pub mod IO_VAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External MDIO supported."]
    pub mod EMDIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RevMII MII rate"]
    pub mod REVMII_RATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reverse Mode Device Configuration"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 3 binding configuration register"]
pub mod L3BCR {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    pub mod SW_PORT_ENETC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Primary link end's NETC Function Type"]
    pub mod NETC_FUNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value indicate an MDIO PHY address"]
    pub mod MDIO_PHYAD_PRTAD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    pub mod SPL_SW_PORT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 3 transmit byte credit comfort threshold register"]
pub mod L3TXBCCTR {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 3 end 0 MAC address register 0"]
pub mod L3E0MAR0 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 3 end 0 MAC address register 1"]
pub mod L3E0MAR1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 4 capability register"]
pub mod L4CAPR {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    pub mod LINK_TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    pub mod NUM_TC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    pub mod NUM_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of congestion groups supported"]
    pub mod NUM_CG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Gate Scheduling"]
    pub mod TGS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Credit Based Shaping"]
    pub mod CBS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 4 MAC capability register"]
pub mod L4MCAPR {
    #[doc = "MAC Variant"]
    pub mod MAC_VAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress frame padding capability"]
    pub mod EFPAD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configurable preamble/IPG capability"]
    pub mod PIPG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Duplex capability"]
    pub mod HD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if frame preemption is supported"]
    pub mod FP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    pub mod MIN_MPDU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the MII protocol supported"]
    pub mod MII_PROT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 4 I/O capability register"]
pub mod L4IOCAPR {
    #[doc = "PCS protocols supported"]
    pub mod PCS_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IO Variants supported"]
    pub mod IO_VAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External MDIO supported."]
    pub mod EMDIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RevMII MII rate"]
    pub mod REVMII_RATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reverse Mode Device Configuration"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 4 binding configuration register"]
pub mod L4BCR {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    pub mod SW_PORT_ENETC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Primary link end's NETC Function Type"]
    pub mod NETC_FUNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This value indicate an MDIO PHY address"]
    pub mod MDIO_PHYAD_PRTAD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    pub mod SPL_SW_PORT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 4 transmit byte credit comfort threshold register"]
pub mod L4TXBCCTR {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 4 end 0 MAC address register 0"]
pub mod L4E0MAR0 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 4 end 0 MAC address register 1"]
pub mod L4E0MAR1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 5 capability register"]
pub mod L5CAPR {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    pub mod LINK_TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    pub mod NUM_TC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    pub mod NUM_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of congestion groups supported"]
    pub mod NUM_CG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Gate Scheduling"]
    pub mod TGS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Credit Based Shaping"]
    pub mod CBS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 5 MAC capability register"]
pub mod L5MCAPR {
    #[doc = "MAC Variant"]
    pub mod MAC_VAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress frame padding capability"]
    pub mod EFPAD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configurable preamble/IPG capability"]
    pub mod PIPG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Duplex capability"]
    pub mod HD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if frame preemption is supported"]
    pub mod FP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    pub mod MIN_MPDU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the MII protocol supported"]
    pub mod MII_PROT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 5 binding configuration register"]
pub mod L5BCR {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    pub mod SW_PORT_ENETC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Primary link end's NETC Function Type"]
    pub mod NETC_FUNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    pub mod SPL_SW_PORT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 5 transmit byte credit comfort threshold register"]
pub mod L5TXBCCTR {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 5 end 0 MAC address register 0"]
pub mod L5E0MAR0 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 5 end 0 MAC address register 1"]
pub mod L5E0MAR1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 5 end 1 MAC address register 0"]
pub mod L5E1MAR0 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Link 5 end 1 MAC address register 1"]
pub mod L5E1MAR1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 binding configuration register"]
pub mod S0BCR {
    #[doc = "Root complex instance number."]
    pub mod RC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    pub mod FN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set if switch instance is bounded to at least 1 link."]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 MSI-X configuration register"]
pub mod S0MCR {
    #[doc = "Number of MSI-X vectors supported for switch function"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 config header device ID and vendor ID register"]
pub mod S0_CFH_DIDVID {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    pub mod VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    pub mod DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 config header subsystem ID and subsystem vendor ID register"]
pub mod S0_CFH_SIDSVID {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    pub mod SUBSYSTEM_VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    pub mod SUBSYSTEM_DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 command cache attribute register"]
pub mod S0CCAR {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when switch writes the command buffer descriptor in memory"]
    pub mod CBD_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when switch writes the command buffer descriptor in memory"]
    pub mod CBD_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when switch writes the command buffer descriptor in memory"]
    pub mod CBD_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data cache type This is the cache attribute setting used when switch writes command data to memory"]
    pub mod CWRCACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data domain This is the domain attribute setting used when switch writes command data to memory"]
    pub mod CWRDOMAIN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when switch writes command data to memory"]
    pub mod CWRSNP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when switch reads the command buffer descriptor from memory"]
    pub mod CBD_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when switch reads the command buffer descriptor from memory"]
    pub mod CBD_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    pub mod CBD_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data cache type This is the cache attribute setting used when switch reads command data from memory"]
    pub mod CRDCACHE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data domain This is the domain attribute setting used when switch reads command data from memory"]
    pub mod CRDDOMAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when switch reads command data from memory"]
    pub mod CRDSNP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 access management qualifier register"]
pub mod S0AMQR {
    #[doc = "Address Read QoS"]
    pub mod ARQOS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Write QoS"]
    pub mod AWQOS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    pub mod BMT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 boot loader parameter register 0"]
pub mod S0BLPR0 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 boot loader parameter register 1"]
pub mod S0BLPR1 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 shared memory buffer allotment register"]
pub mod S0SMBAR {
    #[doc = "Number of words allotted for the switch frame buffering memory"]
    pub mod ALLOC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 hash table memory allotment register"]
pub mod S0HTMAR {
    #[doc = "Maximum number of words allotted to the switch exact match hash tables from the common memory's shared region"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 index table memory allocation register"]
pub mod S0ITMAR {
    #[doc = "Number of words allocated to the switch's index table memory"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 ingress port filter table memory allocation register"]
pub mod S0IPFTMAR {
    #[doc = "Number of words allocated to Ingress Port Filter table from ingress port filter ternary memory"]
    pub mod ALLOC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 rate policer index table memory allocation register"]
pub mod S0RPITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 ingress stream counter index table memory allocation register"]
pub mod S0ISCITMAR {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 ingress stream index table memory allocation register"]
pub mod S0ISITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 ingress sequence generation index table memory allocation register"]
pub mod S0ISQGITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 stream gate instance index table memory allocation register"]
pub mod S0SGIITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 stream gate control list index table memory allocation register"]
pub mod S0SGCLITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 frame modification index table memory allocation register"]
pub mod S0FMITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 frame modification data index table memory allocation register"]
pub mod S0FMDITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 time gate scheduling table allocation register"]
pub mod S0TGSTAR {
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the switch Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list of each switch port"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 time gate scheduling lookahead register"]
pub mod S0TGSLR {
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time gate scheduler (at the frame scheduling timing point), to account for the time required to schedule and dequeue a frame"]
    pub mod MIN_LOOKAHEAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 management port configuration register"]
pub mod S0MPCR {
    #[doc = "Specifies the destination port for frames identified as management"]
    pub mod PORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 VLAN Filter (hash) table default entry configuration registers 0"]
pub mod S0VFHTDECR0 {
    #[doc = "Port n."]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port n is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port n is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Port n."]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port n is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port n is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Port n."]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port n is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port n is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Port n."]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port n is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port n is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Port n."]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port n is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port n is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Spanning Tree Group Member ID Refer to the VLAN Filter table entry STG_ID field description, for more details"]
    pub mod STG_ID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Multicast Filtering Enable Refer to the VLAN Filter table entry IPMFE field description, for more details"]
    pub mod IPMFE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No IP multicast filtering is performed."]
            pub const NO_IP_MULTICAST_FLTR: u32 = 0;
            #[doc = "If the frame is identified as a multicast IP packet, then IP multicast filtering is performed. If the frame is not identified as an IP multicast packet, the IP multicast filtering is not performed."]
            pub const IP_MULTICAST_FLTR: u32 = 0x01;
        }
    }
    #[doc = "IP Multicast Flooding Enable If IP multicast filtering is performed (IPMFE = 1b, and the frame is identified as a multicast IP packet), and there was no match found, then the frame is forwarded according to this field"]
    pub mod IPMFLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP Multicast Flooding disabled, the frame is discarded."]
            pub const DISABLE: u32 = 0;
            #[doc = "IP Multicast Flooding enabled, the frame is flooded."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Switch 0 VLAN filter hash table default entry configuration registers 1"]
pub mod S0VFHTDECR1 {
    #[doc = "Filtering ID"]
    pub mod FID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Learning Mode 0: Independent VLAN learning: FID is set to the VID assigned to the frame 1: Shared VLAN learning: Use the FID specified in this register Used to determine the FID when doing a lookup in the FDB table"]
    pub mod VL_MODE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Base Egress Treatment Entry ID Refer to the VLAN Filter table entry BASE_ET_EID field description, for more details"]
    pub mod BASE_ETEID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch 0 VLAN filter hash table default entry configuration registers 2"]
pub mod S0VFHTDECR2 {
    #[doc = "Egress Treatment Applicability Port"]
    pub mod ES_PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Treatment Applicability Port"]
    pub mod ES_PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Treatment Applicability Port"]
    pub mod ES_PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Treatment Applicability Port"]
    pub mod ES_PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Treatment Applicability Port"]
    pub mod ES_PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC learning options: 0: Reserved 1: Disable MAC learning"]
    pub mod MLO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC forwarding options: 0: Reserved 1: No FDB lookup is performed, the frame is flooded 2: FDB lookup is performed, and if there is no match, the frame is flooded"]
    pub mod MFO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 binding configuration register 0"]
pub mod E0BCR0 {
    #[doc = "Root complex instance this function is bound to."]
    pub mod RC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    pub mod FN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set, this ENETC instance is associated to a link end."]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 binding configuration register 1"]
pub mod E0BCR1 {
    #[doc = "Number of Rx BD rings supported by ENETC"]
    pub mod NUM_RX_BDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Tx BD rings supported by ENETC"]
    pub mod NUM_TX_BDR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 binding configuration register 2"]
pub mod E0BCR2 {
    #[doc = "Number of ENETC SI MAC address filter rules supported"]
    pub mod NUM_MAC_AFTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of ENETC SI VLAN filter rules supported"]
    pub mod NUM_VLAN_FTE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 VSI binding configuration register"]
pub mod E0VBCR {
    #[doc = "Indicates the number of VSIs supported for this ENETC instance"]
    pub mod NUM_VSI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 MSI-X configuration register"]
pub mod E0MCR {
    #[doc = "Number of MSI-X vectors supported for ENETC function"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 config header device ID and vendor ID register"]
pub mod E0_CFH_DIDVID {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    pub mod VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    pub mod DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 config header subsystem ID and subsystem vendor ID register"]
pub mod E0_CFH_SIDSVID {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    pub mod SUBSYSTEM_VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    pub mod SUBSYSTEM_DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 config capability VF device ID register"]
pub mod E0_CFC_VFDID {
    #[doc = "VF Device ID This field identifies the device ID for a virtual function as shown in the PCIe SR-IOV Capability Structure (20h)"]
    pub mod VF_DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 buffer cache attribute register 0"]
pub mod E0BCAR {
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the buffer descriptor in memory"]
    pub mod BD_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when ENETC writes the buffer descriptor in memory"]
    pub mod BD_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the buffer descriptor in memory"]
    pub mod BD_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes frame data to memory"]
    pub mod WRCACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes frame data to memory"]
    pub mod WRDOMAIN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes frame data to memory"]
    pub mod WRSNP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the buffer descriptor from memory"]
    pub mod BD_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when ENETC reads the buffer descriptor from memory"]
    pub mod BD_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    pub mod BD_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads frame data from memory"]
    pub mod RDCACHE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads frame data from memory"]
    pub mod RDDOMAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads frame data from memory"]
    pub mod RDSNP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 message cache attribute register"]
pub mod E0MCAR {
    #[doc = "SI messaging write cache type This is the cache attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    pub mod MSG_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging write domain This is the domain attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    pub mod MSG_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging write snoop This is the snoop attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    pub mod MSG_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging read data cache type This is the cache attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    pub mod MSG_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging read data domain This is the domain attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    pub mod MSG_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging read data snoop This is the snoop attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    pub mod MSG_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 command cache attribute register"]
pub mod E0CAR {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the command buffer descriptor in memory"]
    pub mod CBD_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when ENETC writes the command buffer descriptor in memory"]
    pub mod CBD_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the command buffer descriptor in memory"]
    pub mod CBD_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes command data to memory"]
    pub mod CWRCACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes command data to memory"]
    pub mod CWRDOMAIN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes command data to memory"]
    pub mod CWRSNP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the command buffer descriptor from memory"]
    pub mod CBD_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when ENETC reads the command buffer descriptor from memory"]
    pub mod CBD_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    pub mod CBD_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads command data from memory"]
    pub mod CRDCACHE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads command data from memory"]
    pub mod CRDDOMAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads command data from memory"]
    pub mod CRDSNP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 access management qualifier register"]
pub mod E0AMQR {
    #[doc = "Address Read QoS"]
    pub mod ARQOS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Write QoS"]
    pub mod AWQOS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    pub mod BMT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 boot loader parameter register 0"]
pub mod E0BLPR0 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 boot loader parameter register 1"]
pub mod E0BLPR1 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 receive memory buffer entitlement register"]
pub mod E0RXMBER {
    #[doc = "Receive memory buffer entitlement in words This receive Memory Buffer Entitlement is used in determining smart drop for ingress congestion control"]
    pub mod AMOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 receive memory buffer limit register"]
pub mod E0RXMBLR {
    #[doc = "Receive buffer memory limit in words"]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 transmit high priority tier byte credit register"]
pub mod E0TXHPTBCR {
    #[doc = "This register field is used to configure the maximum number of high priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    pub mod BYTE_CREDIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 transmit low priority tier byte credit register"]
pub mod E0TXLPTBCR {
    #[doc = "This register field is used to configure the maximum number of low priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    pub mod BYTE_CREDIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 hash table memory allotment register"]
pub mod E0HTMAR {
    #[doc = "Number of words allotted to exact match hash table from the common memory's shared region"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 index table memory allocation register"]
pub mod E0ITMAR {
    #[doc = "Number of WORDS allocated to ENETC's index table memory"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 ingress port filter table memory allocation register"]
pub mod E0IPFTMAR {
    #[doc = "Number of words allocated to the ENETC Ingress Port Filter table from ingress port filter ternary memory"]
    pub mod ALLOC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 rate policer index table memory allocation register"]
pub mod E0RPITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 ingress stream counter index table memory allocation register"]
pub mod E0ISCITMAR {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 ingress stream index table memory allocation register"]
pub mod E0ISITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 stream gate instance index table memory allocation register"]
pub mod E0SGIITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 stream gate control list index table memory allocation register"]
pub mod E0SGCLITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 time gate scheduling table allocation register"]
pub mod E0TGSTAR {
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the ENETC instance a Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list for the ENETC instance"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 0 time gate scheduling lookahead register"]
pub mod E0TGSLR {
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler (at the frame scheduling timing point), to account for the time required to schedule, dequeue and load (or DMA) frames from the host memory to NETC internal memory"]
    pub mod MIN_LOOKAHEAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Zero Lookahead"]
    pub mod ZERO_LOOKAHEAD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MIN_LOOKAHEAD value"]
            pub const USE: u32 = 0;
            #[doc = "If a gate control list is configured or when time specific departure is enabled on any traffic class (PTCaTSDR\\[TSDE\\] set to 1, where a corresponds to the traffic class number), use MIN_LOOKAHEAD value, otherwise use value of zero"]
            pub const ZERO: u32 = 0x01;
        }
    }
}
#[doc = "ENETC 1 binding configuration register 0"]
pub mod E1BCR0 {
    #[doc = "Root complex instance this function is bound to."]
    pub mod RC_INST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    pub mod FN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If set, this ENETC instance is associated to a link end."]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 binding configuration register 1"]
pub mod E1BCR1 {
    #[doc = "Number of Rx BD rings supported by ENETC"]
    pub mod NUM_RX_BDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Tx BD rings supported by ENETC"]
    pub mod NUM_TX_BDR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 binding configuration register 2"]
pub mod E1BCR2 {
    #[doc = "Number of ENETC SI MAC address filter rules supported"]
    pub mod NUM_MAC_AFTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of ENETC SI VLAN filter rules supported"]
    pub mod NUM_VLAN_FTE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 VSI binding configuration register"]
pub mod E1VBCR {
    #[doc = "Indicates the number of VSIs supported for this ENETC instance"]
    pub mod NUM_VSI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 MSI-X configuration register"]
pub mod E1MCR {
    #[doc = "Number of MSI-X vectors supported for ENETC function"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 config header device ID and vendor ID register"]
pub mod E1_CFH_DIDVID {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    pub mod VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    pub mod DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 config header subsystem ID and subsystem vendor ID register"]
pub mod E1_CFH_SIDSVID {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    pub mod SUBSYSTEM_VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    pub mod SUBSYSTEM_DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 config capability VF device ID register"]
pub mod E1_CFC_VFDID {
    #[doc = "VF Device ID This field identifies the device ID for a virtual function as shown in the PCIe SR-IOV Capability Structure (20h)"]
    pub mod VF_DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 buffer cache attribute register 0"]
pub mod E1BCAR {
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the buffer descriptor in memory"]
    pub mod BD_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when ENETC writes the buffer descriptor in memory"]
    pub mod BD_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the buffer descriptor in memory"]
    pub mod BD_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes frame data to memory"]
    pub mod WRCACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes frame data to memory"]
    pub mod WRDOMAIN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes frame data to memory"]
    pub mod WRSNP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the buffer descriptor from memory"]
    pub mod BD_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when ENETC reads the buffer descriptor from memory"]
    pub mod BD_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    pub mod BD_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads frame data from memory"]
    pub mod RDCACHE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads frame data from memory"]
    pub mod RDDOMAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads frame data from memory"]
    pub mod RDSNP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 message cache attribute register"]
pub mod E1MCAR {
    #[doc = "SI messaging write cache type This is the cache attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    pub mod MSG_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging write domain This is the domain attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    pub mod MSG_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging write snoop This is the snoop attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    pub mod MSG_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging read data cache type This is the cache attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    pub mod MSG_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging read data domain This is the domain attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    pub mod MSG_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SI messaging read data snoop This is the snoop attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    pub mod MSG_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 command cache attribute register"]
pub mod E1CAR {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the command buffer descriptor in memory"]
    pub mod CBD_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when ENETC writes the command buffer descriptor in memory"]
    pub mod CBD_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the command buffer descriptor in memory"]
    pub mod CBD_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes command data to memory"]
    pub mod CWRCACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes command data to memory"]
    pub mod CWRDOMAIN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes command data to memory"]
    pub mod CWRSNP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the command buffer descriptor from memory"]
    pub mod CBD_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when ENETC reads the command buffer descriptor from memory"]
    pub mod CBD_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    pub mod CBD_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads command data from memory"]
    pub mod CRDCACHE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads command data from memory"]
    pub mod CRDDOMAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads command data from memory"]
    pub mod CRDSNP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 access management qualifier register"]
pub mod E1AMQR {
    #[doc = "Address Read QoS"]
    pub mod ARQOS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Write QoS"]
    pub mod AWQOS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    pub mod BMT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 boot loader parameter register 0"]
pub mod E1BLPR0 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 boot loader parameter register 1"]
pub mod E1BLPR1 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 receive memory buffer entitlement register"]
pub mod E1RXMBER {
    #[doc = "Receive memory buffer entitlement in words This receive Memory Buffer Entitlement is used in determining smart drop for ingress congestion control"]
    pub mod AMOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 receive memory buffer limit register"]
pub mod E1RXMBLR {
    #[doc = "Receive buffer memory limit in words"]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 transmit high priority tier byte credit register"]
pub mod E1TXHPTBCR {
    #[doc = "This register field is used to configure the maximum number of high priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    pub mod BYTE_CREDIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 transmit low priority tier byte credit register"]
pub mod E1TXLPTBCR {
    #[doc = "This register field is used to configure the maximum number of low priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    pub mod BYTE_CREDIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 hash table memory allotment register"]
pub mod E1HTMAR {
    #[doc = "Number of words allotted to exact match hash table from the common memory's shared region"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 index table memory allocation register"]
pub mod E1ITMAR {
    #[doc = "Number of WORDS allocated to ENETC's index table memory"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 ingress port filter table memory allocation register"]
pub mod E1IPFTMAR {
    #[doc = "Number of words allocated to the ENETC Ingress Port Filter table from ingress port filter ternary memory"]
    pub mod ALLOC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 rate policer index table memory allocation register"]
pub mod E1RPITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 ingress stream counter index table memory allocation register"]
pub mod E1ISCITMAR {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 ingress stream index table memory allocation register"]
pub mod E1ISITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 stream gate instance index table memory allocation register"]
pub mod E1SGIITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 stream gate control list index table memory allocation register"]
pub mod E1SGCLITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 time gate scheduling table allocation register"]
pub mod E1TGSTAR {
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the ENETC instance a Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list for the ENETC instance"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC 1 time gate scheduling lookahead register"]
pub mod E1TGSLR {
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler (at the frame scheduling timing point), to account for the time required to schedule, dequeue and load (or DMA) frames from the host memory to NETC internal memory"]
    pub mod MIN_LOOKAHEAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Zero Lookahead"]
    pub mod ZERO_LOOKAHEAD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MIN_LOOKAHEAD value"]
            pub const USE: u32 = 0;
            #[doc = "If a gate control list is configured or when time specific departure is enabled on any traffic class (PTCaTSDR\\[TSDE\\] set to 1, where a corresponds to the traffic class number), use MIN_LOOKAHEAD value, otherwise use value of zero"]
            pub const ZERO: u32 = 0x01;
        }
    }
}
#[doc = "VSI 0 access management qualifier register"]
pub mod V0AMQR {
    #[doc = "Address Read QoS"]
    pub mod ARQOS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Write QoS"]
    pub mod AWQOS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    pub mod BMT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VSI 0 boot loader parameter register 0"]
pub mod V0BLPR0 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VSI 0 boot loader parameter register 1"]
pub mod V0BLPR1 {
    #[doc = "Boot loader parameter value."]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VSI 0 primary MAC address register 0"]
pub mod V0PMAR0 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VSI 0 primary MAC address register 1"]
pub mod V0PMAR1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    pub mod MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
