#[doc = "Switch base"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Switch capability register 0"]
    pub SCAPR0: crate::RORegister<u32>,
    #[doc = "Switch capability register 1"]
    pub SCAPR1: crate::RORegister<u32>,
    #[doc = "Buffer pool capability register"]
    pub BPCAPR: crate::RORegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "Forwarding capability register"]
    pub FCAPR: crate::RORegister<u32>,
    _reserved1: [u8; 0x24],
    #[doc = "Shared memory buffer capability register"]
    pub SMBCAPR: crate::RORegister<u32>,
    #[doc = "Shared memory buffer operational register 0"]
    pub SMBOR0: crate::RORegister<u32>,
    #[doc = "Shared memory buffer operational register 1"]
    pub SMBOR1: crate::RORegister<u32>,
    _reserved2: [u8; 0x34],
    #[doc = "Command cache attribute register"]
    pub CCAR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x037c],
    #[doc = "Management port configuration register"]
    pub MPCR: crate::RORegister<u32>,
    _reserved4: [u8; 0x1c],
    #[doc = "Ingress mirror destination configuration register 0"]
    pub IMDCR0: crate::RWRegister<u32>,
    #[doc = "Ingress mirror destination configuration register 1"]
    pub IMDCR1: crate::RWRegister<u32>,
    _reserved5: [u8; 0x18],
    #[doc = "Cut-through forwarding count register"]
    pub CTFCR: crate::RWRegister<u32>,
    _reserved6: [u8; 0x03bc],
    #[doc = "Set of registers for available Common BDRs."]
    pub NUM_CBDR: [numcbdr::RegisterBlock; 2usize],
    _reserved7: [u8; 0x40],
    #[doc = "Set of interrupt registers for common BD rings."]
    pub NUM_CBDR_INT: [numcbdrint::RegisterBlock; 2usize],
    _reserved8: [u8; 0x40],
    #[doc = "QoS to VLAN mapping profile register set."]
    pub MAP_PCP: [mappcp::RegisterBlock; 2usize],
    _reserved9: [u8; 0x01c0],
    #[doc = "PCP to PCP mapping profile a register"]
    pub PCP2PCPMPR: [crate::RWRegister<u32>; 2usize],
    _reserved10: [u8; 0x14f8],
    #[doc = "Bridge capability register"]
    pub BRCAPR: crate::RORegister<u32>,
    _reserved11: [u8; 0x04],
    #[doc = "VLAN filter hash table capability register"]
    pub VFHTCAPR: crate::RORegister<u32>,
    #[doc = "VLAN filter hash table operational register"]
    pub VFHTOR: crate::RORegister<u32>,
    #[doc = "VLAN Filter (hash) table default entry configuration registers 0"]
    pub VFHTDECR0: crate::RWRegister<u32>,
    #[doc = "VLAN filter hash table default entry configuration registers 1"]
    pub VFHTDECR1: crate::RWRegister<u32>,
    #[doc = "VLAN filter hash table default entry configuration registers 2"]
    pub VFHTDECR2: crate::RWRegister<u32>,
    _reserved12: [u8; 0x04],
    #[doc = "FDB hash table capability register"]
    pub FDBHTCAPR: crate::RORegister<u32>,
    #[doc = "FDB hash table memory configuration register"]
    pub FDBHTMCR: crate::RWRegister<u32>,
    #[doc = "FDB hash table operational register 0"]
    pub FDBHTOR0: crate::RORegister<u32>,
    #[doc = "FDB hash table operational register 1"]
    pub FDBHTOR1: crate::RORegister<u32>,
    _reserved13: [u8; 0x10],
    #[doc = "IP multicast filter hash table capability register"]
    pub IPMFHTCAPR: crate::RORegister<u32>,
    #[doc = "IPv4 multicast filter hash table operation register"]
    pub IPV4MFHTOR: crate::RORegister<u32>,
}
#[doc = "Switch capability register 0"]
pub mod SCAPR0 {
    #[doc = "Number of ports supported"]
    pub mod NUM_PORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of IPVs supported 0: 8 IPV 1: Reserved"]
    pub mod NUM_IPV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of MSI-X vectors supported by switch function"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of VLAN PCP to PCP mapping profiles supported"]
    pub mod NUM_PCPMP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of QoS to VLAN PCP mapping profiles supported"]
    pub mod NUM_QVMP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch capability register 1"]
pub mod SCAPR1 {
    #[doc = "Functional safety capability supported."]
    pub mod FS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Cut-through forwarding supported. 0: Not supported 1: Supported"]
    pub mod CTF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time capture capability supported. 0: Not supported 1: Supported"]
    pub mod TIMCAP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress mirroring functionality supported. 0: Not supported 1: Supported"]
    pub mod IMIR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which FRER Sequence Tags are supported xxxx1: 802"]
    pub mod SQ_TAGS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Buffer pool capability register"]
pub mod BPCAPR {
    #[doc = "Number of buffer pools supported."]
    pub mod NUM_BP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of shared buffer pools supported."]
    pub mod NUM_SPB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Forwarding capability register"]
pub mod FCAPR {
    #[doc = "802.1Q bridge forwarding support."]
    pub mod BR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Stream forwarding supported"]
    pub mod SF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Shared memory buffer capability register"]
pub mod SMBCAPR {
    #[doc = "Number of words available for the switch frame buffering"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word size in bytes."]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "24 bytes"]
            pub const B_24: u32 = 0;
        }
    }
    #[doc = "Indicates memory location"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Common memory"]
            pub const CM: u32 = 0;
        }
    }
}
#[doc = "Shared memory buffer operational register 0"]
pub mod SMBOR0 {
    #[doc = "Number of words in use for frame buffering."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shared memory buffer operational register 1"]
pub mod SMBOR1 {
    #[doc = "High watermark of words in use for frame buffering since the last read"]
    pub mod WATERMARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command cache attribute register"]
pub mod CCAR {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when switch writes the command buffer descriptor in host memory"]
    pub mod CBD_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when switch writes the command buffer descriptor in host memory"]
    pub mod CBD_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when switch writes the command buffer descriptor in host memory"]
    pub mod CBD_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data cache type This is the cache attribute setting used when switch writes command data to host memory"]
    pub mod CWRCACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data domain This is the domain attribute setting used when switch writes command data to host memory"]
    pub mod CWRDOMAIN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when switch writes command data to host memory"]
    pub mod CWRSNP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when switch reads the command buffer descriptor from host memory"]
    pub mod CBD_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when switch reads the command buffer descriptor from host memory"]
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
    #[doc = "Read data cache type This is the cache attribute setting used when switch reads command data from host memory"]
    pub mod CRDCACHE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data domain This is the domain attribute setting used when switch reads command data from host memory"]
    pub mod CRDDOMAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when switch reads command data from host memory"]
    pub mod CRDSNP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Management port configuration register"]
pub mod MPCR {
    #[doc = "Switch Management Port"]
    pub mod PORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress mirror destination configuration register 0"]
pub mod IMDCR0 {
    #[doc = "Mirror enable."]
    pub mod MIREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Ingress mirroring disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Ingress mirroring enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Indicates the mirror destination"]
    pub mod MIRDEST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mirrored packet's IPV."]
    pub mod IPV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mirrored packet's DR (drop resilience)."]
    pub mod DR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port where ingress mirrored frames are sent. Valid if MIRDEST=0."]
    pub mod PORT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress mirror destination configuration register 1"]
pub mod IMDCR1 {
    #[doc = "Egress Frame Modification Entry Id Only applicable if MIRDEST=0"]
    pub mod EFMEID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Frame Modification Frame Length change in 2s complement notation"]
    pub mod EFM_LEN_CHANGE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Cut-through forwarding count register"]
pub mod CTFCR {
    #[doc = "Count the number of cut-through frames which are forwarded to at least 1 egress port."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCP to PCP mapping profile a register"]
pub mod PCP2PCPMPR {
    #[doc = "PCP0 priority code point mapped value."]
    pub mod PCP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCP1 priority code point mapped value."]
    pub mod PCP1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCP2 priority code point mapped value."]
    pub mod PCP2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCP3 priority code point mapped value."]
    pub mod PCP3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCP4 priority code point mapped value."]
    pub mod PCP4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCP5 priority code point mapped value."]
    pub mod PCP5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCP6 priority code point mapped value."]
    pub mod PCP6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCP7 priority code point mapped value."]
    pub mod PCP7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge capability register"]
pub mod BRCAPR {
    #[doc = "L2 IPv4 multicast filtering supported."]
    pub mod IPV4MFLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
    }
    #[doc = "Station Move Disable supported"]
    pub mod STAMVD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
    }
    #[doc = "Storm Control supported."]
    pub mod STRMCTRL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
    }
    #[doc = "Source port pruning disable supported"]
    pub mod SRCPPRND {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Source port pruning disable not supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Source port pruning disable supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
    }
    #[doc = "Egress VLAN translation supported"]
    pub mod EVLANXLATE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Egress VLAN translation not supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Egress VLAN translation supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
    }
    #[doc = "Number of Spanning Tree Groups supported"]
    pub mod NUM_STG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VLAN filter hash table capability register"]
pub mod VFHTCAPR {
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VLAN filter hash table operational register"]
pub mod VFHTOR {
    #[doc = "Number of entries in-use by the VLAN Filter table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VLAN Filter (hash) table default entry configuration registers 0"]
pub mod VFHTDECR0 {
    #[doc = "Port n"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Port n"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Port n"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Port n"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Port n"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port is not a member of this VLAN."]
            pub const NOT_VLAN_MEMBER: u32 = 0;
            #[doc = "Port is a member of this VLAN."]
            pub const VLAN_MEMBER: u32 = 0x01;
        }
    }
    #[doc = "Spanning Tree Group Member ID"]
    pub mod STG_ID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Multicast Filtering Enable"]
    pub mod IPMFE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No IP multicast filtering is performed."]
            pub const NO_IPM_FLTR: u32 = 0;
            #[doc = "If the frame is identified as a multicast IP packet, then IP multicast filtering is performed. If the frame is not identified as an IP multicast packet, the IP multicast filtering is not performed."]
            pub const IPM_FLTR: u32 = 0x01;
        }
    }
    #[doc = "IP Multicast Flooding Enable"]
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
#[doc = "VLAN filter hash table default entry configuration registers 1"]
pub mod VFHTDECR1 {
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
#[doc = "VLAN filter hash table default entry configuration registers 2"]
pub mod VFHTDECR2 {
    #[doc = "Egress Treatment Applicability Port n."]
    pub mod ET_PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port has no entry in the Egress Treatment table"]
            pub const NO_ENTRY: u32 = 0;
            #[doc = "Port has an entry in the Egress Treatment table"]
            pub const PORT_ENTRY: u32 = 0x01;
        }
    }
    #[doc = "Egress Treatment Applicability Port n."]
    pub mod ET_PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port has no entry in the Egress Treatment table"]
            pub const NO_ENTRY: u32 = 0;
            #[doc = "Port has an entry in the Egress Treatment table"]
            pub const PORT_ENTRY: u32 = 0x01;
        }
    }
    #[doc = "Egress Treatment Applicability Port n."]
    pub mod ET_PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port has no entry in the Egress Treatment table"]
            pub const NO_ENTRY: u32 = 0;
            #[doc = "Port has an entry in the Egress Treatment table"]
            pub const PORT_ENTRY: u32 = 0x01;
        }
    }
    #[doc = "Egress Treatment Applicability Port n."]
    pub mod ET_PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port has no entry in the Egress Treatment table"]
            pub const NO_ENTRY: u32 = 0;
            #[doc = "Port has an entry in the Egress Treatment table"]
            pub const PORT_ENTRY: u32 = 0x01;
        }
    }
    #[doc = "Egress Treatment Applicability Port n."]
    pub mod ET_PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Port has no entry in the Egress Treatment table"]
            pub const NO_ENTRY: u32 = 0;
            #[doc = "Port has an entry in the Egress Treatment table"]
            pub const PORT_ENTRY: u32 = 0x01;
        }
    }
    #[doc = "MAC learning options: 0: Reserved 1: Disable MAC learning"]
    pub mod MLO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC forwarding options: 0: Reserved 1: No FDB lookup is performed, the frame is flooded"]
    pub mod MFO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FDB hash table capability register"]
pub mod FDBHTCAPR {
    #[doc = "Number of guaranteed MAC addresses which can be added to the FDB table if an entry cannot be added due to collision limit being exceeded"]
    pub mod NUM_GMAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FDB hash table memory configuration register"]
pub mod FDBHTMCR {
    #[doc = "This field specifies the maximum number of dynamic entries allowed in the FDB table for the entire switch"]
    pub mod DYN_LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FDB hash table operational register 0"]
pub mod FDBHTOR0 {
    #[doc = "Number of static entries in-use in the FDB table."]
    pub mod STATIC_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The number of guaranteed MAC entries in-use in the FDB table"]
    pub mod NUM_GENTRIES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FDB hash table operational register 1"]
pub mod FDBHTOR1 {
    #[doc = "Number of dynamic entries in-use in the FDB table."]
    pub mod DYN_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High water mark of dynamic entries in-use in the FDB table. Value reset to DYN_ENTRIES when read."]
    pub mod HWM_DYN_ENTRIES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP multicast filter hash table capability register"]
pub mod IPMFHTCAPR {
    #[doc = "Indicates which configuration access methods are supported: xxx1: Index xx1x: EntryId x1xx: Search 1xxx: Reserved"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv4 multicast filter hash table operation register"]
pub mod IPV4MFHTOR {
    #[doc = "Number of IPV4 Any Source Multicast (ASM) Multicast Filter table entries in-use."]
    pub mod ASM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of IPV4 Source Specific Multicast (SSM) Multicast Filter table entries in-use."]
    pub mod SSM_ENTRIES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod numcbdr {
    #[doc = "Set of registers for available Common BDRs."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Command BDR a mode register"]
        pub CBDRMR: crate::RWRegister<u32>,
        #[doc = "Command BDR a status register"]
        pub CBDRSR: crate::RORegister<u32>,
        _reserved0: [u8; 0x08],
        #[doc = "Command BDR base address register 0"]
        pub CBDRBAR0: crate::RWRegister<u32>,
        #[doc = "Command BDR a base address register 1"]
        pub CBDRBAR1: crate::RWRegister<u32>,
        #[doc = "Command BDR a producer index register"]
        pub CBDRPIR: crate::RWRegister<u32>,
        #[doc = "Command BDR a consumer index register"]
        pub CBDRCIR: crate::RWRegister<u32>,
        #[doc = "Command BDR a length register"]
        pub CBDRLENR: crate::RWRegister<u32>,
        _reserved1: [u8; 0x0c],
    }
    #[doc = "Command BDR a mode register"]
    pub mod CBDRMR {
        #[doc = "Enable command buffer descriptor ring"]
        pub mod EN {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enabled. When the ring is non-empty, command buffer descriptors will be processed"]
                pub const ENABLE: u32 = 0x01;
            }
        }
    }
    #[doc = "Command BDR a status register"]
    pub mod CBDRSR {
        #[doc = "Busy."]
        pub mod BUSY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Idle"]
                pub const IDLE: u32 = 0;
                #[doc = "Busy"]
                pub const BUSY: u32 = 0x01;
            }
        }
    }
    #[doc = "Command BDR base address register 0"]
    pub mod CBDRBAR0 {
        #[doc = "Lower address bits 31-7."]
        pub mod ADDRL {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0x01ff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Command BDR a base address register 1"]
    pub mod CBDRBAR1 {
        #[doc = "Upper address bits 63-32."]
        pub mod ADDRH {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Command BDR a producer index register"]
    pub mod CBDRPIR {
        #[doc = "Command buffer descriptor ring producer index"]
        pub mod BDR_INDEX {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x03ff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Command BDR a consumer index register"]
    pub mod CBDRCIR {
        #[doc = "Command buffer descriptor ring consumer index"]
        pub mod BDR_INDEX {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x03ff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Command BDR a length register"]
    pub mod CBDRLENR {
        #[doc = "BD ring length Size of ring in sets of 8 BDs. Maximum ring size is 1K."]
        pub mod LENGTH {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0xff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
pub mod numcbdrint {
    #[doc = "Set of interrupt registers for common BD rings."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Command BDR a interrupt enable register"]
        pub CBDRIER: crate::RWRegister<u32>,
        #[doc = "Command BDR a interrupt detect register"]
        pub CBDRIDR: crate::RWRegister<u32>,
        #[doc = "Command BDR a MSI-X vector register"]
        pub CBDRMSIVR: crate::RWRegister<u32>,
        _reserved0: [u8; 0x04],
    }
    #[doc = "Command BDR a interrupt enable register"]
    pub mod CBDRIER {
        #[doc = "Command BD completion interrupt enable 0 Disabled 1 Enabled"]
        pub mod CBDCIE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Command BDR a interrupt detect register"]
    pub mod CBDRIDR {
        #[doc = "Command BD completed"]
        pub mod CBDC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No BD with CI=1 completed"]
                pub const NO_BD: u32 = 0;
                #[doc = "Processed BD with CI=1"]
                pub const BD_CI_1: u32 = 0x01;
            }
        }
    }
    #[doc = "Command BDR a MSI-X vector register"]
    pub mod CBDRMSIVR {
        #[doc = "Index into MSI-X address/data table"]
        pub mod VECTOR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
pub mod mappcp {
    #[doc = "QoS to VLAN mapping profile register set."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "QoS to VLAN mapping profile a register b"]
        pub QOSVLANMPR: [crate::RWRegister<u32>; 4usize],
        _reserved0: [u8; 0x10],
    }
    #[doc = "QoS to VLAN mapping profile a register b"]
    pub mod QOSVLANMPR {
        #[doc = "The IPV0DR0 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
        pub mod IPV0_DR0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "The IPV0DR1 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
        pub mod IPV0_DR1 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "The IPV0DR2 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
        pub mod IPV0_DR2 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "The IPV0DR3 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
        pub mod IPV0_DR3 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "The IPV1DR0 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
        pub mod IPV1_DR0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "The IPV1DR1 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
        pub mod IPV1_DR1 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "The IPV1DR2 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
        pub mod IPV1_DR2 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "The IPV1DR3 field is defined as a 4-bit field as follows: Bit 3 - Reserved Bit 2:0 - Priority Code Point (PCP)"]
        pub mod IPV1_DR3 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x0f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
