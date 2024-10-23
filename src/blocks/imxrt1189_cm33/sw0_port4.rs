#[doc = "Port"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Port capability register"]
    pub PCAPR: crate::RORegister<u32>,
    #[doc = "Port MAC capability register"]
    pub PMCAPR: crate::RORegister<u32>,
    #[doc = "Port I/O capability register"]
    pub PIOCAPR: crate::RORegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Port configuration register"]
    pub PCR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "Port MAC address register 0"]
    pub PMAR0: crate::RWRegister<u32>,
    #[doc = "Port MAC address register 1"]
    pub PMAR1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x28],
    #[doc = "Port TPID acceptance register"]
    pub PTAR: crate::RWRegister<u32>,
    #[doc = "Port QoS mode register"]
    pub PQOSMR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "Port Queue Operational register"]
    pub PQOR: crate::RORegister<u32>,
    _reserved4: [u8; 0x1c],
    #[doc = "Port parser configuration register"]
    pub PPCR: crate::RWRegister<u32>,
    #[doc = "Port ingress port filter configuration register"]
    pub PIPFCR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x18],
    #[doc = "Port stream gate configuration register"]
    pub PSGCR: crate::RWRegister<u32>,
    _reserved6: [u8; 0x5c],
    #[doc = "Port operational register"]
    pub POR: crate::RWRegister<u32>,
    #[doc = "Port status register"]
    pub PSR: crate::RORegister<u32>,
    #[doc = "Port receive SDU overhead register"]
    pub PRXSDUOR: crate::RWRegister<u32>,
    #[doc = "Port transmit SDU overhead register"]
    pub PTXSDUOR: crate::RWRegister<u32>,
    #[doc = "Port time gate scheduling control register"]
    pub PTGSCR: crate::RWRegister<u32>,
    #[doc = "Port time gate scheduling admin gate list status register"]
    pub PTGAGLSR: crate::RORegister<u32>,
    #[doc = "Port time gate scheduling admin gate list length register"]
    pub PTGAGLLR: crate::RORegister<u32>,
    #[doc = "Port time gating operational gate list length register"]
    pub PTGOGLLR: crate::RORegister<u32>,
    _reserved7: [u8; 0x18],
    #[doc = "Port default gate state register"]
    pub PDGSR: crate::RWRegister<u32>,
    _reserved8: [u8; 0x84],
    #[doc = "Port Rx discard count register"]
    pub PRXDCR: crate::RORegister<u32>,
    _reserved9: [u8; 0x04],
    #[doc = "Port Rx discard count reason register 0"]
    pub PRXDCRR0: crate::RWRegister<u32>,
    #[doc = "Port Rx discard count reason register 1"]
    pub PRXDCRR1: crate::RWRegister<u32>,
    _reserved10: [u8; 0x10],
    #[doc = "Port Tx discard count register"]
    pub PTXDCR: crate::RORegister<u32>,
    _reserved11: [u8; 0x04],
    #[doc = "Port Tx discard count reason register 0"]
    pub PTXDCRR0: crate::RWRegister<u32>,
    #[doc = "Port Tx discard count reason register 1"]
    pub PTXDCRR1: crate::RWRegister<u32>,
    _reserved12: [u8; 0x10],
    #[doc = "Array of registers: PTGSTCSR, PTCTMSDUR, PTCCBSR0, PTCCBSR1"]
    pub TCT_NUM: [tctnum::RegisterBlock; 8usize],
    _reserved13: [u8; 0x0100],
    #[doc = "Port buffer pool mapping configuration register 0"]
    pub PBPMCR0: crate::RWRegister<u32>,
    #[doc = "Port buffer pool mapping configuration register 1"]
    pub PBPMCR1: crate::RWRegister<u32>,
    _reserved14: [u8; 0x30],
    #[doc = "Port PCP DEI mapping register"]
    pub PPCPDEIMR: crate::RWRegister<u32>,
    _reserved15: [u8; 0x04],
    #[doc = "Port mirror configuration register"]
    pub PMCR: crate::RWRegister<u32>,
    _reserved16: [u8; 0x14],
    #[doc = "Port LANID configuration register"]
    pub PLANIDCR: crate::RWRegister<u32>,
    _reserved17: [u8; 0x04],
    #[doc = "Port ingress stream identification configuration register"]
    pub PISIDCR: crate::RWRegister<u32>,
    #[doc = "Port frame modification configuration register"]
    pub PFMCR: crate::RWRegister<u32>,
    _reserved18: [u8; 0x08],
    #[doc = "Port IPV to queue mapping register 0"]
    pub PIPV2QMR0: crate::RWRegister<u32>,
    _reserved19: [u8; 0x3c],
    #[doc = "Port time capture minimum latency register"]
    pub PTCMINLR: crate::RORegister<u32>,
    #[doc = "Port time capture maximum latency register"]
    pub PTCMAXLR: crate::RORegister<u32>,
    _reserved20: [u8; 0x48],
    #[doc = "Bridge port configuration register"]
    pub BPCR: crate::RWRegister<u32>,
    _reserved21: [u8; 0x0c],
    #[doc = "Bridge port default VLAN register"]
    pub BPDVR: crate::RWRegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "Bridge port spanning tree group state register"]
    pub BPSTGSR: crate::RWRegister<u32>,
    _reserved23: [u8; 0x04],
    #[doc = "Bridge port storm control register 0"]
    pub BPSCR0: crate::RWRegister<u32>,
    #[doc = "Bridge port storm control register 1"]
    pub BPSCR1: crate::RWRegister<u32>,
    #[doc = "Bridge port operational register"]
    pub BPOR: crate::RORegister<u32>,
    _reserved24: [u8; 0x4c],
    #[doc = "Bridge port discard count register"]
    pub BPDCR: crate::RORegister<u32>,
    _reserved25: [u8; 0x04],
    #[doc = "Bridge port discard count reason register 0"]
    pub BPDCRR0: crate::RWRegister<u32>,
    #[doc = "Bridge port discard count reason register 1"]
    pub BPDCRR1: crate::RWRegister<u32>,
    #[doc = "Bridge port MAC learning failure status register"]
    pub BPMLFSR: crate::RWRegister<u32>,
}
#[doc = "Port capability register"]
pub mod PCAPR {
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
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported Formula: NUM_Q+1 Valid if link is bound to a switch"]
    pub mod NUM_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of congestion groups supported Formula: NUM_CG+1 Valid if link end is bound to a switch port"]
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
#[doc = "Port MAC capability register"]
pub mod PMCAPR {
    #[doc = "MAC Variant"]
    pub mod MAC_VAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress frame padding capability indicates if egress frames smaller than 64B are padded with zeroes"]
    pub mod EFPAD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if configurable Preamble and IPG is supported 0: Static Inter Frame Gap (IPG) size is 12B and Preamble is 7B 1: Configurable IPG and Preamble size Valid if MAC_VAR=1"]
    pub mod PIPG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if Half Duplex Mode is supported on this link"]
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
#[doc = "Port I/O capability register"]
pub mod PIOCAPR {
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
#[doc = "Port configuration register"]
pub mod PCR {
    #[doc = "Indicates the frame format received/transmitted on the port 0 Ethernet frame format 1 Reserved"]
    pub mod HDR_FMT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L2 Ethernet DoS Protection Enable 0 L2 IP DoS protection is disabled"]
    pub mod L2DOSE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Clock Selection: On receive, this field determines which of the two Rx timestamps (synchronized or free running) is reported to the host"]
    pub mod TIMER_CS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Port Speed The speed in 10Mbps increments at which the port is assumed to be running for scheduling purposes and to determine if cut-through forwarding can proceed"]
    pub mod PSPEED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC address register 0"]
pub mod PMAR0 {
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    pub mod PRIM_MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC address register 1"]
pub mod PMAR1 {
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    pub mod PRIM_MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port TPID acceptance register"]
pub mod PTAR {
    #[doc = "Outer VLAN TPID List : bitmap identifying which TPIDs are acceptable as Outer VLAN tag xxx1 Standard C-VLAN 0x8100 xx1x Standard S-VLAN 0x88A8 x1xx Custom VLAN as defined by CVLANR1\\[ETYPE\\] 1xxx Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    pub mod OVTPIDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Outer VLAN TPID List : bitmap identifying which TPIDs are acceptable as Inner VLAN tag xxx1 Standard C-VLAN 0x8100 xx1x Standard S-VLAN 0x88A8 x1xx Custom VLAN as defined by CVLANR1\\[ETYPE\\] 1xxx Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    pub mod IVTPIDL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port QoS mode register"]
pub mod PQOSMR {
    #[doc = "VLAN tag select: 0 Inner VLAN tag will be used if VE is set 1 Outer VLAN tag will be used if VE is set"]
    pub mod VS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN enable 0 Defaults are used 1 Enables use of VLAN info to determine IPV and DR (based on VLANIPVMPaR0/1 and VLANDRMPaR)"]
    pub mod VE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default DR Sets the default DR for the port."]
    pub mod DDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default IPV Sets the default IPV for the port."]
    pub mod DIPV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping profile index"]
    pub mod VQMP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping profile index"]
    pub mod QVMP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port Queue Operational register"]
pub mod PQOR {
    #[doc = "Egress Traffic Management (ETM) class queue 0's state where i={0"]
    pub mod Q0S {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Traffic Management (ETM) class queue 1's state where i={0"]
    pub mod Q1S {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Traffic Management (ETM) class queue 2's state where i={0"]
    pub mod Q2S {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Traffic Management (ETM) class queue 3's state where i={0"]
    pub mod Q3S {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Traffic Management (ETM) class queue 4's state where i={0"]
    pub mod Q4S {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Traffic Management (ETM) class queue 5's state where i={0"]
    pub mod Q5S {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Traffic Management (ETM) class queue 6's state where i={0"]
    pub mod Q6S {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Traffic Management (ETM) class queue 7's state where i={0"]
    pub mod Q7S {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port parser configuration register"]
pub mod PPCR {
    #[doc = "Unused"]
    pub mod L1PFS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L2 payload fields size in bytes This is the largest L2 payload size used by any table lookup key by this port"]
    pub mod L2PFS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L3 header fields present"]
    pub mod L3HFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No L3 header present. Indicates to the parser of not parsing the L3 header. Parsing in this case would go as far as the L2 header regardless of whether or not there is an L3 header in the frame. This option should not be used if there are any table lookup entries that contain L3/L4 key fields that could be matched against a frame."]
            pub const L3_HDR_NOT_PRESET: u32 = 0;
            #[doc = "Parse L3 header if present in the frame."]
            pub const L3_HDR_PRESENT: u32 = 0x01;
        }
    }
    #[doc = "L3 payload fields size in bytes"]
    pub mod L3PFS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L4 Header fields present"]
    pub mod L4HFP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No L4 header present. Indicates to the parser of not parsing the L4 header. Parsing in this case would go as far as the L3 header if configured to parse it (L3HFP=1) regardless of whether or not there is an L4 header in the frame. This option should not be used if there are any table lookup entries that contain L4 key fields that could be matched against a frame."]
            pub const L4_HDR_NOT_PRESENT: u32 = 0;
            #[doc = "Parse L4 header if present in the frame"]
            pub const L4_HDR_PRESENT: u32 = 0x01;
        }
    }
    #[doc = "L4 payload fields size in bytes"]
    pub mod L4PFS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port ingress port filter configuration register"]
pub mod PIPFCR {
    #[doc = "0 Ingress Port Filter table lookup is disabled for this port 1 Ingress Port Filter table lookup is enabled for this port"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port stream gate configuration register"]
pub mod PSGCR {
    #[doc = "Link propagation delay in nanoseconds"]
    pub mod PDELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stream Gate Open Gate Check mode"]
    pub mod OGC {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port operational register"]
pub mod POR {
    #[doc = "Tx Disable."]
    pub mod TXDIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tx path is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Tx path is disabled."]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Rx Disable."]
    pub mod RXDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx path is enabled."]
            pub const ENABLE: u32 = 0;
            #[doc = "Rx path is disabled."]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "Port status register"]
pub mod PSR {
    #[doc = "Transmit busy."]
    pub mod TX_BUSY {
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
    #[doc = "Receive busy."]
    pub mod RX_BUSY {
        pub const offset: u32 = 1;
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
#[doc = "Port receive SDU overhead register"]
pub mod PRXSDUOR {
    #[doc = "PPDU Byte count overhead"]
    pub mod PPDU_BCO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MACSec byte count overhead"]
    pub mod MACSEC_BCO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port transmit SDU overhead register"]
pub mod PTXSDUOR {
    #[doc = "PPDU Byte count overhead"]
    pub mod PPDU_BCO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MACSec byte count overhead"]
    pub mod MACSEC_BCO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port time gate scheduling control register"]
pub mod PTGSCR {
    #[doc = "Time Gating Enable"]
    pub mod TGE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Time gating disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Time gating enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Port time gate scheduling admin gate list status register"]
pub mod PTGAGLSR {
    #[doc = "Time gated (state of the operational gate control list) 0 No operational gate control list is active 1 Operational gate control list is active"]
    pub mod TG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Administrative gate control list pending 0 No administrative gate control list is configured 1 Administrative gate control list is pending (configured but not installed yet)"]
    pub mod CFG_PEND {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port time gate scheduling admin gate list length register"]
pub mod PTGAGLLR {
    #[doc = "Administrative gate control list length (number of entries)"]
    pub mod ADMIN_GATE_LIST_LENGTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port time gating operational gate list length register"]
pub mod PTGOGLLR {
    #[doc = "Operational gate control list length (number of entries)"]
    pub mod OPER_GATE_LIST_LENGTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port default gate state register"]
pub mod PDGSR {
    #[doc = "Default Gate State for Traffic Class 0 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Default Gate State for Traffic Class 1 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Default Gate State for Traffic Class 2 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Default Gate State for Traffic Class 3 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Default Gate State for Traffic Class 4 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Default Gate State for Traffic Class 5 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Default Gate State for Traffic Class 6 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Default Gate State for Traffic Class 7 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
    }
}
#[doc = "Port Rx discard count register"]
pub mod PRXDCR {
    #[doc = "Number of receive frames discarded"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port Rx discard count reason register 0"]
pub mod PRXDCRR0 {
    #[doc = "Pre-Classification Discard Reason Occurs if Parse Classifier Engine (PCE) block does not have any free processing thread to process the received frame"]
    pub mod PCDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shared Memory Resource Exhaustion Discard Reason Occurs if Ethernet Rx I/F cannot allocate shared internal buffer memory to store the entire received frame"]
    pub mod SMREDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive disable discard reason due to port being in receive disabled state (POR\\[RXDIS\\]=1)"]
    pub mod RXDISDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress Port Filter Discard Reason. Cleared when written to 1."]
    pub mod IPFDR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rate Policer Discard Reason"]
    pub mod RPDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress Stream Forwarding Discard Reason Occurs if frame is associated with an ingress stream whose Forwarding Action (FA) is equal to discard"]
    pub mod ISFDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stream Gate Closed Discard Reason Frame received on this port that is dropped because it didn't pass the stream gate"]
    pub mod SGCDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stream Gate Octet Exceeded Discard Reason Frame dropped due to octet count exceeded maximum specified for the open gate interval Cleared when written to 1"]
    pub mod SGOEDR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress Stream Maximum Service Data Unit Exceeded Discard Reason Occurs if packet received is greater than the Maximum SDU size configured for the stream"]
    pub mod MSDUEDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Modification Misconfiguration Error Discard Reason Cleared when written to 1"]
    pub mod FMMEDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Congestion Management Discard Reason Frame discarded due to insufficient amount of memory space in buffer pool or in switch shared internal buffer memory"]
    pub mod CMDR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invalid Table Entry Discard Reason Table entry ID reference is not found in the table (entry has not been added to the table) Table entry ID is outside of its table allocation range Cleared when written to 1"]
    pub mod ITEDR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC Error Discard Reason"]
    pub mod ECCEDR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 2 Denial of Service Discard Reason Occurs if a packet is discarded due to L2DOS protection"]
    pub mod L2DOSDR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No Destination Discard Reason"]
    pub mod NODESTDR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port Rx discard count reason register 1"]
pub mod PRXDCRR1 {
    #[doc = "Entry Id who last incremented discard count"]
    pub mod ENTRYID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Table type which caused the discard"]
    pub mod TT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port Tx discard count register"]
pub mod PTXDCR {
    #[doc = "Number of transmit frames discarded"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port Tx discard count reason register 0"]
pub mod PTXDCRR0 {
    #[doc = "Transmit disable discard reason due to port being in transmit disabled state (POR\\[TXDIS\\]=1)"]
    pub mod TXDISDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECC Error Discard Reason"]
    pub mod ECCEDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity Error Discard Reason"]
    pub mod PEDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Gate Scheduling Frame Too Large Discard Reason Time Gate scheduling is enabled, and the frame was discarded because one of the following conditions was encountered: The frame was too large to transmit during any gate-open interval"]
    pub mod TGSFTLDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Modification Misconfiguration Discard Reason Software Misconfiguration due to: Invalid frame header modification (i"]
    pub mod FMMDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Disable prior to Enqueue to ETM Discard Reason due to Port being in a Transmit disabled state (POR\\[TXDIS\\]=1)"]
    pub mod TXDISEDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Maximum Service Data Unit Exceeded Discard Reason Occurs if store-and-forward frame is greater than the port Traffic Class's configured Maximum SDU size"]
    pub mod MSDUEDR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue Congestion Discard Reason Occurs if a frame is dropped because it couldn't be enqueued to an Egress Traffic Management (ETM) class queue"]
    pub mod QCONGDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invalid Table Entry Discard Reason"]
    pub mod ITEDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invalid Enqueue Port or Queue Discard Reason"]
    pub mod INVEQDR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Stream Sequence Recovery Take No Sequence Discard Reason Occurs if the sequence recovery function discards a frame because it is tagless and ESQA_TGT_EID\\[SQR_TNSQ\\]=0"]
    pub mod SQRTNSQDR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Stream Sequence Recovery Rogue Discard Reason Occurs if a packet is discarded by the vector recovery algorithm because its sequence number has fallen outside of the recovery history window"]
    pub mod SQRRDR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Stream Sequence Recovery Duplicate Discard Reason Occurs if packet's sequence_number is a duplicate applies for either Match or Vector Recovery Algorithm)"]
    pub mod SQRDDR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shared Memory Resource Exhaustion Discard Reason"]
    pub mod SMREDR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port Tx discard count reason register 1"]
pub mod PTXDCRR1 {
    #[doc = "Entry Id who last incremented discard count"]
    pub mod ENTRYID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Table type which was last accessed when frame was discarded"]
    pub mod TT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port buffer pool mapping configuration register 0"]
pub mod PBPMCR0 {
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    pub mod IPV0_INDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    pub mod IPV1_INDEX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    pub mod IPV2_INDEX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    pub mod IPV3_INDEX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port buffer pool mapping configuration register 1"]
pub mod PBPMCR1 {
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    pub mod IPV4_INDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    pub mod IPV5_INDEX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    pub mod IPV6_INDEX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    pub mod IPV7_INDEX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port PCP DEI mapping register"]
pub mod PPCPDEIMR {
    #[doc = "Ingress PCP to PCP Mapping Profile instance"]
    pub mod IPCPMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress PCP to PCP Mapping Profile is valid. Only applicable if outer VLAN tag is present."]
    pub mod IPCPMPV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Ingress PCP to PCP Mapping Profile is not valid."]
            pub const INVALID: u32 = 0;
            #[doc = "Ingress frame modification of outer VLAN tag's PCP value is mapped to a new value based on IPCPMP instance"]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Egress PCP to PCP Mapping Profile instance"]
    pub mod EPCPMP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress PCP to PCP Mapping Profile is valid."]
    pub mod EPCPMPV {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Egress PCP to PCP Mapping Profile is not valid."]
            pub const INVALID: u32 = 0;
            #[doc = "Egress frame modification of outer VLAN tag's PCP value is mapped to a new value based on EPCPMP instance."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Mapping of internal QoS's DR value 0 to VLAN DEI."]
    pub mod DR0DEI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal QoS's DR value 1 to VLAN DEI."]
    pub mod DR1DEI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal QoS's DR value 2 to VLAN DEI."]
    pub mod DR2DEI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal QoS's DR value 3 to VLAN DEI."]
    pub mod DR3DEI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if mapping of internal QoS's DR value d to VLAN DEI is enabled."]
    pub mod DRME {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Preserve the DR value in the outer VLAN."]
            pub const PRESERVE: u32 = 0;
            #[doc = "Update DR value in the outer VLAN based on DEnDEI field."]
            pub const UPDATE: u32 = 0x01;
        }
    }
}
#[doc = "Port mirror configuration register"]
pub mod PMCR {
    #[doc = "Enable Ingress Mirroring on the port"]
    pub mod IMIRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port LANID configuration register"]
pub mod PLANIDCR {
    #[doc = "LAN Identifier"]
    pub mod LANID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port ingress stream identification configuration register"]
pub mod PISIDCR {
    #[doc = "Indicates which Ingress Stream Identification Key Construction pair to use for this port"]
    pub mod KCPAIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key Construction 0 Enable"]
    pub mod KC0EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key Construction 1 Enable"]
    pub mod KC1EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Default Ingress Stream Entry ID"]
    pub mod ISEID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port frame modification configuration register"]
pub mod PFMCR {
    #[doc = "Frame Modification Misconfiguration Action"]
    pub mod FMMA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Discard the frame and counted against the port's Tx discard count register (PTXDCR) along with the setting of the Frame Modification Misconfiguration Discard Reason (FMMDR) flag to 1 in the port's Tx discard count reason register 0 (PTXDCRR0)."]
            pub const DISCARD: u32 = 0;
            #[doc = "Transmit the frame without performing any of the ingress and egress frame modification actions specified."]
            pub const TRANSMIT: u32 = 0x01;
        }
    }
}
#[doc = "Port IPV to queue mapping register 0"]
pub mod PIPV2QMR0 {
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    pub mod IPV0_Q {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    pub mod IPV1_Q {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    pub mod IPV2_Q {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    pub mod IPV3_Q {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    pub mod IPV4_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    pub mod IPV5_Q {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    pub mod IPV6_Q {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    pub mod IPV7_Q {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port time capture minimum latency register"]
pub mod PTCMINLR {
    #[doc = "Indicates the minimum latency between the Tx Timestamp and Rx Timestamp captured by the Ethernet MAC relative to SFD"]
    pub mod LATENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the number of times a frame transmitted out of this port, has fulfilled the timestamp capture criteria"]
    pub mod COUNT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port time capture maximum latency register"]
pub mod PTCMAXLR {
    #[doc = "Indicates the maximum latency between the Tx Timestamp and Rx Timestamp captured by the Ethernet MAC relative to SFD"]
    pub mod LATENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port configuration register"]
pub mod BPCR {
    #[doc = "Dynamic Limit"]
    pub mod DYN_LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unknown Unicast Storm Control Enable"]
    pub mod UUCASTE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Unknown Multicast Storm Control Enable"]
    pub mod UMCASTE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Multicast Storm Control Enable"]
    pub mod MCASTE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Broadcast Storm Control Enable"]
    pub mod BCASTE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Station Move Disallow"]
    pub mod STAMVD {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Disallowed. A received frame for which a MAC station move is detected, will be discarded."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Source Port Pruning Disable"]
    pub mod SRCPRND {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "Bridge port default VLAN register"]
pub mod BPDVR {
    #[doc = "VLAN identifier Specifies the VID value to be used to construct or modify the internal VLAN header."]
    pub mod VID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Drop eligible indicator This field specifies the 1-bit DEI to be used to construct the internal VLAN header"]
    pub mod DEI {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priority code point This field specifies the 3-bit PCP to be used to construct the internal VLAN header"]
    pub mod PCP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Specifies the TPID value to be used to construct the internal VLAN header"]
    pub mod TPID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Bridge Port Tag Acceptance Criteria xxx0 Discard untagged xxx1 Accept untagged xx0x Discard priority tagged xx1x Accept priority tagged x0xx Discard single tagged x1xx Accept single tagged 0xxx Discard double tagged 1xxx Accept double tagged Frames discarded are counted against the bridge port discard count register (BPDCR) along with the setting of the Discard due to Bridge Port Acceptance Criteria Discard Reason (BPACDR) flag to 1 in the Bridge port discard count reason register 0 (BPDCRR0)"]
    pub mod RXTAGA {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive VLAN Aware Mode All frames to be forwarded using the 802"]
    pub mod RXVAM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Bridge Port VLAN Tag Action This field applies only for a VLAN aware bridge, where the frame outer VLAN tag's VID is equal to the port default VID (VID field in this register)"]
    pub mod TXTAGA {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port spanning tree group state register"]
pub mod BPSTGSR {
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    pub mod STG_STATE15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port storm control register 0"]
pub mod BPSCR0 {
    #[doc = "Unknown unicast Rate Policer Entry ID"]
    pub mod UUCASTRPEID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Broadcast Rate Policer Entry ID"]
    pub mod BCASTRPEID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port storm control register 1"]
pub mod BPSCR1 {
    #[doc = "Known multicast Rate Policer Entry ID"]
    pub mod MCASTRPEID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unknown multicast Rate Policer Entry ID"]
    pub mod UMCASTRPEID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port operational register"]
pub mod BPOR {
    #[doc = "Number of FDB entries dynamically learned against this port."]
    pub mod NUM_DYN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port discard count register"]
pub mod BPDCR {
    #[doc = "Frame discard count."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port discard count reason register 0"]
pub mod BPDCRR0 {
    #[doc = "Discard due to Bridge Port Acceptance Criteria Discard Reason Frame was discarded because it failed the Bridge Port Tag Acceptance Criteria"]
    pub mod BPACDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress STG State Discard Reason Occurs if port's STG State is Disabled or Learning"]
    pub mod ISTGSDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Filter (Port VLAN Membership) Discard Reason Occurs when VID (after ingress frame modification if applicable) received is not a member of this port"]
    pub mod BPVFLTDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Learn Not Found Discard Reason Occurs if VFHTDECR2\\[MLO\\] or VLAN Filter Table Entry's MLO = 5 (Disable MAC learning with SMAC validation)"]
    pub mod MACLNFDR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Station Move Disallowed Discard Reason Occurs if STAMVD=1 and Source MAC found in FDB with port not matching received port"]
    pub mod STAMVDDR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Forwarding Default Discard Reason Occurs if MFO = 3 (FDB lookup is performed, and if there is no match, the frame is discarded), MFO is settable by BPCR and VLAN Filter Table Entry"]
    pub mod MACFDDDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No Destination Discard Reason L2 forwarding decision resulted in discarding the frame since it resulted in no destination"]
    pub mod NODESTDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Multicast Filter Discard Reason Frame was discarded due to a miss in L2 IPV4 Multicast Filter table"]
    pub mod IPMFDR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Untagged Frame Modification Misconfiguration Discard Reason Frame was discarded due to bridge port configured as VLAN aware with a non-null Ingress Frame Modification Entry ID (IFM_EID) resulting in the frame containing no VLAN headers, or with an outer priority tag (VID=0)"]
    pub mod UFMMDR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Misconfiguration Discard Reason"]
    pub mod MISCDR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Discard due to Storm Control Policer Discard Reason"]
    pub mod STRMCTRLDR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port discard count reason register 1"]
pub mod BPDCRR1 {
    #[doc = "Entry ID who last incremented Discard Count"]
    pub mod ENTRYID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Table type which caused the discard"]
    pub mod TT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bridge port MAC learning failure status register"]
pub mod BPMLFSR {
    #[doc = "Bridge Port MAC Learn Limit Reached Failure Reason"]
    pub mod BPMLLRFR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Full FDB Table Reached Failure Reason"]
    pub mod FFDBTRFR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hash Collision chain limit Reached Failure Reason"]
    pub mod HCCLRFR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod tctnum {
    #[doc = "Array of registers: PTGSTCSR, PTCTMSDUR, PTCCBSR0, PTCCBSR1"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Port time gate scheduling traffic class a status register"]
        pub PTGSTCSR: crate::RORegister<u32>,
        _reserved0: [u8; 0x04],
        #[doc = "Port traffic class a transmit maximum SDU register"]
        pub PTCTMSDUR: crate::RWRegister<u32>,
        _reserved1: [u8; 0x04],
        #[doc = "Port transmit traffic class a credit based shaper register 0"]
        pub PTCCBSR0: crate::RWRegister<u32>,
        #[doc = "Port traffic class a credit based shaper register 1"]
        pub PTCCBSR1: crate::RWRegister<u32>,
        _reserved2: [u8; 0x08],
    }
    #[doc = "Port time gate scheduling traffic class a status register"]
    pub mod PTGSTCSR {
        #[doc = "Look-ahead gate state 0 Gate closed 1 Gate open IERB registers SaTGSLR / EaTGSLR\\[MIN_LOOKAHEAD\\] + the per port register PTGSATOR\\[ADV_TIME_OFFSET\\] specify the advance time of the gate state"]
        pub mod LH_STATE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Port traffic class a transmit maximum SDU register"]
    pub mod PTCTMSDUR {
        #[doc = "Transmit Maximum SDU size in bytes (i"]
        pub mod MAXSDU {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Specifies the type of PDU/SDU (Protocol/Service Data Unit) whose length is being validated as seen on the link"]
        pub mod SDU_TYPE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "When cleared, the Tx Max SDU check is performed for Store and Forward frames"]
        pub mod SF_MAXSDU_DIS {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Port transmit traffic class a credit based shaper register 0"]
    pub mod PTCCBSR0 {
        #[doc = "Bandwidth"]
        pub mod BW {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x7f << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Credit Based Shaper Enable"]
        pub mod CBSE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enabled"]
                pub const ENABLE: u32 = 0x01;
            }
        }
    }
    #[doc = "Port traffic class a credit based shaper register 1"]
    pub mod PTCCBSR1 {
        #[doc = "hicredit (in credit units)"]
        pub mod HI_CREDIT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
