#[doc = "ENETC base"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ENETC capability register 0"]
    pub ECAPR0: crate::RORegister<u32>,
    #[doc = "ENETC capability register 1"]
    pub ECAPR1: crate::RORegister<u32>,
    #[doc = "ENETC capability register 2"]
    pub ECAPR2: crate::RORegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Port mode register"]
    pub PMR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x6c],
    #[doc = "Port outer native VLAN register"]
    pub PONVLANR: crate::RWRegister<u32>,
    #[doc = "Port inner native VLAN register"]
    pub PINVLANR: crate::RWRegister<u32>,
    #[doc = "Port VLAN classification control register"]
    pub PVCLCTR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x10],
    #[doc = "Parser checksum configuration register"]
    pub PARCSCR: crate::RWRegister<u32>,
    #[doc = "Parser custom Ethertype i configuration register"]
    pub PARCECR: [crate::RWRegister<u32>; 4usize],
    _reserved3: [u8; 0x58],
    #[doc = "Port pause ON threshold register"]
    pub PPAUONTR: crate::RWRegister<u32>,
    #[doc = "Port pause OFF threshold register"]
    pub PPAUOFFTR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x10],
    #[doc = "Port receive memory buffer entitlement register"]
    pub PRXMBER: crate::RORegister<u32>,
    #[doc = "Port receive memory buffer limit register"]
    pub PRXMBLR: crate::RORegister<u32>,
    #[doc = "Port receive buffer count register"]
    pub PRXBCR: crate::RORegister<u32>,
    #[doc = "Port receive buffer count high watermark register"]
    pub PRXBCHWMR: crate::RORegister<u32>,
    _reserved5: [u8; 0x10],
    #[doc = "Set of registers which provides number of frame discarded by the Ingress Congestion Manager."]
    pub PICDRADCR: [picdradcr::RegisterBlock; 4usize],
    #[doc = "Port ingress congestion priority discard status register"]
    pub PICPDSR: crate::RWRegister<u32>,
    _reserved6: [u8; 0x7c],
    #[doc = "Port station interface promiscuous MAC mode register"]
    pub PSIPMMR: crate::RWRegister<u32>,
    #[doc = "Port station interface promiscuous VLAN mode register"]
    pub PSIPVMR: crate::RWRegister<u32>,
    #[doc = "Port broadcast frames dropped due to MAC filtering register"]
    pub PBFDSIR: crate::RORegister<u32>,
    #[doc = "Port frame drop MAC source address pruning register"]
    pub PFDMSAPR: crate::RORegister<u32>,
    _reserved7: [u8; 0x70],
    #[doc = "Port station interface MAC address filtering capability register"]
    pub PSIMAFCAPR: crate::RORegister<u32>,
    #[doc = "Port unicast frames dropped due to MAC filtering register"]
    pub PUFDMFR: crate::RORegister<u32>,
    #[doc = "Port multicast frames dropped due to MAC filtering register"]
    pub PMFDMFR: crate::RORegister<u32>,
    _reserved8: [u8; 0x34],
    #[doc = "Port station interface VLAN filtering capability register"]
    pub PSIVLANFCAPR: crate::RORegister<u32>,
    #[doc = "Port station interface VLAN filtering mode register"]
    pub PSIVLANFMR: crate::RWRegister<u32>,
    _reserved9: [u8; 0x08],
    #[doc = "Port unicast frames dropped VLAN filtering register"]
    pub PUFDVFR: crate::RORegister<u32>,
    #[doc = "Port multicast frames dropped VLAN filtering register"]
    pub PMFDVFR: crate::RORegister<u32>,
    #[doc = "Port broadcast frames dropped VLAN filtering register"]
    pub PBFDVFR: crate::RORegister<u32>,
    _reserved10: [u8; 0x64],
    #[doc = "Port low power mode register"]
    pub PLPMR: crate::RWRegister<u32>,
    #[doc = "Port wake-on status register"]
    pub PWOSR: crate::RORegister<u32>,
    _reserved11: [u8; 0x28],
    #[doc = "Receive IPV to ICM priority mapping register 0"]
    pub IPV2ICMPMR0: crate::RWRegister<u32>,
    _reserved12: [u8; 0x0c],
    #[doc = "Transmit priority to traffic class mapping register 0"]
    pub PRIO2TCMR0: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "Port traffic class a time specific departure register"]
    pub PTCTSDR: [crate::RWRegister<u32>; 8usize],
    _reserved14: [u8; 0x0450],
    #[doc = "Switch management capability register"]
    pub SMCAPR: crate::RORegister<u32>,
    _reserved15: [u8; 0x17fc],
    #[doc = "Port station interface 0 primary MAC address register 0"]
    pub PSI0PMAR0: crate::RORegister<u32>,
    #[doc = "Port station interface 0 primary MAC address register 1"]
    pub PSI0PMAR1: crate::RORegister<u32>,
    #[doc = "Port station interface 0 VLAN register"]
    pub PSI0VLANR: crate::RWRegister<u32>,
    _reserved16: [u8; 0x04],
    #[doc = "Port station interface 0 configuration register 0"]
    pub PSI0CFGR0: crate::RWRegister<u32>,
    _reserved17: [u8; 0x04],
    #[doc = "Port station interface 0 configuration register 2"]
    pub PSI0CFGR2: crate::RWRegister<u32>,
    _reserved18: [u8; 0x14],
    #[doc = "Port station interface 0 VSI MAC address filtering configuration register"]
    pub PSI0VMAFCFGR: crate::RWRegister<u32>,
    #[doc = "Port station interface 0 VLAN filtering configuration register"]
    pub PSI0VLANFCFGR: crate::RWRegister<u32>,
    _reserved19: [u8; 0x18],
    #[doc = "Port station interface 0 unicast MAC hash filter register 0"]
    pub PSI0UMHFR0: crate::RWRegister<u32>,
    #[doc = "Port station interface 0 unicast MAC hash filter register 1"]
    pub PSI0UMHFR1: crate::RWRegister<u32>,
    #[doc = "Port station interface 0 multicast MAC hash filter register 0"]
    pub PSI0MMHFR0: crate::RWRegister<u32>,
    #[doc = "Port station interface 0 multicast MAC hash filter register 1"]
    pub PSI0MMHFR1: crate::RWRegister<u32>,
    #[doc = "Port station interface 0 VLAN hash filter register 0"]
    pub PSI0VHFR0: crate::RWRegister<u32>,
    #[doc = "Port station interface 0 VLAN hash filter register 1"]
    pub PSI0VHFR1: crate::RWRegister<u32>,
}
#[doc = "ENETC capability register 0"]
pub mod ECAPR0 {
    #[doc = "Receive flow steering support 0: Not supported 1: Supported"]
    pub mod RFS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Support for time specific departure. 0: Not supported 1: Supported"]
    pub mod TSD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Support for RSS 0: Not supported 1: Supported"]
    pub mod RSS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Support for Wake-on-LAN in low-power mode 0: Not supported 1: Supported"]
    pub mod WO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Functional safety capability supported."]
    pub mod FS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC capability register 1"]
pub mod ECAPR1 {
    #[doc = "Number of traffic classes 0 - 1 Traffic class (0) 1 - 2 Traffic classes (0-1)"]
    pub mod NUM_TCS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of multi-cast hash entry (1bit/entry) per SI for L2 MAC Filtering 00: 64 multi-cast bins 01: 128 multi-cast bins 10: 256 multi-cast bins 11: 512 multi-cast bins"]
    pub mod NUM_MCH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of uni-cast hash entry (1bit/entry) per SI for L2 MAC Filtering 00: 64 unicast bins 01: 128 unicast bins 10: 256 unicast bins 11: 512 unicast bins"]
    pub mod NUM_UCH {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of MSI-X"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the number of VSI supported"]
    pub mod NUM_VSI {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the number of IPVs supported. 0: 8 IPVs 1: 16 IPVs"]
    pub mod NUM_IPV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ENETC capability register 2"]
pub mod ECAPR2 {
    #[doc = "Number of total transmit buffer descriptor rings assigned to ENETC Range: 0..1023"]
    pub mod NUM_TX_BDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of total receive buffer descriptor rings assigned to ENETC Range: 0..1023"]
    pub mod NUM_RX_BDR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port mode register"]
pub mod PMR {
    #[doc = "Enable station interface 0"]
    pub mod SI0EN {
        pub const offset: u32 = 16;
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
#[doc = "Port outer native VLAN register"]
pub mod PONVLANR {
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    pub mod VID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    pub mod DEI {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    pub mod PCP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Identifies which known VLAN Ethertype is used"]
    pub mod TPID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard C-VLAN 0x8100"]
            pub const C_VLAN: u32 = 0;
            #[doc = "Standard S-VLAN 0x88A8"]
            pub const S_VLAN: u32 = 0x01;
            #[doc = "Custom VLAN as defined by CVLANR1\\[ETYPE\\]"]
            pub const CVLANR1_ETYPE: u32 = 0x02;
            #[doc = "Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
            pub const CVLANR2_ETYPE: u32 = 0x03;
        }
    }
    #[doc = "Port Native VLAN Enable"]
    pub mod PNE {
        pub const offset: u32 = 18;
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
    #[doc = "VID 0 Enable"]
    pub mod VZE {
        pub const offset: u32 = 19;
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
#[doc = "Port inner native VLAN register"]
pub mod PINVLANR {
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    pub mod VID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    pub mod DEI {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    pub mod PCP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Identifies which known VLAN Ethertype is used"]
    pub mod TPID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard C-VLAN 0x8100"]
            pub const C_VLAN: u32 = 0;
            #[doc = "Standard S-VLAN 0x88A8"]
            pub const S_VLAN: u32 = 0x01;
            #[doc = "Custom VLAN as defined by CVLANR1\\[ETYPE\\]"]
            pub const CVLANR1_ETYPE: u32 = 0x02;
            #[doc = "Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
            pub const CVLANR2_ETYPE: u32 = 0x03;
        }
    }
    #[doc = "Port Native VLAN Enable"]
    pub mod PNE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "VID 0 Enable"]
    pub mod VZE {
        pub const offset: u32 = 19;
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
#[doc = "Port VLAN classification control register"]
pub mod PVCLCTR {
    #[doc = "Outer as Inner"]
    pub mod OAI {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Indicates that the Inner is not valid if only one tag is found"]
            pub const NOT_VALID: u32 = 0;
            #[doc = "Indicates that the outer should be used as the Inner if only one tag is found"]
            pub const OUTER: u32 = 0x01;
        }
    }
}
#[doc = "Parser checksum configuration register"]
pub mod PARCSCR {
    #[doc = "Layer 4 TCP and UDP checksum validation Disable."]
    pub mod L4CD {
        pub const offset: u32 = 0;
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
    #[doc = "Layer 3 IPv4 Header checksum validation Disable."]
    pub mod L3CD {
        pub const offset: u32 = 1;
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
#[doc = "Parser custom Ethertype i configuration register"]
pub mod PARCECR {
    #[doc = "Code Point"]
    pub mod CP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable"]
    pub mod EN {
        pub const offset: u32 = 5;
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
    #[doc = "ETYPE"]
    pub mod ETYPE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pause ON threshold register"]
pub mod PPAUONTR {
    #[doc = "Monitors the amount of data accumulated in the receive buffer and if this amount exceeds the Pause ON threshold (expressed in words), it then enters the \"Pause ON\" state if Pause is enabled"]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port pause OFF threshold register"]
pub mod PPAUOFFTR {
    #[doc = "Monitors the amount of data accumulated in the receive buffer and if this amount goes below the Pause OFF threshold (expressed in words) it then enters the \"Pause OFF\" state if Pause is enabled"]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port receive memory buffer entitlement register"]
pub mod PRXMBER {
    #[doc = "Receive memory buffer entitlement in words This receive Memory Buffer Entitlement is used in determining smart drop for ingress congestion control"]
    pub mod AMOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port receive memory buffer limit register"]
pub mod PRXMBLR {
    #[doc = "Receive buffer memory limit in words"]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port receive buffer count register"]
pub mod PRXBCR {
    #[doc = "Current receive buffer usage count in words for this port."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port receive buffer count high watermark register"]
pub mod PRXBCHWMR {
    #[doc = "High watermark in words for receive buffer usage since the last read of this register"]
    pub mod WATERMARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port ingress congestion priority discard status register"]
pub mod PICPDSR {
    #[doc = "DR0 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    pub mod DR0_P0DS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DR0 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    pub mod DR0_P1DS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DR1 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    pub mod DR1_P0DS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DR1 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    pub mod DR1_P1DS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DR2 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    pub mod DR2_P0DS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DR2 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    pub mod DR2_P1DS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DR3 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    pub mod DR3_P0DS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DR3 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    pub mod DR3_P1DS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface promiscuous MAC mode register"]
pub mod PSIPMMR {
    #[doc = "SI 0 MAC unicast promiscuous"]
    pub mod SI0_MAC_UP {
        pub const offset: u32 = 0;
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
    #[doc = "SI 0 MAC multicast promiscuous"]
    pub mod SI0_MAC_MP {
        pub const offset: u32 = 16;
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
#[doc = "Port station interface promiscuous VLAN mode register"]
pub mod PSIPVMR {
    #[doc = "SI 0 VLAN promiscuous. This field specifies if SI 0 qualifies for the reception of all VLAN tags."]
    pub mod SI0_VLAN_P {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SI 0 does not qualify for the reception of all VLAN tags"]
            pub const NOT_QUALIFY: u32 = 0;
            #[doc = "SI 0 does qualify for the reception of all VLAN tags"]
            pub const QUALIFY: u32 = 0x01;
        }
    }
    #[doc = "SI 0 Untagged frames (no VLAN) accepted"]
    pub mod SI0_VLAN_UTA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SI 0 does not qualify for reception of untagged frames"]
            pub const NOT_QUALIFIED: u32 = 0;
            #[doc = "SI 0 does qualify for reception of untagged frames"]
            pub const QUALIFY: u32 = 0x01;
        }
    }
}
#[doc = "Port broadcast frames dropped due to MAC filtering register"]
pub mod PBFDSIR {
    #[doc = "MAC filter broadcast frame drop counter bits 31-0"]
    pub mod FRAME_DROP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port frame drop MAC source address pruning register"]
pub mod PFDMSAPR {
    #[doc = "MAC source address pruning frame drop counter bits 31-0"]
    pub mod FRAME_DROP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface MAC address filtering capability register"]
pub mod PSIMAFCAPR {
    #[doc = "Number of SI MAC address filter rules per port."]
    pub mod NUM_MAC_AFTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port unicast frames dropped due to MAC filtering register"]
pub mod PUFDMFR {
    #[doc = "MAC filter unicast frame drop counter bits 31-0"]
    pub mod FRAME_DROP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port multicast frames dropped due to MAC filtering register"]
pub mod PMFDMFR {
    #[doc = "MAC filter multicast frame drop counter bits 31-0"]
    pub mod FRAME_DROP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface VLAN filtering capability register"]
pub mod PSIVLANFCAPR {
    #[doc = "Number of VLAN filter table entries per port. Range: 0..4K-1"]
    pub mod NUM_VLAN_FTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface VLAN filtering mode register"]
pub mod PSIVLANFMR {
    #[doc = "VLAN tag select"]
    pub mod VS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inner VLAN tag will be used for VLAN filtering"]
            pub const INNER_VLAN: u32 = 0;
            #[doc = "Outer VLAN tag will be used for VLAN filtering"]
            pub const OUTER_VLAN: u32 = 0x01;
        }
    }
}
#[doc = "Port unicast frames dropped VLAN filtering register"]
pub mod PUFDVFR {
    #[doc = "VLAN filter unicast frame drop counter bits 31-0"]
    pub mod FRAME_DROP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port multicast frames dropped VLAN filtering register"]
pub mod PMFDVFR {
    #[doc = "VLAN filter multicast frame drop counter bits 31-0"]
    pub mod FRAME_DROP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port broadcast frames dropped VLAN filtering register"]
pub mod PBFDVFR {
    #[doc = "VLAN filter broadcast frame drop counter bits 31-0"]
    pub mod FRAME_DROP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port low power mode register"]
pub mod PLPMR {
    #[doc = "Wake-on-LAN mode enable When Wake-on-LAN mode is enabled, ENETC will detect Wake-on-LAN events."]
    pub mod WME {
        pub const offset: u32 = 0;
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
#[doc = "Port wake-on status register"]
pub mod PWOSR {
    #[doc = "Wake-On-LAN active"]
    pub mod WOLA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inactive"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Active. ENETC is actively searching for frames matching the Wake-on-LAN event criteria."]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "ICM blocked"]
    pub mod ICMB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not blocked"]
            pub const NOT_BLOCKED: u32 = 0;
            #[doc = "Blocked."]
            pub const BLOCKED: u32 = 0x01;
        }
    }
}
#[doc = "Receive IPV to ICM priority mapping register 0"]
pub mod IPV2ICMPMR0 {
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    pub mod IPV0ICM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    pub mod IPV1ICM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    pub mod IPV2ICM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    pub mod IPV3ICM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    pub mod IPV4ICM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    pub mod IPV5ICM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    pub mod IPV6ICM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    pub mod IPV7ICM {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit priority to traffic class mapping register 0"]
pub mod PRIO2TCMR0 {
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    pub mod PRIO0TC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    pub mod PRIO1TC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    pub mod PRIO2TC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    pub mod PRIO3TC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    pub mod PRIO4TC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    pub mod PRIO5TC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    pub mod PRIO6TC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    pub mod PRIO7TC {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port traffic class a time specific departure register"]
pub mod PTCTSDR {
    #[doc = "Time specific Departure Enable The 1588 timer must be configured and enabled, and time gate scheduling for the port must be enabled (PTGSCR\\[TGE\\] set to 1), before enabling time specific departure on any traffic class"]
    pub mod TSDE {
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
#[doc = "Switch management capability register"]
pub mod SMCAPR {
    #[doc = "Switch Management"]
    pub mod SM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ENETC instance has no switch management capability"]
            pub const NO_SW: u32 = 0;
            #[doc = "ENETC instance has switch management capability"]
            pub const SW: u32 = 0x01;
        }
    }
}
#[doc = "Port station interface 0 primary MAC address register 0"]
pub mod PSI0PMAR0 {
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    pub mod PRIM_MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 primary MAC address register 1"]
pub mod PSI0PMAR1 {
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    pub mod PRIM_MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 VLAN register"]
pub mod PSI0VLANR {
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    pub mod VID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    pub mod DEI {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    pub mod PCP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tag protocol identifier"]
    pub mod TPID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard C-VLAN 0x8100"]
            pub const C_VLAN: u32 = 0;
            #[doc = "Standard S-VLAN 0x88A8"]
            pub const S_VLAN: u32 = 0x01;
            #[doc = "Custom VLAN as defined by CVLANR1\\[ETYPE\\]. Note that CVLANR1\\[V\\] is not checked for SI-based VLAN insertion; TPID value specified in CVLANR1\\[ETYPE\\] will be used to construct the VLAN header regardless of the value specified in CVLANR1\\[V\\]."]
            pub const CVLANR1_V: u32 = 0x02;
            #[doc = "Custom VLAN as defined by CVLANR2\\[ETYPE\\]. Note that CVLANR2\\[V\\] is not checked for SI-based VLAN insertion; TPID value specified in CVLANR2\\[ETYPE\\] will be used to construct the VLAN header regardless of the value specified in CVLANR2\\[V\\]."]
            pub const CVLANR2_V: u32 = 0x03;
        }
    }
    #[doc = "Enable"]
    pub mod E {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled; SI-based VLAN information is added on transmit and removed on receive."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Port station interface 0 configuration register 0"]
pub mod PSI0CFGR0 {
    #[doc = "Number of transmit buffer descriptor rings assigned to the SI Range: 0"]
    pub mod NUM_TX_BDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Pruning Enable"]
    pub mod SPE {
        pub const offset: u32 = 11;
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
    #[doc = "VLAN Tag Extract"]
    pub mod VTE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SI-based VLAN removal disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "SI-based VLAN removal enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SI-based VLAN Insertion Enable"]
    pub mod SIVIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SI-based VLAN insertion disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "SI-based VLAN insertion enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Anti-spoofing enable"]
    pub mod ASE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of receive buffer descriptor rings assigned to the SI Range: 0"]
    pub mod NUM_RX_BDR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Station interface VLAN control Determines which VLAN Ethertypes can be inserted by the SI driver (e"]
    pub mod SIVC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Station interface traffic class bandwidth weight Frames are selected for transmission between station interfaces based on a per traffic class basis using the Weighted Fair Bandwidth Sharing algorithm"]
    pub mod SIBW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 configuration register 2"]
pub mod PSI0CFGR2 {
    #[doc = "Number of MSI-X"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 VSI MAC address filtering configuration register"]
pub mod PSI0VMAFCFGR {
    #[doc = "Number of SI MAC address filter table entries assigned to the SI"]
    pub mod NUM_MAC_AFTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 VLAN filtering configuration register"]
pub mod PSI0VLANFCFGR {
    #[doc = "Number of VLAN filter table entries assigned to the SI"]
    pub mod NUM_VLAN_FTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 unicast MAC hash filter register 0"]
pub mod PSI0UMHFR0 {
    #[doc = "Lower 32-bits (31-0) of unicast MAC hash filter table indexed by unicast MAC address hash."]
    pub mod MAC_HASH_FLT_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 unicast MAC hash filter register 1"]
pub mod PSI0UMHFR1 {
    #[doc = "Upper 32-bits (63-32) of unicast MAC hash filter table indexed by unicast MAC address hash."]
    pub mod MAC_HASH_FLT_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 multicast MAC hash filter register 0"]
pub mod PSI0MMHFR0 {
    #[doc = "Lower 32-bits (31-0) of multicast MAC hash filter table indexed by unicast MAC address hash."]
    pub mod MAC_HASH_FLT_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 multicast MAC hash filter register 1"]
pub mod PSI0MMHFR1 {
    #[doc = "Upper 32-bits (63-32) of multicast MAC hash filter table indexed by unicast MAC address hash."]
    pub mod MAC_HASH_FLT_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 VLAN hash filter register 0"]
pub mod PSI0VHFR0 {
    #[doc = "Lower 32-bits (31-0) of VLAN hash filter table indexed by VLAN hash."]
    pub mod VLAN_HASH_FLT_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port station interface 0 VLAN hash filter register 1"]
pub mod PSI0VHFR1 {
    #[doc = "Upper 32-bits (63-32) of VLAN hash filter table indexed by VLAN hash."]
    pub mod VLAN_HASH_FLT_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod picdradcr {
    #[doc = "Set of registers which provides number of frame discarded by the Ingress Congestion Manager."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Port ingress congestion DRindex discard count register"]
        pub PICDRDCR: crate::RORegister<u32>,
        _reserved0: [u8; 0x04],
        #[doc = "Port ingress congestion DRindex discard count read-reset register"]
        pub PICDRDCRRR: crate::RORegister<u32>,
        _reserved1: [u8; 0x04],
    }
    #[doc = "Port ingress congestion DRindex discard count register"]
    pub mod PICDRDCR {
        #[doc = "Number of frames discarded by Ingress Congestion Manager, for this port, and for each discard resilience (DR) value"]
        pub mod COUNT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Port ingress congestion DRindex discard count read-reset register"]
    pub mod PICDRDCRRR {
        #[doc = "Reading this register provides the count of all frames discarded at this DR level, since the last reset of this count, and also resets the count after the read"]
        pub mod COUNT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
