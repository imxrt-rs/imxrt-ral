#[doc = "Ethernet MAC port"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "Port MAC 0 Command and Configuration Register"]
    pub PM0_COMMAND_CONFIG: crate::RWRegister<u32>,
    #[doc = "Port MAC 0 MAC Address Register 0"]
    pub PM0_MAC_ADDR_0: crate::RORegister<u32>,
    #[doc = "Port MAC 0 MAC Address Register 1"]
    pub PM0_MAC_ADDR_1: crate::RORegister<u32>,
    #[doc = "Port MAC 0 Maximum Frame Length Register"]
    pub PM0_MAXFRM: crate::RWRegister<u32>,
    #[doc = "Port MAC 0 Minimum Frame Length Register"]
    pub PM0_MINFRM: crate::RWRegister<u32>,
    _reserved1: [u8; 0x14],
    #[doc = "Port MAC 0 Internal MDIO Configuration Register"]
    pub PM0_MDIO_CFG: crate::RWRegister<u32>,
    #[doc = "Port MAC 0 Internal MDIO Interface Control Register"]
    pub PM0_MDIO_CTL: crate::RWRegister<u32>,
    #[doc = "Port MAC 0 Internal MDIO Interface Data Register"]
    pub PM0_MDIO_DATA: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Port MAC 0 Interrupt Event Register"]
    pub PM0_IEVENT: crate::RWRegister<u32>,
    #[doc = "Port MAC 0 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
    pub PM0_TX_IPG_PREAMBLE: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Port MAC 0 Interrupt Mask Register(INT_MASK)"]
    pub PM0_IMASK: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Port MAC 0 Pause Quanta Register"]
    pub PM0_PAUSE_QUANTA: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Port MAC 0 Pause Quanta Threshold Register"]
    pub PM0_PAUSE_THRESH: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "Port MAC 0 Receive Pause Status Register"]
    pub PM0_RX_PAUSE_STATUS: crate::RORegister<u32>,
    _reserved7: [u8; 0x40],
    #[doc = "Port MAC 0 EEE Low Power Wakeup Timer Register"]
    pub PM0_LPWAKE_TIMER: crate::RWRegister<u32>,
    #[doc = "Port MAC 0 Transmit EEE Low Power Timer Register"]
    pub PM0_SLEEP_TIMER: crate::RWRegister<u32>,
    #[doc = "Port MAC 0 IEEE1588 Single-Step Control Register"]
    pub PM0_SINGLE_STEP: crate::RWRegister<u32>,
    _reserved8: [u8; 0x0c],
    #[doc = "Port MAC 0 half-duplex backoff entropy register"]
    pub PM0_HD_BACKOFF_ENTROPY: crate::RWRegister<u32>,
    #[doc = "Port MAC 0 Half-Duplex Flow Control Register"]
    pub PM0_HD_FLOW_CTRL: crate::RWRegister<u32>,
    _reserved9: [u8; 0x08],
    #[doc = "Port MAC 0 Statistics Configuration Register"]
    pub PM0_STATN_CONFIG: crate::RWRegister<u32>,
    _reserved10: [u8; 0x1c],
    #[doc = "Port MAC 0 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
    pub PM0_REOCTN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Octets Counter(iflnOctetsn)"]
    pub PM0_ROCTN: crate::RORegister<u64>,
    _reserved11: [u8; 0x08],
    #[doc = "Port MAC 0 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    pub PM0_RXPFN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Frame Counter Register(aFramesReceivedOKn)"]
    pub PM0_RFRMN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Frame Check Sequence Error Counter Register()"]
    pub PM0_RFCSN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
    pub PM0_RVLANN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Frame Error Counter Register(ifInErrorsn)"]
    pub PM0_RERRN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
    pub PM0_RUCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
    pub PM0_RMCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
    pub PM0_RBCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
    pub PM0_RDRPN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Packets Counter Register(etherStatsPktsn)"]
    pub PM0_RPKTN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    pub PM0_RUNDN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
    pub PM0_R64N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
    pub PM0_R127N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
    pub PM0_R255N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
    pub PM0_R511N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
    pub PM0_R1023N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
    pub PM0_R1522N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
    pub PM0_R1523XN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
    pub PM0_ROVRN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
    pub PM0_RJBRN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
    pub PM0_RFRGN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Control Packet Counter Register"]
    pub PM0_RCNPN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
    pub PM0_RDRNTPN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Receive Valid Small Packet Counter Register"]
    pub PM0_RMIN63N: crate::RORegister<u64>,
    _reserved12: [u8; 0x28],
    #[doc = "Port MAC 0 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
    pub PM0_TEOCTN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Octets Counter Register(ifOutOctetsn)"]
    pub PM0_TOCTN: crate::RORegister<u64>,
    _reserved13: [u8; 0x08],
    #[doc = "Port MAC 0 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    pub PM0_TXPFN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
    pub PM0_TFRMN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Frame Check Sequence Error Counter Register()"]
    pub PM0_TFCSN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
    pub PM0_TVLANN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Frame Error Counter Register(ifOutErrorsn)"]
    pub PM0_TERRN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
    pub PM0_TUCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
    pub PM0_TMCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
    pub PM0_TBCAN: crate::RORegister<u64>,
    _reserved14: [u8; 0x08],
    #[doc = "Port MAC 0 Transmit Packets Counter Register(etherStatsPktsn)"]
    pub PM0_TPKTN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    pub PM0_TUNDN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
    pub PM0_T64N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
    pub PM0_T127N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
    pub PM0_T255N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
    pub PM0_T511N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
    pub PM0_T1023N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
    pub PM0_T1522N: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
    pub PM0_T1523XN: crate::RORegister<u64>,
    _reserved15: [u8; 0x18],
    #[doc = "Port MAC 0 Transmit Control Packet Counter Register"]
    pub PM0_TCNPN: crate::RORegister<u64>,
    _reserved16: [u8; 0x08],
    #[doc = "Port MAC 0 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
    pub PM0_TDFRN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
    pub PM0_TMCOLN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
    pub PM0_TSCOLN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Late Collision Counter(aLateCollisions) Register"]
    pub PM0_TLCOLN: crate::RORegister<u64>,
    #[doc = "Port MAC 0 Transmit Excessive Collisions Counter Register"]
    pub PM0_TECOLN: crate::RORegister<u64>,
    _reserved17: [u8; 0x08],
    #[doc = "Port MAC 0 Interface Mode Control Register"]
    pub PM0_IF_MODE: crate::RWRegister<u32>,
    _reserved18: [u8; 0x0104],
    #[doc = "Port MAC 1 Command and Configuration Register"]
    pub PM1_COMMAND_CONFIG: crate::RWRegister<u32>,
    #[doc = "Port MAC 1 MAC Address Register 0"]
    pub PM1_MAC_ADDR_0: crate::RORegister<u32>,
    #[doc = "Port MAC 1 MAC Address Register 1"]
    pub PM1_MAC_ADDR_1: crate::RORegister<u32>,
    #[doc = "Port MAC 1 Maximum Frame Length Register"]
    pub PM1_MAXFRM: crate::RWRegister<u32>,
    #[doc = "Port MAC 1 Minimum Frame Length Register"]
    pub PM1_MINFRM: crate::RWRegister<u32>,
    _reserved19: [u8; 0x24],
    #[doc = "Port MAC 1 Interrupt Event Register"]
    pub PM1_IEVENT: crate::RWRegister<u32>,
    #[doc = "Port MAC 1 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
    pub PM1_TX_IPG_PREAMBLE: crate::RWRegister<u32>,
    _reserved20: [u8; 0x04],
    #[doc = "Port MAC 1 Interrupt Mask Register(INT_MASK)"]
    pub PM1_IMASK: crate::RWRegister<u32>,
    _reserved21: [u8; 0x04],
    #[doc = "Port MAC 1 Pause Quanta Register"]
    pub PM1_PAUSE_QUANTA: crate::RWRegister<u32>,
    _reserved22: [u8; 0x0c],
    #[doc = "Port MAC 1 Pause Quanta Threshold Register"]
    pub PM1_PAUSE_THRESH: crate::RWRegister<u32>,
    _reserved23: [u8; 0x0c],
    #[doc = "Port MAC 1 Receive Pause Status Register"]
    pub PM1_RX_PAUSE_STATUS: crate::RORegister<u32>,
    _reserved24: [u8; 0x40],
    #[doc = "Port MAC 1 EEE Low Power Wakeup Timer Register"]
    pub PM1_LPWAKE_TIMER: crate::RWRegister<u32>,
    #[doc = "Port MAC 1 Transmit EEE Low Power Timer Register"]
    pub PM1_SLEEP_TIMER: crate::RWRegister<u32>,
    #[doc = "Port MAC 1 IEEE1588 Single-Step Control Register"]
    pub PM1_SINGLE_STEP: crate::RWRegister<u32>,
    _reserved25: [u8; 0x0c],
    #[doc = "Port MAC 1 half-duplex backoff entropy register"]
    pub PM1_HD_BACKOFF_ENTROPY: crate::RWRegister<u32>,
    #[doc = "Port MAC 1 Half-Duplex Flow Control Register"]
    pub PM1_HD_FLOW_CTRL: crate::RWRegister<u32>,
    _reserved26: [u8; 0x08],
    #[doc = "Port MAC 1 Statistics Configuration Register"]
    pub PM1_STATN_CONFIG: crate::RWRegister<u32>,
    _reserved27: [u8; 0x1c],
    #[doc = "Port MAC 1 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
    pub PM1_REOCTN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Octets Counter(iflnOctetsn)"]
    pub PM1_ROCTN: crate::RORegister<u64>,
    _reserved28: [u8; 0x08],
    #[doc = "Port MAC 1 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    pub PM1_RXPFN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Frame Counter Register(aFramesReceivedOKn)"]
    pub PM1_RFRMN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Frame Check Sequence Error Counter Register()"]
    pub PM1_RFCSN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
    pub PM1_RVLANN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Frame Error Counter Register(ifInErrorsn)"]
    pub PM1_RERRN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
    pub PM1_RUCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
    pub PM1_RMCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
    pub PM1_RBCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
    pub PM1_RDRPN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Packets Counter Register(etherStatsPktsn)"]
    pub PM1_RPKTN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    pub PM1_RUNDN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
    pub PM1_R64N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
    pub PM1_R127N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
    pub PM1_R255N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
    pub PM1_R511N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
    pub PM1_R1023N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
    pub PM1_R1522N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
    pub PM1_R1523XN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
    pub PM1_ROVRN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
    pub PM1_RJBRN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
    pub PM1_RFRGN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Control Packet Counter Register"]
    pub PM1_RCNPN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
    pub PM1_RDRNTPN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Receive Valid Small Packet Counter Register"]
    pub PM1_RMIN63N: crate::RORegister<u64>,
    _reserved29: [u8; 0x28],
    #[doc = "Port MAC 1 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
    pub PM1_TEOCTN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Octets Counter Register(ifOutOctetsn)"]
    pub PM1_TOCTN: crate::RORegister<u64>,
    _reserved30: [u8; 0x08],
    #[doc = "Port MAC 1 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    pub PM1_TXPFN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
    pub PM1_TFRMN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Frame Check Sequence Error Counter Register()"]
    pub PM1_TFCSN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
    pub PM1_TVLANN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Frame Error Counter Register(ifOutErrorsn)"]
    pub PM1_TERRN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
    pub PM1_TUCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
    pub PM1_TMCAN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
    pub PM1_TBCAN: crate::RORegister<u64>,
    _reserved31: [u8; 0x08],
    #[doc = "Port MAC 1 Transmit Packets Counter Register(etherStatsPktsn)"]
    pub PM1_TPKTN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    pub PM1_TUNDN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
    pub PM1_T64N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
    pub PM1_T127N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
    pub PM1_T255N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
    pub PM1_T511N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
    pub PM1_T1023N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
    pub PM1_T1522N: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
    pub PM1_T1523XN: crate::RORegister<u64>,
    _reserved32: [u8; 0x18],
    #[doc = "Port MAC 1 Transmit Control Packet Counter Register"]
    pub PM1_TCNPN: crate::RORegister<u64>,
    _reserved33: [u8; 0x08],
    #[doc = "Port MAC 1 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
    pub PM1_TDFRN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
    pub PM1_TMCOLN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
    pub PM1_TSCOLN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Late Collision Counter(aLateCollisions) Register"]
    pub PM1_TLCOLN: crate::RORegister<u64>,
    #[doc = "Port MAC 1 Transmit Excessive Collisions Counter Register"]
    pub PM1_TECOLN: crate::RORegister<u64>,
    _reserved34: [u8; 0x08],
    #[doc = "Port MAC 1 Interface Mode Control Register"]
    pub PM1_IF_MODE: crate::RWRegister<u32>,
    _reserved35: [u8; 0xfc],
    #[doc = "Port MAC Merge Control and Status Register"]
    pub MAC_MERGE_MMCSR: crate::RWRegister<u32>,
    _reserved36: [u8; 0x04],
    #[doc = "Port MAC Merge Frame Assembly Error Count Register"]
    pub MAC_MERGE_MMFAECR: crate::RWRegister<u32>,
    #[doc = "Port MAC Merge Frame SMD Error Count Register"]
    pub MAC_MERGE_MMFSECR: crate::RWRegister<u32>,
    #[doc = "Port MAC Merge Frame Assembly OK Count Register"]
    pub MAC_MERGE_MMFAOCR: crate::RWRegister<u32>,
    #[doc = "Port MAC Merge Fragment Count RX Register"]
    pub MAC_MERGE_MMFCRXR: crate::RWRegister<u32>,
    #[doc = "Port MAC Merge Fragment Count TX Register"]
    pub MAC_MERGE_MMFCTXR: crate::RWRegister<u32>,
    #[doc = "Port MAC Merge Hold Count Register"]
    pub MAC_MERGE_MMHCR: crate::RWRegister<u32>,
    _reserved37: [u8; 0x03e0],
    #[doc = "Port external MDIO configuration register"]
    pub PEMDIOCR: crate::RWRegister<u32>,
    #[doc = "Port external MDIO interface control register"]
    pub PEMDIOICR: crate::RWRegister<u32>,
    #[doc = "Port external MDIO interface data register"]
    pub PEMDIOIDR: crate::RWRegister<u32>,
    #[doc = "Port external MDIO register address register"]
    pub PEMDIORAR: crate::RWRegister<u32>,
    #[doc = "Port external MDIO status register"]
    pub PEMDIOSR: crate::RORegister<u32>,
    _reserved38: [u8; 0x0c],
    #[doc = "PHY status configuration register"]
    pub PPSCR: crate::RWRegister<u32>,
    #[doc = "Port PHY status control register"]
    pub PPSCTRLR: crate::RWRegister<u32>,
    #[doc = "Port PHY status data register"]
    pub PPSDR: crate::RORegister<u32>,
    #[doc = "Port PHY status register address register"]
    pub PPSRAR: crate::RWRegister<u32>,
    #[doc = "Port PHY status event register"]
    pub PPSER: crate::RWRegister<u32>,
    #[doc = "Port PHY status mask register"]
    pub PPSMR: crate::RWRegister<u32>,
}
#[doc = "Port MAC 0 Command and Configuration Register"]
pub mod PM0_COMMAND_CONFIG {
    #[doc = "MAC transmit path enable"]
    pub mod TX_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC receive path enable"]
    pub mod RX_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Terminate/forward received PAUSE frames"]
    pub mod PAUSE_FWD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ignore PAUSE frame quanta"]
    pub mod PAUSE_IGN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit source MAC address insertion"]
    pub mod TX_ADDR_INS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loopback enable"]
    pub mod LOOP_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loopback mode"]
    pub mod LPBK_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control frame reception enable"]
    pub mod CNT_FRM_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timestamp Point"]
    pub mod TS_PNT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable padding of frames in transmit direction (1, default)."]
    pub mod TXP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Duplex Flow Control Enable"]
    pub mod HD_FCEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tx flush"]
    pub mod TX_FLUSH {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Low Power Idle Enable."]
    pub mod TX_LOWP_ENA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "(default), the MAC operates in normal mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "The MAC completes the transmission of the current Frame and generates Low Power Idle Sequences to the line. It is advised to inspect IEVENT\\[TX_EMPTY\\] is set before enabling the LPI."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Software Reset. Self clearing bit."]
    pub mod SWR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress flush enable"]
    pub mod RX_FLUSH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit timestamp mode"]
    pub mod TS_MODE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Magic Packet detection enable."]
    pub mod MG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 MAC Address Register 0"]
pub mod PM0_MAC_ADDR_0 {
    #[doc = "MAC address 0"]
    pub mod MAC_ADDR_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 MAC Address Register 1"]
pub mod PM0_MAC_ADDR_1 {
    #[doc = "MAC address 1"]
    pub mod MAC_ADDR_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Maximum Frame Length Register"]
pub mod PM0_MAXFRM {
    #[doc = "Maximum supported received frame length."]
    pub mod MAXFRM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum transmit frame length"]
    pub mod TX_MTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Minimum Frame Length Register"]
pub mod PM0_MINFRM {
    #[doc = "Receive Minimum Frame Length size in bytes."]
    pub mod NUM_BYTES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Internal MDIO Configuration Register"]
pub mod PM0_MDIO_CFG {
    #[doc = "MDIO busy (same as bit 31)"]
    pub mod BSY2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO hold time"]
    pub mod MDIO_HOLD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO preamble disable."]
    pub mod PRE_DIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Clause 45 support."]
    pub mod ENC45 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO clock divisor."]
    pub mod MDIO_CLK_DIV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO command completion interrupt mask."]
    pub mod CIM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO command completion event. Bit is cleared by writing `1'."]
    pub mod CMP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO busy"]
    pub mod BSY1 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Internal MDIO Interface Control Register"]
pub mod PM0_MDIO_CTL {
    #[doc = "MDIO register address (Clause 22)"]
    pub mod DEV_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO PHY address (Clause 22)"]
    pub mod PORT_ADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO read initiation."]
    pub mod READ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Internal MDIO Interface Data Register"]
pub mod PM0_MDIO_DATA {
    #[doc = "16-bit MDIO data."]
    pub mod MDIO_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO busy bit. The state of this bit is also reflected in MDIO_CFG\\[BSY\\]."]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Interrupt Event Register"]
pub mod PM0_IEVENT {
    #[doc = "Transmit fifo empty event"]
    pub mod TX_EMPTY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive idle event"]
    pub mod RX_EMPTY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO overflow event."]
    pub mod TX_OVFL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO underflow event."]
    pub mod TX_UNFL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO overflow event."]
    pub mod RX_OVFL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Magic packet detection indication event"]
    pub mod MGI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tx Clock Stop Detection"]
    pub mod TX_CSD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Clock Stop Detection"]
    pub mod RX_CSD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Speed/Duplex Change"]
    pub mod SPD_DUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC merge frame SMD error received event"]
    pub mod MRG_SERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC merge frame assembly error event"]
    pub mod MRG_AERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
pub mod PM0_TX_IPG_PREAMBLE {
    #[doc = "Transmit inter-packet gap value."]
    pub mod IPG_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flexible Preamble Count"]
    pub mod FLEX_PREAMBLE_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Flexible Preamble Count"]
    pub mod FLEX_PREAMBLE_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Interrupt Mask Register(INT_MASK)"]
pub mod PM0_IMASK {
    #[doc = "Magic packet detection indication event mask."]
    pub mod MGI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tx Clock Stop Detection"]
    pub mod TX_CSD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Clock Stop Detection"]
    pub mod RX_CSD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Speed/Duplex change event mask."]
    pub mod SPD_DUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC merge frame SMD error received event interrupt mask"]
    pub mod MRG_SERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC merge frame assembly error event interrupt mask"]
    pub mod MRG_AERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Pause Quanta Register"]
pub mod PM0_PAUSE_QUANTA {
    #[doc = "Value to be used for the quanta value when XOFF is triggered."]
    pub mod PQNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Pause Quanta Threshold Register"]
pub mod PM0_PAUSE_THRESH {
    #[doc = "Quanta threshold."]
    pub mod QTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Pause Status Register"]
pub mod PM0_RX_PAUSE_STATUS {
    #[doc = "Pause status."]
    pub mod PSTAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 EEE Low Power Wakeup Timer Register"]
pub mod PM0_LPWAKE_TIMER {
    #[doc = "EEE System transmit wait time"]
    pub mod TW_SYS_TX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit EEE Low Power Timer Register"]
pub mod PM0_SLEEP_TIMER {
    #[doc = "Defines the number of NETC cycles (which represents time) where Tx is idle before mac transmits low power EEE"]
    pub mod SLEEPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 IEEE1588 Single-Step Control Register"]
pub mod PM0_SINGLE_STEP {
    #[doc = "Checksum update"]
    pub mod CH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start offset from the beginning of a frame where the field to update is found (index to MS byte)"]
    pub mod OFFSET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IEEE-1588 Single-Step enable."]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 half-duplex backoff entropy register"]
pub mod PM0_HD_BACKOFF_ENTROPY {
    #[doc = "Half duplex backoff entropy"]
    pub mod HD_BACKOFF_ENTROPY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW programmable entropy valid"]
    pub mod SW_ENTROPY_VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Half-Duplex Flow Control Register"]
pub mod PM0_HD_FLOW_CTRL {
    #[doc = "Half-Duplex Back-Pressure Off Minimum"]
    pub mod HD_BP_OFF_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half-Duplex Back-Pressure On Maximum"]
    pub mod HD_BP_ON_MAX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Statistics Configuration Register"]
pub mod PM0_STATN_CONFIG {
    #[doc = "0 - (default) counters are wrapping around1- the counters saturate at the maximum value"]
    pub mod SAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 - (default) counters are not affected by read.1 - a read to a counter resets it to 0."]
    pub mod COD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - all counters will be reset to 0"]
    pub mod CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write enable for Tx/Rx stats registers"]
    pub mod WEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
pub mod PM0_REOCTN {
    #[doc = "Incremented for each octet received in both good and bad packets."]
    pub mod REOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Octets Counter(iflnOctetsn)"]
pub mod PM0_ROCTN {
    #[doc = "Incremented for each octet received except preamble (that is, Header, Payload, Pad and FCS) for all valid frames and valid PAUSE frames received."]
    pub mod ROCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
pub mod PM0_RXPFN {
    #[doc = "Incremented for each valid PAUSE frame received ."]
    pub mod RXPFN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Frame Counter Register(aFramesReceivedOKn)"]
pub mod PM0_RFRMN {
    #[doc = "Incremented for each frame received without error, including PAUSE frames."]
    pub mod RFRMN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Frame Check Sequence Error Counter Register()"]
pub mod PM0_RFCSN {
    #[doc = "Incremented for each frame received with a CRC-32 error but the frame is otherwise of correct length."]
    pub mod RFCSN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
pub mod PM0_RVLANN {
    #[doc = "Incremented for each valid VLAN tagged frame received with ethertype 0x8100"]
    pub mod RVLANN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Frame Error Counter Register(ifInErrorsn)"]
pub mod PM0_RERRN {
    #[doc = "Incremented for each frame received with an error (except for undersized/fragment frame):"]
    pub mod RERRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
pub mod PM0_RUCAN {
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which bit 0 of the destination address was 0 ."]
    pub mod RUCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
pub mod PM0_RMCAN {
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1)"]
    pub mod RMCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
pub mod PM0_RBCAN {
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which all bits of the destination address were 1 ."]
    pub mod RBCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
pub mod PM0_RDRPN {
    #[doc = "Incremented for each dropped packet due to internal errors of the MAC client"]
    pub mod RDRPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Packets Counter Register(etherStatsPktsn)"]
pub mod PM0_RPKTN {
    #[doc = "Incremented for each good or bad packet received."]
    pub mod RPKTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
pub mod PM0_RUNDN {
    #[doc = "Incremented for each packet received that was less than the length programmed in PMa_MINFRM register and greater than or equal to 18 octets"]
    pub mod RUNDN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
pub mod PM0_R64N {
    #[doc = "Incremented for each 64-octet frame received, good or bad."]
    pub mod R64N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
pub mod PM0_R127N {
    #[doc = "Incremented for each 65- to 127-octet frame received, good or bad."]
    pub mod R127N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
pub mod PM0_R255N {
    #[doc = "Incremented for each 128- to 255-octet frame received, good or bad."]
    pub mod R255N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
pub mod PM0_R511N {
    #[doc = "Incremented for each 256- to 511-octet frame received, good or bad."]
    pub mod R511N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
pub mod PM0_R1023N {
    #[doc = "Incremented for each 512- to 1023-octet frame received, good or bad."]
    pub mod R1023N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
pub mod PM0_R1522N {
    #[doc = "Incremented for each 1024- to 1522-octet frame received, good or bad."]
    pub mod R1522N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
pub mod PM0_R1523XN {
    #[doc = "Incremented for each 1523-octet frame and larger (up to the maximum frame length specified in register PMa_MAXFRM\\[MAXFRM\\]) received, good or bad"]
    pub mod R1523XN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
pub mod PM0_ROVRN {
    #[doc = "Incremented for each packet which is larger than the maximum frame length specified in the MAXFRM(FRAME_LENGTH) register (excluding framing bits, but including FCS octets) received with a good frame check sequence"]
    pub mod ROVRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
pub mod PM0_RJBRN {
    #[doc = "Incremented for each packet which is larger than the maximum frame length specified in register PMa_MAXFRM\\[MAXFRM\\] (excluding framing bits, but including FCS octets) received with a bad frame check sequence"]
    pub mod RJBRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
pub mod PM0_RFRGN {
    #[doc = "Incremented for each packet which is shorter than the length programmed in PMa_MINFRM register and received with a wrong CRC"]
    pub mod RFRGN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Control Packet Counter Register"]
pub mod PM0_RCNPN {
    #[doc = "Incremented for each valid control packet (type 0x8808) but not for PAUSE packets"]
    pub mod RCNPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
pub mod PM0_RDRNTPN {
    #[doc = "Incremented for each fully dropped packet (not truncated) due to internal errors of the MAC client. Occurs when a receive FIFO overflows."]
    pub mod RDRNTPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Receive Valid Small Packet Counter Register"]
pub mod PM0_RMIN63N {
    #[doc = "Incremented for each valid small packet less than 64B but greater or equal to the length programmed in PMa_MINFRM register"]
    pub mod RMIN63N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
pub mod PM0_TEOCTN {
    #[doc = "Incremented for each octet transmitted in both good and bad packets."]
    pub mod TEOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Octets Counter Register(ifOutOctetsn)"]
pub mod PM0_TOCTN {
    #[doc = "Incremented for each octet transmitted except preamble (that is, Header, Payload, Pad and FCS) for all valid frames and valid PAUSE frames transmitted"]
    pub mod TOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
pub mod PM0_TXPFN {
    #[doc = "Incremented for each valid PAUSE frame transmitted . Note: Pause frames forwarded to the MAC from MAC Client are not counted by TXPFn."]
    pub mod TXPFN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
pub mod PM0_TFRMN {
    #[doc = "Incremented for each frame transmitted without error, including PAUSE frames."]
    pub mod TFRMN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Frame Check Sequence Error Counter Register()"]
pub mod PM0_TFCSN {
    #[doc = "Incremented for each frame transmitted with a CRC-32 error except for underflows."]
    pub mod TFCSN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
pub mod PM0_TVLANN {
    #[doc = "Incremented for each valid VLAN tagged frame transmitted with ethertype 0x8100."]
    pub mod TVLANN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Frame Error Counter Register(ifOutErrorsn)"]
pub mod PM0_TERRN {
    #[doc = "Transmit frame error count"]
    pub mod TERRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
pub mod PM0_TUCAN {
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which bit 0 of the destination address was 0."]
    pub mod TUCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
pub mod PM0_TMCAN {
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1 )"]
    pub mod TMCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
pub mod PM0_TBCAN {
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which all bits of the destination address were 1 ."]
    pub mod TBCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Packets Counter Register(etherStatsPktsn)"]
pub mod PM0_TPKTN {
    #[doc = "Incremented for each good or bad packet transmitted."]
    pub mod TPKTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
pub mod PM0_TUNDN {
    #[doc = "Incremented for each packet transmitted that was less than 64 octets long with a good CRC."]
    pub mod TUNDN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
pub mod PM0_T64N {
    #[doc = "Incremented for each 64-octet frame transmitted, good or bad."]
    pub mod T64N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
pub mod PM0_T127N {
    #[doc = "Incremented for each 65 to 127-octet frame transmitted, good or bad."]
    pub mod T127N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
pub mod PM0_T255N {
    #[doc = "Incremented for each 128 to 255-octet frame transmitted, good or bad."]
    pub mod T255N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
pub mod PM0_T511N {
    #[doc = "Incremented for each 256 to 511-octet frame transmitted, good or bad."]
    pub mod T511N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
pub mod PM0_T1023N {
    #[doc = "Incremented for each 512 to 1023-octet frame transmitted, good or bad."]
    pub mod T1023N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
pub mod PM0_T1522N {
    #[doc = "Incremented for each 1024- to 1522-octet frame transmitted, good or bad."]
    pub mod T1522N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
pub mod PM0_T1523XN {
    #[doc = "Incremented for each 1523-octet frame and larger (up to the maximum frame length specified in register PMa_MAXFRM\\[TX_MTU\\]) transmitted, good or bad"]
    pub mod T1523XN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Control Packet Counter Register"]
pub mod PM0_TCNPN {
    #[doc = "Incremented for each valid control packet transmitted (type 0x8808) but not for PAUSE packets"]
    pub mod TCNPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
pub mod PM0_TDFRN {
    #[doc = "Increments for successful transmissions, without retransmits, that were deferred (half-duplex only)."]
    pub mod TDFRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
pub mod PM0_TMCOLN {
    #[doc = "Increments for successful transmission after more than one retransmission (half-duplex only)."]
    pub mod TMCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
pub mod PM0_TSCOLN {
    #[doc = "Increments for successful transmission after one retransmission (half-duplex only)."]
    pub mod TSCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Late Collision Counter(aLateCollisions) Register"]
pub mod PM0_TLCOLN {
    #[doc = "Late collision occurred. Frame corrupted / discarded (half-duplex only)"]
    pub mod TLCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Transmit Excessive Collisions Counter Register"]
pub mod PM0_TECOLN {
    #[doc = "Excessive collisions occurred. Frame was discarded (half-duplex only)"]
    pub mod TECOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 0 Interface Mode Control Register"]
pub mod PM0_IF_MODE {
    #[doc = "Interface mode"]
    pub mod IFMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reverse Mode"]
    pub mod REVMII {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reverse mode disabled - port is in MAC mode"]
            pub const MAC: u32 = 0;
            #[doc = "Reverse mode enabled - port is in PHY mode"]
            pub const PHY: u32 = 0x01;
        }
    }
    #[doc = "0 - 100 Mbps RMII/MII 1 - 10 Mbps RMII/MII (MII speed select is valid only in case of RevMII)"]
    pub mod M10 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100 Mbps"]
            pub const M100: u32 = 0;
            #[doc = "10 Mbps"]
            pub const M10: u32 = 0x01;
        }
    }
    #[doc = "Half-duplex"]
    pub mod HD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "full duplex"]
            pub const FD: u32 = 0;
            #[doc = "half duplex"]
            pub const HD: u32 = 0x01;
        }
    }
    #[doc = "Clock Stop"]
    pub mod CLK_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not stoppable"]
            pub const NO_STOP: u32 = 0;
            #[doc = "Stoppable"]
            pub const STOP: u32 = 0x01;
        }
    }
    #[doc = "Set Speed"]
    pub mod SSP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100 Mbps"]
            pub const M100: u32 = 0;
            #[doc = "10 Mbps"]
            pub const M10: u32 = 0x01;
            #[doc = "1 Gbps"]
            pub const G1: u32 = 0x02;
        }
    }
}
#[doc = "Port MAC 1 Command and Configuration Register"]
pub mod PM1_COMMAND_CONFIG {
    #[doc = "MAC transmit path enable"]
    pub mod TX_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC receive path enable"]
    pub mod RX_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Terminate/forward received PAUSE frames"]
    pub mod PAUSE_FWD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ignore PAUSE frame quanta"]
    pub mod PAUSE_IGN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit source MAC address insertion"]
    pub mod TX_ADDR_INS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loopback enable"]
    pub mod LOOP_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loopback mode"]
    pub mod LPBK_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control frame reception enable"]
    pub mod CNT_FRM_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timestamp Point"]
    pub mod TS_PNT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable padding of frames in transmit direction (1, default)."]
    pub mod TXP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half Duplex Flow Control Enable"]
    pub mod HD_FCEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tx flush"]
    pub mod TX_FLUSH {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Low Power Idle Enable."]
    pub mod TX_LOWP_ENA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "(default), the MAC operates in normal mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "The MAC completes the transmission of the current Frame and generates Low Power Idle Sequences to the line. It is advised to inspect IEVENT\\[TX_EMPTY\\] is set before enabling the LPI."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Software Reset. Self clearing bit."]
    pub mod SWR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress flush enable"]
    pub mod RX_FLUSH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit timestamp mode"]
    pub mod TS_MODE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Magic Packet detection enable."]
    pub mod MG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 MAC Address Register 0"]
pub mod PM1_MAC_ADDR_0 {
    #[doc = "MAC address 0"]
    pub mod MAC_ADDR_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 MAC Address Register 1"]
pub mod PM1_MAC_ADDR_1 {
    #[doc = "MAC address 1"]
    pub mod MAC_ADDR_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Maximum Frame Length Register"]
pub mod PM1_MAXFRM {
    #[doc = "Maximum supported received frame length."]
    pub mod MAXFRM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum transmit frame length"]
    pub mod TX_MTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Minimum Frame Length Register"]
pub mod PM1_MINFRM {
    #[doc = "Receive Minimum Frame Length size in bytes."]
    pub mod NUM_BYTES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Interrupt Event Register"]
pub mod PM1_IEVENT {
    #[doc = "Transmit fifo empty event"]
    pub mod TX_EMPTY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive idle event"]
    pub mod RX_EMPTY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO overflow event."]
    pub mod TX_OVFL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO underflow event."]
    pub mod TX_UNFL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO overflow event."]
    pub mod RX_OVFL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Magic packet detection indication event"]
    pub mod MGI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tx Clock Stop Detection"]
    pub mod TX_CSD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Clock Stop Detection"]
    pub mod RX_CSD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Speed/Duplex Change"]
    pub mod SPD_DUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC merge frame SMD error received event"]
    pub mod MRG_SERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC merge frame assembly error event"]
    pub mod MRG_AERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
pub mod PM1_TX_IPG_PREAMBLE {
    #[doc = "Transmit inter-packet gap value."]
    pub mod IPG_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flexible Preamble Count"]
    pub mod FLEX_PREAMBLE_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Flexible Preamble Count"]
    pub mod FLEX_PREAMBLE_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Interrupt Mask Register(INT_MASK)"]
pub mod PM1_IMASK {
    #[doc = "Magic packet detection indication event mask."]
    pub mod MGI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tx Clock Stop Detection"]
    pub mod TX_CSD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Clock Stop Detection"]
    pub mod RX_CSD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Speed/Duplex change event mask."]
    pub mod SPD_DUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC merge frame SMD error received event interrupt mask"]
    pub mod MRG_SERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC merge frame assembly error event interrupt mask"]
    pub mod MRG_AERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Pause Quanta Register"]
pub mod PM1_PAUSE_QUANTA {
    #[doc = "Value to be used for the quanta value when XOFF is triggered."]
    pub mod PQNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Pause Quanta Threshold Register"]
pub mod PM1_PAUSE_THRESH {
    #[doc = "Quanta threshold."]
    pub mod QTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Pause Status Register"]
pub mod PM1_RX_PAUSE_STATUS {
    #[doc = "Pause status."]
    pub mod PSTAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 EEE Low Power Wakeup Timer Register"]
pub mod PM1_LPWAKE_TIMER {
    #[doc = "EEE System transmit wait time"]
    pub mod TW_SYS_TX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit EEE Low Power Timer Register"]
pub mod PM1_SLEEP_TIMER {
    #[doc = "Defines the number of NETC cycles (which represents time) where Tx is idle before mac transmits low power EEE"]
    pub mod SLEEPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 IEEE1588 Single-Step Control Register"]
pub mod PM1_SINGLE_STEP {
    #[doc = "Checksum update"]
    pub mod CH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start offset from the beginning of a frame where the field to update is found (index to MS byte)"]
    pub mod OFFSET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IEEE-1588 Single-Step enable."]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 half-duplex backoff entropy register"]
pub mod PM1_HD_BACKOFF_ENTROPY {
    #[doc = "Half duplex backoff entropy"]
    pub mod HD_BACKOFF_ENTROPY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW programmable entropy valid"]
    pub mod SW_ENTROPY_VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Half-Duplex Flow Control Register"]
pub mod PM1_HD_FLOW_CTRL {
    #[doc = "Half-Duplex Back-Pressure Off Minimum"]
    pub mod HD_BP_OFF_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half-Duplex Back-Pressure On Maximum"]
    pub mod HD_BP_ON_MAX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Statistics Configuration Register"]
pub mod PM1_STATN_CONFIG {
    #[doc = "0 - (default) counters are wrapping around1- the counters saturate at the maximum value"]
    pub mod SAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 - (default) counters are not affected by read.1 - a read to a counter resets it to 0."]
    pub mod COD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 - all counters will be reset to 0"]
    pub mod CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write enable for Tx/Rx stats registers"]
    pub mod WEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
pub mod PM1_REOCTN {
    #[doc = "Incremented for each octet received in both good and bad packets."]
    pub mod REOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Octets Counter(iflnOctetsn)"]
pub mod PM1_ROCTN {
    #[doc = "Incremented for each octet received except preamble (that is, Header, Payload, Pad and FCS) for all valid frames and valid PAUSE frames received."]
    pub mod ROCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
pub mod PM1_RXPFN {
    #[doc = "Incremented for each valid PAUSE frame received ."]
    pub mod RXPFN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Frame Counter Register(aFramesReceivedOKn)"]
pub mod PM1_RFRMN {
    #[doc = "Incremented for each frame received without error, including PAUSE frames."]
    pub mod RFRMN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Frame Check Sequence Error Counter Register()"]
pub mod PM1_RFCSN {
    #[doc = "Incremented for each frame received with a CRC-32 error but the frame is otherwise of correct length."]
    pub mod RFCSN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
pub mod PM1_RVLANN {
    #[doc = "Incremented for each valid VLAN tagged frame received with ethertype 0x8100"]
    pub mod RVLANN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Frame Error Counter Register(ifInErrorsn)"]
pub mod PM1_RERRN {
    #[doc = "Incremented for each frame received with an error (except for undersized/fragment frame):"]
    pub mod RERRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
pub mod PM1_RUCAN {
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which bit 0 of the destination address was 0 ."]
    pub mod RUCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
pub mod PM1_RMCAN {
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1)"]
    pub mod RMCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
pub mod PM1_RBCAN {
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which all bits of the destination address were 1 ."]
    pub mod RBCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
pub mod PM1_RDRPN {
    #[doc = "Incremented for each dropped packet due to internal errors of the MAC client"]
    pub mod RDRPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Packets Counter Register(etherStatsPktsn)"]
pub mod PM1_RPKTN {
    #[doc = "Incremented for each good or bad packet received."]
    pub mod RPKTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
pub mod PM1_RUNDN {
    #[doc = "Incremented for each packet received that was less than the length programmed in PMa_MINFRM register and greater than or equal to 18 octets"]
    pub mod RUNDN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
pub mod PM1_R64N {
    #[doc = "Incremented for each 64-octet frame received, good or bad."]
    pub mod R64N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
pub mod PM1_R127N {
    #[doc = "Incremented for each 65- to 127-octet frame received, good or bad."]
    pub mod R127N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
pub mod PM1_R255N {
    #[doc = "Incremented for each 128- to 255-octet frame received, good or bad."]
    pub mod R255N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
pub mod PM1_R511N {
    #[doc = "Incremented for each 256- to 511-octet frame received, good or bad."]
    pub mod R511N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
pub mod PM1_R1023N {
    #[doc = "Incremented for each 512- to 1023-octet frame received, good or bad."]
    pub mod R1023N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
pub mod PM1_R1522N {
    #[doc = "Incremented for each 1024- to 1522-octet frame received, good or bad."]
    pub mod R1522N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
pub mod PM1_R1523XN {
    #[doc = "Incremented for each 1523-octet frame and larger (up to the maximum frame length specified in register PMa_MAXFRM\\[MAXFRM\\]) received, good or bad"]
    pub mod R1523XN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
pub mod PM1_ROVRN {
    #[doc = "Incremented for each packet which is larger than the maximum frame length specified in the MAXFRM(FRAME_LENGTH) register (excluding framing bits, but including FCS octets) received with a good frame check sequence"]
    pub mod ROVRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
pub mod PM1_RJBRN {
    #[doc = "Incremented for each packet which is larger than the maximum frame length specified in register PMa_MAXFRM\\[MAXFRM\\] (excluding framing bits, but including FCS octets) received with a bad frame check sequence"]
    pub mod RJBRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
pub mod PM1_RFRGN {
    #[doc = "Incremented for each packet which is shorter than the length programmed in PMa_MINFRM register and received with a wrong CRC"]
    pub mod RFRGN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Control Packet Counter Register"]
pub mod PM1_RCNPN {
    #[doc = "Incremented for each valid control packet (type 0x8808) but not for PAUSE packets"]
    pub mod RCNPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
pub mod PM1_RDRNTPN {
    #[doc = "Incremented for each fully dropped packet (not truncated) due to internal errors of the MAC client. Occurs when a receive FIFO overflows."]
    pub mod RDRNTPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Receive Valid Small Packet Counter Register"]
pub mod PM1_RMIN63N {
    #[doc = "Incremented for each valid small packet less than 64B but greater or equal to the length programmed in PMa_MINFRM register"]
    pub mod RMIN63N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
pub mod PM1_TEOCTN {
    #[doc = "Incremented for each octet transmitted in both good and bad packets."]
    pub mod TEOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Octets Counter Register(ifOutOctetsn)"]
pub mod PM1_TOCTN {
    #[doc = "Incremented for each octet transmitted except preamble (that is, Header, Payload, Pad and FCS) for all valid frames and valid PAUSE frames transmitted"]
    pub mod TOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
pub mod PM1_TXPFN {
    #[doc = "Incremented for each valid PAUSE frame transmitted . Note: Pause frames forwarded to the MAC from MAC Client are not counted by TXPFn."]
    pub mod TXPFN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
pub mod PM1_TFRMN {
    #[doc = "Incremented for each frame transmitted without error, including PAUSE frames."]
    pub mod TFRMN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Frame Check Sequence Error Counter Register()"]
pub mod PM1_TFCSN {
    #[doc = "Incremented for each frame transmitted with a CRC-32 error except for underflows."]
    pub mod TFCSN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
pub mod PM1_TVLANN {
    #[doc = "Incremented for each valid VLAN tagged frame transmitted with ethertype 0x8100."]
    pub mod TVLANN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Frame Error Counter Register(ifOutErrorsn)"]
pub mod PM1_TERRN {
    #[doc = "Transmit frame error count"]
    pub mod TERRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
pub mod PM1_TUCAN {
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which bit 0 of the destination address was 0."]
    pub mod TUCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
pub mod PM1_TMCAN {
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1 )"]
    pub mod TMCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
pub mod PM1_TBCAN {
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which all bits of the destination address were 1 ."]
    pub mod TBCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Packets Counter Register(etherStatsPktsn)"]
pub mod PM1_TPKTN {
    #[doc = "Incremented for each good or bad packet transmitted."]
    pub mod TPKTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
pub mod PM1_TUNDN {
    #[doc = "Incremented for each packet transmitted that was less than 64 octets long with a good CRC."]
    pub mod TUNDN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
pub mod PM1_T64N {
    #[doc = "Incremented for each 64-octet frame transmitted, good or bad."]
    pub mod T64N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
pub mod PM1_T127N {
    #[doc = "Incremented for each 65 to 127-octet frame transmitted, good or bad."]
    pub mod T127N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
pub mod PM1_T255N {
    #[doc = "Incremented for each 128 to 255-octet frame transmitted, good or bad."]
    pub mod T255N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
pub mod PM1_T511N {
    #[doc = "Incremented for each 256 to 511-octet frame transmitted, good or bad."]
    pub mod T511N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
pub mod PM1_T1023N {
    #[doc = "Incremented for each 512 to 1023-octet frame transmitted, good or bad."]
    pub mod T1023N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
pub mod PM1_T1522N {
    #[doc = "Incremented for each 1024- to 1522-octet frame transmitted, good or bad."]
    pub mod T1522N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
pub mod PM1_T1523XN {
    #[doc = "Incremented for each 1523-octet frame and larger (up to the maximum frame length specified in register PMa_MAXFRM\\[TX_MTU\\]) transmitted, good or bad"]
    pub mod T1523XN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Control Packet Counter Register"]
pub mod PM1_TCNPN {
    #[doc = "Incremented for each valid control packet transmitted (type 0x8808) but not for PAUSE packets"]
    pub mod TCNPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
pub mod PM1_TDFRN {
    #[doc = "Increments for successful transmissions, without retransmits, that were deferred (half-duplex only)."]
    pub mod TDFRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
pub mod PM1_TMCOLN {
    #[doc = "Increments for successful transmission after more than one retransmission (half-duplex only)."]
    pub mod TMCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
pub mod PM1_TSCOLN {
    #[doc = "Increments for successful transmission after one retransmission (half-duplex only)."]
    pub mod TSCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Late Collision Counter(aLateCollisions) Register"]
pub mod PM1_TLCOLN {
    #[doc = "Late collision occurred. Frame corrupted / discarded (half-duplex only)"]
    pub mod TLCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Transmit Excessive Collisions Counter Register"]
pub mod PM1_TECOLN {
    #[doc = "Excessive collisions occurred. Frame was discarded (half-duplex only)"]
    pub mod TECOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC 1 Interface Mode Control Register"]
pub mod PM1_IF_MODE {
    #[doc = "Interface mode"]
    pub mod IFMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reverse Mode"]
    pub mod REVMII {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reverse mode disabled - port is in MAC mode"]
            pub const MAC: u32 = 0;
            #[doc = "Reverse mode enabled - port is in PHY mode"]
            pub const PHY: u32 = 0x01;
        }
    }
    #[doc = "0 - 100 Mbps RMII/MII 1 - 10 Mbps RMII/MII (MII speed select is valid only in case of RevMII)"]
    pub mod M10 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100 Mbps"]
            pub const M100: u32 = 0;
            #[doc = "10 Mbps"]
            pub const M10: u32 = 0x01;
        }
    }
    #[doc = "Half-duplex"]
    pub mod HD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "full duplex"]
            pub const FD: u32 = 0;
            #[doc = "half duplex"]
            pub const HD: u32 = 0x01;
        }
    }
    #[doc = "Clock Stop"]
    pub mod CLK_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not stoppable"]
            pub const NO_STOP: u32 = 0;
            #[doc = "Stoppable"]
            pub const STOP: u32 = 0x01;
        }
    }
    #[doc = "Set Speed"]
    pub mod SSP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "100 Mbps"]
            pub const M100: u32 = 0;
            #[doc = "10 Mbps"]
            pub const M10: u32 = 0x01;
            #[doc = "1 Gbps"]
            pub const G1: u32 = 0x02;
        }
    }
}
#[doc = "Port MAC Merge Control and Status Register"]
pub mod MAC_MERGE_MMCSR {
    #[doc = "Local preemption supported"]
    pub mod LPS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local preemption enabled"]
    pub mod LPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local preemption active"]
    pub mod LPA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Local additional fragment size"]
    pub mod LAFS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote preemption supported"]
    pub mod RPS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote preemption enabled"]
    pub mod RPE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote preemption active"]
    pub mod RPA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote additional fragment size"]
    pub mod RAFS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Merge enabled"]
    pub mod ME {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Verify disabled"]
    pub mod VDIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Verify status"]
    pub mod VSTS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Merge status"]
    pub mod TXSTS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Verify Time"]
    pub mod VT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Link Fail"]
    pub mod LINK_FAIL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC Merge Frame Assembly Error Count Register"]
pub mod MAC_MERGE_MMFAECR {
    #[doc = "A count of MAC frames with reassembly errors."]
    pub mod MMFAEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC Merge Frame SMD Error Count Register"]
pub mod MAC_MERGE_MMFSECR {
    #[doc = "A count of received MAC frames / MAC frame fragments rejected due to unknown SMD value or arriving with an SMD-C when no frame is in progress"]
    pub mod MMFSEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC Merge Frame Assembly OK Count Register"]
pub mod MAC_MERGE_MMFAOCR {
    #[doc = "A count of MAC frames that were successfully reassembled and delivered to the MAC."]
    pub mod MMFAOC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC Merge Fragment Count RX Register"]
pub mod MAC_MERGE_MMFCRXR {
    #[doc = "A count of the number of additional mPackets received due to preemption."]
    pub mod MMFCRX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC Merge Fragment Count TX Register"]
pub mod MAC_MERGE_MMFCTXR {
    #[doc = "A count of the number of additional mPackets transmitted due to preemption."]
    pub mod MMFCTX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port MAC Merge Hold Count Register"]
pub mod MAC_MERGE_MMHCR {
    #[doc = "A count of the number of times the variable hold transitions from false to true."]
    pub mod MMHC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port external MDIO configuration register"]
pub mod PEMDIOCR {
    #[doc = "Busy 2 (same as bit 31)"]
    pub mod BSY2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An MDIO transaction is not occurring; software may access other MDIO registers."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO transaction is occurring."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Read (and write) Error"]
    pub mod MDIO_RD_ER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error on last MDIO transaction (read or write)."]
            pub const ZERO: u32 = 0;
            #[doc = "An error was detected on the last MDIO transaction (read or write). Errors on internal MDIO accesses can be triggered by an access to an invalid device, or by a write to a shared on-die PHY device that has not been locked."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Hold Time"]
    pub mod MDIO_HOLD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 NETC cycle"]
            pub const NETC1: u32 = 0;
            #[doc = "3 NETC cycles"]
            pub const NETC3: u32 = 0x01;
            #[doc = "5 NETC cycles (default - recommended value)"]
            pub const NETC5: u32 = 0x02;
            #[doc = "7 NETC cycles"]
            pub const NETC7: u32 = 0x03;
            #[doc = "9 NETC cycles"]
            pub const NETC9: u32 = 0x04;
            #[doc = "11 NETC cycles"]
            pub const NETC11: u32 = 0x05;
            #[doc = "13 NETC cycles"]
            pub const NETC13: u32 = 0x06;
            #[doc = "15 NETC cycles"]
            pub const NETC15: u32 = 0x07;
        }
    }
    #[doc = "MDIO Preamble Disable"]
    pub mod PRE_DIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Generation of MDIO preamble is enabled (default operation)."]
            pub const ENABLE: u32 = 0;
            #[doc = "Generation of MDIO preamble is disabled."]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Clause 45 Support"]
    pub mod ENC45 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clause 22 transactions are used."]
            pub const ZERO: u32 = 0;
            #[doc = "Clause 45 transactions are used."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Clock Divisor"]
    pub mod MDIO_CLK_DIV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Returns the link ID"]
    pub mod WHOAMI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended HOLD"]
    pub mod EHOLD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation, MDIO hold time is as specified in PEMDIOCR\\[MDIO_HOLD\\]."]
            pub const ZERO: u32 = 0;
            #[doc = "Extended operation"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Negative Edge"]
    pub mod NEG {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "normal operation - positive edge"]
            pub const ZERO: u32 = 0;
            #[doc = "MDIO is driven by master on MDC negative edge (default for external MDIOs)"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Address Error"]
    pub mod ADDR_ERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal"]
            pub const ZERO: u32 = 0;
            #[doc = "Error. An access control violation has occurred. The request address used does not match the MDIO PHY's address (clause 22) or MDIO port address (clause 45) assigned."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Command Completion Interrupt Mask"]
    pub mod CIM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const ZERO: u32 = 0;
            #[doc = "Enabled"]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Command Completion"]
    pub mod CMP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An MDIO command completion did not occur."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO command completion occurred."]
            pub const ONE: u32 = 0x01;
        }
    }
    #[doc = "Busy 1"]
    pub mod BSY1 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An MDIO transaction is not occurring; software may access other MDIO registers."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO transaction is occurring."]
            pub const ONE: u32 = 0x01;
        }
    }
}
#[doc = "Port external MDIO interface control register"]
pub mod PEMDIOICR {
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    pub mod DEV_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    pub mod PORT_ADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO read with address post-increment initiation. Self-clearing once transaction is complete."]
    pub mod POST_INC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO read initiation."]
    pub mod READ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port external MDIO interface data register"]
pub mod PEMDIOIDR {
    #[doc = "16-bit MDIO data."]
    pub mod MDIO_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port external MDIO register address register"]
pub mod PEMDIORAR {
    #[doc = "MDIO PHY register address."]
    pub mod REGADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port external MDIO status register"]
pub mod PEMDIOSR {
    #[doc = "Global MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY white list"]
    pub mod WHT_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY white list enable"]
    pub mod WHT_LIST_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port ID"]
    pub mod PORT_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port ID"]
    pub mod REQ_TYPE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY status configuration register"]
pub mod PPSCR {
    #[doc = "MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO read error"]
    pub mod MDIO_RD_ER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHY status read interval"]
    pub mod STATUS_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port PHY status control register"]
pub mod PPSCTRLR {
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    pub mod DEV_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    pub mod PORT_ADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port PHY status data register"]
pub mod PPSDR {
    #[doc = "16-bit MDIO data"]
    pub mod MDIO_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current count"]
    pub mod CURR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port PHY status register address register"]
pub mod PPSRAR {
    #[doc = "MDIO PHY register address. Address of the register within the Clause 45 PHY device from which data is to be read."]
    pub mod REGADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port PHY status event register"]
pub mod PPSER {
    #[doc = "Status event high-to-low. Set to 1 if a 1->0 transition on a corresponding data bit has occurred. Write 1 to clear."]
    pub mod STATUS_EVENT_HL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status event low-to-high. Set to 1 if a 0->1 transition on a corresponding data bit has occurred. Write 1 to clear."]
    pub mod STATUS_EVENT_LH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port PHY status mask register"]
pub mod PPSMR {
    #[doc = "Status high-to-low mask. If set to 1, assert an interrupt if the corresponding event bit is set."]
    pub mod STATUS_MASK_HL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status mask low-to-high. If set to 1, assert an interrupt if the corresponding event bit is set."]
    pub mod STATUS_MASK_LH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
