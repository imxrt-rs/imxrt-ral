#[doc = "ENET_QOS"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "MAC Configuration Register"]
    pub MAC_CONFIGURATION: crate::RWRegister<u32>,
    #[doc = "MAC Extended Configuration Register"]
    pub MAC_EXT_CONFIGURATION: crate::RWRegister<u32>,
    #[doc = "MAC Packet Filter"]
    pub MAC_PACKET_FILTER: crate::RWRegister<u32>,
    #[doc = "Watchdog Timeout"]
    pub MAC_WATCHDOG_TIMEOUT: crate::RWRegister<u32>,
    #[doc = "MAC Hash Table Register 0"]
    pub MAC_HASH_TABLE_REG0: crate::RWRegister<u32>,
    #[doc = "MAC Hash Table Register 1"]
    pub MAC_HASH_TABLE_REG1: crate::RWRegister<u32>,
    _reserved0: [u8; 0x38],
    #[doc = "MAC VLAN Tag Control"]
    pub MAC_VLAN_TAG_CTRL: crate::RWRegister<u32>,
    #[doc = "MAC VLAN Tag Data"]
    pub MAC_VLAN_TAG_DATA: crate::RWRegister<u32>,
    #[doc = "MAC VLAN Hash Table"]
    pub MAC_VLAN_HASH_TABLE: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "VLAN Tag Inclusion or Replacement"]
    pub MAC_VLAN_INCL: crate::RWRegister<u32>,
    #[doc = "MAC Inner VLAN Tag Inclusion or Replacement"]
    pub MAC_INNER_VLAN_INCL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "MAC Q0 Tx Flow Control"]
    pub MAC_Q0_TX_FLOW_CTRL: crate::RWRegister<u32>,
    #[doc = "MAC Q1 Tx Flow Control"]
    pub MAC_Q1_TX_FLOW_CTRL: crate::RWRegister<u32>,
    #[doc = "MAC Q2 Tx Flow Control"]
    pub MAC_Q2_TX_FLOW_CTRL: crate::RWRegister<u32>,
    #[doc = "MAC Q3 Tx Flow Control"]
    pub MAC_Q3_TX_FLOW_CTRL: crate::RWRegister<u32>,
    #[doc = "MAC Q4 Tx Flow Control"]
    pub MAC_Q4_TX_FLOW_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "MAC Rx Flow Control"]
    pub MAC_RX_FLOW_CTRL: crate::RWRegister<u32>,
    #[doc = "Receive Queue Control 4"]
    pub MAC_RXQ_CTRL4: crate::RWRegister<u32>,
    #[doc = "Transmit Queue Priority Mapping 0"]
    pub MAC_TXQ_PRTY_MAP0: crate::RWRegister<u32>,
    #[doc = "Transmit Queue Priority Mapping 1"]
    pub MAC_TXQ_PRTY_MAP1: crate::RWRegister<u32>,
    #[doc = "Receive Queue Control 0"]
    pub MAC_RXQ_CTRL0: crate::RWRegister<u32>,
    #[doc = "Receive Queue Control 1"]
    pub MAC_RXQ_CTRL1: crate::RWRegister<u32>,
    #[doc = "Receive Queue Control 2"]
    pub MAC_RXQ_CTRL2: crate::RWRegister<u32>,
    #[doc = "Receive Queue Control 3"]
    pub MAC_RXQ_CTRL3: crate::RWRegister<u32>,
    #[doc = "Interrupt Status"]
    pub MAC_INTERRUPT_STATUS: crate::RORegister<u32>,
    #[doc = "Interrupt Enable"]
    pub MAC_INTERRUPT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Receive Transmit Status"]
    pub MAC_RX_TX_STATUS: crate::RORegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "PMT Control and Status"]
    pub MAC_PMT_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "Remote Wakeup Filter"]
    pub MAC_RWK_PACKET_FILTER: crate::RWRegister<u32>,
    _reserved5: [u8; 0x08],
    #[doc = "LPI Control and Status"]
    pub MAC_LPI_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "LPI Timers Control"]
    pub MAC_LPI_TIMERS_CONTROL: crate::RWRegister<u32>,
    #[doc = "Tx LPI Entry Timer Control"]
    pub MAC_LPI_ENTRY_TIMER: crate::RWRegister<u32>,
    #[doc = "One-microsecond Reference Timer"]
    pub MAC_ONEUS_TIC_COUNTER: crate::RWRegister<u32>,
    _reserved6: [u8; 0x18],
    #[doc = "PHY Interface Control and Status"]
    pub MAC_PHYIF_CONTROL_STATUS: crate::RWRegister<u32>,
    _reserved7: [u8; 0x14],
    #[doc = "MAC Version"]
    pub MAC_VERSION: crate::RORegister<u32>,
    #[doc = "MAC Debug"]
    pub MAC_DEBUG: crate::RORegister<u32>,
    _reserved8: [u8; 0x04],
    #[doc = "Optional Features or Functions 0"]
    pub MAC_HW_FEATURE0: crate::RORegister<u32>,
    #[doc = "Optional Features or Functions 1"]
    pub MAC_HW_FEATURE1: crate::RORegister<u32>,
    #[doc = "Optional Features or Functions 2"]
    pub MAC_HW_FEATURE2: crate::RORegister<u32>,
    #[doc = "Optional Features or Functions 3"]
    pub MAC_HW_FEATURE3: crate::RORegister<u32>,
    _reserved9: [u8; 0xd4],
    #[doc = "MDIO Address"]
    pub MAC_MDIO_ADDRESS: crate::RWRegister<u32>,
    #[doc = "MAC MDIO Data"]
    pub MAC_MDIO_DATA: crate::RWRegister<u32>,
    _reserved10: [u8; 0x28],
    #[doc = "CSR Software Control"]
    pub MAC_CSR_SW_CTRL: crate::RWRegister<u32>,
    #[doc = "Frame Preemption Control"]
    pub MAC_FPE_CTRL_STS: crate::RWRegister<u32>,
    _reserved11: [u8; 0x08],
    #[doc = "32-bit Binary Rollover Equivalent Time"]
    pub MAC_PRESN_TIME_NS: crate::RORegister<u32>,
    #[doc = "MAC 1722 Presentation Time"]
    pub MAC_PRESN_TIME_UPDT: crate::RWRegister<u32>,
    _reserved12: [u8; 0xb8],
    #[doc = "MAC Address0 High"]
    pub MAC_ADDRESS0_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address0 Low"]
    pub MAC_ADDRESS0_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address1 High"]
    pub MAC_ADDRESS1_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address1 Low"]
    pub MAC_ADDRESS1_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address2 High"]
    pub MAC_ADDRESS2_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address2 Low"]
    pub MAC_ADDRESS2_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address3 High"]
    pub MAC_ADDRESS3_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address3 Low"]
    pub MAC_ADDRESS3_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address4 High"]
    pub MAC_ADDRESS4_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address4 Low"]
    pub MAC_ADDRESS4_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address5 High"]
    pub MAC_ADDRESS5_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address5 Low"]
    pub MAC_ADDRESS5_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address6 High"]
    pub MAC_ADDRESS6_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address6 Low"]
    pub MAC_ADDRESS6_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address7 High"]
    pub MAC_ADDRESS7_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address7 Low"]
    pub MAC_ADDRESS7_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address8 High"]
    pub MAC_ADDRESS8_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address8 Low"]
    pub MAC_ADDRESS8_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address9 High"]
    pub MAC_ADDRESS9_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address9 Low"]
    pub MAC_ADDRESS9_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address10 High"]
    pub MAC_ADDRESS10_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address10 Low"]
    pub MAC_ADDRESS10_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address11 High"]
    pub MAC_ADDRESS11_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address11 Low"]
    pub MAC_ADDRESS11_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address12 High"]
    pub MAC_ADDRESS12_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address12 Low"]
    pub MAC_ADDRESS12_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address13 High"]
    pub MAC_ADDRESS13_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address13 Low"]
    pub MAC_ADDRESS13_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address14 High"]
    pub MAC_ADDRESS14_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address14 Low"]
    pub MAC_ADDRESS14_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address15 High"]
    pub MAC_ADDRESS15_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address15 Low"]
    pub MAC_ADDRESS15_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address16 High"]
    pub MAC_ADDRESS16_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address16 Low"]
    pub MAC_ADDRESS16_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address17 High"]
    pub MAC_ADDRESS17_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address17 Low"]
    pub MAC_ADDRESS17_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address18 High"]
    pub MAC_ADDRESS18_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address18 Low"]
    pub MAC_ADDRESS18_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address19 High"]
    pub MAC_ADDRESS19_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address19 Low"]
    pub MAC_ADDRESS19_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address20 High"]
    pub MAC_ADDRESS20_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address20 Low"]
    pub MAC_ADDRESS20_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address21 High"]
    pub MAC_ADDRESS21_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address21 Low"]
    pub MAC_ADDRESS21_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address22 High"]
    pub MAC_ADDRESS22_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address22 Low"]
    pub MAC_ADDRESS22_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address23 High"]
    pub MAC_ADDRESS23_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address23 Low"]
    pub MAC_ADDRESS23_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address24 High"]
    pub MAC_ADDRESS24_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address24 Low"]
    pub MAC_ADDRESS24_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address25 High"]
    pub MAC_ADDRESS25_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address25 Low"]
    pub MAC_ADDRESS25_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address26 High"]
    pub MAC_ADDRESS26_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address26 Low"]
    pub MAC_ADDRESS26_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address27 High"]
    pub MAC_ADDRESS27_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address27 Low"]
    pub MAC_ADDRESS27_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address28 High"]
    pub MAC_ADDRESS28_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address28 Low"]
    pub MAC_ADDRESS28_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address29 High"]
    pub MAC_ADDRESS29_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address29 Low"]
    pub MAC_ADDRESS29_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address30 High"]
    pub MAC_ADDRESS30_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address30 Low"]
    pub MAC_ADDRESS30_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address31 High"]
    pub MAC_ADDRESS31_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address31 Low"]
    pub MAC_ADDRESS31_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address32 High"]
    pub MAC_ADDRESS32_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address32 Low"]
    pub MAC_ADDRESS32_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address33 High"]
    pub MAC_ADDRESS33_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address33 Low"]
    pub MAC_ADDRESS33_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address34 High"]
    pub MAC_ADDRESS34_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address34 Low"]
    pub MAC_ADDRESS34_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address35 High"]
    pub MAC_ADDRESS35_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address35 Low"]
    pub MAC_ADDRESS35_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address36 High"]
    pub MAC_ADDRESS36_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address36 Low"]
    pub MAC_ADDRESS36_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address37 High"]
    pub MAC_ADDRESS37_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address37 Low"]
    pub MAC_ADDRESS37_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address38 High"]
    pub MAC_ADDRESS38_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address38 Low"]
    pub MAC_ADDRESS38_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address39 High"]
    pub MAC_ADDRESS39_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address39 Low"]
    pub MAC_ADDRESS39_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address40 High"]
    pub MAC_ADDRESS40_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address40 Low"]
    pub MAC_ADDRESS40_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address41 High"]
    pub MAC_ADDRESS41_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address41 Low"]
    pub MAC_ADDRESS41_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address42 High"]
    pub MAC_ADDRESS42_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address42 Low"]
    pub MAC_ADDRESS42_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address43 High"]
    pub MAC_ADDRESS43_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address43 Low"]
    pub MAC_ADDRESS43_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address44 High"]
    pub MAC_ADDRESS44_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address44 Low"]
    pub MAC_ADDRESS44_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address45 High"]
    pub MAC_ADDRESS45_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address45 Low"]
    pub MAC_ADDRESS45_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address46 High"]
    pub MAC_ADDRESS46_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address46 Low"]
    pub MAC_ADDRESS46_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address47 High"]
    pub MAC_ADDRESS47_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address47 Low"]
    pub MAC_ADDRESS47_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address48 High"]
    pub MAC_ADDRESS48_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address48 Low"]
    pub MAC_ADDRESS48_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address49 High"]
    pub MAC_ADDRESS49_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address49 Low"]
    pub MAC_ADDRESS49_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address50 High"]
    pub MAC_ADDRESS50_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address50 Low"]
    pub MAC_ADDRESS50_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address51 High"]
    pub MAC_ADDRESS51_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address51 Low"]
    pub MAC_ADDRESS51_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address52 High"]
    pub MAC_ADDRESS52_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address52 Low"]
    pub MAC_ADDRESS52_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address53 High"]
    pub MAC_ADDRESS53_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address53 Low"]
    pub MAC_ADDRESS53_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address54 High"]
    pub MAC_ADDRESS54_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address54 Low"]
    pub MAC_ADDRESS54_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address55 High"]
    pub MAC_ADDRESS55_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address55 Low"]
    pub MAC_ADDRESS55_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address56 High"]
    pub MAC_ADDRESS56_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address56 Low"]
    pub MAC_ADDRESS56_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address57 High"]
    pub MAC_ADDRESS57_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address57 Low"]
    pub MAC_ADDRESS57_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address58 High"]
    pub MAC_ADDRESS58_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address58 Low"]
    pub MAC_ADDRESS58_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address59 High"]
    pub MAC_ADDRESS59_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address59 Low"]
    pub MAC_ADDRESS59_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address60 High"]
    pub MAC_ADDRESS60_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address60 Low"]
    pub MAC_ADDRESS60_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address61 High"]
    pub MAC_ADDRESS61_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address61 Low"]
    pub MAC_ADDRESS61_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address62 High"]
    pub MAC_ADDRESS62_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address62 Low"]
    pub MAC_ADDRESS62_LOW: crate::RWRegister<u32>,
    #[doc = "MAC Address63 High"]
    pub MAC_ADDRESS63_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address63 Low"]
    pub MAC_ADDRESS63_LOW: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0200],
    #[doc = "MMC Control"]
    pub MAC_MMC_CONTROL: crate::RWRegister<u32>,
    #[doc = "MMC Rx Interrupt"]
    pub MAC_MMC_RX_INTERRUPT: crate::RORegister<u32>,
    #[doc = "MMC Tx Interrupt"]
    pub MAC_MMC_TX_INTERRUPT: crate::RORegister<u32>,
    #[doc = "MMC Rx Interrupt Mask"]
    pub MAC_MMC_RX_INTERRUPT_MASK: crate::RWRegister<u32>,
    #[doc = "MMC Tx Interrupt Mask"]
    pub MAC_MMC_TX_INTERRUPT_MASK: crate::RWRegister<u32>,
    #[doc = "Tx Octet Count Good and Bad"]
    pub MAC_TX_OCTET_COUNT_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Tx Packet Count Good and Bad"]
    pub MAC_TX_PACKET_COUNT_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Tx Broadcast Packets Good"]
    pub MAC_TX_BROADCAST_PACKETS_GOOD: crate::RORegister<u32>,
    #[doc = "Tx Multicast Packets Good"]
    pub MAC_TX_MULTICAST_PACKETS_GOOD: crate::RORegister<u32>,
    #[doc = "Tx Good and Bad 64-Byte Packets"]
    pub MAC_TX_64OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Tx Good and Bad 65 to 127-Byte Packets"]
    pub MAC_TX_65TO127OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Tx Good and Bad 128 to 255-Byte Packets"]
    pub MAC_TX_128TO255OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Tx Good and Bad 256 to 511-Byte Packets"]
    pub MAC_TX_256TO511OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Tx Good and Bad 512 to 1023-Byte Packets"]
    pub MAC_TX_512TO1023OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Tx Good and Bad 1024 to Max-Byte Packets"]
    pub MAC_TX_1024TOMAXOCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good and Bad Unicast Packets Transmitted"]
    pub MAC_TX_UNICAST_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good and Bad Multicast Packets Transmitted"]
    pub MAC_TX_MULTICAST_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good and Bad Broadcast Packets Transmitted"]
    pub MAC_TX_BROADCAST_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Tx Packets Aborted By Underflow Error"]
    pub MAC_TX_UNDERFLOW_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Single Collision Good Packets Transmitted"]
    pub MAC_TX_SINGLE_COLLISION_GOOD_PACKETS: crate::RORegister<u32>,
    #[doc = "Multiple Collision Good Packets Transmitted"]
    pub MAC_TX_MULTIPLE_COLLISION_GOOD_PACKETS: crate::RORegister<u32>,
    #[doc = "Deferred Packets Transmitted"]
    pub MAC_TX_DEFERRED_PACKETS: crate::RORegister<u32>,
    #[doc = "Late Collision Packets Transmitted"]
    pub MAC_TX_LATE_COLLISION_PACKETS: crate::RORegister<u32>,
    #[doc = "Excessive Collision Packets Transmitted"]
    pub MAC_TX_EXCESSIVE_COLLISION_PACKETS: crate::RORegister<u32>,
    #[doc = "Carrier Error Packets Transmitted"]
    pub MAC_TX_CARRIER_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Bytes Transmitted in Good Packets"]
    pub MAC_TX_OCTET_COUNT_GOOD: crate::RORegister<u32>,
    #[doc = "Good Packets Transmitted"]
    pub MAC_TX_PACKET_COUNT_GOOD: crate::RORegister<u32>,
    #[doc = "Packets Aborted By Excessive Deferral Error"]
    pub MAC_TX_EXCESSIVE_DEFERRAL_ERROR: crate::RORegister<u32>,
    #[doc = "Pause Packets Transmitted"]
    pub MAC_TX_PAUSE_PACKETS: crate::RORegister<u32>,
    #[doc = "Good VLAN Packets Transmitted"]
    pub MAC_TX_VLAN_PACKETS_GOOD: crate::RORegister<u32>,
    #[doc = "Good Oversize Packets Transmitted"]
    pub MAC_TX_OSIZE_PACKETS_GOOD: crate::RORegister<u32>,
    _reserved14: [u8; 0x04],
    #[doc = "Good and Bad Packets Received"]
    pub MAC_RX_PACKETS_COUNT_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Bytes in Good and Bad Packets Received"]
    pub MAC_RX_OCTET_COUNT_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Bytes in Good Packets Received"]
    pub MAC_RX_OCTET_COUNT_GOOD: crate::RORegister<u32>,
    #[doc = "Good Broadcast Packets Received"]
    pub MAC_RX_BROADCAST_PACKETS_GOOD: crate::RORegister<u32>,
    #[doc = "Good Multicast Packets Received"]
    pub MAC_RX_MULTICAST_PACKETS_GOOD: crate::RORegister<u32>,
    #[doc = "CRC Error Packets Received"]
    pub MAC_RX_CRC_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Alignment Error Packets Received"]
    pub MAC_RX_ALIGNMENT_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Runt Error Packets Received"]
    pub MAC_RX_RUNT_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Jabber Error Packets Received"]
    pub MAC_RX_JABBER_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Good Undersize Packets Received"]
    pub MAC_RX_UNDERSIZE_PACKETS_GOOD: crate::RORegister<u32>,
    #[doc = "Good Oversize Packets Received"]
    pub MAC_RX_OVERSIZE_PACKETS_GOOD: crate::RORegister<u32>,
    #[doc = "Good and Bad 64-Byte Packets Received"]
    pub MAC_RX_64OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good and Bad 64-to-127 Byte Packets Received"]
    pub MAC_RX_65TO127OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good and Bad 128-to-255 Byte Packets Received"]
    pub MAC_RX_128TO255OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good and Bad 256-to-511 Byte Packets Received"]
    pub MAC_RX_256TO511OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good and Bad 512-to-1023 Byte Packets Received"]
    pub MAC_RX_512TO1023OCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good and Bad 1024-to-Max Byte Packets Received"]
    pub MAC_RX_1024TOMAXOCTETS_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Good Unicast Packets Received"]
    pub MAC_RX_UNICAST_PACKETS_GOOD: crate::RORegister<u32>,
    #[doc = "Length Error Packets Received"]
    pub MAC_RX_LENGTH_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Out-of-range Type Packets Received"]
    pub MAC_RX_OUT_OF_RANGE_TYPE_PACKETS: crate::RORegister<u32>,
    #[doc = "Pause Packets Received"]
    pub MAC_RX_PAUSE_PACKETS: crate::RORegister<u32>,
    #[doc = "Missed Packets Due to FIFO Overflow"]
    pub MAC_RX_FIFO_OVERFLOW_PACKETS: crate::RORegister<u32>,
    #[doc = "Good and Bad VLAN Packets Received"]
    pub MAC_RX_VLAN_PACKETS_GOOD_BAD: crate::RORegister<u32>,
    #[doc = "Watchdog Error Packets Received"]
    pub MAC_RX_WATCHDOG_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Receive Error Packets Received"]
    pub MAC_RX_RECEIVE_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "Good Control Packets Received"]
    pub MAC_RX_CONTROL_PACKETS_GOOD: crate::RORegister<u32>,
    _reserved15: [u8; 0x04],
    #[doc = "Microseconds Tx LPI Asserted"]
    pub MAC_TX_LPI_USEC_CNTR: crate::RORegister<u32>,
    #[doc = "Number of Times Tx LPI Asserted"]
    pub MAC_TX_LPI_TRAN_CNTR: crate::RORegister<u32>,
    #[doc = "Microseconds Rx LPI Sampled"]
    pub MAC_RX_LPI_USEC_CNTR: crate::RORegister<u32>,
    #[doc = "Number of Times Rx LPI Entered"]
    pub MAC_RX_LPI_TRAN_CNTR: crate::RORegister<u32>,
    _reserved16: [u8; 0x04],
    #[doc = "MMC IPC Receive Interrupt Mask"]
    pub MAC_MMC_IPC_RX_INTERRUPT_MASK: crate::RWRegister<u32>,
    _reserved17: [u8; 0x04],
    #[doc = "MMC IPC Receive Interrupt"]
    pub MAC_MMC_IPC_RX_INTERRUPT: crate::RORegister<u32>,
    _reserved18: [u8; 0x04],
    #[doc = "Good IPv4 Datagrams Received"]
    pub MAC_RXIPV4_GOOD_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv4 Datagrams Received with Header Errors"]
    pub MAC_RXIPV4_HEADER_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv4 Datagrams Received with No Payload"]
    pub MAC_RXIPV4_NO_PAYLOAD_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv4 Datagrams Received with Fragmentation"]
    pub MAC_RXIPV4_FRAGMENTED_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv4 Datagrams Received with UDP Checksum Disabled"]
    pub MAC_RXIPV4_UDP_CHECKSUM_DISABLED_PACKETS: crate::RORegister<u32>,
    #[doc = "Good IPv6 Datagrams Received"]
    pub MAC_RXIPV6_GOOD_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv6 Datagrams Received with Header Errors"]
    pub MAC_RXIPV6_HEADER_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv6 Datagrams Received with No Payload"]
    pub MAC_RXIPV6_NO_PAYLOAD_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv6 Datagrams Received with Good UDP"]
    pub MAC_RXUDP_GOOD_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv6 Datagrams Received with UDP Checksum Error"]
    pub MAC_RXUDP_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv6 Datagrams Received with Good TCP Payload"]
    pub MAC_RXTCP_GOOD_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv6 Datagrams Received with TCP Checksum Error"]
    pub MAC_RXTCP_ERROR_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv6 Datagrams Received with Good ICMP Payload"]
    pub MAC_RXICMP_GOOD_PACKETS: crate::RORegister<u32>,
    #[doc = "IPv6 Datagrams Received with ICMP Checksum Error"]
    pub MAC_RXICMP_ERROR_PACKETS: crate::RORegister<u32>,
    _reserved19: [u8; 0x08],
    #[doc = "Good Bytes Received in IPv4 Datagrams"]
    pub MAC_RXIPV4_GOOD_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in IPv4 Datagrams with Header Errors"]
    pub MAC_RXIPV4_HEADER_ERROR_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in IPv4 Datagrams with No Payload"]
    pub MAC_RXIPV4_NO_PAYLOAD_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in Fragmented IPv4 Datagrams"]
    pub MAC_RXIPV4_FRAGMENTED_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received with UDP Checksum Disabled"]
    pub MAC_RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in Good IPv6 Datagrams"]
    pub MAC_RXIPV6_GOOD_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in IPv6 Datagrams with Data Errors"]
    pub MAC_RXIPV6_HEADER_ERROR_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in IPv6 Datagrams with No Payload"]
    pub MAC_RXIPV6_NO_PAYLOAD_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in Good UDP Segment"]
    pub MAC_RXUDP_GOOD_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in UDP Segment with Checksum Errors"]
    pub MAC_RXUDP_ERROR_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in Good TCP Segment"]
    pub MAC_RXTCP_GOOD_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in TCP Segment with Checksum Errors"]
    pub MAC_RXTCP_ERROR_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in Good ICMP Segment"]
    pub MAC_RXICMP_GOOD_OCTETS: crate::RORegister<u32>,
    #[doc = "Bytes Received in ICMP Segment with Checksum Errors"]
    pub MAC_RXICMP_ERROR_OCTETS: crate::RORegister<u32>,
    _reserved20: [u8; 0x18],
    #[doc = "MMC FPE Transmit Interrupt"]
    pub MAC_MMC_FPE_TX_INTERRUPT: crate::RORegister<u32>,
    #[doc = "MMC FPE Transmit Mask Interrupt"]
    pub MAC_MMC_FPE_TX_INTERRUPT_MASK: crate::RWRegister<u32>,
    #[doc = "MMC FPE Transmitted Fragment Counter"]
    pub MAC_MMC_TX_FPE_FRAGMENT_CNTR: crate::RORegister<u32>,
    #[doc = "MMC FPE Transmitted Hold Request Counter"]
    pub MAC_MMC_TX_HOLD_REQ_CNTR: crate::RORegister<u32>,
    _reserved21: [u8; 0x10],
    #[doc = "MMC FPE Receive Interrupt"]
    pub MAC_MMC_FPE_RX_INTERRUPT: crate::RORegister<u32>,
    #[doc = "MMC FPE Receive Interrupt Mask"]
    pub MAC_MMC_FPE_RX_INTERRUPT_MASK: crate::RWRegister<u32>,
    #[doc = "MMC Receive Packet Reassembly Error Counter"]
    pub MAC_MMC_RX_PACKET_ASSEMBLY_ERR_CNTR: crate::RORegister<u32>,
    #[doc = "MMC Receive Packet SMD Error Counter"]
    pub MAC_MMC_RX_PACKET_SMD_ERR_CNTR: crate::RORegister<u32>,
    #[doc = "MMC Receive Packet Successful Reassembly Counter"]
    pub MAC_MMC_RX_PACKET_ASSEMBLY_OK_CNTR: crate::RORegister<u32>,
    #[doc = "MMC FPE Received Fragment Counter"]
    pub MAC_MMC_RX_FPE_FRAGMENT_CNTR: crate::RORegister<u32>,
    _reserved22: [u8; 0x28],
    #[doc = "Layer 3 and Layer 4 Control of Filter 0"]
    pub MAC_L3_L4_CONTROL0: crate::RWRegister<u32>,
    #[doc = "Layer 4 Address 0"]
    pub MAC_LAYER4_ADDRESS0: crate::RWRegister<u32>,
    _reserved23: [u8; 0x08],
    #[doc = "Layer 3 Address 0 Register 0"]
    pub MAC_LAYER3_ADDR0_REG0: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 1 Register 0"]
    pub MAC_LAYER3_ADDR1_REG0: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 2 Register 0"]
    pub MAC_LAYER3_ADDR2_REG0: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 3 Register 0"]
    pub MAC_LAYER3_ADDR3_REG0: crate::RWRegister<u32>,
    _reserved24: [u8; 0x10],
    #[doc = "Layer 3 and Layer 4 Control of Filter 1"]
    pub MAC_L3_L4_CONTROL1: crate::RWRegister<u32>,
    #[doc = "Layer 4 Address 0"]
    pub MAC_LAYER4_ADDRESS1: crate::RWRegister<u32>,
    _reserved25: [u8; 0x08],
    #[doc = "Layer 3 Address 0 Register 1"]
    pub MAC_LAYER3_ADDR0_REG1: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 1 Register 1"]
    pub MAC_LAYER3_ADDR1_REG1: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 2 Register 1"]
    pub MAC_LAYER3_ADDR2_REG1: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 3 Register 1"]
    pub MAC_LAYER3_ADDR3_REG1: crate::RWRegister<u32>,
    _reserved26: [u8; 0x10],
    #[doc = "Layer 3 and Layer 4 Control of Filter 2"]
    pub MAC_L3_L4_CONTROL2: crate::RWRegister<u32>,
    #[doc = "Layer 4 Address 2"]
    pub MAC_LAYER4_ADDRESS2: crate::RWRegister<u32>,
    _reserved27: [u8; 0x08],
    #[doc = "Layer 3 Address 0 Register 2"]
    pub MAC_LAYER3_ADDR0_REG2: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 0 Register 2"]
    pub MAC_LAYER3_ADDR1_REG2: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 2 Register 2"]
    pub MAC_LAYER3_ADDR2_REG2: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 3 Register 2"]
    pub MAC_LAYER3_ADDR3_REG2: crate::RWRegister<u32>,
    _reserved28: [u8; 0x10],
    #[doc = "Layer 3 and Layer 4 Control of Filter 3"]
    pub MAC_L3_L4_CONTROL3: crate::RWRegister<u32>,
    #[doc = "Layer 4 Address 3"]
    pub MAC_LAYER4_ADDRESS3: crate::RWRegister<u32>,
    _reserved29: [u8; 0x08],
    #[doc = "Layer 3 Address 0 Register 3"]
    pub MAC_LAYER3_ADDR0_REG3: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 1 Register 3"]
    pub MAC_LAYER3_ADDR1_REG3: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 2 Register 3"]
    pub MAC_LAYER3_ADDR2_REG3: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 3 Register 3"]
    pub MAC_LAYER3_ADDR3_REG3: crate::RWRegister<u32>,
    _reserved30: [u8; 0x10],
    #[doc = "Layer 3 and Layer 4 Control of Filter 4"]
    pub MAC_L3_L4_CONTROL4: crate::RWRegister<u32>,
    #[doc = "Layer 4 Address 4"]
    pub MAC_LAYER4_ADDRESS4: crate::RWRegister<u32>,
    _reserved31: [u8; 0x08],
    #[doc = "Layer 3 Address 0 Register 4"]
    pub MAC_LAYER3_ADDR0_REG4: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 1 Register 4"]
    pub MAC_LAYER3_ADDR1_REG4: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 2 Register 4"]
    pub MAC_LAYER3_ADDR2_REG4: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 3 Register 4"]
    pub MAC_LAYER3_ADDR3_REG4: crate::RWRegister<u32>,
    _reserved32: [u8; 0x10],
    #[doc = "Layer 3 and Layer 4 Control of Filter 5"]
    pub MAC_L3_L4_CONTROL5: crate::RWRegister<u32>,
    #[doc = "Layer 4 Address 5"]
    pub MAC_LAYER4_ADDRESS5: crate::RWRegister<u32>,
    _reserved33: [u8; 0x08],
    #[doc = "Layer 3 Address 0 Register 5"]
    pub MAC_LAYER3_ADDR0_REG5: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 1 Register 5"]
    pub MAC_LAYER3_ADDR1_REG5: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 2 Register 5"]
    pub MAC_LAYER3_ADDR2_REG5: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 3 Register 5"]
    pub MAC_LAYER3_ADDR3_REG5: crate::RWRegister<u32>,
    _reserved34: [u8; 0x10],
    #[doc = "Layer 3 and Layer 4 Control of Filter 6"]
    pub MAC_L3_L4_CONTROL6: crate::RWRegister<u32>,
    #[doc = "Layer 4 Address 6"]
    pub MAC_LAYER4_ADDRESS6: crate::RWRegister<u32>,
    _reserved35: [u8; 0x08],
    #[doc = "Layer 3 Address 0 Register 6"]
    pub MAC_LAYER3_ADDR0_REG6: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 1 Register 6"]
    pub MAC_LAYER3_ADDR1_REG6: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 2 Register 6"]
    pub MAC_LAYER3_ADDR2_REG6: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 3 Register 6"]
    pub MAC_LAYER3_ADDR3_REG6: crate::RWRegister<u32>,
    _reserved36: [u8; 0x10],
    #[doc = "Layer 3 and Layer 4 Control of Filter 0"]
    pub MAC_L3_L4_CONTROL7: crate::RWRegister<u32>,
    #[doc = "Layer 4 Address 7"]
    pub MAC_LAYER4_ADDRESS7: crate::RWRegister<u32>,
    _reserved37: [u8; 0x08],
    #[doc = "Layer 3 Address 0 Register 7"]
    pub MAC_LAYER3_ADDR0_REG7: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 1 Register 7"]
    pub MAC_LAYER3_ADDR1_REG7: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 2 Register 7"]
    pub MAC_LAYER3_ADDR2_REG7: crate::RWRegister<u32>,
    #[doc = "Layer 3 Address 3 Register 7"]
    pub MAC_LAYER3_ADDR3_REG7: crate::RWRegister<u32>,
    _reserved38: [u8; 0x90],
    #[doc = "Timestamp Control"]
    pub MAC_TIMESTAMP_CONTROL: crate::RWRegister<u32>,
    #[doc = "Subsecond Increment"]
    pub MAC_SUB_SECOND_INCREMENT: crate::RWRegister<u32>,
    #[doc = "System Time Seconds"]
    pub MAC_SYSTEM_TIME_SECONDS: crate::RORegister<u32>,
    #[doc = "System Time Nanoseconds"]
    pub MAC_SYSTEM_TIME_NANOSECONDS: crate::RORegister<u32>,
    #[doc = "System Time Seconds Update"]
    pub MAC_SYSTEM_TIME_SECONDS_UPDATE: crate::RWRegister<u32>,
    #[doc = "System Time Nanoseconds Update"]
    pub MAC_SYSTEM_TIME_NANOSECONDS_UPDATE: crate::RWRegister<u32>,
    #[doc = "Timestamp Addend"]
    pub MAC_TIMESTAMP_ADDEND: crate::RWRegister<u32>,
    #[doc = "System Time - Higher Word Seconds"]
    pub MAC_SYSTEM_TIME_HIGHER_WORD_SECONDS: crate::RWRegister<u32>,
    #[doc = "Timestamp Status"]
    pub MAC_TIMESTAMP_STATUS: crate::RORegister<u32>,
    _reserved39: [u8; 0x0c],
    #[doc = "Transmit Timestamp Status Nanoseconds"]
    pub MAC_TX_TIMESTAMP_STATUS_NANOSECONDS: crate::RORegister<u32>,
    #[doc = "Transmit Timestamp Status Seconds"]
    pub MAC_TX_TIMESTAMP_STATUS_SECONDS: crate::RORegister<u32>,
    _reserved40: [u8; 0x08],
    #[doc = "Auxiliary Timestamp Control"]
    pub MAC_AUXILIARY_CONTROL: crate::RWRegister<u32>,
    _reserved41: [u8; 0x04],
    #[doc = "Auxiliary Timestamp Nanoseconds"]
    pub MAC_AUXILIARY_TIMESTAMP_NANOSECONDS: crate::RORegister<u32>,
    #[doc = "Auxiliary Timestamp Seconds"]
    pub MAC_AUXILIARY_TIMESTAMP_SECONDS: crate::RORegister<u32>,
    #[doc = "Timestamp Ingress Asymmetry Correction"]
    pub MAC_TIMESTAMP_INGRESS_ASYM_CORR: crate::RWRegister<u32>,
    #[doc = "imestamp Egress Asymmetry Correction"]
    pub MAC_TIMESTAMP_EGRESS_ASYM_CORR: crate::RWRegister<u32>,
    #[doc = "Timestamp Ingress Correction Nanosecond"]
    pub MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND: crate::RWRegister<u32>,
    #[doc = "Timestamp Egress Correction Nanosecond"]
    pub MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND: crate::RWRegister<u32>,
    #[doc = "Timestamp Ingress Correction Subnanosecond"]
    pub MAC_TIMESTAMP_INGRESS_CORR_SUBNANOSEC: crate::RWRegister<u32>,
    #[doc = "Timestamp Egress Correction Subnanosecond"]
    pub MAC_TIMESTAMP_EGRESS_CORR_SUBNANOSEC: crate::RWRegister<u32>,
    #[doc = "Timestamp Ingress Latency"]
    pub MAC_TIMESTAMP_INGRESS_LATENCY: crate::RORegister<u32>,
    #[doc = "Timestamp Egress Latency"]
    pub MAC_TIMESTAMP_EGRESS_LATENCY: crate::RORegister<u32>,
    #[doc = "PPS Control"]
    pub MAC_PPS_CONTROL: crate::RWRegister<u32>,
    _reserved42: [u8; 0x0c],
    #[doc = "PPS0 Target Time Seconds"]
    pub MAC_PPS0_TARGET_TIME_SECONDS: crate::RWRegister<u32>,
    #[doc = "PPS0 Target Time Nanoseconds"]
    pub MAC_PPS0_TARGET_TIME_NANOSECONDS: crate::RWRegister<u32>,
    #[doc = "PPS0 Interval"]
    pub MAC_PPS0_INTERVAL: crate::RWRegister<u32>,
    #[doc = "PPS0 Width"]
    pub MAC_PPS0_WIDTH: crate::RWRegister<u32>,
    #[doc = "PPS1 Target Time Seconds"]
    pub MAC_PPS1_TARGET_TIME_SECONDS: crate::RWRegister<u32>,
    #[doc = "PPS1 Target Time Nanoseconds"]
    pub MAC_PPS1_TARGET_TIME_NANOSECONDS: crate::RWRegister<u32>,
    #[doc = "PPS1 Interval"]
    pub MAC_PPS1_INTERVAL: crate::RWRegister<u32>,
    #[doc = "PPS1 Width"]
    pub MAC_PPS1_WIDTH: crate::RWRegister<u32>,
    #[doc = "PPS2 Target Time Seconds"]
    pub MAC_PPS2_TARGET_TIME_SECONDS: crate::RWRegister<u32>,
    #[doc = "PPS2 Target Time Nanoseconds"]
    pub MAC_PPS2_TARGET_TIME_NANOSECONDS: crate::RWRegister<u32>,
    #[doc = "PPS2 Interval"]
    pub MAC_PPS2_INTERVAL: crate::RWRegister<u32>,
    #[doc = "PPS2 Width"]
    pub MAC_PPS2_WIDTH: crate::RWRegister<u32>,
    #[doc = "PPS3 Target Time Seconds"]
    pub MAC_PPS3_TARGET_TIME_SECONDS: crate::RWRegister<u32>,
    #[doc = "PPS3 Target Time Nanoseconds"]
    pub MAC_PPS3_TARGET_TIME_NANOSECONDS: crate::RWRegister<u32>,
    #[doc = "PPS3 Interval"]
    pub MAC_PPS3_INTERVAL: crate::RWRegister<u32>,
    #[doc = "PPS3 Width"]
    pub MAC_PPS3_WIDTH: crate::RWRegister<u32>,
    #[doc = "PTP Offload Engine Control"]
    pub MAC_PTO_CONTROL: crate::RWRegister<u32>,
    #[doc = "Source Port Identity 0"]
    pub MAC_SOURCE_PORT_IDENTITY0: crate::RWRegister<u32>,
    #[doc = "Source Port Identity 1"]
    pub MAC_SOURCE_PORT_IDENTITY1: crate::RWRegister<u32>,
    #[doc = "Source Port Identity 2"]
    pub MAC_SOURCE_PORT_IDENTITY2: crate::RWRegister<u32>,
    #[doc = "Log Message Interval"]
    pub MAC_LOG_MESSAGE_INTERVAL: crate::RWRegister<u32>,
    _reserved43: [u8; 0x2c],
    #[doc = "MTL Operation Mode"]
    pub MTL_OPERATION_MODE: crate::RWRegister<u32>,
    _reserved44: [u8; 0x04],
    #[doc = "FIFO Debug Access Control and Status"]
    pub MTL_DBG_CTL: crate::RWRegister<u32>,
    #[doc = "FIFO Debug Status"]
    pub MTL_DBG_STS: crate::RWRegister<u32>,
    #[doc = "FIFO Debug Data"]
    pub MTL_FIFO_DEBUG_DATA: crate::RWRegister<u32>,
    _reserved45: [u8; 0x0c],
    #[doc = "MTL Interrupt Status"]
    pub MTL_INTERRUPT_STATUS: crate::RORegister<u32>,
    _reserved46: [u8; 0x0c],
    #[doc = "Receive Queue and DMA Channel Mapping 0"]
    pub MTL_RXQ_DMA_MAP0: crate::RWRegister<u32>,
    #[doc = "Receive Queue and DMA Channel Mapping 1"]
    pub MTL_RXQ_DMA_MAP1: crate::RWRegister<u32>,
    _reserved47: [u8; 0x08],
    #[doc = "Time Based Scheduling Control"]
    pub MTL_TBS_CTRL: crate::RWRegister<u32>,
    _reserved48: [u8; 0x0c],
    #[doc = "Enhancements to Scheduled Transmission Control"]
    pub MTL_EST_CONTROL: crate::RWRegister<u32>,
    _reserved49: [u8; 0x04],
    #[doc = "Enhancements to Scheduled Transmission Status"]
    pub MTL_EST_STATUS: crate::RWRegister<u32>,
    _reserved50: [u8; 0x04],
    #[doc = "EST Scheduling Error"]
    pub MTL_EST_SCH_ERROR: crate::RWRegister<u32>,
    #[doc = "EST Frame Size Error"]
    pub MTL_EST_FRM_SIZE_ERROR: crate::RWRegister<u32>,
    #[doc = "EST Frame Size Capture"]
    pub MTL_EST_FRM_SIZE_CAPTURE: crate::RORegister<u32>,
    _reserved51: [u8; 0x04],
    #[doc = "EST Interrupt Enable"]
    pub MTL_EST_INTR_ENABLE: crate::RWRegister<u32>,
    _reserved52: [u8; 0x0c],
    #[doc = "EST GCL Control"]
    pub MTL_EST_GCL_CONTROL: crate::RWRegister<u32>,
    #[doc = "EST GCL Data"]
    pub MTL_EST_GCL_DATA: crate::RWRegister<u32>,
    _reserved53: [u8; 0x08],
    #[doc = "Frame Preemption Control and Status"]
    pub MTL_FPE_CTRL_STS: crate::RWRegister<u32>,
    #[doc = "Frame Preemption Hold and Release Advance"]
    pub MTL_FPE_ADVANCE: crate::RWRegister<u32>,
    _reserved54: [u8; 0x08],
    #[doc = "RXP Control Status"]
    pub MTL_RXP_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "RXP Interrupt Control Status"]
    pub MTL_RXP_INTERRUPT_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "RXP Drop Count"]
    pub MTL_RXP_DROP_CNT: crate::RORegister<u32>,
    #[doc = "RXP Error Count"]
    pub MTL_RXP_ERROR_CNT: crate::RORegister<u32>,
    #[doc = "RXP Indirect Access Control and Status"]
    pub MTL_RXP_INDIRECT_ACC_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "RXP Indirect Access Data"]
    pub MTL_RXP_INDIRECT_ACC_DATA: crate::RWRegister<u32>,
    _reserved55: [u8; 0x48],
    #[doc = "Queue 0 Transmit Operation Mode"]
    pub MTL_TXQ0_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 0 Underflow Counter"]
    pub MTL_TXQ0_UNDERFLOW: crate::RORegister<u32>,
    #[doc = "Queue 0 Transmit Debug"]
    pub MTL_TXQ0_DEBUG: crate::RORegister<u32>,
    _reserved56: [u8; 0x08],
    #[doc = "Queue 0 ETS Status"]
    pub MTL_TXQ0_ETS_STATUS: crate::RORegister<u32>,
    #[doc = "Queue 0 Quantum or Weights"]
    pub MTL_TXQ0_QUANTUM_WEIGHT: crate::RWRegister<u32>,
    _reserved57: [u8; 0x10],
    #[doc = "Queue 0 Interrupt Control Status"]
    pub MTL_Q0_INTERRUPT_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "Queue 0 Receive Operation Mode"]
    pub MTL_RXQ0_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 0 Missed Packet and Overflow Counter"]
    pub MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT: crate::RORegister<u32>,
    #[doc = "Queue 0 Receive Debug"]
    pub MTL_RXQ0_DEBUG: crate::RORegister<u32>,
    #[doc = "Queue 0 Receive Control"]
    pub MTL_RXQ0_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 1 Transmit Operation Mode"]
    pub MTL_TXQ1_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 1 Underflow Counter"]
    pub MTL_TXQ1_UNDERFLOW: crate::RORegister<u32>,
    #[doc = "Queue 1 Transmit Debug"]
    pub MTL_TXQ1_DEBUG: crate::RORegister<u32>,
    _reserved58: [u8; 0x04],
    #[doc = "Queue 1 ETS Control"]
    pub MTL_TXQ1_ETS_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 1 ETS Status"]
    pub MTL_TXQ1_ETS_STATUS: crate::RORegister<u32>,
    #[doc = "Queue 1 idleSlopeCredit, Quantum or Weights"]
    pub MTL_TXQ1_QUANTUM_WEIGHT: crate::RWRegister<u32>,
    #[doc = "Queue 1 sendSlopeCredit"]
    pub MTL_TXQ1_SENDSLOPECREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 1 hiCredit"]
    pub MTL_TXQ1_HICREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 1 loCredit"]
    pub MTL_TXQ1_LOCREDIT: crate::RWRegister<u32>,
    _reserved59: [u8; 0x04],
    #[doc = "Queue 1 Interrupt Control Status"]
    pub MTL_Q1_INTERRUPT_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "Queue 1 Receive Operation Mode"]
    pub MTL_RXQ1_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 1 Missed Packet and Overflow Counter"]
    pub MTL_RXQ1_MISSED_PACKET_OVERFLOW_CNT: crate::RORegister<u32>,
    #[doc = "Queue 1 Receive Debug"]
    pub MTL_RXQ1_DEBUG: crate::RORegister<u32>,
    #[doc = "Queue 1 Receive Control"]
    pub MTL_RXQ1_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 2 Transmit Operation Mode"]
    pub MTL_TXQ2_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 2 Underflow Counter"]
    pub MTL_TXQ2_UNDERFLOW: crate::RORegister<u32>,
    #[doc = "Queue 2 Transmit Debug"]
    pub MTL_TXQ2_DEBUG: crate::RORegister<u32>,
    _reserved60: [u8; 0x04],
    #[doc = "Queue 2 ETS Control"]
    pub MTL_TXQ2_ETS_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 2 ETS Status"]
    pub MTL_TXQ2_ETS_STATUS: crate::RORegister<u32>,
    #[doc = "Queue 2 idleSlopeCredit, Quantum or Weights"]
    pub MTL_TXQ2_QUANTUM_WEIGHT: crate::RWRegister<u32>,
    #[doc = "Queue 2 sendSlopeCredit"]
    pub MTL_TXQ2_SENDSLOPECREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 2 hiCredit"]
    pub MTL_TXQ2_HICREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 2 loCredit"]
    pub MTL_TXQ2_LOCREDIT: crate::RWRegister<u32>,
    _reserved61: [u8; 0x04],
    #[doc = "Queue 2 Interrupt Control Status"]
    pub MTL_Q2_INTERRUPT_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "Queue 2 Receive Operation Mode"]
    pub MTL_RXQ2_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 2 Missed Packet and Overflow Counter"]
    pub MTL_RXQ2_MISSED_PACKET_OVERFLOW_CNT: crate::RORegister<u32>,
    #[doc = "Queue 2 Receive Debug"]
    pub MTL_RXQ2_DEBUG: crate::RORegister<u32>,
    #[doc = "Queue 2 Receive Control"]
    pub MTL_RXQ2_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 3 Transmit Operation Mode"]
    pub MTL_TXQ3_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 3 Underflow Counter"]
    pub MTL_TXQ3_UNDERFLOW: crate::RORegister<u32>,
    #[doc = "Queue 3 Transmit Debug"]
    pub MTL_TXQ3_DEBUG: crate::RORegister<u32>,
    _reserved62: [u8; 0x04],
    #[doc = "Queue 3 ETS Control"]
    pub MTL_TXQ3_ETS_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 3 ETS Status"]
    pub MTL_TXQ3_ETS_STATUS: crate::RORegister<u32>,
    #[doc = "Queue 3 idleSlopeCredit, Quantum or Weights"]
    pub MTL_TXQ3_QUANTUM_WEIGHT: crate::RWRegister<u32>,
    #[doc = "Queue 3 sendSlopeCredit"]
    pub MTL_TXQ3_SENDSLOPECREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 3 hiCredit"]
    pub MTL_TXQ3_HICREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 3 loCredit"]
    pub MTL_TXQ3_LOCREDIT: crate::RWRegister<u32>,
    _reserved63: [u8; 0x04],
    #[doc = "Queue 3 Interrupt Control Status"]
    pub MTL_Q3_INTERRUPT_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "Queue 3 Receive Operation Mode"]
    pub MTL_RXQ3_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 3 Missed Packet and Overflow Counter"]
    pub MTL_RXQ3_MISSED_PACKET_OVERFLOW_CNT: crate::RORegister<u32>,
    #[doc = "Queue 3 Receive Debug"]
    pub MTL_RXQ3_DEBUG: crate::RORegister<u32>,
    #[doc = "Queue 3 Receive Control"]
    pub MTL_RXQ3_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 4 Transmit Operation Mode"]
    pub MTL_TXQ4_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 4 Underflow Counter"]
    pub MTL_TXQ4_UNDERFLOW: crate::RORegister<u32>,
    #[doc = "Queue 4 Transmit Debug"]
    pub MTL_TXQ4_DEBUG: crate::RORegister<u32>,
    _reserved64: [u8; 0x04],
    #[doc = "Queue 4 ETS Control"]
    pub MTL_TXQ4_ETS_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 4 ETS Status"]
    pub MTL_TXQ4_ETS_STATUS: crate::RORegister<u32>,
    #[doc = "Queue 4 idleSlopeCredit, Quantum or Weights"]
    pub MTL_TXQ4_QUANTUM_WEIGHT: crate::RWRegister<u32>,
    #[doc = "Queue 4 sendSlopeCredit"]
    pub MTL_TXQ4_SENDSLOPECREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 4 hiCredit"]
    pub MTL_TXQ4_HICREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 4 loCredit"]
    pub MTL_TXQ4_LOCREDIT: crate::RWRegister<u32>,
    _reserved65: [u8; 0x04],
    #[doc = "Queue 4 Interrupt Control Status"]
    pub MTL_Q4_INTERRUPT_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "Queue 4 Receive Operation Mode"]
    pub MTL_RXQ4_OPERATION_MODE: crate::RWRegister<u32>,
    #[doc = "Queue 4 Missed Packet and Overflow Counter"]
    pub MTL_RXQ4_MISSED_PACKET_OVERFLOW_CNT: crate::RORegister<u32>,
    #[doc = "Queue 4 Receive Debug"]
    pub MTL_RXQ4_DEBUG: crate::RORegister<u32>,
    #[doc = "Queue 4 Receive Control"]
    pub MTL_RXQ4_CONTROL: crate::RWRegister<u32>,
    _reserved66: [u8; 0x01c0],
    #[doc = "DMA Bus Mode"]
    pub DMA_MODE: crate::RWRegister<u32>,
    #[doc = "DMA System Bus Mode"]
    pub DMA_SYSBUS_MODE: crate::RWRegister<u32>,
    #[doc = "DMA Interrupt Status"]
    pub DMA_INTERRUPT_STATUS: crate::RORegister<u32>,
    #[doc = "DMA Debug Status 0"]
    pub DMA_DEBUG_STATUS0: crate::RORegister<u32>,
    #[doc = "DMA Debug Status 1"]
    pub DMA_DEBUG_STATUS1: crate::RORegister<u32>,
    _reserved67: [u8; 0x2c],
    #[doc = "AXI LPI Entry Interval Control"]
    pub DMA_AXI_LPI_ENTRY_INTERVAL: crate::RWRegister<u32>,
    _reserved68: [u8; 0x0c],
    #[doc = "TBS Control"]
    pub DMA_TBS_CTRL: crate::RWRegister<u32>,
    _reserved69: [u8; 0xac],
    #[doc = "DMA Channel 0 Control"]
    pub DMA_CH0_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 0 Transmit Control"]
    pub DMA_CH0_TX_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 0 Receive Control"]
    pub DMA_CH0_RX_CONTROL: crate::RWRegister<u32>,
    _reserved70: [u8; 0x08],
    #[doc = "Channel 0 Tx Descriptor List Address register"]
    pub DMA_CH0_TXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    _reserved71: [u8; 0x04],
    #[doc = "Channel 0 Rx Descriptor List Address register"]
    pub DMA_CH0_RXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    #[doc = "Channel 0 Tx Descriptor Tail Pointer"]
    pub DMA_CH0_TXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    _reserved72: [u8; 0x04],
    #[doc = "Channel 0 Rx Descriptor Tail Pointer"]
    pub DMA_CH0_RXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    #[doc = "Channel 0 Tx Descriptor Ring Length"]
    pub DMA_CH0_TXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 0 Rx Descriptor Ring Length"]
    pub DMA_CH0_RXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 0 Interrupt Enable"]
    pub DMA_CH0_INTERRUPT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Channel 0 Receive Interrupt Watchdog Timer"]
    pub DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER: crate::RWRegister<u32>,
    #[doc = "Channel 0 Slot Function Control and Status"]
    pub DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS: crate::RWRegister<u32>,
    _reserved73: [u8; 0x04],
    #[doc = "Channel 0 Current Application Transmit Descriptor"]
    pub DMA_CH0_CURRENT_APP_TXDESC: crate::RORegister<u32>,
    _reserved74: [u8; 0x04],
    #[doc = "Channel 0 Current Application Receive Descriptor"]
    pub DMA_CH0_CURRENT_APP_RXDESC: crate::RORegister<u32>,
    _reserved75: [u8; 0x04],
    #[doc = "Channel 0 Current Application Transmit Buffer Address"]
    pub DMA_CH0_CURRENT_APP_TXBUFFER: crate::RORegister<u32>,
    _reserved76: [u8; 0x04],
    #[doc = "Channel 0 Current Application Receive Buffer Address"]
    pub DMA_CH0_CURRENT_APP_RXBUFFER: crate::RORegister<u32>,
    #[doc = "DMA Channel 0 Status"]
    pub DMA_CH0_STATUS: crate::RWRegister<u32>,
    #[doc = "Channel 0 Missed Frame Counter"]
    pub DMA_CH0_MISS_FRAME_CNT: crate::RORegister<u32>,
    #[doc = "Channel 0 RXP Frames Accepted Counter"]
    pub DMA_CH0_RXP_ACCEPT_CNT: crate::RORegister<u32>,
    #[doc = "Channel 0 Receive ERI Counter"]
    pub DMA_CH0_RX_ERI_CNT: crate::RORegister<u32>,
    _reserved77: [u8; 0x10],
    #[doc = "DMA Channel 1 Control"]
    pub DMA_CH1_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 1 Transmit Control"]
    pub DMA_CH1_TX_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 1 Receive Control"]
    pub DMA_CH1_RX_CONTROL: crate::RWRegister<u32>,
    _reserved78: [u8; 0x08],
    #[doc = "Channel 1 Tx Descriptor List Address"]
    pub DMA_CH1_TXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    _reserved79: [u8; 0x04],
    #[doc = "Channel 1 Rx Descriptor List Address"]
    pub DMA_CH1_RXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    #[doc = "Channel 1 Tx Descriptor Tail Pointer"]
    pub DMA_CH1_TXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    _reserved80: [u8; 0x04],
    #[doc = "Channel 1 Rx Descriptor Tail Pointer"]
    pub DMA_CH1_RXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    #[doc = "Channel 1 Tx Descriptor Ring Length"]
    pub DMA_CH1_TXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 1 Rx Descriptor Ring Length"]
    pub DMA_CH1_RXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 1 Interrupt Enable"]
    pub DMA_CH1_INTERRUPT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Channel 1 Receive Interrupt Watchdog Timer"]
    pub DMA_CH1_RX_INTERRUPT_WATCHDOG_TIMER: crate::RWRegister<u32>,
    #[doc = "Channel 1 Slot Function Control and Status"]
    pub DMA_CH1_SLOT_FUNCTION_CONTROL_STATUS: crate::RWRegister<u32>,
    _reserved81: [u8; 0x04],
    #[doc = "Channel 1 Current Application Transmit Descriptor"]
    pub DMA_CH1_CURRENT_APP_TXDESC: crate::RORegister<u32>,
    _reserved82: [u8; 0x04],
    #[doc = "Channel 1 Current Application Receive Descriptor"]
    pub DMA_CH1_CURRENT_APP_RXDESC: crate::RORegister<u32>,
    _reserved83: [u8; 0x04],
    #[doc = "Channel 1 Current Application Transmit Buffer Address"]
    pub DMA_CH1_CURRENT_APP_TXBUFFER: crate::RORegister<u32>,
    _reserved84: [u8; 0x04],
    #[doc = "Channel 1 Current Application Receive Buffer Address"]
    pub DMA_CH1_CURRENT_APP_RXBUFFER: crate::RORegister<u32>,
    #[doc = "DMA Channel 1 Status"]
    pub DMA_CH1_STATUS: crate::RWRegister<u32>,
    #[doc = "Channel 1 Missed Frame Counter"]
    pub DMA_CH1_MISS_FRAME_CNT: crate::RORegister<u32>,
    #[doc = "Channel 1 RXP Frames Accepted Counter"]
    pub DMA_CH1_RXP_ACCEPT_CNT: crate::RORegister<u32>,
    #[doc = "Channel 1 Receive ERI Counter"]
    pub DMA_CH1_RX_ERI_CNT: crate::RORegister<u32>,
    _reserved85: [u8; 0x10],
    #[doc = "DMA Channel 2 Control"]
    pub DMA_CH2_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 2 Transmit Control"]
    pub DMA_CH2_TX_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 2 Receive Control"]
    pub DMA_CH2_RX_CONTROL: crate::RWRegister<u32>,
    _reserved86: [u8; 0x08],
    #[doc = "Channel 2 Tx Descriptor List Address"]
    pub DMA_CH2_TXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    _reserved87: [u8; 0x04],
    #[doc = "Channel 2 Rx Descriptor List Address"]
    pub DMA_CH2_RXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    #[doc = "Channel 2 Tx Descriptor Tail Pointer"]
    pub DMA_CH2_TXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    _reserved88: [u8; 0x04],
    #[doc = "Channel 2 Rx Descriptor Tail Pointer"]
    pub DMA_CH2_RXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    #[doc = "Channel 2 Tx Descriptor Ring Length"]
    pub DMA_CH2_TXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 2 Rx Descriptor Ring Length"]
    pub DMA_CH2_RXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 2 Interrupt Enable"]
    pub DMA_CH2_INTERRUPT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Channel 2 Receive Interrupt Watchdog Timer"]
    pub DMA_CH2_RX_INTERRUPT_WATCHDOG_TIMER: crate::RWRegister<u32>,
    #[doc = "Channel 2 Slot Function Control and Status"]
    pub DMA_CH2_SLOT_FUNCTION_CONTROL_STATUS: crate::RWRegister<u32>,
    _reserved89: [u8; 0x04],
    #[doc = "Channel 2 Current Application Transmit Descriptor"]
    pub DMA_CH2_CURRENT_APP_TXDESC: crate::RORegister<u32>,
    _reserved90: [u8; 0x04],
    #[doc = "Channel 2 Current Application Receive Descriptor"]
    pub DMA_CH2_CURRENT_APP_RXDESC: crate::RORegister<u32>,
    _reserved91: [u8; 0x04],
    #[doc = "Channel 2 Current Application Transmit Buffer Address"]
    pub DMA_CH2_CURRENT_APP_TXBUFFER: crate::RORegister<u32>,
    _reserved92: [u8; 0x04],
    #[doc = "Channel 2 Current Application Receive Buffer Address"]
    pub DMA_CH2_CURRENT_APP_RXBUFFER: crate::RORegister<u32>,
    #[doc = "DMA Channel 2 Status"]
    pub DMA_CH2_STATUS: crate::RWRegister<u32>,
    #[doc = "Channel 2 Missed Frame Counter"]
    pub DMA_CH2_MISS_FRAME_CNT: crate::RORegister<u32>,
    #[doc = "Channel 2 RXP Frames Accepted Counter"]
    pub DMA_CH2_RXP_ACCEPT_CNT: crate::RORegister<u32>,
    #[doc = "Channel 2 Receive ERI Counter"]
    pub DMA_CH2_RX_ERI_CNT: crate::RORegister<u32>,
    _reserved93: [u8; 0x10],
    #[doc = "DMA Channel 3 Control"]
    pub DMA_CH3_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 3 Transmit Control"]
    pub DMA_CH3_TX_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 3 Receive Control"]
    pub DMA_CH3_RX_CONTROL: crate::RWRegister<u32>,
    _reserved94: [u8; 0x08],
    #[doc = "Channel 3 Tx Descriptor List Address"]
    pub DMA_CH3_TXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    _reserved95: [u8; 0x04],
    #[doc = "Channel 3 Rx Descriptor List Address"]
    pub DMA_CH3_RXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    #[doc = "Channel 3 Tx Descriptor Tail Pointer"]
    pub DMA_CH3_TXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    _reserved96: [u8; 0x04],
    #[doc = "Channel 3 Rx Descriptor Tail Pointer"]
    pub DMA_CH3_RXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    #[doc = "Channel 3 Tx Descriptor Ring Length"]
    pub DMA_CH3_TXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 3 Rx Descriptor Ring Length"]
    pub DMA_CH3_RXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 3 Interrupt Enable"]
    pub DMA_CH3_INTERRUPT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Channel 3 Receive Interrupt Watchdog Time"]
    pub DMA_CH3_RX_INTERRUPT_WATCHDOG_TIMER: crate::RWRegister<u32>,
    #[doc = "Channel 3 Slot Function Control and Status"]
    pub DMA_CH3_SLOT_FUNCTION_CONTROL_STATUS: crate::RWRegister<u32>,
    _reserved97: [u8; 0x04],
    #[doc = "Channel 3 Current Application Transmit Descriptor"]
    pub DMA_CH3_CURRENT_APP_TXDESC: crate::RORegister<u32>,
    _reserved98: [u8; 0x04],
    #[doc = "Channel 3 Current Application Receive Descriptor"]
    pub DMA_CH3_CURRENT_APP_RXDESC: crate::RORegister<u32>,
    _reserved99: [u8; 0x04],
    #[doc = "Channel 3 Current Application Transmit Buffer Address"]
    pub DMA_CH3_CURRENT_APP_TXBUFFER: crate::RORegister<u32>,
    _reserved100: [u8; 0x04],
    #[doc = "Channel 3 Current Application Receive Buffer Address"]
    pub DMA_CH3_CURRENT_APP_RXBUFFER: crate::RORegister<u32>,
    #[doc = "DMA Channel 3 Status"]
    pub DMA_CH3_STATUS: crate::RWRegister<u32>,
    #[doc = "Channel 3 Missed Frame Counter"]
    pub DMA_CH3_MISS_FRAME_CNT: crate::RORegister<u32>,
    #[doc = "Channel 3 RXP Frames Accepted Counter"]
    pub DMA_CH3_RXP_ACCEPT_CNT: crate::RORegister<u32>,
    #[doc = "Channel 3 Receive ERI Counter"]
    pub DMA_CH3_RX_ERI_CNT: crate::RORegister<u32>,
    _reserved101: [u8; 0x10],
    #[doc = "DMA Channel 4 Control"]
    pub DMA_CH4_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 4 Transmit Control"]
    pub DMA_CH4_TX_CONTROL: crate::RWRegister<u32>,
    #[doc = "DMA Channel 4 Receive Control"]
    pub DMA_CH4_RX_CONTROL: crate::RWRegister<u32>,
    _reserved102: [u8; 0x08],
    #[doc = "Channel 4 Tx Descriptor List Address"]
    pub DMA_CH4_TXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    _reserved103: [u8; 0x04],
    #[doc = "Channel 4 Rx Descriptor List Address"]
    pub DMA_CH4_RXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
    #[doc = "Channel 4 Tx Descriptor Tail Pointer"]
    pub DMA_CH4_TXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    _reserved104: [u8; 0x04],
    #[doc = "Channel 4 Rx Descriptor Tail Pointer"]
    pub DMA_CH4_RXDESC_TAIL_POINTER: crate::RWRegister<u32>,
    #[doc = "Channel 4 Tx Descriptor Ring Length"]
    pub DMA_CH4_TXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 4 Rx Descriptor Ring Length"]
    pub DMA_CH4_RXDESC_RING_LENGTH: crate::RWRegister<u32>,
    #[doc = "Channel 4 Interrupt Enable"]
    pub DMA_CH4_INTERRUPT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Channel 4 Receive Interrupt Watchdog Timer"]
    pub DMA_CH4_RX_INTERRUPT_WATCHDOG_TIMER: crate::RWRegister<u32>,
    #[doc = "Channel 4 Slot Function Control and Status"]
    pub DMA_CH4_SLOT_FUNCTION_CONTROL_STATUS: crate::RWRegister<u32>,
    _reserved105: [u8; 0x04],
    #[doc = "Channel 4 Current Application Transmit Descriptor"]
    pub DMA_CH4_CURRENT_APP_TXDESC: crate::RORegister<u32>,
    _reserved106: [u8; 0x04],
    #[doc = "Channel 4 Current Application Receive Descriptor"]
    pub DMA_CH4_CURRENT_APP_RXDESC: crate::RORegister<u32>,
    _reserved107: [u8; 0x04],
    #[doc = "Channel 4 Current Application Transmit Buffer Address"]
    pub DMA_CH4_CURRENT_APP_TXBUFFER: crate::RORegister<u32>,
    _reserved108: [u8; 0x04],
    #[doc = "Channel 4 Current Application Receive Buffer Address"]
    pub DMA_CH4_CURRENT_APP_RXBUFFER: crate::RORegister<u32>,
    #[doc = "DMA Channel 4 Status"]
    pub DMA_CH4_STATUS: crate::RWRegister<u32>,
    #[doc = "Channel 4 Missed Frame Counter"]
    pub DMA_CH4_MISS_FRAME_CNT: crate::RORegister<u32>,
    #[doc = "Channel 4 RXP Frames Accepted Counter"]
    pub DMA_CH4_RXP_ACCEPT_CNT: crate::RORegister<u32>,
    #[doc = "Channel 4 Receive ERI Counter"]
    pub DMA_CH4_RX_ERI_CNT: crate::RORegister<u32>,
}
#[doc = "MAC Configuration Register"]
pub mod MAC_CONFIGURATION {
    #[doc = "Receiver Enable"]
    pub mod RE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receiver is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmitter Enable"]
    pub mod TE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmitter is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Preamble Length for Transmit packets"]
    pub mod PRELEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "7 bytes of preamble"]
            pub const BYTES_7: u32 = 0;
            #[doc = "5 bytes of preamble"]
            pub const BYTES_5: u32 = 0x01;
            #[doc = "3 bytes of preamble"]
            pub const BYTES_3: u32 = 0x02;
        }
    }
    #[doc = "Deferral Check"]
    pub mod DC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deferral check function is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Deferral check function is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Back-Off Limit"]
    pub mod BL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "k = min(n,10)"]
            pub const MIN_N_10: u32 = 0;
            #[doc = "k = min(n,8)"]
            pub const MIN_N_8: u32 = 0x01;
            #[doc = "k = min(n,4)"]
            pub const MIN_N_4: u32 = 0x02;
            #[doc = "k = min(n,1)"]
            pub const MIN_N_1: u32 = 0x03;
        }
    }
    #[doc = "Disable Retry"]
    pub mod DR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Retry"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable Retry"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Carrier Sense During Transmission"]
    pub mod DCRS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Carrier Sense During Transmission"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable Carrier Sense During Transmission"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Receive Own"]
    pub mod DO {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Receive Own"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable Receive Own"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
    pub mod ECRSFD {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECRSFD is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "ECRSFD is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Loopback Mode"]
    pub mod LM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Loopback is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Loopback is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Duplex Mode"]
    pub mod DM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Half-duplex mode"]
            pub const HDUPLX: u32 = 0;
            #[doc = "Full-duplex mode"]
            pub const FDUPLX: u32 = 0x01;
        }
    }
    #[doc = "Speed"]
    pub mod FES {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "10 Mbps when PS bit is 1 and 1 Gbps when PS bit is 0"]
            pub const MBPS_10_1000M: u32 = 0;
            #[doc = "100 Mbps when PS bit is 1 and 2.5 Gbps when PS bit is 0"]
            pub const MBPS_100_2500M: u32 = 0x01;
        }
    }
    #[doc = "Port Select"]
    pub mod PS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "For 1000 or 2500 Mbps operations"]
            pub const BF_1000_2500M: u32 = 0;
            #[doc = "For 10 or 100 Mbps operations"]
            pub const BF_10_100M: u32 = 0x01;
        }
    }
    #[doc = "Jumbo Packet Enable When this bit is set, the MAC allows jumbo packets of 9,018 bytes (9,022 bytes for VLAN tagged packets) without reporting a giant packet error in the Rx packet status."]
    pub mod JE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Jumbo packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Jumbo packet is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Jabber Disable"]
    pub mod JD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Jabber is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Jabber is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the GMII half-duplex mode."]
    pub mod BE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet Burst is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Packet Burst is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Watchdog Disable"]
    pub mod WD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Watchdog is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
    pub mod ACS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic Pad or CRC Stripping is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic Pad or CRC Stripping is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
    pub mod CST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CRC stripping for Type packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "CRC stripping for Type packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "IEEE 802."]
    pub mod S2KP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Support upto 2K packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Support upto 2K packet is Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Giant Packet Size Limit Control Enable"]
    pub mod GPSLCE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Giant Packet Size Limit Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Giant Packet Size Limit Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
    pub mod IPG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "96 bit times IPG"]
            pub const IPG96: u32 = 0;
            #[doc = "88 bit times IPG"]
            pub const IPG88: u32 = 0x01;
            #[doc = "80 bit times IPG"]
            pub const IPG80: u32 = 0x02;
            #[doc = "72 bit times IPG"]
            pub const IPG72: u32 = 0x03;
            #[doc = "64 bit times IPG"]
            pub const IPG64: u32 = 0x04;
            #[doc = "56 bit times IPG"]
            pub const IPG56: u32 = 0x05;
            #[doc = "48 bit times IPG"]
            pub const IPG48: u32 = 0x06;
            #[doc = "40 bit times IPG"]
            pub const IPG40: u32 = 0x07;
        }
    }
    #[doc = "Checksum Offload"]
    pub mod IPC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP header/payload checksum checking is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "IP header/payload checksum checking is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Source Address Insertion or Replacement Control"]
    pub mod SARC {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "mti_sa_ctrl_i and ati_sa_ctrl_i input signals control the SA field generation"]
            pub const SA_CTRL_IN: u32 = 0;
            #[doc = "Contents of MAC Addr-0 inserted in SA field"]
            pub const MAC0_INS_SA: u32 = 0x02;
            #[doc = "Contents of MAC Addr-0 replaces SA field"]
            pub const MAC0_REP_SA: u32 = 0x03;
            #[doc = "Contents of MAC Addr-1 inserted in SA field"]
            pub const MAC1_INS_SA: u32 = 0x06;
            #[doc = "Contents of MAC Addr-1 replaces SA field"]
            pub const MAC1_REP_SA: u32 = 0x07;
        }
    }
}
#[doc = "MAC Extended Configuration Register"]
pub mod MAC_EXT_CONFIGURATION {
    #[doc = "Giant Packet Size Limit"]
    pub mod GPSL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable CRC Checking for Received Packets"]
    pub mod DCRCC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CRC Checking is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "CRC Checking is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Slow Protocol Detection Enable"]
    pub mod SPEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Protocol Detection is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slow Protocol Detection is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Unicast Slow Protocol Packet Detect"]
    pub mod USP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unicast Slow Protocol Packet Detection is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unicast Slow Protocol Packet Detection is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Packet Duplication Control"]
    pub mod PDC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet Duplication Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Packet Duplication Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Extended Inter-Packet Gap Enable"]
    pub mod EIPGEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Extended Inter-Packet Gap is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Extended Inter-Packet Gap is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Extended Inter-Packet Gap"]
    pub mod EIPG {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Packet Filter"]
pub mod MAC_PACKET_FILTER {
    #[doc = "Promiscuous Mode"]
    pub mod PR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Promiscuous Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Promiscuous Mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Hash Unicast"]
    pub mod HUC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hash Unicast is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Hash Unicast is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Hash Multicast"]
    pub mod HMC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hash Multicast is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Hash Multicast is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DA Inverse Filtering"]
    pub mod DAIF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DA Inverse Filtering is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DA Inverse Filtering is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Pass All Multicast"]
    pub mod PM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pass All Multicast is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Pass All Multicast is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Broadcast Packets"]
    pub mod DBF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Broadcast Packets"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable Broadcast Packets"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Pass Control Packets These bits control the forwarding of all control packets (including unicast and multicast Pause packets)."]
    pub mod PCF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC filters all control packets from reaching the application"]
            pub const FLTR_ALL: u32 = 0;
            #[doc = "MAC forwards all control packets except Pause packets to the application even if they fail the Address filter"]
            pub const FW_XCPT_PAU: u32 = 0x01;
            #[doc = "MAC forwards all control packets to the application even if they fail the Address filter"]
            pub const FW_ALL: u32 = 0x02;
            #[doc = "MAC forwards the control packets that pass the Address filter"]
            pub const FW_PASS: u32 = 0x03;
        }
    }
    #[doc = "SA Inverse Filtering"]
    pub mod SAIF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SA Inverse Filtering is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "SA Inverse Filtering is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Source Address Filter Enable"]
    pub mod SAF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SA Filtering is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "SA Filtering is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Hash or Perfect Filter"]
    pub mod HPF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hash or Perfect Filter is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Hash or Perfect Filter is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "VLAN Tag Filter Enable"]
    pub mod VTFE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Filter is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Filter is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 and Layer 4 Filter Enable"]
    pub mod IPFE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 and Layer 4 Filters are disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 and Layer 4 Filters are enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Drop Non-TCP/UDP over IP Packets"]
    pub mod DNTU {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Non-TCP/UDP over IP Packets"]
            pub const FWD: u32 = 0;
            #[doc = "Drop Non-TCP/UDP over IP Packets"]
            pub const DROP: u32 = 0x01;
        }
    }
    #[doc = "Receive All"]
    pub mod RA {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive All is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive All is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Watchdog Timeout"]
pub mod MAC_WATCHDOG_TIMEOUT {
    #[doc = "Watchdog Timeout"]
    pub mod WTO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2 KB"]
            pub const BF_2KBYTES: u32 = 0;
            #[doc = "3 KB"]
            pub const BF_3KBYTES: u32 = 0x01;
            #[doc = "4 KB"]
            pub const BF_4KBYTES: u32 = 0x02;
            #[doc = "5 KB"]
            pub const BF_5KBYTES: u32 = 0x03;
            #[doc = "6 KB"]
            pub const BF_6KBYTES: u32 = 0x04;
            #[doc = "7 KB"]
            pub const BF_7KBYTES: u32 = 0x05;
            #[doc = "8 KB"]
            pub const BF_8KBYTES: u32 = 0x06;
            #[doc = "9 KB"]
            pub const BF_9KBYTES: u32 = 0x07;
            #[doc = "10 KB"]
            pub const BF_10KBYTES: u32 = 0x08;
            #[doc = "11 KB"]
            pub const BF_11KBYTES: u32 = 0x09;
            #[doc = "12 KB"]
            pub const BF_12KBYTES: u32 = 0x0a;
            #[doc = "13 KB"]
            pub const BF_13KBYTES: u32 = 0x0b;
            #[doc = "14 KB"]
            pub const BF_14KBYTES: u32 = 0x0c;
            #[doc = "15 KB"]
            pub const BF_15KBYTES: u32 = 0x0d;
            #[doc = "16383 Bytes"]
            pub const BF_16383BYTES: u32 = 0x0e;
        }
    }
    #[doc = "Programmable Watchdog Enable"]
    pub mod PWE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Programmable Watchdog is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Programmable Watchdog is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Hash Table Register 0"]
pub mod MAC_HASH_TABLE_REG0 {
    #[doc = "MAC Hash Table First 32 Bits This field contains the first 32 Bits \\[31:0\\] of the Hash table."]
    pub mod HT31T0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Hash Table Register 1"]
pub mod MAC_HASH_TABLE_REG1 {
    #[doc = "MAC Hash Table Second 32 Bits This field contains the second 32 Bits \\[63:32\\] of the Hash table."]
    pub mod HT63T32 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC VLAN Tag Control"]
pub mod MAC_VLAN_TAG_CTRL {
    #[doc = "Operation Busy"]
    pub mod OB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Operation Busy is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Operation Busy is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Command Type"]
    pub mod CT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write operation"]
            pub const WRITE: u32 = 0;
            #[doc = "Read operation"]
            pub const READ: u32 = 0x01;
        }
    }
    #[doc = "Offset"]
    pub mod OFS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Tag Inverse Match Enable"]
    pub mod VTIM {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable S-VLAN When this bit is set, the MAC transmitter and receiver consider the S-VLAN packets (Type = 0x88A8) as valid VLAN tagged packets."]
    pub mod ESVL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "S-VLAN is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "S-VLAN is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable VLAN Tag Stripping on Receive This field indicates the stripping operation on the outer VLAN Tag in received packet."]
    pub mod EVLS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not strip"]
            pub const DONOT: u32 = 0;
            #[doc = "Strip if VLAN filter passes"]
            pub const IFPASS: u32 = 0x01;
            #[doc = "Strip if VLAN filter fails"]
            pub const IFFAIL: u32 = 0x02;
            #[doc = "Always strip"]
            pub const ALWAYS: u32 = 0x03;
        }
    }
    #[doc = "Enable VLAN Tag in Rx status"]
    pub mod EVLRXS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag in Rx status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag in Rx status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "VLAN Tag Hash Table Match Enable"]
    pub mod VTHM {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Hash Table Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Hash Table Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Double VLAN Processing"]
    pub mod EDVLP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double VLAN Processing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Double VLAN Processing is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ERIVLT"]
    pub mod ERIVLT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inner VLAN tag is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Inner VLAN tag is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Inner VLAN Tag Stripping on Receive This field indicates the stripping operation on inner VLAN Tag in received packet."]
    pub mod EIVLS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not strip"]
            pub const DONOT: u32 = 0;
            #[doc = "Strip if VLAN filter passes"]
            pub const IFPASS: u32 = 0x01;
            #[doc = "Strip if VLAN filter fails"]
            pub const IFFAIL: u32 = 0x02;
            #[doc = "Always strip"]
            pub const ALWAYS: u32 = 0x03;
        }
    }
    #[doc = "Enable Inner VLAN Tag in Rx Status"]
    pub mod EIVLRXS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inner VLAN Tag in Rx status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Inner VLAN Tag in Rx status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC VLAN Tag Data"]
pub mod MAC_VLAN_TAG_DATA {
    #[doc = "VLAN Tag ID"]
    pub mod VID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Tag Enable"]
    pub mod VEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "12bits or 16bits VLAN comparison"]
    pub mod ETV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16 bit VLAN comparison"]
            pub const BF_16BIT: u32 = 0;
            #[doc = "12 bit VLAN comparison"]
            pub const BF_12BIT: u32 = 0x01;
        }
    }
    #[doc = "Disable VLAN Type Comparison"]
    pub mod DOVLTC {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN type comparison is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "VLAN type comparison is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable S-VLAN Match for received Frames"]
    pub mod ERSVLM {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive S-VLAN Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive S-VLAN Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Inner VLAN Tag Comparison"]
    pub mod ERIVLT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inner VLAN tag comparison is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Inner VLAN tag comparison is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number Enable"]
    pub mod DMACHEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Number is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Number is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number"]
    pub mod DMACHN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC VLAN Hash Table"]
pub mod MAC_VLAN_HASH_TABLE {
    #[doc = "VLAN Hash Table This field contains the 16-bit VLAN Hash Table."]
    pub mod VLHT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VLAN Tag Inclusion or Replacement"]
pub mod MAC_VLAN_INCL {
    #[doc = "VLAN Tag for Transmit Packets"]
    pub mod VLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Tag Control in Transmit Packets - 2'b00: No VLAN tag deletion, insertion, or replacement - 2'b01: VLAN tag deletion The MAC removes the VLAN type (bytes 13 and 14) and VLAN tag (bytes 15 and 16) of all transmitted packets with VLAN tags."]
    pub mod VLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No VLAN tag deletion, insertion, or replacement"]
            pub const NONE: u32 = 0;
            #[doc = "VLAN tag deletion"]
            pub const DELETE: u32 = 0x01;
            #[doc = "VLAN tag insertion"]
            pub const INSERT: u32 = 0x02;
            #[doc = "VLAN tag replacement"]
            pub const REPLACE: u32 = 0x03;
        }
    }
    #[doc = "VLAN Priority Control"]
    pub mod VLP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Priority Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Priority Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "C-VLAN or S-VLAN"]
    pub mod CSVL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "C-VLAN type (0x8100) is inserted or replaced"]
            pub const C_VLAN: u32 = 0;
            #[doc = "S-VLAN type (0x88A8) is inserted or replaced"]
            pub const S_VLAN: u32 = 0x01;
        }
    }
    #[doc = "VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from: - The Tx descriptor"]
    pub mod VLTI {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Input is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Input is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Channel based tag insertion"]
    pub mod CBTI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel based tag insertion is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Channel based tag insertion is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read write control"]
    pub mod RDWR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read operation of indirect access"]
            pub const READ: u32 = 0;
            #[doc = "Write operation of indirect access"]
            pub const WRITE: u32 = 0x01;
        }
    }
    #[doc = "Busy"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Busy status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Busy status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Inner VLAN Tag Inclusion or Replacement"]
pub mod MAC_INNER_VLAN_INCL {
    #[doc = "VLAN Tag for Transmit Packets"]
    pub mod VLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Tag Control in Transmit Packets"]
    pub mod VLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No VLAN tag deletion, insertion, or replacement"]
            pub const NONE: u32 = 0;
            #[doc = "VLAN tag deletion"]
            pub const DELETE: u32 = 0x01;
            #[doc = "VLAN tag insertion"]
            pub const INSERT: u32 = 0x02;
            #[doc = "VLAN tag replacement"]
            pub const REPLACE: u32 = 0x03;
        }
    }
    #[doc = "VLAN Priority Control"]
    pub mod VLP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Priority Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Priority Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "C-VLAN or S-VLAN"]
    pub mod CSVL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "C-VLAN type (0x8100) is inserted"]
            pub const C_VLAN: u32 = 0;
            #[doc = "S-VLAN type (0x88A8) is inserted"]
            pub const S_VLAN: u32 = 0x01;
        }
    }
    #[doc = "VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from: - The Tx descriptor"]
    pub mod VLTI {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Input is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Input is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Q0 Tx Flow Control"]
pub mod MAC_Q0_TX_FLOW_CTRL {
    #[doc = "Flow Control Busy or Backpressure Activate"]
    pub mod FCB_BPA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flow Control Busy or Backpressure Activate is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flow Control Busy or Backpressure Activate is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Flow Control Enable"]
    pub mod TFE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Low Threshold"]
    pub mod PLT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)"]
            pub const PT4: u32 = 0;
            #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)"]
            pub const PT28: u32 = 0x01;
            #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)"]
            pub const PT36: u32 = 0x02;
            #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)"]
            pub const PT144: u32 = 0x03;
            #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)"]
            pub const PT256: u32 = 0x04;
            #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)"]
            pub const PT512: u32 = 0x05;
        }
    }
    #[doc = "Disable Zero-Quanta Pause"]
    pub mod DZPQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero-Quanta Pause packet generation is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Zero-Quanta Pause packet generation is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Time"]
    pub mod PT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Q1 Tx Flow Control"]
pub mod MAC_Q1_TX_FLOW_CTRL {
    #[doc = "Flow Control Busy"]
    pub mod FCB_BPA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flow Control Busy or Backpressure Activate is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flow Control Busy or Backpressure Activate is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Flow Control Enable"]
    pub mod TFE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Low Threshold"]
    pub mod PLT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)"]
            pub const PT4: u32 = 0;
            #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)"]
            pub const PT28: u32 = 0x01;
            #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)"]
            pub const PT36: u32 = 0x02;
            #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)"]
            pub const PT144: u32 = 0x03;
            #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)"]
            pub const PT256: u32 = 0x04;
            #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)"]
            pub const PT512: u32 = 0x05;
        }
    }
    #[doc = "Disable Zero-Quanta Pause"]
    pub mod DZPQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero-Quanta Pause packet generation is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Zero-Quanta Pause packet generation is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Time"]
    pub mod PT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Q2 Tx Flow Control"]
pub mod MAC_Q2_TX_FLOW_CTRL {
    #[doc = "Flow Control Busy"]
    pub mod FCB_BPA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flow Control Busy or Backpressure Activate is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flow Control Busy or Backpressure Activate is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Flow Control Enable"]
    pub mod TFE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Low Threshold"]
    pub mod PLT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)"]
            pub const PT4: u32 = 0;
            #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)"]
            pub const PT28: u32 = 0x01;
            #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)"]
            pub const PT36: u32 = 0x02;
            #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)"]
            pub const PT144: u32 = 0x03;
            #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)"]
            pub const PT256: u32 = 0x04;
            #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)"]
            pub const PT512: u32 = 0x05;
        }
    }
    #[doc = "Disable Zero-Quanta Pause"]
    pub mod DZPQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero-Quanta Pause packet generation is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Zero-Quanta Pause packet generation is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Time"]
    pub mod PT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Q3 Tx Flow Control"]
pub mod MAC_Q3_TX_FLOW_CTRL {
    #[doc = "Flow Control Busy"]
    pub mod FCB_BPA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flow Control Busy or Backpressure Activate is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flow Control Busy or Backpressure Activate is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Flow Control Enable"]
    pub mod TFE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Low Threshold"]
    pub mod PLT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)"]
            pub const PT4: u32 = 0;
            #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)"]
            pub const PT28: u32 = 0x01;
            #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)"]
            pub const PT36: u32 = 0x02;
            #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)"]
            pub const PT144: u32 = 0x03;
            #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)"]
            pub const PT256: u32 = 0x04;
            #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)"]
            pub const PT512: u32 = 0x05;
        }
    }
    #[doc = "Disable Zero-Quanta Pause"]
    pub mod DZPQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero-Quanta Pause packet generation is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Zero-Quanta Pause packet generation is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Time"]
    pub mod PT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Q4 Tx Flow Control"]
pub mod MAC_Q4_TX_FLOW_CTRL {
    #[doc = "Flow Control Busy"]
    pub mod FCB_BPA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flow Control Busy or Backpressure Activate is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flow Control Busy or Backpressure Activate is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Flow Control Enable"]
    pub mod TFE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Low Threshold"]
    pub mod PLT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)"]
            pub const PT4: u32 = 0;
            #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)"]
            pub const PT28: u32 = 0x01;
            #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)"]
            pub const PT36: u32 = 0x02;
            #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)"]
            pub const PT144: u32 = 0x03;
            #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)"]
            pub const PT256: u32 = 0x04;
            #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)"]
            pub const PT512: u32 = 0x05;
        }
    }
    #[doc = "Disable Zero-Quanta Pause"]
    pub mod DZPQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero-Quanta Pause packet generation is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Zero-Quanta Pause packet generation is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Pause Time"]
    pub mod PT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Rx Flow Control"]
pub mod MAC_RX_FLOW_CTRL {
    #[doc = "Receive Flow Control Enable"]
    pub mod RFE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Unicast Pause Packet Detect"]
    pub mod UP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unicast Pause Packet Detect disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unicast Pause Packet Detect enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Priority Based Flow Control Enable"]
    pub mod PFCE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Priority Based Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Priority Based Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Receive Queue Control 4"]
pub mod MAC_RXQ_CTRL4 {
    #[doc = "Unicast Address Filter Fail Packets Queuing Enable."]
    pub mod UFFQE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unicast Address Filter Fail Packets Queuing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unicast Address Filter Fail Packets Queuing is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Unicast Address Filter Fail Packets Queue."]
    pub mod UFFQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multicast Address Filter Fail Packets Queuing Enable."]
    pub mod MFFQE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multicast Address Filter Fail Packets Queuing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Multicast Address Filter Fail Packets Queuing is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Multicast Address Filter Fail Packets Queue."]
    pub mod MFFQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Tag Filter Fail Packets Queuing Enable"]
    pub mod VFFQE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN tag Filter Fail Packets Queuing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN tag Filter Fail Packets Queuing is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "VLAN Tag Filter Fail Packets Queue"]
    pub mod VFFQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Queue Priority Mapping 0"]
pub mod MAC_TXQ_PRTY_MAP0 {
    #[doc = "Priorities Selected in Transmit Queue 0"]
    pub mod PSTQ0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priorities Selected in Transmit Queue 1 This bit is similar to the PSTQ0 bit."]
    pub mod PSTQ1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priorities Selected in Transmit Queue 2 This bit is similar to the PSTQ0 bit."]
    pub mod PSTQ2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priorities Selected in Transmit Queue 3 This bit is similar to the PSTQ0 bit."]
    pub mod PSTQ3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Queue Priority Mapping 1"]
pub mod MAC_TXQ_PRTY_MAP1 {
    #[doc = "Priorities Selected in Transmit Queue 4"]
    pub mod PSTQ4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Queue Control 0"]
pub mod MAC_RXQ_CTRL0 {
    #[doc = "Receive Queue 0 Enable This field indicates whether Rx Queue 0 is enabled for AV or DCB."]
    pub mod RXQ0EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue enabled for AV"]
            pub const EN_AV: u32 = 0x01;
            #[doc = "Queue enabled for DCB/Generic"]
            pub const EN_DCB_GEN: u32 = 0x02;
        }
    }
    #[doc = "Receive Queue 1 Enable This field is similar to the RXQ0EN field."]
    pub mod RXQ1EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue enabled for AV"]
            pub const EN_AV: u32 = 0x01;
            #[doc = "Queue enabled for DCB/Generic"]
            pub const EN_DCB_GEN: u32 = 0x02;
        }
    }
    #[doc = "Receive Queue 2 Enable This field is similar to the RXQ0EN field."]
    pub mod RXQ2EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue enabled for AV"]
            pub const EN_AV: u32 = 0x01;
            #[doc = "Queue enabled for DCB/Generic"]
            pub const EN_DCB_GEN: u32 = 0x02;
        }
    }
    #[doc = "Receive Queue 3 Enable This field is similar to the RXQ0EN field."]
    pub mod RXQ3EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue enabled for AV"]
            pub const EN_AV: u32 = 0x01;
            #[doc = "Queue enabled for DCB/Generic"]
            pub const EN_DCB_GEN: u32 = 0x02;
        }
    }
    #[doc = "Receive Queue 4 Enable This field is similar to the RXQ0EN field."]
    pub mod RXQ4EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue enabled for AV"]
            pub const EN_AV: u32 = 0x01;
            #[doc = "Queue enabled for DCB/Generic"]
            pub const EN_DCB_GEN: u32 = 0x02;
        }
    }
}
#[doc = "Receive Queue Control 1"]
pub mod MAC_RXQ_CTRL1 {
    #[doc = "AV Untagged Control Packets Queue"]
    pub mod AVCPQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 0x01;
            #[doc = "Receive Queue 2"]
            pub const QUEUE2: u32 = 0x02;
            #[doc = "Receive Queue 3"]
            pub const QUEUE3: u32 = 0x03;
            #[doc = "Receive Queue 4"]
            pub const QUEUE4: u32 = 0x04;
        }
    }
    #[doc = "PTP Packets Queue"]
    pub mod PTPQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 0x01;
            #[doc = "Receive Queue 2"]
            pub const QUEUE2: u32 = 0x02;
            #[doc = "Receive Queue 3"]
            pub const QUEUE3: u32 = 0x03;
            #[doc = "Receive Queue 4"]
            pub const QUEUE4: u32 = 0x04;
        }
    }
    #[doc = "DCB Control Packets Queue"]
    pub mod DCBCPQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 0x01;
            #[doc = "Receive Queue 2"]
            pub const QUEUE2: u32 = 0x02;
            #[doc = "Receive Queue 3"]
            pub const QUEUE3: u32 = 0x03;
            #[doc = "Receive Queue 4"]
            pub const QUEUE4: u32 = 0x04;
        }
    }
    #[doc = "Untagged Packet Queue"]
    pub mod UPQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 0x01;
            #[doc = "Receive Queue 2"]
            pub const QUEUE2: u32 = 0x02;
            #[doc = "Receive Queue 3"]
            pub const QUEUE3: u32 = 0x03;
            #[doc = "Receive Queue 4"]
            pub const QUEUE4: u32 = 0x04;
        }
    }
    #[doc = "Multicast and Broadcast Queue"]
    pub mod MCBCQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 0x01;
            #[doc = "Receive Queue 2"]
            pub const QUEUE2: u32 = 0x02;
            #[doc = "Receive Queue 3"]
            pub const QUEUE3: u32 = 0x03;
            #[doc = "Receive Queue 4"]
            pub const QUEUE4: u32 = 0x04;
        }
    }
    #[doc = "Multicast and Broadcast Queue Enable This bit specifies that Multicast or Broadcast packets routing to the Rx Queue is enabled and the Multicast or Broadcast packets must be routed to Rx Queue specified in MCBCQ field."]
    pub mod MCBCQEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multicast and Broadcast Queue is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Multicast and Broadcast Queue is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Tagged AV Control Packets Queuing Enable."]
    pub mod TACPQE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tagged AV Control Packets Queuing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Tagged AV Control Packets Queuing is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Tagged PTP over Ethernet Packets Queuing Control."]
    pub mod TPQC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Preemption Residue Queue"]
    pub mod FPRQ {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Queue Control 2"]
pub mod MAC_RXQ_CTRL2 {
    #[doc = "Priorities Selected in the Receive Queue 0"]
    pub mod PSRQ0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priorities Selected in the Receive Queue 1"]
    pub mod PSRQ1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priorities Selected in the Receive Queue 2"]
    pub mod PSRQ2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priorities Selected in the Receive Queue 3"]
    pub mod PSRQ3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Queue Control 3"]
pub mod MAC_RXQ_CTRL3 {
    #[doc = "Priorities Selected in the Receive Queue 4"]
    pub mod PSRQ4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status"]
pub mod MAC_INTERRUPT_STATUS {
    #[doc = "RGMII or SMII Interrupt Status"]
    pub mod RGSMIIIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGMII or SMII Interrupt Status is not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "RGMII or SMII Interrupt Status is active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "PHY Interrupt"]
    pub mod PHYIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PHY Interrupt not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PHY Interrupt detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "PMT Interrupt Status"]
    pub mod PMTIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMT Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PMT Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "LPI Interrupt Status"]
    pub mod LPIIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "LPI Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Interrupt Status"]
    pub mod MMCIS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Interrupt Status"]
    pub mod MMCRXIS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Interrupt Status"]
    pub mod MMCTXIS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Checksum Offload Interrupt Status"]
    pub mod MMCRXIPIS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Checksum Offload Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Checksum Offload Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Interrupt Status"]
    pub mod TSIS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Status Interrupt"]
    pub mod TXSTSIS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Status Interrupt"]
    pub mod RXSTSIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Frame Preemption Interrupt Status"]
    pub mod FPEIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Preemption Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Frame Preemption Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Interrupt Status"]
    pub mod MDIOIS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MDIO Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MDIO Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC FPE Transmit Interrupt Status"]
    pub mod MFTIS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC FPE Transmit Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC FPE Transmit Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC FPE Receive Interrupt Status"]
    pub mod MFRIS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC FPE Receive Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC FPE Receive Interrupt status active"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod MAC_INTERRUPT_ENABLE {
    #[doc = "RGMII or SMII Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of RGSMIIIS bit in MAC_INTERRUPT_STATUS register."]
    pub mod RGSMIIIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RGMII or SMII Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "RGMII or SMII Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\\[PHYIS\\]."]
    pub mod PHYIE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PHY Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PHY Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PMT Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\\[PMTIS\\]."]
    pub mod PMTIE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMT Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PMT Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\\[LPIIS\\]."]
    pub mod LPIIE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\\[TSIS\\]."]
    pub mod TSIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\\[TXSTSIS\\]."]
    pub mod TXSTSIE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Status Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Status Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\\[RXSTSIS\\]."]
    pub mod RXSTSIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Status Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Status Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Frame Preemption Interrupt Enable When this bit is set, it enables the assertion of the interrupt when FPEIS field is set in the MAC_INTERRUPT_STATUS."]
    pub mod FPEIE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Preemption Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Frame Preemption Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MDIO Interrupt Enable When this bit is set, it enables the assertion of the interrupt when MDIOIS field is set in the MAC_INTERRUPT_STATUS register."]
    pub mod MDIOIE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MDIO Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MDIO Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Receive Transmit Status"]
pub mod MAC_RX_TX_STATUS {
    #[doc = "Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired which happens when the packet size exceeds 2,048 bytes (10,240 bytes when the Jumbo packet is enabled) and JD bit is reset in the MAC_CONFIGURATION register."]
    pub mod TJT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Transmit Jabber Timeout"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Jabber Timeout occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "No Carrier When the DTXSTS bit is set in the MAC_OPERATION_MODE register, this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission."]
    pub mod NCARR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Carrier is present"]
            pub const INACTIVE: u32 = 0;
            #[doc = "No carrier"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Loss of Carrier When the DTXSTS bit is set in the MAC_OPERATION_MODE register, this bit indicates that the loss of carrier occurred during packet transmission, that is, the phy_crs_i signal was inactive for one or more transmission clock periods during packet transmission."]
    pub mod LCARR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Carrier is present"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Loss of carrier"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Excessive Deferral When the DTXSTS bit is set in the MAC_OPERATION_MODE register and the DC bit is set in the MAC_CONFIGURATION register, this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 in 1000/2500 Mbps mode or when Jumbo packet is enabled)."]
    pub mod EXDEF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Excessive deferral"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Excessive deferral"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Late Collision When the DTXSTS bit is set in the MAC_OPERATION_MODE register, this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode; 512 bytes including Preamble and Carrier Extension in GMII mode)."]
    pub mod LCOL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No collision"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Late collision is sensed"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Excessive Collisions When the DTXSTS bit is set in the MAC_OPERATION_MODE register, this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet."]
    pub mod EXCOL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No collision"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Excessive collision is sensed"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10, 240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the MAC_CONFIGURATION register."]
    pub mod RWT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No receive watchdog timeout"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive watchdog timed out"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "PMT Control and Status"]
pub mod MAC_PMT_CONTROL_STATUS {
    #[doc = "Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wake-up packet."]
    pub mod PWRDWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power down is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Power down is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet."]
    pub mod MGKPKTEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Magic Packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Magic Packet is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Remote Wake-Up Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wake-up packet."]
    pub mod RWKPKTEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote wake-up packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Remote wake-up packet is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet."]
    pub mod MGKPRCVD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Magic packet is received"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Magic packet is received"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Remote Wake-Up Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a remote wake-up packet."]
    pub mod RWKPRCVD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote wake-up packet is received"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Remote wake-up packet is received"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    pub mod GLBLUCAST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Global unicast is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Global unicast is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected Wake-up frame."]
    pub mod RWKPFE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote Wake-up Packet Forwarding is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Remote Wake-up Packet Forwarding is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Remote Wake-up FIFO Pointer This field gives the current value (0 to 7, 15, or 31 when 4, 8, or 16 Remote Wake-up Packet Filters are selected) of the Remote Wake-up Packet Filter register pointer."]
    pub mod RWKPTR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    pub mod RWKFILTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote Wake-Up Packet Filter Register Pointer is not Reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Remote Wake-Up Packet Filter Register Pointer is Reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Remote Wakeup Filter"]
pub mod MAC_RWK_PACKET_FILTER {
    #[doc = "RWK Packet Filter This field contains the various controls of RWK Packet filter."]
    pub mod WKUPFRMFTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LPI Control and Status"]
pub mod MAC_LPI_CONTROL_STATUS {
    #[doc = "Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
    pub mod TLPIEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit LPI entry not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit LPI entry detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit LPI Exit When this bit is set, it indicates that the MAC transmitter exited the LPI state after the application cleared the LPIEN bit and the LPI TW Timer has expired."]
    pub mod TLPIEX {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit LPI exit not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit LPI exit detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive LPI Entry When this bit is set, it indicates that the MAC Receiver has received an LPI pattern and entered the LPI state."]
    pub mod RLPIEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive LPI entry not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive LPI entry detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive LPI Exit When this bit is set, it indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception."]
    pub mod RLPIEX {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive LPI exit not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive LPI exit detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit LPI State When this bit is set, it indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface."]
    pub mod TLPIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit LPI state not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit LPI state detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive LPI State When this bit is set, it indicates that the MAC is receiving the LPI pattern on the GMII or MII interface."]
    pub mod RLPIST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive LPI state not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive LPI state detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
    pub mod LPIEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI state is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI state is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PHY Link Status This bit indicates the link status of the PHY."]
    pub mod PLS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "link is down"]
            pub const DISABLE: u32 = 0;
            #[doc = "link is okay (UP)"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII Receive paths to be used for activating the LPI LS TIMER."]
    pub mod PLSEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PHY Link Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PHY Link Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
    pub mod LPITXA {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Tx Automate is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI Tx Automate is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
    pub mod LPIATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Timer is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI Timer is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "LPI Tx Clock Stop Enable When this bit is set, the MAC asserts sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
    pub mod LPITCSE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Tx Clock Stop is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI Tx Clock Stop is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "LPI Timers Control"]
pub mod MAC_LPI_TIMERS_CONTROL {
    #[doc = "LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission."]
    pub mod TWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY."]
    pub mod LST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx LPI Entry Timer Control"]
pub mod MAC_LPI_ENTRY_TIMER {
    #[doc = "LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames."]
    pub mod LPIET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "One-microsecond Reference Timer"]
pub mod MAC_ONEUS_TIC_COUNTER {
    #[doc = "1US TIC Counter The application must program this counter so that the number of clock cycles of CSR clock is 1us."]
    pub mod TIC_1US_CNTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHY Interface Control and Status"]
pub mod MAC_PHYIF_CONTROL_STATUS {
    #[doc = "Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port."]
    pub mod TC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Transmit Configuration in RGMII, SGMII, or SMII"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable Transmit Configuration in RGMII, SGMII, or SMII"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Link Up or Down This bit indicates whether the link is up or down during transmission of configuration in the RGMII, SGMII, or SMII interface."]
    pub mod LUD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Link down"]
            pub const LINKDOWN: u32 = 0;
            #[doc = "Link up"]
            pub const LINKUP: u32 = 0x01;
        }
    }
    #[doc = "Link Mode This bit indicates the current mode of operation of the link."]
    pub mod LNKMOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Half-duplex mode"]
            pub const HDUPLX: u32 = 0;
            #[doc = "Full-duplex mode"]
            pub const FDUPLX: u32 = 0x01;
        }
    }
    #[doc = "Link Speed This bit indicates the current speed of the link."]
    pub mod LNKSPEED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.5 MHz"]
            pub const BF_2500K: u32 = 0;
            #[doc = "25 MHz"]
            pub const BF_25M: u32 = 0x01;
            #[doc = "125 MHz"]
            pub const BF_125M: u32 = 0x02;
        }
    }
    #[doc = "Link Status This bit indicates whether the link is up (1'b1) or down (1'b0)."]
    pub mod LNKSTS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Link down"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Link up"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Version"]
pub mod MAC_VERSION {
    #[doc = "Synopsys-defined Version"]
    pub mod SNPSVER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User-defined Version (8'h10)"]
    pub mod USERVER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Debug"]
pub mod MAC_DEBUG {
    #[doc = "MAC GMII or MII Receive Protocol Engine Status When this bit is set, it indicates that the MAC GMII or MII receive protocol engine is actively receiving data, and it is not in the Idle state."]
    pub mod RPESTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC GMII or MII Receive Protocol Engine Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC GMII or MII Receive Protocol Engine Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MAC Receive Packet Controller FIFO Status When this bit is set, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Packet Controller module."]
    pub mod RFCFCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC GMII or MII Transmit Protocol Engine Status When this bit is set, it indicates that the MAC GMII or MII transmit protocol engine is actively transmitting data, and it is not in the Idle state."]
    pub mod TPESTS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC GMII or MII Transmit Protocol Engine Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC GMII or MII Transmit Protocol Engine Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MAC Transmit Packet Controller Status This field indicates the state of the MAC Transmit Packet Controller module."]
    pub mod TFCSTS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Waiting for one of the following: Status of the previous packet OR IPG or back off period to be over"]
            pub const WAITING: u32 = 0x01;
            #[doc = "Generating and transmitting a Pause control packet (in full-duplex mode)"]
            pub const GEN_TX_PAU: u32 = 0x02;
            #[doc = "Transferring input packet for transmission"]
            pub const TRNSFR: u32 = 0x03;
        }
    }
}
#[doc = "Optional Features or Functions 0"]
pub mod MAC_HW_FEATURE0 {
    #[doc = "10 or 100 Mbps Support This bit is set to 1 when 10/100 Mbps is selected as the Mode of Operation"]
    pub mod MIISEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No 10 or 100 Mbps support"]
            pub const INACTIVE: u32 = 0;
            #[doc = "10 or 100 Mbps support"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "1000 Mbps Support This bit is set to 1 when 1000 Mbps is selected as the Mode of Operation"]
    pub mod GMIISEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No 1000 Mbps support"]
            pub const INACTIVE: u32 = 0;
            #[doc = "1000 Mbps support"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Half-duplex Support This bit is set to 1 when the half-duplex mode is selected"]
    pub mod HDSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Half-duplex support"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Half-duplex support"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface) This bit is set to 1 when the TBI, SGMII, or RTBI PHY interface option is selected"]
    pub mod PCSSEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "VLAN Hash Filter Selected This bit is set to 1 when the Enable VLAN Hash Table Based Filtering option is selected"]
    pub mod VLHASH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Hash Filter not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "VLAN Hash Filter selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "SMA (MDIO) Interface This bit is set to 1 when the Enable Station Management (MDIO Interface) option is selected"]
    pub mod SMASEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SMA (MDIO) Interface not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "SMA (MDIO) Interface selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "PMT Remote Wake-up Packet Enable This bit is set to 1 when the Enable Remote Wake-Up Packet Detection option is selected"]
    pub mod RWKSEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMT Remote Wake-up Packet Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PMT Remote Wake-up Packet Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "PMT Magic Packet Enable This bit is set to 1 when the Enable Magic Packet Detection option is selected"]
    pub mod MGKSEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMT Magic Packet Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PMT Magic Packet Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "RMON Module Enable This bit is set to 1 when the Enable MAC Management Counters (MMC) option is selected"]
    pub mod MMCSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RMON Module Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "RMON Module Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "ARP Offload Enabled This bit is set to 1 when the Enable IPv4 ARP Offload option is selected"]
    pub mod ARPOFFSEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ARP Offload Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "ARP Offload Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "IEEE 1588-2008 Timestamp Enabled This bit is set to 1 when the Enable IEEE 1588 Timestamp Support option is selected"]
    pub mod TSSEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IEEE 1588-2008 Timestamp Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "IEEE 1588-2008 Timestamp Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Energy Efficient Ethernet Enabled This bit is set to 1 when the Enable Energy Efficient Ethernet (EEE) option is selected"]
    pub mod EEESEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Energy Efficient Ethernet Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Energy Efficient Ethernet Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Checksum Offload Enabled This bit is set to 1 when the Enable Transmit TCP/IP Checksum Insertion option is selected"]
    pub mod TXCOESEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Checksum Offload Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Checksum Offload Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Checksum Offload Enabled This bit is set to 1 when the Enable Receive TCP/IP Checksum Check option is selected"]
    pub mod RXCOESEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Checksum Offload Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Checksum Offload Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MAC Addresses 1-31 Selected This bit is set to 1 when the non-zero value is selected for Enable Additional 1-31 MAC Address Registers option"]
    pub mod ADDMACADRSEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Addresses 32-63 Selected This bit is set to 1 when the Enable Additional 32 MAC Address Registers (32-63) option is selected"]
    pub mod MACADR32SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC Addresses 32-63 Select option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC Addresses 32-63 Select option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MAC Addresses 64-127 Selected This bit is set to 1 when the Enable Additional 64 MAC Address Registers (64-127) option is selected"]
    pub mod MACADR64SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC Addresses 64-127 Select option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC Addresses 64-127 Select option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp System Time Source This bit indicates the source of the Timestamp system time: This bit is set to 1 when the Enable IEEE 1588 Timestamp Support option is selected"]
    pub mod TSSTSSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal"]
            pub const INTRNL: u32 = 0;
            #[doc = "External"]
            pub const EXTRNL: u32 = 0x01;
            #[doc = "Both"]
            pub const BOTH: u32 = 0x02;
        }
    }
    #[doc = "Source Address or VLAN Insertion Enable This bit is set to 1 when the Enable SA and VLAN Insertion on Tx option is selected"]
    pub mod SAVLANINS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Source Address or VLAN Insertion Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Source Address or VLAN Insertion Enable option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Active PHY Selected When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset de-assertion."]
    pub mod ACTPHYSEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GMII or MII"]
            pub const GMII_MII: u32 = 0;
            #[doc = "RGMII"]
            pub const RGMII: u32 = 0x01;
            #[doc = "SGMII"]
            pub const SGMII: u32 = 0x02;
            #[doc = "TBI"]
            pub const TBI: u32 = 0x03;
            #[doc = "RMII"]
            pub const RMII: u32 = 0x04;
            #[doc = "RTBI"]
            pub const RTBI: u32 = 0x05;
            #[doc = "SMII"]
            pub const SMII: u32 = 0x06;
            #[doc = "RevMII"]
            pub const REVMIII: u32 = 0x07;
        }
    }
}
#[doc = "Optional Features or Functions 1"]
pub mod MAC_HW_FEATURE1 {
    #[doc = "MTL Receive FIFO Size This field contains the configured value of MTL Rx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(RXFIFO_SIZE) -7:"]
    pub mod RXFIFOSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "128 bytes"]
            pub const BF_128B: u32 = 0;
            #[doc = "256 bytes"]
            pub const BF_256B: u32 = 0x01;
            #[doc = "512 bytes"]
            pub const BF_512B: u32 = 0x02;
            #[doc = "1024 bytes"]
            pub const BF_1024B: u32 = 0x03;
            #[doc = "2048 bytes"]
            pub const BF_2048B: u32 = 0x04;
            #[doc = "4096 bytes"]
            pub const BF_4096B: u32 = 0x05;
            #[doc = "8192 bytes"]
            pub const BF_8192B: u32 = 0x06;
            #[doc = "16384 bytes"]
            pub const BF_16384B: u32 = 0x07;
            #[doc = "32 KB"]
            pub const BF_32KB: u32 = 0x08;
            #[doc = "64 KB"]
            pub const BF_64KB: u32 = 0x09;
            #[doc = "128 KB"]
            pub const BF_128KB: u32 = 0x0a;
            #[doc = "256 KB"]
            pub const BF_256KB: u32 = 0x0b;
        }
    }
    #[doc = "Single Port RAM Enable This bit is set to 1 when the Use single port RAM Feature is selected."]
    pub mod SPRAM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single Port RAM feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Single Port RAM feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Transmit FIFO Size This field contains the configured value of MTL Tx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(TXFIFO_SIZE) -7:"]
    pub mod TXFIFOSIZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "128 bytes"]
            pub const BF_128B: u32 = 0;
            #[doc = "256 bytes"]
            pub const BF_256B: u32 = 0x01;
            #[doc = "512 bytes"]
            pub const BF_512B: u32 = 0x02;
            #[doc = "1024 bytes"]
            pub const BF_1024B: u32 = 0x03;
            #[doc = "2048 bytes"]
            pub const BF_2048B: u32 = 0x04;
            #[doc = "4096 bytes"]
            pub const BF_4096B: u32 = 0x05;
            #[doc = "8192 bytes"]
            pub const BF_8192B: u32 = 0x06;
            #[doc = "16384 bytes"]
            pub const BF_16384B: u32 = 0x07;
            #[doc = "32 KB"]
            pub const BF_32KB: u32 = 0x08;
            #[doc = "64 KB"]
            pub const BF_64KB: u32 = 0x09;
            #[doc = "128 KB"]
            pub const BF_128KB: u32 = 0x0a;
        }
    }
    #[doc = "One-Step Timestamping Enable This bit is set to 1 when the Enable One-Step Timestamp Feature is selected."]
    pub mod OSTEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One-Step Timestamping feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "One-Step Timestamping feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "PTP Offload Enable This bit is set to 1 when the Enable PTP Timestamp Offload Feature is selected."]
    pub mod PTOEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PTP Offload feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PTP Offload feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "IEEE 1588 High Word Register Enable This bit is set to 1 when the Add IEEE 1588 Higher Word Register option is selected"]
    pub mod ADVTHWORD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IEEE 1588 High Word Register option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "IEEE 1588 High Word Register option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Address Width."]
    pub mod ADDR64 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32"]
            pub const BF_32: u32 = 0;
            #[doc = "40"]
            pub const BF_40: u32 = 0x01;
            #[doc = "48"]
            pub const BF_48: u32 = 0x02;
        }
    }
    #[doc = "DCB Feature Enable This bit is set to 1 when the Enable Data Center Bridging option is selected"]
    pub mod DCBEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DCB Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DCB Feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Split Header Feature Enable This bit is set to 1 when the Enable Split Header Structure option is selected"]
    pub mod SPHEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Split Header Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Split Header Feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "TCP Segmentation Offload Enable This bit is set to 1 when the Enable TCP Segmentation Offloading for TCP/IP Packets option is selected"]
    pub mod TSOEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TCP Segmentation Offload Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "TCP Segmentation Offload Feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "DMA Debug Registers Enable This bit is set to 1 when the Debug Mode Enable option is selected"]
    pub mod DBGMEMA {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Debug Registers option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Debug Registers option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "AV Feature Enable This bit is set to 1 when the Enable Audio Video Bridging option is selected."]
    pub mod AVSEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AV Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "AV Feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Rx Side Only AV Feature Enable This bit is set to 1 when the Enable Audio Video Bridging option on Rx Side Only is selected."]
    pub mod RAVSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Side Only AV Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Side Only AV Feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "One Step for PTP over UDP/IP Feature Enable This bit is set to 1 when the Enable One step timestamp for PTP over UDP/IP feature is selected."]
    pub mod POUOST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One Step for PTP over UDP/IP Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "One Step for PTP over UDP/IP Feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Hash Table Size This field indicates the size of the hash table:"]
    pub mod HASHTBLSZ {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No hash table"]
            pub const NO_HT: u32 = 0;
            #[doc = "64"]
            pub const BF_64: u32 = 0x01;
            #[doc = "128"]
            pub const BF_128: u32 = 0x02;
            #[doc = "256"]
            pub const BF_256: u32 = 0x03;
        }
    }
    #[doc = "Total number of L3 or L4 Filters This field indicates the total number of L3 or L4 filters:"]
    pub mod L3L4FNUM {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No L3 or L4 Filter"]
            pub const NOFILT: u32 = 0;
            #[doc = "1 L3 or L4 Filter"]
            pub const BF_1FILT: u32 = 0x01;
            #[doc = "2 L3 or L4 Filters"]
            pub const BF_2FILT: u32 = 0x02;
            #[doc = "3 L3 or L4 Filters"]
            pub const BF_3FILT: u32 = 0x03;
            #[doc = "4 L3 or L4 Filters"]
            pub const BF_4FILT: u32 = 0x04;
            #[doc = "5 L3 or L4 Filters"]
            pub const BF_5FILT: u32 = 0x05;
            #[doc = "6 L3 or L4 Filters"]
            pub const BF_6FILT: u32 = 0x06;
            #[doc = "7 L3 or L4 Filters"]
            pub const BF_7FILT: u32 = 0x07;
            #[doc = "8 L3 or L4 Filters"]
            pub const BF_8FILT: u32 = 0x08;
        }
    }
}
#[doc = "Optional Features or Functions 2"]
pub mod MAC_HW_FEATURE2 {
    #[doc = "Number of MTL Receive Queues This field indicates the number of MTL Receive queues:"]
    pub mod RXQCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 MTL Rx Queue"]
            pub const BF_1RXQ: u32 = 0;
            #[doc = "2 MTL Rx Queues"]
            pub const BF_2RXQ: u32 = 0x01;
            #[doc = "3 MTL Rx Queues"]
            pub const BF_3RXQ: u32 = 0x02;
            #[doc = "4 MTL Rx Queues"]
            pub const BF_4RXQ: u32 = 0x03;
            #[doc = "5 MTL Rx Queues"]
            pub const BF_5RXQ: u32 = 0x04;
        }
    }
    #[doc = "Number of MTL Transmit Queues This field indicates the number of MTL Transmit queues:"]
    pub mod TXQCNT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 MTL Tx Queue"]
            pub const BF_1TXQ: u32 = 0;
            #[doc = "2 MTL Tx Queues"]
            pub const BF_2TXQ: u32 = 0x01;
            #[doc = "3 MTL Tx Queues"]
            pub const BF_3TXQ: u32 = 0x02;
            #[doc = "4 MTL Tx Queues"]
            pub const BF_4TXQ: u32 = 0x03;
            #[doc = "5 MTL Tx Queues"]
            pub const BF_5TXQ: u32 = 0x04;
        }
    }
    #[doc = "Number of DMA Receive Channels This field indicates the number of DMA Receive channels:"]
    pub mod RXCHCNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 MTL Rx Channel"]
            pub const BF_1RXCH: u32 = 0;
            #[doc = "2 MTL Rx Channels"]
            pub const BF_2RXCH: u32 = 0x01;
            #[doc = "3 MTL Rx Channels"]
            pub const BF_3RXCH: u32 = 0x02;
            #[doc = "4 MTL Rx Channels"]
            pub const BF_4RXCH: u32 = 0x03;
            #[doc = "5 MTL Rx Channels"]
            pub const BF_5RXCH: u32 = 0x04;
        }
    }
    #[doc = "Number of DMA Transmit Channels This field indicates the number of DMA Transmit channels:"]
    pub mod TXCHCNT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 MTL Tx Channel"]
            pub const BF_1TXCH: u32 = 0;
            #[doc = "2 MTL Tx Channels"]
            pub const BF_2TXCH: u32 = 0x01;
            #[doc = "3 MTL Tx Channels"]
            pub const BF_3TXCH: u32 = 0x02;
            #[doc = "4 MTL Tx Channels"]
            pub const BF_4TXCH: u32 = 0x03;
            #[doc = "5 MTL Tx Channels"]
            pub const BF_5TXCH: u32 = 0x04;
        }
    }
    #[doc = "Number of PPS Outputs This field indicates the number of PPS outputs:"]
    pub mod PPSOUTNUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No PPS output"]
            pub const NO_PPSO: u32 = 0;
            #[doc = "1 PPS output"]
            pub const BF_1_PPSO: u32 = 0x01;
            #[doc = "2 PPS output"]
            pub const BF_2_PPSO: u32 = 0x02;
            #[doc = "3 PPS output"]
            pub const BF_3_PPSO: u32 = 0x03;
            #[doc = "4 PPS output"]
            pub const BF_4_PPSO: u32 = 0x04;
        }
    }
    #[doc = "Number of Auxiliary Snapshot Inputs This field indicates the number of auxiliary snapshot inputs:"]
    pub mod AUXSNAPNUM {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No auxiliary input"]
            pub const NO_AUXI: u32 = 0;
            #[doc = "1 auxiliary input"]
            pub const BF_1_AUXI: u32 = 0x01;
            #[doc = "2 auxiliary input"]
            pub const BF_2_AUXI: u32 = 0x02;
            #[doc = "3 auxiliary input"]
            pub const BF_3_AUXI: u32 = 0x03;
            #[doc = "4 auxiliary input"]
            pub const BF_4_AUXI: u32 = 0x04;
        }
    }
}
#[doc = "Optional Features or Functions 3"]
pub mod MAC_HW_FEATURE3 {
    #[doc = "Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected:"]
    pub mod NRVF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Extended Rx VLAN Filters"]
            pub const NO_ERVLAN: u32 = 0;
            #[doc = "4 Extended Rx VLAN Filters"]
            pub const BF_4_ERVLAN: u32 = 0x01;
            #[doc = "8 Extended Rx VLAN Filters"]
            pub const BF_8_ERVLAN: u32 = 0x02;
            #[doc = "16 Extended Rx VLAN Filters"]
            pub const BF_16_ERVLAN: u32 = 0x03;
            #[doc = "24 Extended Rx VLAN Filters"]
            pub const BF_24_ERVLAN: u32 = 0x04;
            #[doc = "32 Extended Rx VLAN Filters"]
            pub const BF_32_ERVLAN: u32 = 0x05;
        }
    }
    #[doc = "Queue/Channel based VLAN tag insertion on Tx Enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx Feature is selected."]
    pub mod CBTISEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Queue/Channel based VLAN tag insertion on Tx feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Enable Queue/Channel based VLAN tag insertion on Tx feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Double VLAN Tag Processing Selected This bit is set to 1 when the Enable Double VLAN Processing Feature is selected."]
    pub mod DVLAN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double VLAN option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Double VLAN option is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Broadcast/Multicast Packet Duplication This bit is set to 1 when the Broadcast/Multicast Packet Duplication feature is selected."]
    pub mod PDUPSEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Broadcast/Multicast Packet Duplication feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Broadcast/Multicast Packet Duplication feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Flexible Receive Parser Selected This bit is set to 1 when the Enable Flexible Programmable Receive Parser option is selected."]
    pub mod FRPSEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flexible Receive Parser feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Flexible Receive Parser feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Flexible Receive Parser Buffer size This field indicates the supported Max Number of bytes of the packet data to be Parsed by Flexible Receive Parser."]
    pub mod FRPBS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64 Bytes"]
            pub const BF_64BYTES: u32 = 0;
            #[doc = "128 Bytes"]
            pub const BF_128BYTES: u32 = 0x01;
            #[doc = "256 Bytes"]
            pub const BF_256BYTES: u32 = 0x02;
        }
    }
    #[doc = "Flexible Receive Parser Table Entries size This field indicates the Max Number of Parser Entries supported by Flexible Receive Parser."]
    pub mod FRPES {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64 Entries"]
            pub const BF_64ENTR: u32 = 0;
            #[doc = "128 Entries"]
            pub const BF_128ENTR: u32 = 0x01;
            #[doc = "256 Entries"]
            pub const BF_256ENTR: u32 = 0x02;
        }
    }
    #[doc = "Enhancements to Scheduling Traffic Enable This bit is set to 1 when the Enable Enhancements to Scheduling Traffic feature is selected."]
    pub mod ESTSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Enhancements to Scheduling Traffic feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Enable Enhancements to Scheduling Traffic feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Depth of the Gate Control List This field indicates the depth of Gate Control list expressed as Log2(DWC_EQOS_EST_DEP)-5"]
    pub mod ESTDEP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Depth configured"]
            pub const NODEPTH: u32 = 0;
            #[doc = "64"]
            pub const DEPTH64: u32 = 0x01;
            #[doc = "128"]
            pub const DEPTH128: u32 = 0x02;
            #[doc = "256"]
            pub const DEPTH256: u32 = 0x03;
            #[doc = "512"]
            pub const DEPTH512: u32 = 0x04;
            #[doc = "1024"]
            pub const DEPTH1024: u32 = 0x05;
        }
    }
    #[doc = "Width of the Time Interval field in the Gate Control List This field indicates the width of the Configured Time Interval Field"]
    pub mod ESTWID {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Width not configured"]
            pub const NOWIDTH: u32 = 0;
            #[doc = "16"]
            pub const WIDTH16: u32 = 0x01;
            #[doc = "20"]
            pub const WIDTH20: u32 = 0x02;
            #[doc = "24"]
            pub const WIDTH24: u32 = 0x03;
        }
    }
    #[doc = "Frame Preemption Enable This bit is set to 1 when the Enable Frame preemption feature is selected."]
    pub mod FPESEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Preemption Enable feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Frame Preemption Enable feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Time Based Scheduling Enable This bit is set to 1 when the Time Based Scheduling feature is selected."]
    pub mod TBSSEL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Time Based Scheduling Enable feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Time Based Scheduling Enable feature is selected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Automotive Safety Package Following are the encoding for the different Safety features"]
    pub mod ASP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Safety features selected"]
            pub const NONE: u32 = 0;
            #[doc = "Only \"ECC protection for external memory\" feature is selected"]
            pub const ECC_ONLY: u32 = 0x01;
            #[doc = "All the Automotive Safety features are selected without the \"Parity Port Enable for external interface\" feature"]
            pub const AS_NPPE: u32 = 0x02;
            #[doc = "All the Automotive Safety features are selected with the \"Parity Port Enable for external interface\" feature"]
            pub const AS_PPE: u32 = 0x03;
        }
    }
}
#[doc = "MDIO Address"]
pub mod MAC_MDIO_ADDRESS {
    #[doc = "GMII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIO slave."]
    pub mod GB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GMII Busy is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "GMII Busy is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO."]
    pub mod C45E {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clause 45 PHY is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clause 45 PHY is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GMII Operation Command 0 This is the lower bit of the operation command to the PHY or RevMII."]
    pub mod GOC_0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GMII Operation Command 0 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "GMII Operation Command 0 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GMII Operation Command 1 This bit is higher bit of the operation command to the PHY or RevMII, GOC_1 and GOC_O is encoded as follows: - 00: Reserved - 01: Write - 10: Post Read Increment Address for Clause 45 PHY - 11: Read When Clause 22 PHY or RevMII is enabled, only Write and Read commands are valid."]
    pub mod GOC_1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GMII Operation Command 1 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "GMII Operation Command 1 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets."]
    pub mod SKAP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Skip Address Packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Skip Address Packet is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: - 0000: CSR clock = 60-100 MHz; MDC clock = CSR clock/42 - 0001: CSR clock = 100-150 MHz; MDC clock = CSR clock/62 - 0010: CSR clock = 20-35 MHz; MDC clock = CSR clock/16 - 0011: CSR clock = 35-60 MHz; MDC clock = CSR clock/26 - 0100: CSR clock = 150-250 MHz; MDC clock = CSR clock/102 - 0101: CSR clock = 250-300 MHz; MDC clock = CSR clock/124 - 0110: CSR clock = 300-500 MHz; MDC clock = CSR clock/204 - 0111: CSR clock = 500-800 MHz; MDC clock = CSR clock/324 The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1."]
    pub mod CR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Trailing Clocks This field controls the number of trailing clock cycles generated on gmii_mdc_o (MDC) after the end of transmission of MDIO frame."]
    pub mod NTC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Register/Device Address These bits select the PHY register in selected Clause 22 PHY device."]
    pub mod RDA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing."]
    pub mod PA {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted)."]
    pub mod BTB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Back to Back transactions disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Back to Back transactions enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmits MDIO frames with only 1 preamble bit."]
    pub mod PSE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Preamble Suppression disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Preamble Suppression enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC MDIO Data"]
pub mod MAC_MDIO_DATA {
    #[doc = "GMII Data This field contains the 16-bit data value read from the PHY or RevMII after a Management Read operation or the 16-bit data value to be written to the PHY or RevMII before a Management Write operation."]
    pub mod GD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Register Address This field is valid only when C45E is set."]
    pub mod RA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSR Software Control"]
pub mod MAC_CSR_SW_CTRL {
    #[doc = "Register Clear on Write 1 Enable When this bit is set, the access mode of some register fields changes to Clear on Write 1, the application needs to set that respective bit to 1 to clear it."]
    pub mod RCWE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register Clear on Write 1 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Register Clear on Write 1 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Frame Preemption Control"]
pub mod MAC_FPE_CTRL_STS {
    #[doc = "Enable Tx Frame Preemption When set Frame Preemption Tx functionality is enabled."]
    pub mod EFPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tx Frame Preemption is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Tx Frame Preemption is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Send Verify mPacket When set indicates hardware to send a verify mPacket."]
    pub mod SVER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Send Verify mPacket is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Send Verify mPacket is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Send Respond mPacket When set indicates hardware to send a Respond mPacket."]
    pub mod SRSP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Send Respond mPacket is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Send Respond mPacket is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Synopsys Reserved, Must be set to \"0\"."]
    pub mod S1_SET_0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Received Verify Frame Set when a Verify mPacket is received."]
    pub mod RVER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not received Verify Frame"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Received Verify Frame"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Received Respond Frame Set when a Respond mPacket is received."]
    pub mod RRSP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not received Respond Frame"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Received Respond Frame"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmitted Verify Frame Set when a Verify mPacket is transmitted (triggered by setting SVER field)."]
    pub mod TVER {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not transmitted Verify Frame"]
            pub const INACTIVE: u32 = 0;
            #[doc = "transmitted Verify Frame"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmitted Respond Frame Set when a Respond mPacket is transmitted (triggered by setting SRSP field)."]
    pub mod TRSP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not transmitted Respond Frame"]
            pub const INACTIVE: u32 = 0;
            #[doc = "transmitted Respond Frame"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "32-bit Binary Rollover Equivalent Time"]
pub mod MAC_PRESN_TIME_NS {
    #[doc = "MAC 1722 Presentation Time in ns These bits indicate the value of the 32-bit binary rollover equivalent time of the PTP System Time in ns"]
    pub mod MPTN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC 1722 Presentation Time"]
pub mod MAC_PRESN_TIME_UPDT {
    #[doc = "MAC 1722 Presentation Time Update This field holds the init value or the update value for the presentation time."]
    pub mod MPTU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address0 High"]
pub mod MAC_ADDRESS0_HIGH {
    #[doc = "MAC Address0\\[47:32\\] This field contains the upper 16 bits \\[47:32\\] of the first 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address0 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable This bit is always set to 1."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "INVALID : This bit must be always set to 1"]
            pub const DISABLE: u32 = 0;
            #[doc = "This bit is always set to 1"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address0 Low"]
pub mod MAC_ADDRESS0_LOW {
    #[doc = "MAC Address0\\[31:0\\] This field contains the lower 32 bits of the first 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address1 High"]
pub mod MAC_ADDRESS1_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address1 Low"]
pub mod MAC_ADDRESS1_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address2 High"]
pub mod MAC_ADDRESS2_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address2 Low"]
pub mod MAC_ADDRESS2_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address3 High"]
pub mod MAC_ADDRESS3_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address3 Low"]
pub mod MAC_ADDRESS3_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address4 High"]
pub mod MAC_ADDRESS4_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address4 Low"]
pub mod MAC_ADDRESS4_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address5 High"]
pub mod MAC_ADDRESS5_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address5 Low"]
pub mod MAC_ADDRESS5_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address6 High"]
pub mod MAC_ADDRESS6_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address6 Low"]
pub mod MAC_ADDRESS6_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address7 High"]
pub mod MAC_ADDRESS7_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address7 Low"]
pub mod MAC_ADDRESS7_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address8 High"]
pub mod MAC_ADDRESS8_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address8 Low"]
pub mod MAC_ADDRESS8_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address9 High"]
pub mod MAC_ADDRESS9_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address9 Low"]
pub mod MAC_ADDRESS9_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address10 High"]
pub mod MAC_ADDRESS10_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address10 Low"]
pub mod MAC_ADDRESS10_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address11 High"]
pub mod MAC_ADDRESS11_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address11 Low"]
pub mod MAC_ADDRESS11_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address12 High"]
pub mod MAC_ADDRESS12_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address12 Low"]
pub mod MAC_ADDRESS12_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address13 High"]
pub mod MAC_ADDRESS13_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address13 Low"]
pub mod MAC_ADDRESS13_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address14 High"]
pub mod MAC_ADDRESS14_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address14 Low"]
pub mod MAC_ADDRESS14_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address15 High"]
pub mod MAC_ADDRESS15_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address15 Low"]
pub mod MAC_ADDRESS15_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address16 High"]
pub mod MAC_ADDRESS16_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address16 Low"]
pub mod MAC_ADDRESS16_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address17 High"]
pub mod MAC_ADDRESS17_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address17 Low"]
pub mod MAC_ADDRESS17_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address18 High"]
pub mod MAC_ADDRESS18_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address18 Low"]
pub mod MAC_ADDRESS18_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address19 High"]
pub mod MAC_ADDRESS19_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address19 Low"]
pub mod MAC_ADDRESS19_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address20 High"]
pub mod MAC_ADDRESS20_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address20 Low"]
pub mod MAC_ADDRESS20_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address21 High"]
pub mod MAC_ADDRESS21_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address21 Low"]
pub mod MAC_ADDRESS21_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address22 High"]
pub mod MAC_ADDRESS22_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address22 Low"]
pub mod MAC_ADDRESS22_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address23 High"]
pub mod MAC_ADDRESS23_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address23 Low"]
pub mod MAC_ADDRESS23_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address24 High"]
pub mod MAC_ADDRESS24_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address24 Low"]
pub mod MAC_ADDRESS24_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address25 High"]
pub mod MAC_ADDRESS25_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address25 Low"]
pub mod MAC_ADDRESS25_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address26 High"]
pub mod MAC_ADDRESS26_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address26 Low"]
pub mod MAC_ADDRESS26_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address27 High"]
pub mod MAC_ADDRESS27_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address27 Low"]
pub mod MAC_ADDRESS27_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address28 High"]
pub mod MAC_ADDRESS28_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address28 Low"]
pub mod MAC_ADDRESS28_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address29 High"]
pub mod MAC_ADDRESS29_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address29 Low"]
pub mod MAC_ADDRESS29_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address30 High"]
pub mod MAC_ADDRESS30_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address30 Low"]
pub mod MAC_ADDRESS30_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address31 High"]
pub mod MAC_ADDRESS31_HIGH {
    #[doc = "MAC ADDRESS1 \\[47:32\\] This field contains the upper 16 bits\\[47:32\\] of the second 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes."]
    pub mod MBC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address When this bit is set, the MAC ADDRESS1\\[47:0\\] is used to compare with the SA fields of the received packet."]
    pub mod SA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compare with Destination Address"]
            pub const DA: u32 = 0;
            #[doc = "Compare with Source Address"]
            pub const SA: u32 = 0x01;
        }
    }
    #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address31 Low"]
pub mod MAC_ADDRESS31_LOW {
    #[doc = "MAC ADDRESS1 \\[31:0\\] This field contains the lower 32 bits of second 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address32 High"]
pub mod MAC_ADDRESS32_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address32 Low"]
pub mod MAC_ADDRESS32_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address33 High"]
pub mod MAC_ADDRESS33_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address33 Low"]
pub mod MAC_ADDRESS33_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address34 High"]
pub mod MAC_ADDRESS34_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address34 Low"]
pub mod MAC_ADDRESS34_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address35 High"]
pub mod MAC_ADDRESS35_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address35 Low"]
pub mod MAC_ADDRESS35_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address36 High"]
pub mod MAC_ADDRESS36_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address36 Low"]
pub mod MAC_ADDRESS36_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address37 High"]
pub mod MAC_ADDRESS37_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address37 Low"]
pub mod MAC_ADDRESS37_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address38 High"]
pub mod MAC_ADDRESS38_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address38 Low"]
pub mod MAC_ADDRESS38_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address39 High"]
pub mod MAC_ADDRESS39_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address39 Low"]
pub mod MAC_ADDRESS39_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address40 High"]
pub mod MAC_ADDRESS40_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address40 Low"]
pub mod MAC_ADDRESS40_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address41 High"]
pub mod MAC_ADDRESS41_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address41 Low"]
pub mod MAC_ADDRESS41_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address42 High"]
pub mod MAC_ADDRESS42_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address42 Low"]
pub mod MAC_ADDRESS42_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address43 High"]
pub mod MAC_ADDRESS43_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address43 Low"]
pub mod MAC_ADDRESS43_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address44 High"]
pub mod MAC_ADDRESS44_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address44 Low"]
pub mod MAC_ADDRESS44_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address45 High"]
pub mod MAC_ADDRESS45_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address45 Low"]
pub mod MAC_ADDRESS45_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address46 High"]
pub mod MAC_ADDRESS46_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address46 Low"]
pub mod MAC_ADDRESS46_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address47 High"]
pub mod MAC_ADDRESS47_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address47 Low"]
pub mod MAC_ADDRESS47_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address48 High"]
pub mod MAC_ADDRESS48_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address48 Low"]
pub mod MAC_ADDRESS48_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address49 High"]
pub mod MAC_ADDRESS49_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address49 Low"]
pub mod MAC_ADDRESS49_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address50 High"]
pub mod MAC_ADDRESS50_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address50 Low"]
pub mod MAC_ADDRESS50_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address51 High"]
pub mod MAC_ADDRESS51_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address51 Low"]
pub mod MAC_ADDRESS51_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address52 High"]
pub mod MAC_ADDRESS52_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address52 Low"]
pub mod MAC_ADDRESS52_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address53 High"]
pub mod MAC_ADDRESS53_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address53 Low"]
pub mod MAC_ADDRESS53_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address54 High"]
pub mod MAC_ADDRESS54_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address54 Low"]
pub mod MAC_ADDRESS54_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address55 High"]
pub mod MAC_ADDRESS55_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address55 Low"]
pub mod MAC_ADDRESS55_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address56 High"]
pub mod MAC_ADDRESS56_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address56 Low"]
pub mod MAC_ADDRESS56_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address57 High"]
pub mod MAC_ADDRESS57_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address57 Low"]
pub mod MAC_ADDRESS57_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address58 High"]
pub mod MAC_ADDRESS58_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address58 Low"]
pub mod MAC_ADDRESS58_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address59 High"]
pub mod MAC_ADDRESS59_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address59 Low"]
pub mod MAC_ADDRESS59_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address60 High"]
pub mod MAC_ADDRESS60_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address60 Low"]
pub mod MAC_ADDRESS60_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address61 High"]
pub mod MAC_ADDRESS61_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address61 Low"]
pub mod MAC_ADDRESS61_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address62 High"]
pub mod MAC_ADDRESS62_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address62 Low"]
pub mod MAC_ADDRESS62_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address63 High"]
pub mod MAC_ADDRESS63_HIGH {
    #[doc = "MAC ADDRESS32 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address."]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed."]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering."]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address is ignored"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MAC Address63 Low"]
pub mod MAC_ADDRESS63_LOW {
    #[doc = "MAC ADDRESS32 \\[31:0\\] This field contains the lower 32 bits of the 33rd 6-byte MAC address."]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MMC Control"]
pub mod MAC_MMC_CONTROL {
    #[doc = "Counters Reset When this bit is set, all counters are reset."]
    pub mod CNTRST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counters are not reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "All counters are reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Counter Stop Rollover When this bit is set, the counter does not roll over to zero after reaching the maximum value."]
    pub mod CNTSTOPRO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter Stop Rollover is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Counter Stop Rollover is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Reset on Read When this bit is set, the MMC counters are reset to zero after Read (self-clearing after reset)."]
    pub mod RSTONRD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset on Read is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Reset on Read is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Counter Freeze When this bit is set, it freezes all MMC counters to their current value."]
    pub mod CNTFREEZ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Counter Freeze is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Counter Freeze is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Counters Preset When this bit is set, all counters are initialized or preset to almost full or almost half according to the CNTPRSTLVL bit."]
    pub mod CNTPRST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counters Preset is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Counters Preset is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Full-Half Preset When this bit is low and the CNTPRST bit is set, all MMC counters get preset to almost-half value."]
    pub mod CNTPRSTLVL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Full-Half Preset is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Full-Half Preset is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Update MMC Counters for Dropped Broadcast Packets Note: The CNTRST bit has a higher priority than the CNTPRST bit."]
    pub mod UCDBC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Update MMC Counters for Dropped Broadcast Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Update MMC Counters for Dropped Broadcast Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MMC Rx Interrupt"]
pub mod MAC_MMC_RX_INTERRUPT {
    #[doc = "MMC Receive Good Bad Packet Counter Interrupt Status This bit is set when the rxpacketcount_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RXGBPKTIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Good Bad Octet Counter Interrupt Status This bit is set when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RXGBOCTIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Good Bad Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Good Bad Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Good Octet Counter Interrupt Status This bit is set when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXGOCTIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Good Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Good Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Broadcast Good Packet Counter Interrupt Status This bit is set when the rxbroadcastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXBCGPIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Broadcast Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Broadcast Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Multicast Good Packet Counter Interrupt Status This bit is set when the rxmulticastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXMCGPIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Multicast Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Multicast Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
    pub mod RXCRCERPIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive CRC Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive CRC Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
    pub mod RXALGNERPIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Runt Packet Counter Interrupt Status This bit is set when the rxrunterror counter reaches half of the maximum value or the maximum value."]
    pub mod RXRUNTPIS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Runt Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Runt Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Jabber Error Packet Counter Interrupt Status This bit is set when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
    pub mod RXJABERPIS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Jabber Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Jabber Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Undersize Good Packet Counter Interrupt Status This bit is set when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXUSIZEGPIS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Undersize Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Undersize Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Oversize Good Packet Counter Interrupt Status This bit is set when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXOSIZEGPIS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Oversize Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Oversize Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 64 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX64OCTGBPIS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 64 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive 64 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX65T127OCTGBPIS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX128T255OCTGBPIS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX256T511OCTGBPIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX512T1023OCTGBPIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX1024TMAXOCTGBPIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the rxunicastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXUCGPIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Length Error Packet Counter Interrupt Status This bit is set when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
    pub mod RXLENERPIS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Length Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Length Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Out Of Range Error Packet Counter Interrupt Status."]
    pub mod RXORANGEPIS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Out Of Range Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Out Of Range Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Pause Packet Counter Interrupt Status This bit is set when the rxpausepackets counter reaches half of the maximum value or the maximum value."]
    pub mod RXPAUSPIS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Pause Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Pause Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive FIFO Overflow Packet Counter Interrupt Status This bit is set when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
    pub mod RXFOVPIS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive FIFO Overflow Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive FIFO Overflow Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive VLAN Good Bad Packet Counter Interrupt Status This bit is set when the rxvlanpackets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RXVLANGBPIS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive VLAN Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive VLAN Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Watchdog Error Packet Counter Interrupt Status This bit is set when the rxwatchdog error counter reaches half of the maximum value or the maximum value."]
    pub mod RXWDOGPIS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Watchdog Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Watchdog Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Error Packet Counter Interrupt Status This bit is set when the rxrcverror counter reaches half of the maximum value or the maximum value."]
    pub mod RXRCVERRPIS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Control Packet Counter Interrupt Status This bit is set when the rxctrlpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXCTRLPIS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Control Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive Control Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx_LPI_USEC_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod RXLPIUSCIS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive LPI microsecond Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive LPI microsecond Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive LPI transition counter interrupt status This bit is set when the Rx_LPI_Tran_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod RXLPITRCIS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive LPI transition Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive LPI transition Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "MMC Tx Interrupt"]
pub mod MAC_MMC_TX_INTERRUPT {
    #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Status This bit is set when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXGBOCTIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Good Bad Packet Counter Interrupt Status This bit is set when the txpacketcount_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXGBPKTIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Broadcast Good Packet Counter Interrupt Status This bit is set when the txbroadcastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXBCGPIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Broadcast Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Broadcast Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Multicast Good Packet Counter Interrupt Status This bit is set when the txmulticastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXMCGPIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Multicast Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Multicast Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX64OCTGBPIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx65to127octets_gb counter reaches half the maximum value, and also when it reaches the maximum value."]
    pub mod TX65T127OCTGBPIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX128T255OCTGBPIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX256T511OCTGBPIS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX512T1023OCTGBPIS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX1024TMAXOCTGBPIS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Unicast Good Bad Packet Counter Interrupt Status This bit is set when the txunicastpackets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXUCGBPIS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Unicast Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Unicast Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Multicast Good Bad Packet Counter Interrupt Status The bit is set when the txmulticastpackets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXMCGBPIS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Multicast Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Multicast Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Broadcast Good Bad Packet Counter Interrupt Status This bit is set when the txbroadcastpackets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXBCGBPIS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Broadcast Good Bad Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Broadcast Good Bad Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Underflow Error Packet Counter Interrupt Status This bit is set when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
    pub mod TXUFLOWERPIS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Underflow Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Underflow Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXSCOLGPIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXMCOLGPIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Deferred Packet Counter Interrupt Status This bit is set when the txdeferred counter reaches half of the maximum value or the maximum value."]
    pub mod TXDEFPIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Deferred Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Deferred Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Late Collision Packet Counter Interrupt Status This bit is set when the txlatecol counter reaches half of the maximum value or the maximum value."]
    pub mod TXLATCOLPIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Late Collision Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Late Collision Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Excessive Collision Packet Counter Interrupt Status This bit is set when the txexesscol counter reaches half of the maximum value or the maximum value."]
    pub mod TXEXCOLPIS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Excessive Collision Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Excessive Collision Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Carrier Error Packet Counter Interrupt Status This bit is set when the txcarriererror counter reaches half of the maximum value or the maximum value."]
    pub mod TXCARERPIS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Carrier Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Carrier Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Good Octet Counter Interrupt Status This bit is set when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXGOCTIS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Good Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Good Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Good Packet Counter Interrupt Status This bit is set when the txpacketcount_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXGPKTIS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Excessive Deferral Packet Counter Interrupt Status This bit is set when the txexcessdef counter reaches half of the maximum value or the maximum value."]
    pub mod TXEXDEFPIS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Excessive Deferral Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Excessive Deferral Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Pause Packet Counter Interrupt Status This bit is set when the txpausepacketserror counter reaches half of the maximum value or the maximum value."]
    pub mod TXPAUSPIS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Pause Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Pause Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit VLAN Good Packet Counter Interrupt Status This bit is set when the txvlanpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXVLANGPIS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit VLAN Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit VLAN Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Oversize Good Packet Counter Interrupt Status This bit is set when the txoversize_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXOSIZEGPIS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Oversize Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit Oversize Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx_LPI_USEC_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod TXLPIUSCIS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit LPI microsecond Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit LPI microsecond Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit LPI transition counter interrupt status This bit is set when the Tx_LPI_Tran_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod TXLPITRCIS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit LPI transition Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Transmit LPI transition Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "MMC Rx Interrupt Mask"]
pub mod MAC_MMC_RX_INTERRUPT_MASK {
    #[doc = "MMC Receive Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxpacketcount_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RXGBPKTIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RXGBOCTIM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Good Bad Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Good Bad Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXGOCTIM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Good Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Good Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Broadcast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxbroadcastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXBCGPIM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Broadcast Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Broadcast Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Multicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxmulticastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXMCGPIM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Multicast Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Multicast Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive CRC Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
    pub mod RXCRCERPIM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive CRC Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive CRC Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
    pub mod RXALGNERPIM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Runt Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxrunterror counter reaches half of the maximum value or the maximum value."]
    pub mod RXRUNTPIM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Runt Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Runt Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Jabber Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
    pub mod RXJABERPIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Jabber Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Jabber Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Undersize Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXUSIZEGPIM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Undersize Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Undersize Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Oversize Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXOSIZEGPIM {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Oversize Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Oversize Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 64 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX64OCTGBPIM {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 64 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive 64 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX65T127OCTGBPIM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX128T255OCTGBPIM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX256T511OCTGBPIM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RX512T1023OCTGBPIM {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask."]
    pub mod RX1024TMAXOCTGBPIM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxunicastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXUCGPIM {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Length Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
    pub mod RXLENERPIM {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Length Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Length Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Out Of Range Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
    pub mod RXORANGEPIM {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Out Of Range Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Out Of Range Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Pause Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxpausepackets counter reaches half of the maximum value or the maximum value."]
    pub mod RXPAUSPIM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Pause Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Pause Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive FIFO Overflow Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
    pub mod RXFOVPIM {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive FIFO Overflow Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive FIFO Overflow Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive VLAN Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxvlanpackets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod RXVLANGBPIM {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive VLAN Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive VLAN Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Watchdog Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxwatchdog counter reaches half of the maximum value or the maximum value."]
    pub mod RXWDOGPIM {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Watchdog Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Watchdog Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxrcverror counter reaches half of the maximum value or the maximum value."]
    pub mod RXRCVERRPIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive Control Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxctrlpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod RXCTRLPIM {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive Control Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive Control Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive LPI microsecond counter interrupt Mask Setting this bit masks the interrupt when the Rx_LPI_USEC_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod RXLPIUSCIM {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive LPI microsecond counter interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive LPI microsecond counter interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive LPI transition counter interrupt Mask Setting this bit masks the interrupt when the Rx_LPI_Tran_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod RXLPITRCIM {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive LPI transition counter interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive LPI transition counter interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MMC Tx Interrupt Mask"]
pub mod MAC_MMC_TX_INTERRUPT_MASK {
    #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXGBOCTIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txpacketcount_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXGBPKTIM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Broadcast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXBCGPIM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Broadcast Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Broadcast Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Multicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXMCGPIM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Multicast Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Multicast Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX64OCTGBPIM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX65T127OCTGBPIM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX128T255OCTGBPIM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX256T511OCTGBPIM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX512T1023OCTGBPIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TX1024TMAXOCTGBPIM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Unicast Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txunicastpackets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXUCGBPIM {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Unicast Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Unicast Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Multicast Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastpackets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXMCGBPIM {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Multicast Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Multicast Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Broadcast Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastpackets_gb counter reaches half of the maximum value or the maximum value."]
    pub mod TXBCGBPIM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Broadcast Good Bad Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Broadcast Good Bad Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Underflow Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
    pub mod TXUFLOWERPIM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Underflow Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Underflow Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXSCOLGPIM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXMCOLGPIM {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Deferred Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txdeferred counter reaches half of the maximum value or the maximum value."]
    pub mod TXDEFPIM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Deferred Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Deferred Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Late Collision Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txlatecol counter reaches half of the maximum value or the maximum value."]
    pub mod TXLATCOLPIM {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Late Collision Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Late Collision Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Excessive Collision Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txexcesscol counter reaches half of the maximum value or the maximum value."]
    pub mod TXEXCOLPIM {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Excessive Collision Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Excessive Collision Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Carrier Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txcarriererror counter reaches half of the maximum value or the maximum value."]
    pub mod TXCARERPIM {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Carrier Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Carrier Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXGOCTIM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Good Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Good Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txpacketcount_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXGPKTIM {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Excessive Deferral Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txexcessdef counter reaches half of the maximum value or the maximum value."]
    pub mod TXEXDEFPIM {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Excessive Deferral Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Excessive Deferral Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Pause Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txpausepackets counter reaches half of the maximum value or the maximum value."]
    pub mod TXPAUSPIM {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Pause Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Pause Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit VLAN Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txvlanpackets_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXVLANGPIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit VLAN Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit VLAN Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Oversize Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txoversize_g counter reaches half of the maximum value or the maximum value."]
    pub mod TXOSIZEGPIM {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Oversize Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Oversize Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit LPI microsecond counter interrupt Mask Setting this bit masks the interrupt when the Tx_LPI_USEC_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod TXLPIUSCIM {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit LPI microsecond counter interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit LPI microsecond counter interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit LPI transition counter interrupt Mask Setting this bit masks the interrupt when the Tx_LPI_Tran_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod TXLPITRCIM {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit LPI transition counter interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit LPI transition counter interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Tx Octet Count Good and Bad"]
pub mod MAC_TX_OCTET_COUNT_GOOD_BAD {
    #[doc = "Tx Octet Count Good Bad This field indicates the number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad packets."]
    pub mod TXOCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Packet Count Good and Bad"]
pub mod MAC_TX_PACKET_COUNT_GOOD_BAD {
    #[doc = "Tx Packet Count Good Bad This field indicates the number of good and bad packets transmitted, exclusive of retried packets."]
    pub mod TXPKTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Broadcast Packets Good"]
pub mod MAC_TX_BROADCAST_PACKETS_GOOD {
    #[doc = "Tx Broadcast Packets Good This field indicates the number of good broadcast packets transmitted."]
    pub mod TXBCASTG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Multicast Packets Good"]
pub mod MAC_TX_MULTICAST_PACKETS_GOOD {
    #[doc = "Tx Multicast Packets Good This field indicates the number of good multicast packets transmitted."]
    pub mod TXMCASTG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Good and Bad 64-Byte Packets"]
pub mod MAC_TX_64OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Tx 64Octets Packets Good_Bad This field indicates the number of good and bad packets transmitted with length 64 bytes, exclusive of preamble and retried packets."]
    pub mod TX64OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Good and Bad 65 to 127-Byte Packets"]
pub mod MAC_TX_65TO127OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Tx 65To127Octets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried packets."]
    pub mod TX65_127OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Good and Bad 128 to 255-Byte Packets"]
pub mod MAC_TX_128TO255OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Tx 128To255Octets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried packets."]
    pub mod TX128_255OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Good and Bad 256 to 511-Byte Packets"]
pub mod MAC_TX_256TO511OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Tx 256To511Octets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried packets."]
    pub mod TX256_511OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Good and Bad 512 to 1023-Byte Packets"]
pub mod MAC_TX_512TO1023OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Tx 512To1023Octets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 512 and 1023 (inclusive) bytes, exclusive of preamble and retried packets."]
    pub mod TX512_1023OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Good and Bad 1024 to Max-Byte Packets"]
pub mod MAC_TX_1024TOMAXOCTETS_PACKETS_GOOD_BAD {
    #[doc = "Tx 1024ToMaxOctets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 1024 and maxsize (inclusive) bytes, exclusive of preamble and retried packets."]
    pub mod TX1024_MAXOCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad Unicast Packets Transmitted"]
pub mod MAC_TX_UNICAST_PACKETS_GOOD_BAD {
    #[doc = "Tx Unicast Packets Good Bad This field indicates the number of good and bad unicast packets transmitted."]
    pub mod TXUCASTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad Multicast Packets Transmitted"]
pub mod MAC_TX_MULTICAST_PACKETS_GOOD_BAD {
    #[doc = "Tx Multicast Packets Good Bad This field indicates the number of good and bad multicast packets transmitted."]
    pub mod TXMCASTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad Broadcast Packets Transmitted"]
pub mod MAC_TX_BROADCAST_PACKETS_GOOD_BAD {
    #[doc = "Tx Broadcast Packets Good Bad This field indicates the number of good and bad broadcast packets transmitted."]
    pub mod TXBCASTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Packets Aborted By Underflow Error"]
pub mod MAC_TX_UNDERFLOW_ERROR_PACKETS {
    #[doc = "Tx Underflow Error Packets This field indicates the number of packets aborted because of packets underflow error."]
    pub mod TXUNDRFLW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Single Collision Good Packets Transmitted"]
pub mod MAC_TX_SINGLE_COLLISION_GOOD_PACKETS {
    #[doc = "Tx Single Collision Good Packets This field indicates the number of successfully transmitted packets after a single collision in the half-duplex mode."]
    pub mod TXSNGLCOLG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Multiple Collision Good Packets Transmitted"]
pub mod MAC_TX_MULTIPLE_COLLISION_GOOD_PACKETS {
    #[doc = "Tx Multiple Collision Good Packets This field indicates the number of successfully transmitted packets after multiple collisions in the half-duplex mode."]
    pub mod TXMULTCOLG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Deferred Packets Transmitted"]
pub mod MAC_TX_DEFERRED_PACKETS {
    #[doc = "Tx Deferred Packets This field indicates the number of successfully transmitted after a deferral in the half-duplex mode."]
    pub mod TXDEFRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Late Collision Packets Transmitted"]
pub mod MAC_TX_LATE_COLLISION_PACKETS {
    #[doc = "Tx Late Collision Packets This field indicates the number of packets aborted because of late collision error."]
    pub mod TXLATECOL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Excessive Collision Packets Transmitted"]
pub mod MAC_TX_EXCESSIVE_COLLISION_PACKETS {
    #[doc = "Tx Excessive Collision Packets This field indicates the number of packets aborted because of excessive (16) collision errors."]
    pub mod TXEXSCOL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Carrier Error Packets Transmitted"]
pub mod MAC_TX_CARRIER_ERROR_PACKETS {
    #[doc = "Tx Carrier Error Packets This field indicates the number of packets aborted because of carrier sense error (no carrier or loss of carrier)."]
    pub mod TXCARR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Transmitted in Good Packets"]
pub mod MAC_TX_OCTET_COUNT_GOOD {
    #[doc = "Tx Octet Count Good This field indicates the number of bytes transmitted, exclusive of preamble, only in good packets."]
    pub mod TXOCTG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Packets Transmitted"]
pub mod MAC_TX_PACKET_COUNT_GOOD {
    #[doc = "Tx Packet Count Good This field indicates the number of good packets transmitted."]
    pub mod TXPKTG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Packets Aborted By Excessive Deferral Error"]
pub mod MAC_TX_EXCESSIVE_DEFERRAL_ERROR {
    #[doc = "Tx Excessive Deferral Error This field indicates the number of packets aborted because of excessive deferral error (deferred for more than two max-sized packet times)."]
    pub mod TXEXSDEF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pause Packets Transmitted"]
pub mod MAC_TX_PAUSE_PACKETS {
    #[doc = "Tx Pause Packets This field indicates the number of good Pause packets transmitted."]
    pub mod TXPAUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good VLAN Packets Transmitted"]
pub mod MAC_TX_VLAN_PACKETS_GOOD {
    #[doc = "Tx VLAN Packets Good This field provides the number of good VLAN packets transmitted."]
    pub mod TXVLANG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Oversize Packets Transmitted"]
pub mod MAC_TX_OSIZE_PACKETS_GOOD {
    #[doc = "Tx OSize Packets Good This field indicates the number of packets transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged packets; 2000 bytes if enabled in S2KP bit of the CONFIGURATION register)."]
    pub mod TXOSIZG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad Packets Received"]
pub mod MAC_RX_PACKETS_COUNT_GOOD_BAD {
    #[doc = "Rx Packets Count Good Bad This field indicates the number of good and bad packets received."]
    pub mod RXPKTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes in Good and Bad Packets Received"]
pub mod MAC_RX_OCTET_COUNT_GOOD_BAD {
    #[doc = "Rx Octet Count Good Bad This field indicates the number of bytes received, exclusive of preamble, in good and bad packets."]
    pub mod RXOCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes in Good Packets Received"]
pub mod MAC_RX_OCTET_COUNT_GOOD {
    #[doc = "Rx Octet Count Good This field indicates the number of bytes received, exclusive of preamble, only in good packets."]
    pub mod RXOCTG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Broadcast Packets Received"]
pub mod MAC_RX_BROADCAST_PACKETS_GOOD {
    #[doc = "Rx Broadcast Packets Good This field indicates the number of good broadcast packets received."]
    pub mod RXBCASTG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Multicast Packets Received"]
pub mod MAC_RX_MULTICAST_PACKETS_GOOD {
    #[doc = "Rx Multicast Packets Good This field indicates the number of good multicast packets received."]
    pub mod RXMCASTG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CRC Error Packets Received"]
pub mod MAC_RX_CRC_ERROR_PACKETS {
    #[doc = "Rx CRC Error Packets This field indicates the number of packets received with CRC error."]
    pub mod RXCRCERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alignment Error Packets Received"]
pub mod MAC_RX_ALIGNMENT_ERROR_PACKETS {
    #[doc = "Rx Alignment Error Packets This field indicates the number of packets received with alignment (dribble) error."]
    pub mod RXALGNERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Runt Error Packets Received"]
pub mod MAC_RX_RUNT_ERROR_PACKETS {
    #[doc = "Rx Runt Error Packets This field indicates the number of packets received with runt (length less than 64 bytes and CRC error) error."]
    pub mod RXRUNTERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Jabber Error Packets Received"]
pub mod MAC_RX_JABBER_ERROR_PACKETS {
    #[doc = "Rx Jabber Error Packets This field indicates the number of giant packets received with length (including CRC) greater than 1,518 bytes (1,522 bytes for VLAN tagged) and with CRC error."]
    pub mod RXJABERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Undersize Packets Received"]
pub mod MAC_RX_UNDERSIZE_PACKETS_GOOD {
    #[doc = "Rx Undersize Packets Good This field indicates the number of packets received with length less than 64 bytes, without any errors."]
    pub mod RXUNDERSZG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Oversize Packets Received"]
pub mod MAC_RX_OVERSIZE_PACKETS_GOOD {
    #[doc = "Rx Oversize Packets Good This field indicates the number of packets received without errors, with length greater than the maxsize (1,518 bytes or 1,522 bytes for VLAN tagged packets; 2000 bytes if enabled in the S2KP bit of the MAC_CONFIGURATION register)."]
    pub mod RXOVERSZG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad 64-Byte Packets Received"]
pub mod MAC_RX_64OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Rx 64 Octets Packets Good Bad This field indicates the number of good and bad packets received with length 64 bytes, exclusive of the preamble."]
    pub mod RX64OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad 64-to-127 Byte Packets Received"]
pub mod MAC_RX_65TO127OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Rx 65-127 Octets Packets Good Bad This field indicates the number of good and bad packets received with length between 65 and 127 (inclusive) bytes, exclusive of the preamble."]
    pub mod RX65_127OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad 128-to-255 Byte Packets Received"]
pub mod MAC_RX_128TO255OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Rx 128-255 Octets Packets Good Bad This field indicates the number of good and bad packets received with length between 128 and 255 (inclusive) bytes, exclusive of the preamble."]
    pub mod RX128_255OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad 256-to-511 Byte Packets Received"]
pub mod MAC_RX_256TO511OCTETS_PACKETS_GOOD_BAD {
    #[doc = "Rx 256-511 Octets Packets Good Bad This field indicates the number of good and bad packets received with length between 256 and 511 (inclusive) bytes, exclusive of the preamble."]
    pub mod RX256_511OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad 512-to-1023 Byte Packets Received"]
pub mod MAC_RX_512TO1023OCTETS_PACKETS_GOOD_BAD {
    #[doc = "RX 512-1023 Octets Packets Good Bad This field indicates the number of good and bad packets received with length between 512 and 1023 (inclusive) bytes, exclusive of the preamble."]
    pub mod RX512_1023OCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad 1024-to-Max Byte Packets Received"]
pub mod MAC_RX_1024TOMAXOCTETS_PACKETS_GOOD_BAD {
    #[doc = "Rx 1024-Max Octets Good Bad This field indicates the number of good and bad packets received with length between 1024 and maxsize (inclusive) bytes, exclusive of the preamble."]
    pub mod RX1024_MAXOCTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Unicast Packets Received"]
pub mod MAC_RX_UNICAST_PACKETS_GOOD {
    #[doc = "Rx Unicast Packets Good This field indicates the number of good unicast packets received."]
    pub mod RXUCASTG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Length Error Packets Received"]
pub mod MAC_RX_LENGTH_ERROR_PACKETS {
    #[doc = "Rx Length Error Packets This field indicates the number of packets received with length error (Length Type field not equal to packet size), for all packets with valid length field."]
    pub mod RXLENERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out-of-range Type Packets Received"]
pub mod MAC_RX_OUT_OF_RANGE_TYPE_PACKETS {
    #[doc = "Rx Out of Range Type Packet This field indicates the number of packets received with length field not equal to the valid packet size (greater than 1,500 but less than 1,536)."]
    pub mod RXOUTOFRNG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pause Packets Received"]
pub mod MAC_RX_PAUSE_PACKETS {
    #[doc = "Rx Pause Packets This field indicates the number of good and valid Pause packets received."]
    pub mod RXPAUSEPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Missed Packets Due to FIFO Overflow"]
pub mod MAC_RX_FIFO_OVERFLOW_PACKETS {
    #[doc = "Rx FIFO Overflow Packets This field indicates the number of missed received packets because of FIFO overflow."]
    pub mod RXFIFOOVFL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good and Bad VLAN Packets Received"]
pub mod MAC_RX_VLAN_PACKETS_GOOD_BAD {
    #[doc = "Rx VLAN Packets Good Bad This field indicates the number of good and bad VLAN packets received."]
    pub mod RXVLANPKTGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog Error Packets Received"]
pub mod MAC_RX_WATCHDOG_ERROR_PACKETS {
    #[doc = "Rx Watchdog Error Packets This field indicates the number of packets received with error because of watchdog timeout error (packets with a data load larger than 2,048 bytes (when JE and WD bits are reset in MAC_CONFIGURATION register), 10,240 bytes (when JE bit is set and WD bit is reset in MAC_CONFIGURATION register), 16,384 bytes (when WD bit is set in MAC_CONFIGURATION register) or the value programmed in the MAC_WATCHDOG_TIMEOUT register)."]
    pub mod RXWDGERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Error Packets Received"]
pub mod MAC_RX_RECEIVE_ERROR_PACKETS {
    #[doc = "Rx Receive Error Packets This field indicates the number of packets received with Receive error or Packet Extension error on the GMII or MII interface."]
    pub mod RXRCVERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Control Packets Received"]
pub mod MAC_RX_CONTROL_PACKETS_GOOD {
    #[doc = "Rx Control Packets Good This field indicates the number of good control packets received."]
    pub mod RXCTRLG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Microseconds Tx LPI Asserted"]
pub mod MAC_TX_LPI_USEC_CNTR {
    #[doc = "Tx LPI Microseconds Counter This field indicates the number of microseconds Tx LPI is asserted."]
    pub mod TXLPIUSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Number of Times Tx LPI Asserted"]
pub mod MAC_TX_LPI_TRAN_CNTR {
    #[doc = "Tx LPI Transition counter This field indicates the number of times Tx LPI Entry has occurred."]
    pub mod TXLPITRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Microseconds Rx LPI Sampled"]
pub mod MAC_RX_LPI_USEC_CNTR {
    #[doc = "Rx LPI Microseconds Counter This field indicates the number of microseconds Rx LPI is asserted."]
    pub mod RXLPIUSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Number of Times Rx LPI Entered"]
pub mod MAC_RX_LPI_TRAN_CNTR {
    #[doc = "Rx LPI Transition counter This field indicates the number of times Rx LPI Entry has occurred."]
    pub mod RXLPITRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MMC IPC Receive Interrupt Mask"]
pub mod MAC_MMC_IPC_RX_INTERRUPT_MASK {
    #[doc = "MMC Receive IPV4 Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4GPIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Header Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4HERPIM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Header Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 Header Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 No Payload Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4NOPAYPIM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 No Payload Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 No Payload Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Fragmented Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4FRAGPIM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Fragmented Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 Fragmented Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4UDSBLPIM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6GPIM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV6 Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Header Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6HERPIM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Header Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV6 Header Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 No Payload Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6NOPAYPIM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 No Payload Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV6 No Payload Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive UDP Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXUDPGPIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive UDP Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive UDP Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive UDP Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXUDPERPIM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive UDP Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive UDP Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive TCP Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXTCPGPIM {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive TCP Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive TCP Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive TCP Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXTCPERPIM {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive TCP Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive TCP Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive ICMP Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXICMPGPIM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive ICMP Good Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive ICMP Good Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive ICMP Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXICMPERPIM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive ICMP Error Packet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive ICMP Error Packet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4GOIM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4HEROIM {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4NOPAYOIM {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4FRAGOIM {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4UDSBLOIM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6GOIM {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6HEROIM {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6NOPAYOIM {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXUDPGOIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive UDP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXUDPEROIM {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive UDP Good Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive UDP Good Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive TCP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXTCPGOIM {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive TCP Good Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive TCP Good Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive TCP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXTCPEROIM {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive TCP Error Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive TCP Error Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXICMPGOIM {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXICMPEROIM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MMC IPC Receive Interrupt"]
pub mod MAC_MMC_IPC_RX_INTERRUPT {
    #[doc = "MMC Receive IPV4 Good Packet Counter Interrupt Status This bit is set when the rxipv4_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4GPIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Header Error Packet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4HERPIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Header Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 Header Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 No Payload Packet Counter Interrupt Status This bit is set when the rxipv4_nopay_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4NOPAYPIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 No Payload Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 No Payload Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Fragmented Packet Counter Interrupt Status This bit is set when the rxipv4_frag_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4FRAGPIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Fragmented Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 Fragmented Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Status This bit is set when the rxipv4_udsbl_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4UDSBLPIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Good Packet Counter Interrupt Status This bit is set when the rxipv6_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6GPIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV6 Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Header Error Packet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6HERPIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Header Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV6 Header Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 No Payload Packet Counter Interrupt Status This bit is set when the rxipv6_nopay_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6NOPAYPIS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 No Payload Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV6 No Payload Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MC Receive UDP Good Packet Counter Interrupt Status This bit is set when the rxudp_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXUDPGPIS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive UDP Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive UDP Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive UDP Error Packet Counter Interrupt Status This bit is set when the rxudp_err_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXUDPERPIS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive UDP Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive UDP Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive TCP Good Packet Counter Interrupt Status This bit is set when the rxtcp_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXTCPGPIS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive TCP Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive TCP Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive TCP Error Packet Counter Interrupt Status This bit is set when the rxtcp_err_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXTCPERPIS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive TCP Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive TCP Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive ICMP Good Packet Counter Interrupt Status This bit is set when the rxicmp_gd_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXICMPGPIS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive ICMP Good Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive ICMP Good Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive ICMP Error Packet Counter Interrupt Status This bit is set when the rxicmp_err_pkts counter reaches half of the maximum value or the maximum value."]
    pub mod RXICMPERPIS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive ICMP Error Packet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive ICMP Error Packet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Status This bit is set when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4GOIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4HEROIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Status This bit is set when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4NOPAYOIS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Status This bit is set when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4FRAGOIS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status This bit is set when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV4UDSBLOIS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Status This bit is set when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6GOIS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6HEROIS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Status This bit is set when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXIPV6NOPAYOIS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive UDP Good Octet Counter Interrupt Status This bit is set when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXUDPGOIS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive UDP Good Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive UDP Good Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive UDP Error Octet Counter Interrupt Status This bit is set when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXUDPEROIS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive UDP Error Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive UDP Error Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive TCP Good Octet Counter Interrupt Status This bit is set when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXTCPGOIS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive TCP Good Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive TCP Good Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive TCP Error Octet Counter Interrupt Status This bit is set when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXTCPEROIS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive TCP Error Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive TCP Error Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Status This bit is set when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXICMPGOIS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Status This bit is set when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
    pub mod RXICMPEROIS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Good IPv4 Datagrams Received"]
pub mod MAC_RXIPV4_GOOD_PACKETS {
    #[doc = "RxIPv4 Good Packets This field indicates the number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
    pub mod RXIPV4GDPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv4 Datagrams Received with Header Errors"]
pub mod MAC_RXIPV4_HEADER_ERROR_PACKETS {
    #[doc = "RxIPv4 Header Error Packets This field indicates the number of IPv4 datagrams received with header (checksum, length, or version mismatch) errors."]
    pub mod RXIPV4HDRERRPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv4 Datagrams Received with No Payload"]
pub mod MAC_RXIPV4_NO_PAYLOAD_PACKETS {
    #[doc = "RxIPv4 Payload Packets This field indicates the number of IPv4 datagram packets received that did not have a TCP, UDP, or ICMP payload."]
    pub mod RXIPV4NOPAYPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv4 Datagrams Received with Fragmentation"]
pub mod MAC_RXIPV4_FRAGMENTED_PACKETS {
    #[doc = "RxIPv4 Fragmented Packets This field indicates the number of good IPv4 datagrams received with fragmentation."]
    pub mod RXIPV4FRAGPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv4 Datagrams Received with UDP Checksum Disabled"]
pub mod MAC_RXIPV4_UDP_CHECKSUM_DISABLED_PACKETS {
    #[doc = "RxIPv4 UDP Checksum Disabled Packets This field indicates the number of good IPv4 datagrams received that had a UDP payload with checksum disabled."]
    pub mod RXIPV4UDSBLPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good IPv6 Datagrams Received"]
pub mod MAC_RXIPV6_GOOD_PACKETS {
    #[doc = "RxIPv6 Good Packets This field indicates the number of good IPv6 datagrams received with the TCP, UDP, or ICMP payload."]
    pub mod RXIPV6GDPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv6 Datagrams Received with Header Errors"]
pub mod MAC_RXIPV6_HEADER_ERROR_PACKETS {
    #[doc = "RxIPv6 Header Error Packets This field indicates the number of IPv6 datagrams received with header (length or version mismatch) errors."]
    pub mod RXIPV6HDRERRPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv6 Datagrams Received with No Payload"]
pub mod MAC_RXIPV6_NO_PAYLOAD_PACKETS {
    #[doc = "RxIPv6 Payload Packets This field indicates the number of IPv6 datagram packets received that did not have a TCP, UDP, or ICMP payload."]
    pub mod RXIPV6NOPAYPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv6 Datagrams Received with Good UDP"]
pub mod MAC_RXUDP_GOOD_PACKETS {
    #[doc = "RxUDP Good Packets This field indicates the number of good IP datagrams received with a good UDP payload."]
    pub mod RXUDPGDPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv6 Datagrams Received with UDP Checksum Error"]
pub mod MAC_RXUDP_ERROR_PACKETS {
    #[doc = "RxUDP Error Packets This field indicates the number of good IP datagrams received whose UDP payload has a checksum error."]
    pub mod RXUDPERRPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv6 Datagrams Received with Good TCP Payload"]
pub mod MAC_RXTCP_GOOD_PACKETS {
    #[doc = "RxTCP Good Packets This field indicates the number of good IP datagrams received with a good TCP payload."]
    pub mod RXTCPGDPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv6 Datagrams Received with TCP Checksum Error"]
pub mod MAC_RXTCP_ERROR_PACKETS {
    #[doc = "RxTCP Error Packets This field indicates the number of good IP datagrams received whose TCP payload has a checksum error."]
    pub mod RXTCPERRPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv6 Datagrams Received with Good ICMP Payload"]
pub mod MAC_RXICMP_GOOD_PACKETS {
    #[doc = "RxICMP Good Packets This field indicates the number of good IP datagrams received with a good ICMP payload."]
    pub mod RXICMPGDPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPv6 Datagrams Received with ICMP Checksum Error"]
pub mod MAC_RXICMP_ERROR_PACKETS {
    #[doc = "RxICMP Error Packets This field indicates the number of good IP datagrams received whose ICMP payload has a checksum error."]
    pub mod RXICMPERRPKT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Good Bytes Received in IPv4 Datagrams"]
pub mod MAC_RXIPV4_GOOD_OCTETS {
    #[doc = "RxIPv4 Good Octets This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data."]
    pub mod RXIPV4GDOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in IPv4 Datagrams with Header Errors"]
pub mod MAC_RXIPV4_HEADER_ERROR_OCTETS {
    #[doc = "RxIPv4 Header Error Octets This field indicates the number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch)."]
    pub mod RXIPV4HDRERROCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in IPv4 Datagrams with No Payload"]
pub mod MAC_RXIPV4_NO_PAYLOAD_OCTETS {
    #[doc = "RxIPv4 Payload Octets This field indicates the number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload."]
    pub mod RXIPV4NOPAYOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in Fragmented IPv4 Datagrams"]
pub mod MAC_RXIPV4_FRAGMENTED_OCTETS {
    #[doc = "RxIPv4 Fragmented Octets This field indicates the number of bytes received in fragmented IPv4 datagrams."]
    pub mod RXIPV4FRAGOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received with UDP Checksum Disabled"]
pub mod MAC_RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS {
    #[doc = "RxIPv4 UDP Checksum Disable Octets This field indicates the number of bytes received in a UDP segment that had the UDP checksum disabled."]
    pub mod RXIPV4UDSBLOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in Good IPv6 Datagrams"]
pub mod MAC_RXIPV6_GOOD_OCTETS {
    #[doc = "RxIPv6 Good Octets This field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP, or ICMP data."]
    pub mod RXIPV6GDOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in IPv6 Datagrams with Data Errors"]
pub mod MAC_RXIPV6_HEADER_ERROR_OCTETS {
    #[doc = "RxIPv6 Header Error Octets This field indicates the number of bytes received in IPv6 datagrams with header errors (length, version mismatch)."]
    pub mod RXIPV6HDRERROCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in IPv6 Datagrams with No Payload"]
pub mod MAC_RXIPV6_NO_PAYLOAD_OCTETS {
    #[doc = "RxIPv6 Payload Octets This field indicates the number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload."]
    pub mod RXIPV6NOPAYOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in Good UDP Segment"]
pub mod MAC_RXUDP_GOOD_OCTETS {
    #[doc = "RxUDP Good Octets This field indicates the number of bytes received in a good UDP segment."]
    pub mod RXUDPGDOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in UDP Segment with Checksum Errors"]
pub mod MAC_RXUDP_ERROR_OCTETS {
    #[doc = "RxUDP Error Octets This field indicates the number of bytes received in a UDP segment that had checksum errors."]
    pub mod RXUDPERROCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in Good TCP Segment"]
pub mod MAC_RXTCP_GOOD_OCTETS {
    #[doc = "RxTCP Good Octets This field indicates the number of bytes received in a good TCP segment."]
    pub mod RXTCPGDOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in TCP Segment with Checksum Errors"]
pub mod MAC_RXTCP_ERROR_OCTETS {
    #[doc = "RxTCP Error Octets This field indicates the number of bytes received in a TCP segment that had checksum errors."]
    pub mod RXTCPERROCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in Good ICMP Segment"]
pub mod MAC_RXICMP_GOOD_OCTETS {
    #[doc = "RxICMP Good Octets This field indicates the number of bytes received in a good ICMP segment."]
    pub mod RXICMPGDOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bytes Received in ICMP Segment with Checksum Errors"]
pub mod MAC_RXICMP_ERROR_OCTETS {
    #[doc = "RxICMP Error Octets This field indicates the number of bytes received in a ICMP segment that had checksum errors."]
    pub mod RXICMPERROCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MMC FPE Transmit Interrupt"]
pub mod MAC_MMC_FPE_TX_INTERRUPT {
    #[doc = "MMC Tx FPE Fragment Counter Interrupt status This bit is set when the Tx_FPE_Fragment_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod FCIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Tx FPE Fragment Counter Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Tx FPE Fragment Counter Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Tx Hold Request Counter Interrupt Status This bit is set when the Tx_Hold_Req_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod HRCIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Tx Hold Request Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Tx Hold Request Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "MMC FPE Transmit Mask Interrupt"]
pub mod MAC_MMC_FPE_TX_INTERRUPT_MASK {
    #[doc = "MMC Transmit Fragment Counter Interrupt Mask Setting this bit masks the interrupt when the Tx_FPE_Fragment_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod FCIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Fragment Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Fragment Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Transmit Hold Request Counter Interrupt Mask Setting this bit masks the interrupt when the Tx_Hold_Req_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod HRCIM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Transmit Hold Request Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Transmit Hold Request Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MMC FPE Transmitted Fragment Counter"]
pub mod MAC_MMC_TX_FPE_FRAGMENT_CNTR {
    #[doc = "Tx FPE Fragment counter This field indicates the number of additional mPackets that has been transmitted due to preemption Exists when any one of the RX/TX MMC counters are enabled during FPE Enabled configuration."]
    pub mod TXFFC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MMC FPE Transmitted Hold Request Counter"]
pub mod MAC_MMC_TX_HOLD_REQ_CNTR {
    #[doc = "Tx Hold Request Counter This field indicates count of number of a hold request is given to MAC."]
    pub mod TXHRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MMC FPE Receive Interrupt"]
pub mod MAC_MMC_FPE_RX_INTERRUPT {
    #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Status This bit is set when the Rx_Packet_Assemble_Err_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod PAECIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Rx Packet SMD Error Counter Interrupt Status This bit is set when the Rx_Packet_SMD_Err_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod PSECIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Rx Packet SMD Error Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Rx Packet SMD Error Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Status This bit is set when the Rx_Packet_Assemble_Ok_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod PAOCIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MMC Rx FPE Fragment Counter Interrupt Status This bit is set when the Rx_FPE_Fragment_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod FCIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Rx FPE Fragment Counter Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MMC Rx FPE Fragment Counter Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "MMC FPE Receive Interrupt Mask"]
pub mod MAC_MMC_FPE_RX_INTERRUPT_MASK {
    #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Mask Setting this bit masks the interrupt when the R Rx_Packet_Assemble_Err_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod PAECIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Rx Packet SMD Error Counter Interrupt Mask Setting this bit masks the interrupt when the R Rx_Packet_SMD_Err_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod PSECIM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Rx Packet SMD Error Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Rx Packet SMD Error Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Mask Setting this bit masks the interrupt when the Rx_Packet_Assemble_Ok_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod PAOCIM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "MMC Rx FPE Fragment Counter Interrupt Mask Setting this bit masks the interrupt when the Tx_FPE_Fragment_Cntr counter reaches half of the maximum value or the maximum value."]
    pub mod FCIM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MMC Rx FPE Fragment Counter Interrupt Mask is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MMC Rx FPE Fragment Counter Interrupt Mask is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "MMC Receive Packet Reassembly Error Counter"]
pub mod MAC_MMC_RX_PACKET_ASSEMBLY_ERR_CNTR {
    #[doc = "Rx Packet Assembly Error Counter This field indicates the number of MAC frames with reassembly errors on the Receiver, due to mismatch in the Fragment Count value."]
    pub mod PAEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MMC Receive Packet SMD Error Counter"]
pub mod MAC_MMC_RX_PACKET_SMD_ERR_CNTR {
    #[doc = "Rx Packet SMD Error Counter This field indicates the number of MAC frames rejected due to unknown SMD value and MAC frame fragments rejected due to arriving with an SMD-C when there was no preceding preempted frame."]
    pub mod PSEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MMC Receive Packet Successful Reassembly Counter"]
pub mod MAC_MMC_RX_PACKET_ASSEMBLY_OK_CNTR {
    #[doc = "Rx Packet Assembly OK Counter This field indicates the number of MAC frames that were successfully reassembled and delivered to MAC."]
    pub mod PAOC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MMC FPE Received Fragment Counter"]
pub mod MAC_MMC_RX_FPE_FRAGMENT_CNTR {
    #[doc = "Rx FPE Fragment Counter This field indicates the number of additional mPackets received due to preemption Exists when at least one of the RX/TX MMC counters are enabled during FPE Enabled configuration."]
    pub mod FFC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 and Layer 4 Control of Filter 0"]
pub mod MAC_L3_L4_CONTROL0 {
    #[doc = "Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets."]
    pub mod L3PEN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching."]
    pub mod L3SAM0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching."]
    pub mod L3SAIM0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching."]
    pub mod L3DAM0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching."]
    pub mod L3DAIM0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets."]
    pub mod L3HSBM0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets."]
    pub mod L3HDBM0 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching."]
    pub mod L4PEN0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching."]
    pub mod L4SPM0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching."]
    pub mod L4SPIM0 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching."]
    pub mod L4DPM0 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching."]
    pub mod L4DPIM0 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed."]
    pub mod DMCHN0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter."]
    pub mod DMCHEN0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Select is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Select is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Layer 4 Address 0"]
pub mod MAC_LAYER4_ADDRESS0 {
    #[doc = "Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4SP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4DP0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 0"]
pub mod MAC_LAYER3_ADDR0_REG0 {
    #[doc = "Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[31:0\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A00 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 1 Register 0"]
pub mod MAC_LAYER3_ADDR1_REG0 {
    #[doc = "Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[63:32\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A10 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 2 Register 0"]
pub mod MAC_LAYER3_ADDR2_REG0 {
    #[doc = "Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[95:64\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A20 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 3 Register 0"]
pub mod MAC_LAYER3_ADDR3_REG0 {
    #[doc = "Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[127:96\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A30 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 and Layer 4 Control of Filter 1"]
pub mod MAC_L3_L4_CONTROL1 {
    #[doc = "Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets."]
    pub mod L3PEN1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching."]
    pub mod L3SAM1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching."]
    pub mod L3SAIM1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching."]
    pub mod L3DAM1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching."]
    pub mod L3DAIM1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets."]
    pub mod L3HSBM1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets."]
    pub mod L3HDBM1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching."]
    pub mod L4PEN1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching."]
    pub mod L4SPM1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching."]
    pub mod L4SPIM1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching."]
    pub mod L4DPM1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching."]
    pub mod L4DPIM1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed."]
    pub mod DMCHN1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter."]
    pub mod DMCHEN1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Select is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Select is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Layer 4 Address 0"]
pub mod MAC_LAYER4_ADDRESS1 {
    #[doc = "Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4SP1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4DP1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 1"]
pub mod MAC_LAYER3_ADDR0_REG1 {
    #[doc = "Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[31:0\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A01 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 1 Register 1"]
pub mod MAC_LAYER3_ADDR1_REG1 {
    #[doc = "Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[63:32\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 2 Register 1"]
pub mod MAC_LAYER3_ADDR2_REG1 {
    #[doc = "Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[95:64\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A21 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 3 Register 1"]
pub mod MAC_LAYER3_ADDR3_REG1 {
    #[doc = "Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[127:96\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 and Layer 4 Control of Filter 2"]
pub mod MAC_L3_L4_CONTROL2 {
    #[doc = "Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets."]
    pub mod L3PEN2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching."]
    pub mod L3SAM2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching."]
    pub mod L3SAIM2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching."]
    pub mod L3DAM2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching."]
    pub mod L3DAIM2 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets."]
    pub mod L3HSBM2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets."]
    pub mod L3HDBM2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching."]
    pub mod L4PEN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching."]
    pub mod L4SPM2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching."]
    pub mod L4SPIM2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching."]
    pub mod L4DPM2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching."]
    pub mod L4DPIM2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed."]
    pub mod DMCHN2 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter."]
    pub mod DMCHEN2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Select is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Select is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Layer 4 Address 2"]
pub mod MAC_LAYER4_ADDRESS2 {
    #[doc = "Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4SP2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4DP2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 2"]
pub mod MAC_LAYER3_ADDR0_REG2 {
    #[doc = "Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[31:0\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A02 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 2"]
pub mod MAC_LAYER3_ADDR1_REG2 {
    #[doc = "Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[63:32\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A12 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 2 Register 2"]
pub mod MAC_LAYER3_ADDR2_REG2 {
    #[doc = "Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[95:64\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A22 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 3 Register 2"]
pub mod MAC_LAYER3_ADDR3_REG2 {
    #[doc = "Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[127:96\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A32 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 and Layer 4 Control of Filter 3"]
pub mod MAC_L3_L4_CONTROL3 {
    #[doc = "Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets."]
    pub mod L3PEN3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching."]
    pub mod L3SAM3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching."]
    pub mod L3SAIM3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching."]
    pub mod L3DAM3 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching."]
    pub mod L3DAIM3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets."]
    pub mod L3HSBM3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets."]
    pub mod L3HDBM3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching."]
    pub mod L4PEN3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching."]
    pub mod L4SPM3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching."]
    pub mod L4SPIM3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching."]
    pub mod L4DPM3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching."]
    pub mod L4DPIM3 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed."]
    pub mod DMCHN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter."]
    pub mod DMCHEN3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Select is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Select is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Layer 4 Address 3"]
pub mod MAC_LAYER4_ADDRESS3 {
    #[doc = "Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4SP3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4DP3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 3"]
pub mod MAC_LAYER3_ADDR0_REG3 {
    #[doc = "Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[31:0\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A03 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 1 Register 3"]
pub mod MAC_LAYER3_ADDR1_REG3 {
    #[doc = "Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[63:32\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A13 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 2 Register 3"]
pub mod MAC_LAYER3_ADDR2_REG3 {
    #[doc = "Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[95:64\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A23 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 3 Register 3"]
pub mod MAC_LAYER3_ADDR3_REG3 {
    #[doc = "Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[127:96\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A33 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 and Layer 4 Control of Filter 4"]
pub mod MAC_L3_L4_CONTROL4 {
    #[doc = "Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets."]
    pub mod L3PEN4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching."]
    pub mod L3SAM4 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching."]
    pub mod L3SAIM4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching."]
    pub mod L3DAM4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching."]
    pub mod L3DAIM4 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets."]
    pub mod L3HSBM4 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets."]
    pub mod L3HDBM4 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching."]
    pub mod L4PEN4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching."]
    pub mod L4SPM4 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching."]
    pub mod L4SPIM4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching."]
    pub mod L4DPM4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching."]
    pub mod L4DPIM4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed."]
    pub mod DMCHN4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter."]
    pub mod DMCHEN4 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Select is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Select is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Layer 4 Address 4"]
pub mod MAC_LAYER4_ADDRESS4 {
    #[doc = "Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4SP4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4DP4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 4"]
pub mod MAC_LAYER3_ADDR0_REG4 {
    #[doc = "Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[31:0\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A04 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 1 Register 4"]
pub mod MAC_LAYER3_ADDR1_REG4 {
    #[doc = "Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[63:32\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A14 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 2 Register 4"]
pub mod MAC_LAYER3_ADDR2_REG4 {
    #[doc = "Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[95:64\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A24 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 3 Register 4"]
pub mod MAC_LAYER3_ADDR3_REG4 {
    #[doc = "Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[127:96\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A34 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 and Layer 4 Control of Filter 5"]
pub mod MAC_L3_L4_CONTROL5 {
    #[doc = "Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets."]
    pub mod L3PEN5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching."]
    pub mod L3SAM5 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching."]
    pub mod L3SAIM5 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching."]
    pub mod L3DAM5 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching."]
    pub mod L3DAIM5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets."]
    pub mod L3HSBM5 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets."]
    pub mod L3HDBM5 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching."]
    pub mod L4PEN5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching."]
    pub mod L4SPM5 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching."]
    pub mod L4SPIM5 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching."]
    pub mod L4DPM5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching."]
    pub mod L4DPIM5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed."]
    pub mod DMCHN5 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter."]
    pub mod DMCHEN5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Select is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Select is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Layer 4 Address 5"]
pub mod MAC_LAYER4_ADDRESS5 {
    #[doc = "Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4SP5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4DP5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 5"]
pub mod MAC_LAYER3_ADDR0_REG5 {
    #[doc = "Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[31:0\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A05 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 1 Register 5"]
pub mod MAC_LAYER3_ADDR1_REG5 {
    #[doc = "Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[63:32\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 2 Register 5"]
pub mod MAC_LAYER3_ADDR2_REG5 {
    #[doc = "Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[95:64\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A25 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 3 Register 5"]
pub mod MAC_LAYER3_ADDR3_REG5 {
    #[doc = "Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[127:96\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A35 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 and Layer 4 Control of Filter 6"]
pub mod MAC_L3_L4_CONTROL6 {
    #[doc = "Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets."]
    pub mod L3PEN6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching."]
    pub mod L3SAM6 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching."]
    pub mod L3SAIM6 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching."]
    pub mod L3DAM6 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching."]
    pub mod L3DAIM6 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets."]
    pub mod L3HSBM6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets."]
    pub mod L3HDBM6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching."]
    pub mod L4PEN6 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching."]
    pub mod L4SPM6 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching."]
    pub mod L4SPIM6 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching."]
    pub mod L4DPM6 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching."]
    pub mod L4DPIM6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed."]
    pub mod DMCHN6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter."]
    pub mod DMCHEN6 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Select is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Select is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Layer 4 Address 6"]
pub mod MAC_LAYER4_ADDRESS6 {
    #[doc = "Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4SP6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4DP6 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 6"]
pub mod MAC_LAYER3_ADDR0_REG6 {
    #[doc = "Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[31:0\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A06 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 1 Register 6"]
pub mod MAC_LAYER3_ADDR1_REG6 {
    #[doc = "Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[63:32\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 2 Register 6"]
pub mod MAC_LAYER3_ADDR2_REG6 {
    #[doc = "Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[95:64\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A26 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 3 Register 6"]
pub mod MAC_LAYER3_ADDR3_REG6 {
    #[doc = "Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[127:96\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A36 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 and Layer 4 Control of Filter 0"]
pub mod MAC_L3_L4_CONTROL7 {
    #[doc = "Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets."]
    pub mod L3PEN7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching."]
    pub mod L3SAM7 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching."]
    pub mod L3SAIM7 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP SA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP SA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching."]
    pub mod L3DAM7 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching."]
    pub mod L3DAIM7 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 3 IP DA Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 3 IP DA Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets."]
    pub mod L3HSBM7 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets."]
    pub mod L3HDBM7 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching."]
    pub mod L4PEN7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Protocol is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Protocol is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching."]
    pub mod L4SPM7 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching."]
    pub mod L4SPIM7 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Source Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Source Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching."]
    pub mod L4DPM7 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching."]
    pub mod L4DPIM7 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Layer 4 Destination Port Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Layer 4 Destination Port Inverse Match is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed."]
    pub mod DMCHN7 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter."]
    pub mod DMCHEN7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel Select is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DMA Channel Select is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Layer 4 Address 7"]
pub mod MAC_LAYER4_ADDRESS7 {
    #[doc = "Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4SP7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets."]
    pub mod L4DP7 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 0 Register 7"]
pub mod MAC_LAYER3_ADDR0_REG7 {
    #[doc = "Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[31:0\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A07 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 1 Register 7"]
pub mod MAC_LAYER3_ADDR1_REG7 {
    #[doc = "Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[63:32\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A17 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 2 Register 7"]
pub mod MAC_LAYER3_ADDR2_REG7 {
    #[doc = "Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[95:64\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A27 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer 3 Address 3 Register 7"]
pub mod MAC_LAYER3_ADDR3_REG7 {
    #[doc = "Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\\[127:96\\] of the IP Source Address field in the IPv6 packets."]
    pub mod L3A37 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Control"]
pub mod MAC_TIMESTAMP_CONTROL {
    #[doc = "Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
    pub mod TSENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
    pub mod TSCFUPDT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Coarse method is used to update system timestamp"]
            pub const COARSE: u32 = 0;
            #[doc = "Fine method is used to update system timestamp"]
            pub const FINE: u32 = 0x01;
        }
    }
    #[doc = "Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC_System_Time_Seconds_Update and MAC_System_Time_Nanoseconds_Update registers."]
    pub mod TSINIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp is not initialized"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp is initialized"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC_System_Time_Seconds_Update and MAC_System_Time_Nanoseconds_Update registers."]
    pub mod TSUPDT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp is not updated"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp is updated"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
    pub mod TSADDREG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Addend Register is not updated"]
            pub const DISABLE: u32 = 0;
            #[doc = "Addend Register is updated"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Presentation Time Generation Enable When this bit is set the Presentation Time generation will be enabled."]
    pub mod PTGE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Presentation Time Generation is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Presentation Time Generation is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    pub mod TSENALL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp for All Packets disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp for All Packets enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9A_C9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
    pub mod TSCTRLSSR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Digital or Binary Rollover Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Digital or Binary Rollover Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
    pub mod TSVER2ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PTP Packet Processing for Version 2 Format is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PTP Packet Processing for Version 2 Format is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
    pub mod TSIPENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing of PTP over Ethernet Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Processing of PTP over Ethernet Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
    pub mod TSIPV6ENA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing of PTP Packets Sent over IPv6-UDP is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Processing of PTP Packets Sent over IPv6-UDP is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
    pub mod TSIPV4ENA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing of PTP Packets Sent over IPv4-UDP is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Processing of PTP Packets Sent over IPv4-UDP is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
    pub mod TSEVNTENA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Snapshot for Event Messages is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Snapshot for Event Messages is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
    pub mod TSMSTRENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Snapshot for Messages Relevant to Master is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Snapshot for Messages Relevant to Master is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
    pub mod SNAPTYPSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
    pub mod TSENMACADDR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC Address for PTP Packet Filtering is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MAC Address for PTP Packet Filtering is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable checksum correction during OST for PTP over UDP/IPv4 packets When this bit is set, the last two bytes of PTP message sent over UDP/IPv4 is updated to keep the UDP checksum correct, for changes made to origin timestamp and/or correction field as part of one step timestamp operation."]
    pub mod CSC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "checksum correction during OST for PTP over UDP/IPv4 packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "checksum correction during OST for PTP over UDP/IPv4 packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "External System Time Input When this bit is set, the MAC uses the external 64-bit reference System Time input for the following: - To take the timestamp provided as status - To insert the timestamp in transmit PTP packets when One-step Timestamp or Timestamp Offload feature is enabled."]
    pub mod ESTI {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External System Time Input is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "External System Time Input is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
    pub mod TXTSSTSM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Timestamp Status Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Timestamp Status Mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "AV 802."]
    pub mod AV8021ASMEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AV 802.1AS Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "AV 802.1AS Mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Subsecond Increment"]
pub mod MAC_SUB_SECOND_INCREMENT {
    #[doc = "Sub-nanosecond Increment Value This field contains the sub-nanosecond increment value, represented in nanoseconds multiplied by 2^8."]
    pub mod SNSINC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sub-second Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the sub-second register."]
    pub mod SSINC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time Seconds"]
pub mod MAC_SYSTEM_TIME_SECONDS {
    #[doc = "Timestamp Second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
    pub mod TSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time Nanoseconds"]
pub mod MAC_SYSTEM_TIME_NANOSECONDS {
    #[doc = "Timestamp Sub Seconds The value in this field has the sub-second representation of time, with an accuracy of 0."]
    pub mod TSSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time Seconds Update"]
pub mod MAC_SYSTEM_TIME_SECONDS_UPDATE {
    #[doc = "Timestamp Seconds The value in this field is the seconds part of the update."]
    pub mod TSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time Nanoseconds Update"]
pub mod MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {
    #[doc = "Timestamp Sub Seconds The value in this field is the sub-seconds part of the update."]
    pub mod TSSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register."]
    pub mod ADDSUB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Add time"]
            pub const ADD: u32 = 0;
            #[doc = "Subtract time"]
            pub const SUB: u32 = 0x01;
        }
    }
}
#[doc = "Timestamp Addend"]
pub mod MAC_TIMESTAMP_ADDEND {
    #[doc = "Timestamp Addend Register This field indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
    pub mod TSAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time - Higher Word Seconds"]
pub mod MAC_SYSTEM_TIME_HIGHER_WORD_SECONDS {
    #[doc = "Timestamp Higher Word Register This field contains the most-significant 16-bits of timestamp seconds value."]
    pub mod TSHWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Status"]
pub mod MAC_TIMESTAMP_STATUS {
    #[doc = "Timestamp Seconds Overflow When this bit is set, it indicates that the seconds value of the timestamp (when supporting version 2 format) has overflowed beyond 32'hFFFF_FFFF."]
    pub mod TSSOVF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Seconds Overflow status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Seconds Overflow status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the MAC_PPS0_Target_Time_Seconds and MAC_PPS0_Target_Time_Nanoseconds registers."]
    pub mod TSTARGT0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Target Time Reached status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Reached status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO."]
    pub mod AUXTSTRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auxiliary Timestamp Trigger Snapshot status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Auxiliary Timestamp Trigger Snapshot status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Target Time Error This bit is set when the latest target time programmed in the MAC_PPS0_Target_Time_Seconds and MAC_PPS0_Target_Time_Nanoseconds registers elapses."]
    pub mod TSTRGTERR0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Target Time Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Target Time Reached for Target Time PPS1 When set, this bit indicates that the value of system time is greater than or equal to the value specified in the MAC_PPS1_TARGET_TIME_SECONDS and MAC_PPS1_TARGET_TIME_NANOSECONDS registers."]
    pub mod TSTARGT1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Target Time Reached for Target Time PPS1 status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Reached for Target Time PPS1 status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Target Time Error This bit is set when the latest target time programmed in the MAC_PPS1_TARGET_TIME_SECONDS and MAC_PPS1_TARGET_TIME_NANOSECONDS registers elapses."]
    pub mod TSTRGTERR1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Target Time Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Target Time Reached for Target Time PPS2 When set, this bit indicates that the value of system time is greater than or equal to the value specified in the MAC_PPS2_TARGET_TIME_SECONDS and MAC_PPS2_TARGET_TIME_NANOSECONDS registers."]
    pub mod TSTARGT2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Target Time Reached for Target Time PPS2 status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Reached for Target Time PPS2 status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Target Time Error This bit is set when the latest target time programmed in the MAC_PPS2_TARGET_TIME_SECONDS and MAC_PPS2_TARGET_TIME_NANOSECONDS registers elapses."]
    pub mod TSTRGTERR2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Target Time Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Target Time Reached for Target Time PPS3 When this bit is set, it indicates that the value of system time is greater than or equal to the value specified in the MAC_PPS3_TARGET_TIME_SECONDS and MAC_PPS3_TARGET_TIME_NANOSECONDS registers."]
    pub mod TSTARGT3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Target Time Reached for Target Time PPS3 status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Reached for Target Time PPS3 status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Timestamp Target Time Error This bit is set when the latest target time programmed in the MAC_PPS3_TARGET_TIME_SECONDS and MAC_PPS3_TARGET_TIME_NANOSECONDS registers elapses."]
    pub mod TSTRGTERR3 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Target Time Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Tx Timestamp Status Interrupt Status In non-EQOS_CORE configurations when drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the MAC_TX_TIMESTAMP_STATUS_NANOSECONDS and MAC_TX_TIMESTAMP_STATUS_SECONDS registers."]
    pub mod TXTSSIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tx Timestamp Status Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Tx Timestamp Status Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable."]
    pub mod ATSSTN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set."]
    pub mod ATSSTM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auxiliary Timestamp Snapshot Trigger Missed status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Auxiliary Timestamp Snapshot Trigger Missed status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO."]
    pub mod ATSNS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Timestamp Status Nanoseconds"]
pub mod MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
    #[doc = "Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp."]
    pub mod TXTSSLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Timestamp Status Missed When this bit is set, it indicates one of the following: - The timestamp of the current packet is ignored if TXTSSTSM bit of the TIMESTAMP_CONTROL register is reset - The timestamp of the previous packet is overwritten with timestamp of the current packet if TXTSSTSM bit of the MAC_TIMESTAMP_CONTROL register is set."]
    pub mod TXTSSMIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Timestamp Status Missed status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Timestamp Status Missed status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Transmit Timestamp Status Seconds"]
pub mod MAC_TX_TIMESTAMP_STATUS_SECONDS {
    #[doc = "Transmit Timestamp Status High This field contains the lower 32 bits of the Seconds field of Transmit packet's captured timestamp."]
    pub mod TXTSSHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Auxiliary Timestamp Control"]
pub mod MAC_AUXILIARY_CONTROL {
    #[doc = "Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO."]
    pub mod ATSFC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auxiliary Snapshot FIFO Clear is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Auxiliary Snapshot FIFO Clear is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0."]
    pub mod ATSEN0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auxiliary Snapshot $i is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Auxiliary Snapshot $i is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1."]
    pub mod ATSEN1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auxiliary Snapshot $i is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Auxiliary Snapshot $i is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2."]
    pub mod ATSEN2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auxiliary Snapshot $i is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Auxiliary Snapshot $i is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3."]
    pub mod ATSEN3 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auxiliary Snapshot $i is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Auxiliary Snapshot $i is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Auxiliary Timestamp Nanoseconds"]
pub mod MAC_AUXILIARY_TIMESTAMP_NANOSECONDS {
    #[doc = "Auxiliary Timestamp Contains the lower 31 bits (nanoseconds field) of the auxiliary timestamp."]
    pub mod AUXTSLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Auxiliary Timestamp Seconds"]
pub mod MAC_AUXILIARY_TIMESTAMP_SECONDS {
    #[doc = "Auxiliary Timestamp Contains the lower 32 bits of the Seconds field of the auxiliary timestamp."]
    pub mod AUXTSHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Ingress Asymmetry Correction"]
pub mod MAC_TIMESTAMP_INGRESS_ASYM_CORR {
    #[doc = "One-Step Timestamp Ingress Asymmetry Correction This field contains the ingress path asymmetry value to be added to correctionField of Pdelay_Resp PTP packet."]
    pub mod OSTIAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "imestamp Egress Asymmetry Correction"]
pub mod MAC_TIMESTAMP_EGRESS_ASYM_CORR {
    #[doc = "One-Step Timestamp Egress Asymmetry Correction This field contains the egress path asymmetry value to be subtracted from correctionField of Pdelay_Resp PTP packet."]
    pub mod OSTEAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Ingress Correction Nanosecond"]
pub mod MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND {
    #[doc = "Timestamp Ingress Correction This field contains the ingress path correction value as defined by the Ingress Correction expression."]
    pub mod TSIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Egress Correction Nanosecond"]
pub mod MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND {
    #[doc = "Timestamp Egress Correction This field contains the nanoseconds part of the egress path correction value as defined by the Egress Correction expression."]
    pub mod TSEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Ingress Correction Subnanosecond"]
pub mod MAC_TIMESTAMP_INGRESS_CORR_SUBNANOSEC {
    #[doc = "Timestamp Ingress Correction, sub-nanoseconds This field contains the sub-nanoseconds part of the ingress path correction value as defined by the \"Ingress Correction\" expression."]
    pub mod TSICSNS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Egress Correction Subnanosecond"]
pub mod MAC_TIMESTAMP_EGRESS_CORR_SUBNANOSEC {
    #[doc = "Timestamp Egress Correction, sub-nanoseconds This field contains the sub-nanoseconds part of the egress path correction value as defined by the \"Egress Correction\" expression."]
    pub mod TSECSNS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Ingress Latency"]
pub mod MAC_TIMESTAMP_INGRESS_LATENCY {
    #[doc = "Ingress Timestamp Latency, in nanoseconds This register holds the average latency in nanoseconds between the input ports (phy_rxd_i) of MAC and the actual point (GMII/MII) where the ingress timestamp is taken."]
    pub mod ITLSNS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress Timestamp Latency, in sub-nanoseconds This register holds the average latency in sub-nanoseconds between the input ports (phy_rxd_i) of MAC and the actual point (GMII/MII) where the ingress timestamp is taken."]
    pub mod ITLNS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Egress Latency"]
pub mod MAC_TIMESTAMP_EGRESS_LATENCY {
    #[doc = "Egress Timestamp Latency, in sub-nanoseconds This register holds the average latency in sub-nanoseconds between the actual point (GMII/MII) where the egress timestamp is taken and the output ports (phy_txd_o) of the MAC."]
    pub mod ETLSNS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Timestamp Latency, in nanoseconds This register holds the average latency in nanoseconds between the actual point (GMII/MII) where the egress timestamp is taken and the output ports (phy_txd_o) of the MAC."]
    pub mod ETLNS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS Control"]
pub mod MAC_PPS_CONTROL {
    #[doc = "PPS Output Frequency Control This field controls the frequency of the PPS0 output (ptp_pps_o\\[0\\]) signal."]
    pub mod PPSCTRL_PPSCMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flexible PPS Output Mode Enable When this bit is set, Bits\\[3:0\\] function as PPSCMD."]
    pub mod PPSEN0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flexible PPS Output Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flexible PPS Output Mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Target Time Register Mode for PPS0 Output This field indicates the Target Time registers (MAC_PPS0_TARGET_TIME_SECONDS and MAC_PPS0_TARGET_TIME_NANOSECONDS) mode for PPS0 output signal:"]
    pub mod TRGTMODSEL0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target Time registers are programmed only for generating the interrupt event. The Flexible PPS function must not be enabled in this mode, otherwise spurious transitions may be observed on the corresponding ptp_pps_o output port"]
            pub const ONLY_INT: u32 = 0;
            #[doc = "Target Time registers are programmed for generating the interrupt event and starting or stopping the PPS0 output signal generation"]
            pub const INT_ST: u32 = 0x02;
            #[doc = "Target Time registers are programmed only for starting or stopping the PPS0 output signal generation. No interrupt is asserted"]
            pub const ONLY_ST: u32 = 0x03;
        }
    }
    #[doc = "MCGR Mode Enable for PPS0 Output This field enables the 0th PPS instance to operate in PPS or MCGR mode."]
    pub mod MCGREN0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0th PPS instance is enabled to operate in PPS mode"]
            pub const PPS: u32 = 0;
            #[doc = "0th PPS instance is enabled to operate in MCGR mode"]
            pub const MCGR: u32 = 0x01;
        }
    }
    #[doc = "Flexible PPS1 Output Control This field controls the flexible PPS1 output (ptp_pps_o\\[1\\]) signal."]
    pub mod PPSCMD1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Time Register Mode for PPS1 Output This field indicates the Target Time registers (MAC_PPS1_TARGET_TIME_SECONDS and MAC_PPS1_TARGET_TIME_NANOSECONDS) mode for PPS1 output signal."]
    pub mod TRGTMODSEL1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target Time registers are programmed only for generating the interrupt event. The Flexible PPS function must not be enabled in this mode, otherwise spurious transitions may be observed on the corresponding ptp_pps_o output port"]
            pub const ONLY_INT: u32 = 0;
            #[doc = "Target Time registers are programmed for generating the interrupt event and starting or stopping the PPS0 output signal generation"]
            pub const INT_ST: u32 = 0x02;
            #[doc = "Target Time registers are programmed only for starting or stopping the PPS0 output signal generation. No interrupt is asserted"]
            pub const ONLY_ST: u32 = 0x03;
        }
    }
    #[doc = "MCGR Mode Enable for PPS1 Output This field enables the 1st PPS instance to operate in PPS or MCGR mode."]
    pub mod MCGREN1 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1st PPS instance is disabled to operate in PPS or MCGR mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "1st PPS instance is enabled to operate in PPS or MCGR mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Flexible PPS2 Output Control This field controls the flexible PPS2 output (ptp_pps_o\\[2\\]) signal."]
    pub mod PPSCMD2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Time Register Mode for PPS2 Output This field indicates the Target Time registers (MAC_PPS2_TARGET_TIME_SECONDS and MAC_PPS2_TARGET_TIME_NANOSECONDS) mode for PPS2 output signal."]
    pub mod TRGTMODSEL2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target Time registers are programmed only for generating the interrupt event. The Flexible PPS function must not be enabled in this mode, otherwise spurious transitions may be observed on the corresponding ptp_pps_o output port"]
            pub const ONLY_INT: u32 = 0;
            #[doc = "Target Time registers are programmed for generating the interrupt event and starting or stopping the PPS0 output signal generation"]
            pub const INT_ST: u32 = 0x02;
            #[doc = "Target Time registers are programmed only for starting or stopping the PPS0 output signal generation. No interrupt is asserted"]
            pub const ONLY_ST: u32 = 0x03;
        }
    }
    #[doc = "MCGR Mode Enable for PPS2 Output This field enables the 2nd PPS instance to operate in PPS or MCGR mode."]
    pub mod MCGREN2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2nd PPS instance is disabled to operate in PPS or MCGR mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "2nd PPS instance is enabled to operate in PPS or MCGR mode"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Flexible PPS3 Output Control This field controls the flexible PPS3 output (ptp_pps_o\\[3\\]) signal."]
    pub mod PPSCMD3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Time Register Mode for PPS3 Output This field indicates the Target Time registers (MAC_PPS3_TARGET_TIME_SECONDS and MAC_PPS3_TARGET_TIME_NANOSECONDS) mode for PPS3 output signal."]
    pub mod TRGTMODSEL3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target Time registers are programmed only for generating the interrupt event. The Flexible PPS function must not be enabled in this mode, otherwise spurious transitions may be observed on the corresponding ptp_pps_o output port"]
            pub const ONLY_INT: u32 = 0;
            #[doc = "Target Time registers are programmed for generating the interrupt event and starting or stopping the PPS0 output signal generation"]
            pub const INT_ST: u32 = 0x02;
            #[doc = "Target Time registers are programmed only for starting or stopping the PPS0 output signal generation. No interrupt is asserted"]
            pub const ONLY_ST: u32 = 0x03;
        }
    }
    #[doc = "MCGR Mode Enable for PPS3 Output This field enables the 3rd PPS instance to operate in PPS or MCGR mode."]
    pub mod MCGREN3 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS0 Target Time Seconds"]
pub mod MAC_PPS0_TARGET_TIME_SECONDS {
    #[doc = "PPS Target Time Seconds Register This field stores the time in seconds."]
    pub mod TSTRH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS0 Target Time Nanoseconds"]
pub mod MAC_PPS0_TARGET_TIME_NANOSECONDS {
    #[doc = "Target Time Low for PPS Register This register stores the time in (signed) nanoseconds."]
    pub mod TTSL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS_CONTROL register is programmed to 010 or 011."]
    pub mod TRGTBUSY0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PPS Target Time Register Busy status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PPS Target Time Register Busy is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "PPS0 Interval"]
pub mod MAC_PPS0_INTERVAL {
    #[doc = "PPS Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output."]
    pub mod PPSINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS0 Width"]
pub mod MAC_PPS0_WIDTH {
    #[doc = "PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS0 signal output."]
    pub mod PPSWIDTH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS1 Target Time Seconds"]
pub mod MAC_PPS1_TARGET_TIME_SECONDS {
    #[doc = "PPS Target Time Seconds Register This field stores the time in seconds."]
    pub mod TSTRH1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS1 Target Time Nanoseconds"]
pub mod MAC_PPS1_TARGET_TIME_NANOSECONDS {
    #[doc = "Target Time Low for PPS Register This register stores the time in (signed) nanoseconds."]
    pub mod TTSL1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS_CONTROL register is programmed to 010 or 011."]
    pub mod TRGTBUSY1 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PPS Target Time Register Busy status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PPS Target Time Register Busy is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "PPS1 Interval"]
pub mod MAC_PPS1_INTERVAL {
    #[doc = "PPS Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output."]
    pub mod PPSINT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS1 Width"]
pub mod MAC_PPS1_WIDTH {
    #[doc = "PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS0 signal output."]
    pub mod PPSWIDTH1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS2 Target Time Seconds"]
pub mod MAC_PPS2_TARGET_TIME_SECONDS {
    #[doc = "PPS Target Time Seconds Register This field stores the time in seconds."]
    pub mod TSTRH2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS2 Target Time Nanoseconds"]
pub mod MAC_PPS2_TARGET_TIME_NANOSECONDS {
    #[doc = "Target Time Low for PPS Register This register stores the time in (signed) nanoseconds."]
    pub mod TTSL2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS_CONTROL register is programmed to 010 or 011."]
    pub mod TRGTBUSY2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PPS Target Time Register Busy status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PPS Target Time Register Busy is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "PPS2 Interval"]
pub mod MAC_PPS2_INTERVAL {
    #[doc = "PPS Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output."]
    pub mod PPSINT2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS2 Width"]
pub mod MAC_PPS2_WIDTH {
    #[doc = "PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS0 signal output."]
    pub mod PPSWIDTH2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS3 Target Time Seconds"]
pub mod MAC_PPS3_TARGET_TIME_SECONDS {
    #[doc = "PPS Target Time Seconds Register This field stores the time in seconds."]
    pub mod TSTRH3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS3 Target Time Nanoseconds"]
pub mod MAC_PPS3_TARGET_TIME_NANOSECONDS {
    #[doc = "Target Time Low for PPS Register This register stores the time in (signed) nanoseconds."]
    pub mod TTSL3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS_CONTROL register is programmed to 010 or 011."]
    pub mod TRGTBUSY3 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PPS Target Time Register Busy status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PPS Target Time Register Busy is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "PPS3 Interval"]
pub mod MAC_PPS3_INTERVAL {
    #[doc = "PPS Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output."]
    pub mod PPSINT3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS3 Width"]
pub mod MAC_PPS3_WIDTH {
    #[doc = "PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS0 signal output."]
    pub mod PPSWIDTH3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PTP Offload Engine Control"]
pub mod MAC_PTO_CONTROL {
    #[doc = "PTP Offload Enable When this bit is set, the PTP Offload feature is enabled."]
    pub mod PTOEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PTP Offload feature is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PTP Offload feature is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Automatic PTP SYNC message Enable When this bit is set, PTP SYNC message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Clock Master mode."]
    pub mod ASYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic PTP SYNC message is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic PTP SYNC message is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Automatic PTP Pdelay_Req message Enable When this bit is set, PTP Pdelay_Req message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Peer-to-Peer Transparent mode."]
    pub mod APDREQEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic PTP Pdelay_Req message is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic PTP Pdelay_Req message is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Automatic PTP SYNC message Trigger When this bit is set, one PTP SYNC message is transmitted."]
    pub mod ASYNCTRIG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic PTP SYNC message Trigger is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic PTP SYNC message Trigger is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Automatic PTP Pdelay_Req message Trigger When this bit is set, one PTP Pdelay_Req message is transmitted."]
    pub mod APDREQTRIG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic PTP Pdelay_Req message Trigger is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic PTP Pdelay_Req message Trigger is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable PTO Delay Request/Response response generation When this bit is set, the Delay Request and Delay response is not generated for received SYNC and Delay request packet respectively, as required by the programmed mode."]
    pub mod DRRDIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PTO Delay Request/Response response generation is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "PTO Delay Request/Response response generation is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Peer Delay Response response generation When this bit is set, the Peer Delay Response (Pdelay_Resp) response is not be generated for received Peer Delay Request (Pdelay_Req) request packet, as required by the programmed mode."]
    pub mod PDRDIS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Peer Delay Response response generation is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Peer Delay Response response generation is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Domain Number This field indicates the domain Number in which the PTP node is operating."]
    pub mod DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Source Port Identity 0"]
pub mod MAC_SOURCE_PORT_IDENTITY0 {
    #[doc = "Source Port Identity 0 This field indicates bits \\[31:0\\] of sourcePortIdentity of PTP node."]
    pub mod SPI0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Source Port Identity 1"]
pub mod MAC_SOURCE_PORT_IDENTITY1 {
    #[doc = "Source Port Identity 1 This field indicates bits \\[63:32\\] of sourcePortIdentity of PTP node."]
    pub mod SPI1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Source Port Identity 2"]
pub mod MAC_SOURCE_PORT_IDENTITY2 {
    #[doc = "Source Port Identity 2 This field indicates bits \\[79:64\\] of sourcePortIdentity of PTP node."]
    pub mod SPI2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Log Message Interval"]
pub mod MAC_LOG_MESSAGE_INTERVAL {
    #[doc = "Log Sync Interval This field indicates the periodicity of the automatically generated SYNC message when the PTP node is Master."]
    pub mod LSI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay_Req to SYNC Ratio In Slave mode, it is used for controlling frequency of Delay_Req messages transmitted."]
    pub mod DRSYNCR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DelayReq generated for every received SYNC"]
            pub const SYNC1: u32 = 0;
            #[doc = "DelayReq generated every alternate reception of SYNC"]
            pub const SYNC2: u32 = 0x01;
            #[doc = "for every 4 SYNC messages"]
            pub const SYNC4: u32 = 0x02;
            #[doc = "for every 8 SYNC messages"]
            pub const SYNC8: u32 = 0x03;
            #[doc = "for every 16 SYNC messages"]
            pub const SYNC16: u32 = 0x04;
            #[doc = "for every 32 SYNC messages"]
            pub const SYNC32: u32 = 0x05;
        }
    }
    #[doc = "Log Min Pdelay_Req Interval This field indicates logMinPdelayReqInterval of PTP node."]
    pub mod LMPDRI {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MTL Operation Mode"]
pub mod MTL_OPERATION_MODE {
    #[doc = "Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
    pub mod DTXSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Drop Transmit Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Drop Transmit Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Arbitration Algorithm This field is used to select the arbitration algorithm for the Rx side."]
    pub mod RAA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Strict priority (SP)"]
            pub const SP: u32 = 0;
            #[doc = "Weighted Strict Priority (WSP)"]
            pub const WSP: u32 = 0x01;
        }
    }
    #[doc = "Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling:"]
    pub mod SCHALG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WRR algorithm"]
            pub const WRR: u32 = 0;
            #[doc = "WFQ algorithm when DCB feature is selected.Otherwise, Reserved"]
            pub const WFQ: u32 = 0x01;
            #[doc = "DWRR algorithm when DCB feature is selected.Otherwise, Reserved"]
            pub const DWRR: u32 = 0x02;
            #[doc = "Strict priority algorithm"]
            pub const SP: u32 = 0x03;
        }
    }
    #[doc = "Counters Preset When this bit is set, - MTL_TxQ\\[0-7\\]_Underflow register is initialized/preset to 12'h7F0."]
    pub mod CNTPRST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counters Preset is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Counters Preset is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Counters Reset When this bit is set, all counters are reset."]
    pub mod CNTCLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counters are not reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "All counters are reset"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Flexible Rx parser Enable When this bit is set to 1, the Programmable Rx Parser functionality is enabled."]
    pub mod FRPE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flexible Rx parser is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flexible Rx parser is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "FIFO Debug Access Control and Status"]
pub mod MTL_DBG_CTL {
    #[doc = "FIFO Debug Access Enable When this bit is set, it indicates that the debug mode access to the FIFO is enabled."]
    pub mod FDBGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO Debug Access is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "FIFO Debug Access is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Debug Mode Access to FIFO When this bit is set, it indicates that the current access to the FIFO is read, write, and debug access."]
    pub mod DBGMOD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Debug Mode Access to FIFO is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Debug Mode Access to FIFO is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Byte Enables This field indicates the number of data bytes valid in the data register during Write operation."]
    pub mod BYTEEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte 0 valid"]
            pub const B0_VAL: u32 = 0;
            #[doc = "Byte 0 and Byte 1 are valid"]
            pub const B01_VAL: u32 = 0x01;
            #[doc = "Byte 0, Byte 1, and Byte 2 are valid"]
            pub const B012_VAL: u32 = 0x02;
            #[doc = "All four bytes are valid"]
            pub const B0123_VAL: u32 = 0x03;
        }
    }
    #[doc = "Encoded Packet State This field is used to write the control information to the Tx FIFO or Rx FIFO."]
    pub mod PKTSTATE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet Data"]
            pub const PKT_DATA: u32 = 0;
            #[doc = "Control Word/Normal Status"]
            pub const CW_NS: u32 = 0x01;
            #[doc = "SOP Data/Last Status"]
            pub const SOP_LS: u32 = 0x02;
            #[doc = "EOP Data/EOP"]
            pub const EOP: u32 = 0x03;
        }
    }
    #[doc = "Reset All Pointers When this bit is set, the pointers of all FIFOs are reset when FIFO Debug Access is enabled."]
    pub mod RSTALL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset All Pointers is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Reset All Pointers is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Reset Pointers of Selected FIFO When this bit is set, the pointers of the currently-selected FIFO are reset when FIFO Debug Access is enabled."]
    pub mod RSTSEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset Pointers of Selected FIFO is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Reset Pointers of Selected FIFO is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FIFO Read Enable When this bit is set, it enables the Read operation on selected FIFO when FIFO Debug Access is enabled."]
    pub mod FIFORDEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO Read is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "FIFO Read is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FIFO Write Enable When this bit is set, it enables the Write operation on selected FIFO when FIFO Debug Access is enabled."]
    pub mod FIFOWREN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO Write is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "FIFO Write is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "FIFO Selected for Access This field indicates the FIFO selected for debug access:"]
    pub mod FIFOSEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tx FIFO"]
            pub const TXFIFO: u32 = 0;
            #[doc = "Tx Status FIFO (only read access when SLVMOD is set)"]
            pub const TXSTSFIFO: u32 = 0x01;
            #[doc = "TSO FIFO (cannot be accessed when SLVMOD is set)"]
            pub const TSOFIFO: u32 = 0x02;
            #[doc = "Rx FIFO"]
            pub const RXFIFO: u32 = 0x03;
        }
    }
    #[doc = "Receive Packet Available Interrupt Status Enable When this bit is set, an interrupt is generated when EOP of received packet is written to the Rx FIFO."]
    pub mod PKTIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Packet Available Interrupt Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Packet Available Interrupt Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Status Available Interrupt Status Enable When this bit is set, an interrupt is generated when Transmit status is available in slave mode."]
    pub mod STSIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Packet Available Interrupt Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Packet Available Interrupt Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "FIFO Debug Status"]
pub mod MTL_DBG_STS {
    #[doc = "FIFO Busy When set, this bit indicates that a FIFO operation is in progress in the MAC and content of the following fields is not valid: - All other fields of this register - All fields of the MTL_FIFO_DEBUG_DATA register"]
    pub mod FIFOBUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO Busy not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "FIFO Busy detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Encoded Packet State This field is used to get the control or status information of the selected FIFO."]
    pub mod PKTSTATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet Data"]
            pub const PKT_DATA: u32 = 0;
            #[doc = "Control Word/Normal Status"]
            pub const CW_NS: u32 = 0x01;
            #[doc = "SOP Data/Last Status"]
            pub const SOP_LS: u32 = 0x02;
            #[doc = "EOP Data/EOP"]
            pub const EOP: u32 = 0x03;
        }
    }
    #[doc = "Byte Enables This field indicates the number of data bytes valid in the data register during Read operation."]
    pub mod BYTEEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte 0 valid"]
            pub const B0_VAL: u32 = 0;
            #[doc = "Byte 0 and Byte 1 are valid"]
            pub const B01_VAL: u32 = 0x01;
            #[doc = "Byte 0, Byte 1, and Byte 2 are valid"]
            pub const B012_VAL: u32 = 0x02;
            #[doc = "All four bytes are valid"]
            pub const B0123_VAL: u32 = 0x03;
        }
    }
    #[doc = "Receive Packet Available Interrupt Status When set, this bit indicates that MAC layer has written the EOP of received packet to the Rx FIFO."]
    pub mod PKTI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Packet Available Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Packet Available Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Status Available Interrupt Status When set, this bit indicates that the Slave mode Tx packet is transmitted, and the status is available in Tx Status FIFO."]
    pub mod STSI {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Status Available Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Status Available Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Remaining Locations in the FIFO Slave Access Mode: This field indicates the space available in selected FIFO."]
    pub mod LOCR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO Debug Data"]
pub mod MTL_FIFO_DEBUG_DATA {
    #[doc = "FIFO Debug Data During debug or slave access write operation, this field contains the data to be written to the Tx FIFO, Rx FIFO, or TSO FIFO."]
    pub mod FDBGDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MTL Interrupt Status"]
pub mod MTL_INTERRUPT_STATUS {
    #[doc = "Queue 0 Interrupt status This bit indicates that there is an interrupt from Queue 0."]
    pub mod Q0IS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 0 Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Queue 0 Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Queue 1 Interrupt status This bit indicates that there is an interrupt from Queue 1."]
    pub mod Q1IS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 1 Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Queue 1 Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Queue 2 Interrupt status This bit indicates that there is an interrupt from Queue 2."]
    pub mod Q2IS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 2 Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Queue 2 Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Queue 3 Interrupt status This bit indicates that there is an interrupt from Queue 3."]
    pub mod Q3IS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 3 Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Queue 3 Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Queue 4 Interrupt status This bit indicates that there is an interrupt from Queue 4."]
    pub mod Q4IS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 4 Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Queue 4 Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Debug Interrupt status This bit indicates an interrupt event during the slave access."]
    pub mod DBGIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Debug Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Debug Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "EST (TAS- 802."]
    pub mod ESTIS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EST (TAS- 802.1Qbv) Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "EST (TAS- 802.1Qbv) Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Rx Parser Interrupt Status This bit indicates that there is an interrupt from Rx Parser Block."]
    pub mod MTLPIS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Rx Parser Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Rx Parser Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Receive Queue and DMA Channel Mapping 0"]
pub mod MTL_RXQ_DMA_MAP0 {
    #[doc = "Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q0DDMACH field is reset."]
    pub mod Q0MDMACH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    pub mod Q0DDMACH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 0 disabled for DA-based DMA Channel Selection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue 0 enabled for DA-based DMA Channel Selection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q1DDMACH field is reset."]
    pub mod Q1MDMACH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    pub mod Q1DDMACH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 1 disabled for DA-based DMA Channel Selection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue 1 enabled for DA-based DMA Channel Selection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Queue 2 Mapped to DMA Channel This field controls the routing of the received packet in Queue 2 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q2DDMACH field is reset."]
    pub mod Q2MDMACH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue 2 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 2 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    pub mod Q2DDMACH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 2 disabled for DA-based DMA Channel Selection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue 2 enabled for DA-based DMA Channel Selection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Queue 3 Mapped to DMA Channel This field controls the routing of the received packet in Queue 3 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q3DDMACH field is reset."]
    pub mod Q3MDMACH {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue 3 Enabled for Dynamic (per packet) DMA Channel Selection When set, this bit indicates that the packets received in Queue 3 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    pub mod Q3DDMACH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 3 disabled for DA-based DMA Channel Selection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue 3 enabled for DA-based DMA Channel Selection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Receive Queue and DMA Channel Mapping 1"]
pub mod MTL_RXQ_DMA_MAP1 {
    #[doc = "Queue 4 Mapped to DMA Channel This field controls the routing of the packet received in Queue 4 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q4DDMACH field is reset."]
    pub mod Q4MDMACH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue 4 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 4 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    pub mod Q4DDMACH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 4 disabled for DA-based DMA Channel Selection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue 4 enabled for DA-based DMA Channel Selection"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Time Based Scheduling Control"]
pub mod MTL_TBS_CTRL {
    #[doc = "EST offset Mode When this bit is set, the Launch Time value used in Time Based Scheduling is interpreted as an EST offset value and is added to the Base Time Register (BTR) of the current list."]
    pub mod ESTM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EST offset Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "EST offset Mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Launch Expiry Offset Valid When set indicates the LEOS field is valid."]
    pub mod LEOV {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LEOS field is invalid"]
            pub const INVALID: u32 = 0;
            #[doc = "LEOS field is valid"]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Launch Expiry GSN Offset The number GSN slots that has to be added to the Launch GSN to compute the Launch Expiry time."]
    pub mod LEGOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Launch Expiry Offset The value in units of 256 nanoseconds that has to be added to the Launch time to compute the Launch Expiry time."]
    pub mod LEOS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhancements to Scheduled Transmission Control"]
pub mod MTL_EST_CONTROL {
    #[doc = "Enable EST When reset, the gate control list processing is halted and all gates are assumed to be in Open state."]
    pub mod EEST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EST is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "EST is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Switch to S/W owned list When set indicates that the software has programmed that list that it currently owns (SWOL) and the hardware should switch to the new list based on the new BTR."]
    pub mod SSWL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Switch to S/W owned list is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Switch to S/W owned list is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Do not Drop frames during Frame Size Error When set, frames are not be dropped during Head-of-Line blocking due to Frame Size Error (HLBF field of MTL_EST_STATUS register)."]
    pub mod DDBF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Drop frames during Frame Size Error"]
            pub const DROP: u32 = 0;
            #[doc = "Do not Drop frames during Frame Size Error"]
            pub const DONT_DROP: u32 = 0x01;
        }
    }
    #[doc = "Drop Frames causing Scheduling Error When set frames reported to cause HOL Blocking due to not getting scheduled (HLBS field of EST_STATUS register) after 4,8,16,32 (based on LCSE field of this register) GCL iterations are dropped."]
    pub mod DFBS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not Drop Frames causing Scheduling Error"]
            pub const DONT_DROP: u32 = 0;
            #[doc = "Drop Frames causing Scheduling Error"]
            pub const DROP: u32 = 0x01;
        }
    }
    #[doc = "Loop Count to report Scheduling Error Programmable number of GCL list iterations before reporting an HLBS error defined in EST_STATUS register."]
    pub mod LCSE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4 iterations"]
            pub const BF_4_ITERNS: u32 = 0;
            #[doc = "8 iterations"]
            pub const BF_8_ITERNS: u32 = 0x01;
            #[doc = "16 iterations"]
            pub const BF_16_ITERNS: u32 = 0x02;
            #[doc = "32 iterations"]
            pub const BF_32_ITERNS: u32 = 0x03;
        }
    }
    #[doc = "Time Interval Left Shift Amount This field provides the left shift amount for the programmed Time Interval values used in the Gate Control Lists."]
    pub mod TILS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current Time Offset Value Provides a 12 bit time offset value in nano second that is added to the current time to compensate for all the implementation pipeline delays such as the CDC sync delay, buffering delays, data path delays etc."]
    pub mod CTOV {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PTP Time Offset Value The value of PTP Clock period multiplied by 6 in nanoseconds."]
    pub mod PTOV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhancements to Scheduled Transmission Status"]
pub mod MTL_EST_STATUS {
    #[doc = "Switch to S/W owned list Complete When \"1\" indicates the hardware has successfully switched to the SWOL, and the SWOL bit has been updated to that effect."]
    pub mod SWLC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Switch to S/W owned list Complete not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Switch to S/W owned list Complete detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "BTR Error When \"1\" indicates a programming error in the BTR of SWOL where the programmed value is less than current time."]
    pub mod BTRE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "BTR Error not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "BTR Error detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Head-Of-Line Blocking due to Frame Size Set when HOL Blocking is noticed on one or more Queues as a result of none of the Time Intervals of gate open in the GCL being greater than or equal to the duration needed for frame size (or frame fragment size when preemption is enabled) transmission."]
    pub mod HLBF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Head-Of-Line Blocking due to Frame Size not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Head-Of-Line Blocking due to Frame Size detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Head-Of-Line Blocking due to Scheduling Set when the frame is not able to win arbitration and get scheduled even after 4 iterations of the GCL."]
    pub mod HLBS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Head-Of-Line Blocking due to Scheduling not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Head-Of-Line Blocking due to Scheduling detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Constant Gate Control Error This error occurs when the list length (LLR) is 1 and the programmed Time Interval (TI) value after the optional Left Shifting is less than or equal to the Cycle Time (CTR)."]
    pub mod CGCE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant Gate Control Error not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Constant Gate Control Error detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "S/W owned list When '0' indicates Gate control list number \"0\" is owned by software and when \"1\" indicates the Gate Control list \"1\" is owned by the software."]
    pub mod SWOL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Gate control list number \"0\" is owned by software"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Gate control list number \"1\" is owned by software"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "BTR Error Loop Count Provides the minimum count (N) for which the equation Current Time =< New BTR + (N * New Cycle Time) becomes true."]
    pub mod BTRL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current GCL Slot Number Indicates the slot number of the GCL list."]
    pub mod CGSN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EST Scheduling Error"]
pub mod MTL_EST_SCH_ERROR {
    #[doc = "Schedule Error Queue Number The One Hot Encoded Queue Numbers that have experienced error/timeout described in HLBS field of status register."]
    pub mod SEQN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EST Frame Size Error"]
pub mod MTL_EST_FRM_SIZE_ERROR {
    #[doc = "Frame Size Error Queue Number The One Hot Encoded Queue Numbers that have experienced error described in HLBF field of status register."]
    pub mod FEQN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EST Frame Size Capture"]
pub mod MTL_EST_FRM_SIZE_CAPTURE {
    #[doc = "Frame Size of HLBF Captures the Frame Size of the dropped frame related to queue number indicated in HBFQ field of this register."]
    pub mod HBFS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue Number of HLBF Captures the binary value of the of the first Queue (number) experiencing HLBF error (see HLBF field of status register)."]
    pub mod HBFQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EST Interrupt Enable"]
pub mod MTL_EST_INTR_ENABLE {
    #[doc = "Interrupt Enable for Switch List When set, generates interrupt when the configuration change is successful and the hardware has switched to the new list."]
    pub mod IECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt for Switch List is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt for Switch List is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Enable for BTR Error When set, generates interrupt when the BTR Error occurs and is indicated in the status."]
    pub mod IEBE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt for BTR Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt for BTR Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Enable for HLBF When set, generates interrupt when the Head-of-Line Blocking due to Frame Size error occurs and is indicated in the status."]
    pub mod IEHF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt for HLBF is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt for HLBF is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Enable for HLBS When set, generates interrupt when the Head-of-Line Blocking due to Scheduling issue and is indicated in the status."]
    pub mod IEHS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt for HLBS is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt for HLBS is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Enable for CGCE When set, generates interrupt when the Constant Gate Control Error occurs and is indicated in the status."]
    pub mod CGCE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt for CGCE is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt for CGCE is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "EST GCL Control"]
pub mod MTL_EST_GCL_CONTROL {
    #[doc = "Start Read/Write Op When set indicates a Read/Write Op has started and is in progress."]
    pub mod SRWO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start Read/Write Op disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Start Read/Write Op enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Read '1', Write '0': When set to '1': Read Operation When set to '0': Write Operation."]
    pub mod R1W0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write Operation"]
            pub const WRITE: u32 = 0;
            #[doc = "Read Operation"]
            pub const READ: u32 = 0x01;
        }
    }
    #[doc = "Gate Control Related Registers When set to \"1\" indicates the R/W access is for the GCL related registers (BTR, CTR, TER, LLR) whose address is provided by GCRA."]
    pub mod GCRR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Gate Control Related Registers are disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Gate Control Related Registers are enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Debug Mode When set to \"1\" indicates R/W in debug mode where the memory bank (for GCL and Time related registers) is explicitly provided by DBGB value, when set to \"0\" SWOL bit is used to determine which bank to use."]
    pub mod DBGM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Debug Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Debug Mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Debug Mode Bank Select When set to \"0\" indicates R/W in debug mode should be directed to Bank 0 (GCL0 and corresponding Time related registers)."]
    pub mod DBGB {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "R/W in debug mode should be directed to Bank 0"]
            pub const BANK0: u32 = 0;
            #[doc = "R/W in debug mode should be directed to Bank 1"]
            pub const BANK1: u32 = 0x01;
        }
    }
    #[doc = "Gate Control List Address: (GCLA when GCRR is \"0\")."]
    pub mod ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set indicates the last write operation was aborted as software writes to GCL and GCL registers is prohibited when SSWL bit of MTL_EST_CONTROL Register is set."]
    pub mod ERR0 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ERR0 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "ERR1 is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "EST ECC Inject Error Enable When set along with EEST bit of MTL_EST_CONTROL register, enables the ECC error injection feature."]
    pub mod ESTEIEE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EST ECC Inject Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "EST ECC Inject Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "ECC Inject Error Control for EST Memory When EIEE bit of this register is set, following are the errors inserted based on the value encoded in this field."]
    pub mod ESTEIEC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Insert 1 bit error"]
            pub const BF_1BIT: u32 = 0;
            #[doc = "Insert 2 bit errors"]
            pub const BF_2BIT: u32 = 0x01;
            #[doc = "Insert 3 bit errors"]
            pub const BF_3BIT: u32 = 0x02;
            #[doc = "Insert 1 bit error in address field"]
            pub const BF_1BIT_ADDR: u32 = 0x03;
        }
    }
}
#[doc = "EST GCL Data"]
pub mod MTL_EST_GCL_DATA {
    #[doc = "Gate Control Data The data corresponding to the address selected in the MTL_GCL_CONTROL register."]
    pub mod GCD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame Preemption Control and Status"]
pub mod MTL_FPE_CTRL_STS {
    #[doc = "Additional Fragment Size used to indicate, in units of 64 bytes, the minimum number of bytes over 64 bytes required in non-final fragments of preempted frames."]
    pub mod AFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Preemption Classification When set indicates the corresponding Queue must be classified as preemptable, when '0' Queue is classified as express."]
    pub mod PEC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hold/Release Status - 1: Indicates a Set-and-Hold-MAC operation was last executed and the pMAC is in Hold State."]
    pub mod HRS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Indicates a Set-and-Release-MAC operation was last executed and the pMAC is in Release State"]
            pub const SET_REL: u32 = 0;
            #[doc = "Indicates a Set-and-Hold-MAC operation was last executed and the pMAC is in Hold State"]
            pub const SET_HOLD: u32 = 0x01;
        }
    }
}
#[doc = "Frame Preemption Hold and Release Advance"]
pub mod MTL_FPE_ADVANCE {
    #[doc = "Hold Advance The maximum time in nanoseconds that can elapse between issuing a HOLD to the MAC and the MAC ceasing to transmit any preemptable frame that is in the process of transmission or any preemptable frames that are queued for transmission."]
    pub mod HADV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Release Advance The maximum time in nanoseconds that can elapse between issuing a RELEASE to the MAC and the MAC being ready to resume transmission of preemptable frames, in the absence of there being any express frames available for transmission."]
    pub mod RADV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RXP Control Status"]
pub mod MTL_RXP_CONTROL_STATUS {
    #[doc = "Number of valid entries in the Instruction table This control indicates the number of valid entries in the Instruction Memory."]
    pub mod NVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of parsable entries in the Instruction table This control indicates the number of parsable entries in the Instruction Memory."]
    pub mod NPE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Parser in Idle state This status bit is set to 1 when the Rx parser is in Idle State and waiting for a new packet for processing."]
    pub mod RXPI {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RX Parser not in Idle state"]
            pub const INACTIVE: u32 = 0;
            #[doc = "RX Parser in Idle state"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "RXP Interrupt Control Status"]
pub mod MTL_RXP_INTERRUPT_CONTROL_STATUS {
    #[doc = "Number of Valid Entries Overflow Interrupt Status While parsing if the Instruction address found to be more than NVE (Number of Valid Entries in MTL_RXP_CONTROL register), then this bit is set to 1."]
    pub mod NVEOVIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of Valid Entries Overflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Number of Valid Entries Overflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Number of Parsable Entries Overflow Interrupt Status While parsing a packet if the number of parsed entries found to be more than NPE\\[\\] (Number of Parseable Entries in MTL_RXP_CONTROL register),then this bit is set to 1."]
    pub mod NPEOVIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of Parsable Entries Overflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Number of Parsable Entries Overflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Frame Offset Overflow Interrupt Status While parsing if the Instruction table entry's 'Frame Offset' found to be more than EOF offset, then then this bit is set."]
    pub mod FOOVIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Offset Overflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Frame Offset Overflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Packet Dropped due to RF Interrupt Status If the Rx Parser result says to drop the packet by setting RF=1 in the instruction memory, then this bit is set to 1."]
    pub mod PDRFIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet Dropped due to RF Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Packet Dropped due to RF Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Number of Valid Entries Overflow Interrupt Enable When this bit is set, the NVEOVIS interrupt is enabled."]
    pub mod NVEOVIE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of Valid Entries Overflow Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Number of Valid Entries Overflow Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Number of Parsable Entries Overflow Interrupt Enable When this bit is set, the NPEOVIS interrupt is enabled."]
    pub mod NPEOVIE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Number of Parsable Entries Overflow Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Number of Parsable Entries Overflow Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Frame Offset Overflow Interrupt Enable When this bit is set, the FOOVIS interrupt is enabled."]
    pub mod FOOVIE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Offset Overflow Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Frame Offset Overflow Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Packet Drop due to RF Interrupt Enable When this bit is set, the PDRFIS interrupt is enabled."]
    pub mod PDRFIE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet Drop due to RF Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Packet Drop due to RF Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "RXP Drop Count"]
pub mod MTL_RXP_DROP_CNT {
    #[doc = "Rx Parser Drop count This 31-bit counter is implemented whenever a Rx Parser Drops a packet due to RF =1."]
    pub mod RXPDC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Parser Drop Counter Overflow Bit When set, this bit indicates that the MTL_RXP_DROP_CNT (RXPDC) Counter field crossed the maximum limit."]
    pub mod RXPDCOVF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Parser Drop count overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Parser Drop count overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "RXP Error Count"]
pub mod MTL_RXP_ERROR_CNT {
    #[doc = "Rx Parser Error count This 31-bit counter is implemented whenever a Rx Parser encounters following Error scenarios - Entry address >= NVE\\[\\] - Number Parsed Entries >= NPE\\[\\] - Entry address > EOF data entry address The counter is cleared when the register is read."]
    pub mod RXPEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Parser Error Counter Overflow Bit When set, this bit indicates that the MTL_RXP_ERROR_CNT (RXPEC) Counter field crossed the maximum limit."]
    pub mod RXPECOVF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Parser Error count overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Parser Error count overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "RXP Indirect Access Control and Status"]
pub mod MTL_RXP_INDIRECT_ACC_CONTROL_STATUS {
    #[doc = "FRP Instruction Table Offset Address This field indicates the ADDR of the 32-bit entry in Rx parser instruction table."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Write Control When this bit is set to 1 indicates the write operation to the Rx Parser Memory."]
    pub mod WRRDN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read operation to the Rx Parser Memory"]
            pub const READ: u32 = 0;
            #[doc = "Write operation to the Rx Parser Memory"]
            pub const WRITE: u32 = 0x01;
        }
    }
    #[doc = "FRP Instruction Table Access Busy When this bit is set to 1 by the software then it indicates to start the Read/Write operation from/to the Rx Parser Memory."]
    pub mod STARTBUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "hardware not busy"]
            pub const INACTIVE: u32 = 0;
            #[doc = "hardware is busy (Read/Write operation from/to the Rx Parser Memory)"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "RXP Indirect Access Data"]
pub mod MTL_RXP_INDIRECT_ACC_DATA {
    #[doc = "FRP Instruction Table Write/Read Data Software should write this register before issuing any write command."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 0 Transmit Operation Mode"]
pub mod MTL_TXQ0_OPERATION_MODE {
    #[doc = "Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    pub mod FTQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flush Transmit Queue is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flush Transmit Queue is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    pub mod TSF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    pub mod TXQEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable in AV mode (Reserved in non-AV)"]
            pub const EN_IF_AV: u32 = 0x01;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    pub mod TTC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32"]
            pub const BF_32BYTES: u32 = 0;
            #[doc = "64"]
            pub const BF_64BYTES: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTES: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTES: u32 = 0x03;
            #[doc = "192"]
            pub const BF_192BYTES: u32 = 0x04;
            #[doc = "256"]
            pub const BF_256BYTES: u32 = 0x05;
            #[doc = "384"]
            pub const BF_384BYTES: u32 = 0x06;
            #[doc = "512"]
            pub const BF_512BYTES: u32 = 0x07;
        }
    }
    #[doc = "Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    pub mod TQS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 0 Underflow Counter"]
pub mod MTL_TXQ0_UNDERFLOW {
    #[doc = "Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx Queue Underflow."]
    pub mod UFFRMCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count."]
    pub mod UFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow not detected for Underflow Packet Counter"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow detected for Underflow Packet Counter"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 0 Transmit Debug"]
pub mod MTL_TXQ0_DEBUG {
    #[doc = "Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802."]
    pub mod TXQPAUSED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue in Pause status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue in Pause status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller:"]
    pub mod TRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Read state (transferring data to the MAC transmitter)"]
            pub const READ: u32 = 0x01;
            #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
            pub const WAIT: u32 = 0x02;
            #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue."]
    pub mod TWCSTS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Write Controller status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Write Controller status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission."]
    pub mod TXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Not Empty status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Not Empty status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full."]
    pub mod TXSTSFSTS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Status FIFO Full status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Status FIFO Full status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue."]
    pub mod PTXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue."]
    pub mod STXSTSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 0 ETS Status"]
pub mod MTL_TXQ0_ETS_STATUS {
    #[doc = "Average Bits per Slot This field contains the average transmitted bits per slot."]
    pub mod ABS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 0 Quantum or Weights"]
pub mod MTL_TXQ0_QUANTUM_WEIGHT {
    #[doc = "Quantum or Weights When the DCB operation is enabled with DWRR algorithm for Queue 0 traffic, this field contains the quantum value in bytes to be added to credit during every queue scanning cycle."]
    pub mod ISCQW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 0 Interrupt Control Status"]
pub mod MTL_Q0_INTERRUPT_CONTROL_STATUS {
    #[doc = "Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    pub mod TXUNFIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    pub mod ABPSIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    pub mod TXUIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the sbd_intr_o or mci_intr_o interrupt when the average bits per slot status is updated."]
    pub mod ABPSIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    pub mod RXOVFIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    pub mod RXOIE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 0 Receive Operation Mode"]
pub mod MTL_RXQ0_OPERATION_MODE {
    #[doc = "Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const BF_64BYTE: u32 = 0;
            #[doc = "32"]
            pub const BF_32BYTE: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTE: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTE: u32 = 0x03;
        }
    }
    #[doc = "Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    pub mod FUP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Undersized Good Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Undersized Good Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, GMII_ER, watchdog timeout, or overflow)."]
    pub mod FEP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Error Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Error Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Store and Forward When this bit is set, the DWC_ether_qos reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    pub mod RSF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    pub mod DIS_TCP_EF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Hardware Flow Control When this bit is set, the flow control signal operation, based on the fill-level of Rx queue, is enabled."]
    pub mod EHFC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Hardware Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex These bits control the threshold (fill-level of Rx queue) at which the flow control is activated: For more information on encoding for this field, see RFD."]
    pub mod RFA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (fill-level of Rx queue) at which the flow control is de-asserted after activation: - 0: Full minus 1 KB, that is, FULL 1 KB - 1: Full minus 1."]
    pub mod RFD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Size This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    pub mod RQS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 0 Missed Packet and Overflow Counter"]
pub mod MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT {
    #[doc = "Overflow Packet Counter This field indicates the number of packets discarded by the DWC_ether_qos because of Receive queue overflow."]
    pub mod OVFPKTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
    pub mod OVFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Missed Packet Counter This field indicates the number of packets missed by the DWC_ether_qos because the application asserted ari_pkt_flush_i\\[\\] for this queue."]
    pub mod MISPKTCNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit."]
    pub mod MISCNTOVF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Missed Packet Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Missed Packet Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 0 Receive Debug"]
pub mod MTL_RXQ0_DEBUG {
    #[doc = "MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    pub mod RWCSTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Rx Queue Write Controller Active Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Rx Queue Write Controller Active Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:"]
    pub mod RRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Reading packet data"]
            pub const READ_DATA: u32 = 0x01;
            #[doc = "Reading packet status (or timestamp)"]
            pub const READ_STS: u32 = 0x02;
            #[doc = "Flushing the packet data and status"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue:"]
    pub mod RXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Queue empty"]
            pub const EMPTY: u32 = 0;
            #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
            pub const BLW_THR: u32 = 0x01;
            #[doc = "Rx Queue fill-level above flow-control activate threshold"]
            pub const ABV_THR: u32 = 0x02;
            #[doc = "Rx Queue full"]
            pub const FULL: u32 = 0x03;
        }
    }
    #[doc = "Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
    pub mod PRXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 0 Receive Control"]
pub mod MTL_RXQ0_CONTROL {
    #[doc = "Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    pub mod RXQ_WEGT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Packet Arbitration When this bit is set, the DWC_ether_qos drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    pub mod RXQ_FRM_ARBIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Packet Arbitration is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Packet Arbitration is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 1 Transmit Operation Mode"]
pub mod MTL_TXQ1_OPERATION_MODE {
    #[doc = "Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    pub mod FTQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flush Transmit Queue is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flush Transmit Queue is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    pub mod TSF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    pub mod TXQEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable in AV mode (Reserved in non-AV)"]
            pub const EN_IF_AV: u32 = 0x01;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    pub mod TTC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32"]
            pub const BF_32BYTES: u32 = 0;
            #[doc = "64"]
            pub const BF_64BYTES: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTES: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTES: u32 = 0x03;
            #[doc = "192"]
            pub const BF_192BYTES: u32 = 0x04;
            #[doc = "256"]
            pub const BF_256BYTES: u32 = 0x05;
            #[doc = "384"]
            pub const BF_384BYTES: u32 = 0x06;
            #[doc = "512"]
            pub const BF_512BYTES: u32 = 0x07;
        }
    }
    #[doc = "Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    pub mod TQS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 Underflow Counter"]
pub mod MTL_TXQ1_UNDERFLOW {
    #[doc = "Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx Queue Underflow."]
    pub mod UFFRMCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count."]
    pub mod UFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow not detected for Underflow Packet Counter"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow detected for Underflow Packet Counter"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 1 Transmit Debug"]
pub mod MTL_TXQ1_DEBUG {
    #[doc = "Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802."]
    pub mod TXQPAUSED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue in Pause status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue in Pause status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller:"]
    pub mod TRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Read state (transferring data to the MAC transmitter)"]
            pub const READ: u32 = 0x01;
            #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
            pub const WAIT: u32 = 0x02;
            #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue."]
    pub mod TWCSTS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Write Controller status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Write Controller status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission."]
    pub mod TXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Not Empty status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Not Empty status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full."]
    pub mod TXSTSFSTS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Status FIFO Full status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Status FIFO Full status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue."]
    pub mod PTXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue."]
    pub mod STXSTSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 ETS Control"]
pub mod MTL_TXQ1_ETS_CONTROL {
    #[doc = "AV Algorithm When Queue 1 is programmed for AV, this field configures the scheduling algorithm for this queue: This bit when set, indicates credit based shaper algorithm (CBS) is selected for Queue 1 traffic."]
    pub mod AVALG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CBS Algorithm is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "CBS Algorithm is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Credit Control When this bit is set, the accumulated credit parameter in the credit-based shaper algorithm logic is not reset to zero when there is positive credit and no packet to transmit in Channel 1."]
    pub mod CC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Credit Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Credit Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Count If the credit-based shaper algorithm is enabled, the software can program the number of slots (of duration programmed in DMA_CH\\[n\\]_Slot_Interval register) over which the average transmitted bits per slot, provided in the MTL_TXQ\\[N\\]_ETS_STATUS register, need to be computed for Queue."]
    pub mod SLC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 slot"]
            pub const BF_1_SLOT: u32 = 0;
            #[doc = "2 slots"]
            pub const BF_2_SLOT: u32 = 0x01;
            #[doc = "4 slots"]
            pub const BF_4_SLOT: u32 = 0x02;
            #[doc = "8 slots"]
            pub const BF_8_SLOT: u32 = 0x03;
            #[doc = "16 slots"]
            pub const BF_16_SLOT: u32 = 0x04;
        }
    }
}
#[doc = "Queue 1 ETS Status"]
pub mod MTL_TXQ1_ETS_STATUS {
    #[doc = "Average Bits per Slot This field contains the average transmitted bits per slot."]
    pub mod ABS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 idleSlopeCredit, Quantum or Weights"]
pub mod MTL_TXQ1_QUANTUM_WEIGHT {
    #[doc = "idleSlopeCredit, Quantum or Weights - idleSlopeCredit When AV feature is enabled, this field contains the idleSlopeCredit value required for the credit-based shaper algorithm for Queue 1."]
    pub mod ISCQW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 sendSlopeCredit"]
pub mod MTL_TXQ1_SENDSLOPECREDIT {
    #[doc = "sendSlopeCredit Value When AV operation is enabled, this field contains the sendSlopeCredit value required for credit-based shaper algorithm for Queue 1."]
    pub mod SSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 hiCredit"]
pub mod MTL_TXQ1_HICREDIT {
    #[doc = "hiCredit Value When the AV feature is enabled, this field contains the hiCredit value required for the credit-based shaper algorithm."]
    pub mod HC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 loCredit"]
pub mod MTL_TXQ1_LOCREDIT {
    #[doc = "loCredit Value When AV operation is enabled, this field contains the loCredit value required for the credit-based shaper algorithm."]
    pub mod LC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 Interrupt Control Status"]
pub mod MTL_Q1_INTERRUPT_CONTROL_STATUS {
    #[doc = "Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    pub mod TXUNFIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    pub mod ABPSIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    pub mod TXUIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the sbd_intr_o or mci_intr_o interrupt when the average bits per slot status is updated."]
    pub mod ABPSIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    pub mod RXOVFIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    pub mod RXOIE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 1 Receive Operation Mode"]
pub mod MTL_RXQ1_OPERATION_MODE {
    #[doc = "Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const BF_64BYTE: u32 = 0;
            #[doc = "32"]
            pub const BF_32BYTE: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTE: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTE: u32 = 0x03;
        }
    }
    #[doc = "Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    pub mod FUP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Undersized Good Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Undersized Good Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, GMII_ER, watchdog timeout, or overflow)."]
    pub mod FEP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Error Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Error Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Store and Forward When this bit is set, the DWC_ether_qos reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    pub mod RSF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    pub mod DIS_TCP_EF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Hardware Flow Control When this bit is set, the flow control signal operation, based on the fill-level of Rx queue, is enabled."]
    pub mod EHFC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Hardware Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex These bits control the threshold (fill-level of Rx queue) at which the flow control is activated: For more information on encoding for this field, see RFD."]
    pub mod RFA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (fill-level of Rx queue) at which the flow control is de-asserted after activation: - 0: Full minus 1 KB, that is, FULL 1 KB - 1: Full minus 1."]
    pub mod RFD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Size This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    pub mod RQS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 Missed Packet and Overflow Counter"]
pub mod MTL_RXQ1_MISSED_PACKET_OVERFLOW_CNT {
    #[doc = "Overflow Packet Counter This field indicates the number of packets discarded by the DWC_ether_qos because of Receive queue overflow."]
    pub mod OVFPKTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
    pub mod OVFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Missed Packet Counter This field indicates the number of packets missed by the DWC_ether_qos because the application asserted ari_pkt_flush_i\\[\\] for this queue."]
    pub mod MISPKTCNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit."]
    pub mod MISCNTOVF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Missed Packet Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Missed Packet Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 1 Receive Debug"]
pub mod MTL_RXQ1_DEBUG {
    #[doc = "MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    pub mod RWCSTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Rx Queue Write Controller Active Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Rx Queue Write Controller Active Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:"]
    pub mod RRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Reading packet data"]
            pub const READ_DATA: u32 = 0x01;
            #[doc = "Reading packet status (or timestamp)"]
            pub const READ_STS: u32 = 0x02;
            #[doc = "Flushing the packet data and status"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue:"]
    pub mod RXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Queue empty"]
            pub const EMPTY: u32 = 0;
            #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
            pub const BLW_THR: u32 = 0x01;
            #[doc = "Rx Queue fill-level above flow-control activate threshold"]
            pub const ABV_THR: u32 = 0x02;
            #[doc = "Rx Queue full"]
            pub const FULL: u32 = 0x03;
        }
    }
    #[doc = "Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
    pub mod PRXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 Receive Control"]
pub mod MTL_RXQ1_CONTROL {
    #[doc = "Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    pub mod RXQ_WEGT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Packet Arbitration When this bit is set, the DWC_ether_qos drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    pub mod RXQ_FRM_ARBIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Packet Arbitration is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Packet Arbitration is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 2 Transmit Operation Mode"]
pub mod MTL_TXQ2_OPERATION_MODE {
    #[doc = "Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    pub mod FTQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flush Transmit Queue is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flush Transmit Queue is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    pub mod TSF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    pub mod TXQEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable in AV mode (Reserved in non-AV)"]
            pub const EN_IF_AV: u32 = 0x01;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    pub mod TTC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32"]
            pub const BF_32BYTES: u32 = 0;
            #[doc = "64"]
            pub const BF_64BYTES: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTES: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTES: u32 = 0x03;
            #[doc = "192"]
            pub const BF_192BYTES: u32 = 0x04;
            #[doc = "256"]
            pub const BF_256BYTES: u32 = 0x05;
            #[doc = "384"]
            pub const BF_384BYTES: u32 = 0x06;
            #[doc = "512"]
            pub const BF_512BYTES: u32 = 0x07;
        }
    }
    #[doc = "Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    pub mod TQS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 Underflow Counter"]
pub mod MTL_TXQ2_UNDERFLOW {
    #[doc = "Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx Queue Underflow."]
    pub mod UFFRMCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count."]
    pub mod UFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow not detected for Underflow Packet Counter"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow detected for Underflow Packet Counter"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 2 Transmit Debug"]
pub mod MTL_TXQ2_DEBUG {
    #[doc = "Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802."]
    pub mod TXQPAUSED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue in Pause status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue in Pause status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller:"]
    pub mod TRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Read state (transferring data to the MAC transmitter)"]
            pub const READ: u32 = 0x01;
            #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
            pub const WAIT: u32 = 0x02;
            #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue."]
    pub mod TWCSTS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Write Controller status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Write Controller status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission."]
    pub mod TXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Not Empty status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Not Empty status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full."]
    pub mod TXSTSFSTS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Status FIFO Full status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Status FIFO Full status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue."]
    pub mod PTXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue."]
    pub mod STXSTSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 ETS Control"]
pub mod MTL_TXQ2_ETS_CONTROL {
    #[doc = "AV Algorithm When Queue 1 is programmed for AV, this field configures the scheduling algorithm for this queue: This bit when set, indicates credit based shaper algorithm (CBS) is selected for Queue 1 traffic."]
    pub mod AVALG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CBS Algorithm is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "CBS Algorithm is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Credit Control When this bit is set, the accumulated credit parameter in the credit-based shaper algorithm logic is not reset to zero when there is positive credit and no packet to transmit in Channel 1."]
    pub mod CC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Credit Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Credit Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Count If the credit-based shaper algorithm is enabled, the software can program the number of slots (of duration programmed in DMA_CH\\[N\\]_SLOT_INTERVAL register) over which the average transmitted bits per slot, provided in the MTL_TXQ\\[N\\]_ETS_STATUS register, need to be computed for Queue."]
    pub mod SLC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 slot"]
            pub const BF_1_SLOT: u32 = 0;
            #[doc = "2 slots"]
            pub const BF_2_SLOT: u32 = 0x01;
            #[doc = "4 slots"]
            pub const BF_4_SLOT: u32 = 0x02;
            #[doc = "8 slots"]
            pub const BF_8_SLOT: u32 = 0x03;
            #[doc = "16 slots"]
            pub const BF_16_SLOT: u32 = 0x04;
        }
    }
}
#[doc = "Queue 2 ETS Status"]
pub mod MTL_TXQ2_ETS_STATUS {
    #[doc = "Average Bits per Slot This field contains the average transmitted bits per slot."]
    pub mod ABS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 idleSlopeCredit, Quantum or Weights"]
pub mod MTL_TXQ2_QUANTUM_WEIGHT {
    #[doc = "idleSlopeCredit, Quantum or Weights - idleSlopeCredit When AV feature is enabled, this field contains the idleSlopeCredit value required for the credit-based shaper algorithm for Queue 1."]
    pub mod ISCQW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 sendSlopeCredit"]
pub mod MTL_TXQ2_SENDSLOPECREDIT {
    #[doc = "sendSlopeCredit Value When AV operation is enabled, this field contains the sendSlopeCredit value required for credit-based shaper algorithm for Queue 1."]
    pub mod SSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 hiCredit"]
pub mod MTL_TXQ2_HICREDIT {
    #[doc = "hiCredit Value When the AV feature is enabled, this field contains the hiCredit value required for the credit-based shaper algorithm."]
    pub mod HC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 loCredit"]
pub mod MTL_TXQ2_LOCREDIT {
    #[doc = "loCredit Value When AV operation is enabled, this field contains the loCredit value required for the credit-based shaper algorithm."]
    pub mod LC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 Interrupt Control Status"]
pub mod MTL_Q2_INTERRUPT_CONTROL_STATUS {
    #[doc = "Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    pub mod TXUNFIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    pub mod ABPSIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    pub mod TXUIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the sbd_intr_o or mci_intr_o interrupt when the average bits per slot status is updated."]
    pub mod ABPSIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    pub mod RXOVFIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    pub mod RXOIE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 2 Receive Operation Mode"]
pub mod MTL_RXQ2_OPERATION_MODE {
    #[doc = "Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const BF_64BYTE: u32 = 0;
            #[doc = "32"]
            pub const BF_32BYTE: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTE: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTE: u32 = 0x03;
        }
    }
    #[doc = "Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    pub mod FUP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Undersized Good Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Undersized Good Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, GMII_ER, watchdog timeout, or overflow)."]
    pub mod FEP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Error Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Error Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Store and Forward When this bit is set, the DWC_ether_qos reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    pub mod RSF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    pub mod DIS_TCP_EF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Hardware Flow Control When this bit is set, the flow control signal operation, based on the fill-level of Rx queue, is enabled."]
    pub mod EHFC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Hardware Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex These bits control the threshold (fill-level of Rx queue) at which the flow control is activated: For more information on encoding for this field, see RFD."]
    pub mod RFA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (fill-level of Rx queue) at which the flow control is de-asserted after activation: - 0: Full minus 1 KB, that is, FULL 1 KB - 1: Full minus 1."]
    pub mod RFD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Size This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    pub mod RQS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 Missed Packet and Overflow Counter"]
pub mod MTL_RXQ2_MISSED_PACKET_OVERFLOW_CNT {
    #[doc = "Overflow Packet Counter This field indicates the number of packets discarded by the DWC_ether_qos because of Receive queue overflow."]
    pub mod OVFPKTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
    pub mod OVFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Missed Packet Counter This field indicates the number of packets missed by the DWC_ether_qos because the application asserted ari_pkt_flush_i\\[\\] for this queue."]
    pub mod MISPKTCNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit."]
    pub mod MISCNTOVF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Missed Packet Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Missed Packet Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 2 Receive Debug"]
pub mod MTL_RXQ2_DEBUG {
    #[doc = "MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    pub mod RWCSTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Rx Queue Write Controller Active Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Rx Queue Write Controller Active Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:"]
    pub mod RRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Reading packet data"]
            pub const READ_DATA: u32 = 0x01;
            #[doc = "Reading packet status (or timestamp)"]
            pub const READ_STS: u32 = 0x02;
            #[doc = "Flushing the packet data and status"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue:"]
    pub mod RXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Queue empty"]
            pub const EMPTY: u32 = 0;
            #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
            pub const BLW_THR: u32 = 0x01;
            #[doc = "Rx Queue fill-level above flow-control activate threshold"]
            pub const ABV_THR: u32 = 0x02;
            #[doc = "Rx Queue full"]
            pub const FULL: u32 = 0x03;
        }
    }
    #[doc = "Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
    pub mod PRXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 2 Receive Control"]
pub mod MTL_RXQ2_CONTROL {
    #[doc = "Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    pub mod RXQ_WEGT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Packet Arbitration When this bit is set, the DWC_ether_qos drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    pub mod RXQ_FRM_ARBIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Packet Arbitration is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Packet Arbitration is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 3 Transmit Operation Mode"]
pub mod MTL_TXQ3_OPERATION_MODE {
    #[doc = "Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    pub mod FTQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flush Transmit Queue is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flush Transmit Queue is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    pub mod TSF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    pub mod TXQEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable in AV mode (Reserved in non-AV)"]
            pub const EN_IF_AV: u32 = 0x01;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    pub mod TTC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32"]
            pub const BF_32BYTES: u32 = 0;
            #[doc = "64"]
            pub const BF_64BYTES: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTES: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTES: u32 = 0x03;
            #[doc = "192"]
            pub const BF_192BYTES: u32 = 0x04;
            #[doc = "256"]
            pub const BF_256BYTES: u32 = 0x05;
            #[doc = "384"]
            pub const BF_384BYTES: u32 = 0x06;
            #[doc = "512"]
            pub const BF_512BYTES: u32 = 0x07;
        }
    }
    #[doc = "Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    pub mod TQS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 Underflow Counter"]
pub mod MTL_TXQ3_UNDERFLOW {
    #[doc = "Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx Queue Underflow."]
    pub mod UFFRMCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count."]
    pub mod UFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow not detected for Underflow Packet Counter"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow detected for Underflow Packet Counter"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 3 Transmit Debug"]
pub mod MTL_TXQ3_DEBUG {
    #[doc = "Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802."]
    pub mod TXQPAUSED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue in Pause status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue in Pause status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller:"]
    pub mod TRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Read state (transferring data to the MAC transmitter)"]
            pub const READ: u32 = 0x01;
            #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
            pub const WAIT: u32 = 0x02;
            #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue."]
    pub mod TWCSTS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Write Controller status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Write Controller status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission."]
    pub mod TXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Not Empty status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Not Empty status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full."]
    pub mod TXSTSFSTS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Status FIFO Full status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Status FIFO Full status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue."]
    pub mod PTXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue."]
    pub mod STXSTSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 ETS Control"]
pub mod MTL_TXQ3_ETS_CONTROL {
    #[doc = "AV Algorithm When Queue 1 is programmed for AV, this field configures the scheduling algorithm for this queue: This bit when set, indicates credit based shaper algorithm (CBS) is selected for Queue 1 traffic."]
    pub mod AVALG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CBS Algorithm is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "CBS Algorithm is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Credit Control When this bit is set, the accumulated credit parameter in the credit-based shaper algorithm logic is not reset to zero when there is positive credit and no packet to transmit in Channel 1."]
    pub mod CC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Credit Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Credit Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Count If the credit-based shaper algorithm is enabled, the software can program the number of slots (of duration programmed in DMA_CH\\[N\\]_SLOT_INTERVAL register) over which the average transmitted bits per slot, provided in the MTL_TXQ\\[N\\]_ETS_STATUS register, need to be computed for Queue."]
    pub mod SLC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 slot"]
            pub const BF_1_SLOT: u32 = 0;
            #[doc = "2 slots"]
            pub const BF_2_SLOT: u32 = 0x01;
            #[doc = "4 slots"]
            pub const BF_4_SLOT: u32 = 0x02;
            #[doc = "8 slots"]
            pub const BF_8_SLOT: u32 = 0x03;
            #[doc = "16 slots"]
            pub const BF_16_SLOT: u32 = 0x04;
        }
    }
}
#[doc = "Queue 3 ETS Status"]
pub mod MTL_TXQ3_ETS_STATUS {
    #[doc = "Average Bits per Slot This field contains the average transmitted bits per slot."]
    pub mod ABS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 idleSlopeCredit, Quantum or Weights"]
pub mod MTL_TXQ3_QUANTUM_WEIGHT {
    #[doc = "idleSlopeCredit, Quantum or Weights - idleSlopeCredit When AV feature is enabled, this field contains the idleSlopeCredit value required for the credit-based shaper algorithm for Queue 1."]
    pub mod ISCQW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 sendSlopeCredit"]
pub mod MTL_TXQ3_SENDSLOPECREDIT {
    #[doc = "sendSlopeCredit Value When AV operation is enabled, this field contains the sendSlopeCredit value required for credit-based shaper algorithm for Queue 1."]
    pub mod SSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 hiCredit"]
pub mod MTL_TXQ3_HICREDIT {
    #[doc = "hiCredit Value When the AV feature is enabled, this field contains the hiCredit value required for the credit-based shaper algorithm."]
    pub mod HC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 loCredit"]
pub mod MTL_TXQ3_LOCREDIT {
    #[doc = "loCredit Value When AV operation is enabled, this field contains the loCredit value required for the credit-based shaper algorithm."]
    pub mod LC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 Interrupt Control Status"]
pub mod MTL_Q3_INTERRUPT_CONTROL_STATUS {
    #[doc = "Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    pub mod TXUNFIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    pub mod ABPSIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    pub mod TXUIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the sbd_intr_o or mci_intr_o interrupt when the average bits per slot status is updated."]
    pub mod ABPSIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    pub mod RXOVFIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    pub mod RXOIE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 3 Receive Operation Mode"]
pub mod MTL_RXQ3_OPERATION_MODE {
    #[doc = "Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const BF_64BYTE: u32 = 0;
            #[doc = "32"]
            pub const BF_32BYTE: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTE: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTE: u32 = 0x03;
        }
    }
    #[doc = "Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    pub mod FUP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Undersized Good Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Undersized Good Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, GMII_ER, watchdog timeout, or overflow)."]
    pub mod FEP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Error Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Error Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Store and Forward When this bit is set, the DWC_ether_qos reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    pub mod RSF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    pub mod DIS_TCP_EF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Hardware Flow Control When this bit is set, the flow control signal operation, based on the fill-level of Rx queue, is enabled."]
    pub mod EHFC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Hardware Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex These bits control the threshold (fill-level of Rx queue) at which the flow control is activated: For more information on encoding for this field, see RFD."]
    pub mod RFA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (fill-level of Rx queue) at which the flow control is de-asserted after activation: - 0: Full minus 1 KB, that is, FULL 1 KB - 1: Full minus 1."]
    pub mod RFD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Size This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    pub mod RQS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 Missed Packet and Overflow Counter"]
pub mod MTL_RXQ3_MISSED_PACKET_OVERFLOW_CNT {
    #[doc = "Overflow Packet Counter This field indicates the number of packets discarded by the DWC_ether_qos because of Receive queue overflow."]
    pub mod OVFPKTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
    pub mod OVFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Missed Packet Counter This field indicates the number of packets missed by the DWC_ether_qos because the application asserted ari_pkt_flush_i\\[\\] for this queue."]
    pub mod MISPKTCNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit."]
    pub mod MISCNTOVF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Missed Packet Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Missed Packet Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 3 Receive Debug"]
pub mod MTL_RXQ3_DEBUG {
    #[doc = "MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    pub mod RWCSTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Rx Queue Write Controller Active Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Rx Queue Write Controller Active Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:"]
    pub mod RRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Reading packet data"]
            pub const READ_DATA: u32 = 0x01;
            #[doc = "Reading packet status (or timestamp)"]
            pub const READ_STS: u32 = 0x02;
            #[doc = "Flushing the packet data and status"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue:"]
    pub mod RXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Queue empty"]
            pub const EMPTY: u32 = 0;
            #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
            pub const BLW_THR: u32 = 0x01;
            #[doc = "Rx Queue fill-level above flow-control activate threshold"]
            pub const ABV_THR: u32 = 0x02;
            #[doc = "Rx Queue full"]
            pub const FULL: u32 = 0x03;
        }
    }
    #[doc = "Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
    pub mod PRXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 3 Receive Control"]
pub mod MTL_RXQ3_CONTROL {
    #[doc = "Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    pub mod RXQ_WEGT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Packet Arbitration When this bit is set, the DWC_ether_qos drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    pub mod RXQ_FRM_ARBIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Packet Arbitration is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Packet Arbitration is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 4 Transmit Operation Mode"]
pub mod MTL_TXQ4_OPERATION_MODE {
    #[doc = "Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    pub mod FTQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flush Transmit Queue is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flush Transmit Queue is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    pub mod TSF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    pub mod TXQEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable in AV mode (Reserved in non-AV)"]
            pub const EN_IF_AV: u32 = 0x01;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x02;
        }
    }
    #[doc = "Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    pub mod TTC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32"]
            pub const BF_32BYTES: u32 = 0;
            #[doc = "64"]
            pub const BF_64BYTES: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTES: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTES: u32 = 0x03;
            #[doc = "192"]
            pub const BF_192BYTES: u32 = 0x04;
            #[doc = "256"]
            pub const BF_256BYTES: u32 = 0x05;
            #[doc = "384"]
            pub const BF_384BYTES: u32 = 0x06;
            #[doc = "512"]
            pub const BF_512BYTES: u32 = 0x07;
        }
    }
    #[doc = "Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    pub mod TQS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 Underflow Counter"]
pub mod MTL_TXQ4_UNDERFLOW {
    #[doc = "Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx Queue Underflow."]
    pub mod UFFRMCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count."]
    pub mod UFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow not detected for Underflow Packet Counter"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow detected for Underflow Packet Counter"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 4 Transmit Debug"]
pub mod MTL_TXQ4_DEBUG {
    #[doc = "Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802."]
    pub mod TXQPAUSED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue in Pause status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue in Pause status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller:"]
    pub mod TRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Read state (transferring data to the MAC transmitter)"]
            pub const READ: u32 = 0x01;
            #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
            pub const WAIT: u32 = 0x02;
            #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue."]
    pub mod TWCSTS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Write Controller status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Write Controller status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission."]
    pub mod TXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Queue Not Empty status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Queue Not Empty status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full."]
    pub mod TXSTSFSTS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Tx Status FIFO Full status is not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Tx Status FIFO Full status is detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue."]
    pub mod PTXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue."]
    pub mod STXSTSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 ETS Control"]
pub mod MTL_TXQ4_ETS_CONTROL {
    #[doc = "AV Algorithm When Queue 1 is programmed for AV, this field configures the scheduling algorithm for this queue: This bit when set, indicates credit based shaper algorithm (CBS) is selected for Queue 1 traffic."]
    pub mod AVALG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CBS Algorithm is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "CBS Algorithm is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Credit Control When this bit is set, the accumulated credit parameter in the credit-based shaper algorithm logic is not reset to zero when there is positive credit and no packet to transmit in Channel 1."]
    pub mod CC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Credit Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Credit Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Count If the credit-based shaper algorithm is enabled, the software can program the number of slots (of duration programmed in DMA_CH\\[N\\]_SLOT_INTERVAL register) over which the average transmitted bits per slot, provided in the MTL_TXQ\\[N\\]_ETS_STATUS register, need to be computed for Queue."]
    pub mod SLC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 slot"]
            pub const BF_1_SLOT: u32 = 0;
            #[doc = "2 slots"]
            pub const BF_2_SLOT: u32 = 0x01;
            #[doc = "4 slots"]
            pub const BF_4_SLOT: u32 = 0x02;
            #[doc = "8 slots"]
            pub const BF_8_SLOT: u32 = 0x03;
            #[doc = "16 slots"]
            pub const BF_16_SLOT: u32 = 0x04;
        }
    }
}
#[doc = "Queue 4 ETS Status"]
pub mod MTL_TXQ4_ETS_STATUS {
    #[doc = "Average Bits per Slot This field contains the average transmitted bits per slot."]
    pub mod ABS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 idleSlopeCredit, Quantum or Weights"]
pub mod MTL_TXQ4_QUANTUM_WEIGHT {
    #[doc = "idleSlopeCredit, Quantum or Weights - idleSlopeCredit When AV feature is enabled, this field contains the idleSlopeCredit value required for the credit-based shaper algorithm for Queue 1."]
    pub mod ISCQW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 sendSlopeCredit"]
pub mod MTL_TXQ4_SENDSLOPECREDIT {
    #[doc = "sendSlopeCredit Value When AV operation is enabled, this field contains the sendSlopeCredit value required for credit-based shaper algorithm for Queue 1."]
    pub mod SSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 hiCredit"]
pub mod MTL_TXQ4_HICREDIT {
    #[doc = "hiCredit Value When the AV feature is enabled, this field contains the hiCredit value required for the credit-based shaper algorithm."]
    pub mod HC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 loCredit"]
pub mod MTL_TXQ4_LOCREDIT {
    #[doc = "loCredit Value When AV operation is enabled, this field contains the loCredit value required for the credit-based shaper algorithm."]
    pub mod LC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 Interrupt Control Status"]
pub mod MTL_Q4_INTERRUPT_CONTROL_STATUS {
    #[doc = "Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    pub mod TXUNFIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    pub mod ABPSIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    pub mod TXUIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Queue Underflow Interrupt Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Queue Underflow Interrupt Status is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the sbd_intr_o or mci_intr_o interrupt when the average bits per slot status is updated."]
    pub mod ABPSIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Average Bits Per Slot Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Average Bits Per Slot Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    pub mod RXOVFIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    pub mod RXOIE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Overflow Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Overflow Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 4 Receive Operation Mode"]
pub mod MTL_RXQ4_OPERATION_MODE {
    #[doc = "Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    pub mod RTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const BF_64BYTE: u32 = 0;
            #[doc = "32"]
            pub const BF_32BYTE: u32 = 0x01;
            #[doc = "96"]
            pub const BF_96BYTE: u32 = 0x02;
            #[doc = "128"]
            pub const BF_128BYTE: u32 = 0x03;
        }
    }
    #[doc = "Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    pub mod FUP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Undersized Good Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Undersized Good Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, GMII_ER, watchdog timeout, or overflow)."]
    pub mod FEP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forward Error Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forward Error Packets is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Queue Store and Forward When this bit is set, the DWC_ether_qos reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    pub mod RSF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Store and Forward is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Store and Forward is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    pub mod DIS_TCP_EF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Hardware Flow Control When this bit is set, the flow control signal operation, based on the fill-level of Rx queue, is enabled."]
    pub mod EHFC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Hardware Flow Control is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex These bits control the threshold (fill-level of Rx queue) at which the flow control is activated: For more information on encoding for this field, see RFD."]
    pub mod RFA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (fill-level of Rx queue) at which the flow control is de-asserted after activation: - 0: Full minus 1 KB, that is, FULL 1 KB - 1: Full minus 1."]
    pub mod RFD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Size This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    pub mod RQS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 Missed Packet and Overflow Counter"]
pub mod MTL_RXQ4_MISSED_PACKET_OVERFLOW_CNT {
    #[doc = "Overflow Packet Counter This field indicates the number of packets discarded by the DWC_ether_qos because of Receive queue overflow."]
    pub mod OVFPKTCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
    pub mod OVFCNTOVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overflow Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Overflow Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Missed Packet Counter This field indicates the number of packets missed by the DWC_ether_qos because the application asserted ari_pkt_flush_i\\[\\] for this queue."]
    pub mod MISPKTCNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit."]
    pub mod MISCNTOVF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Missed Packet Counter overflow not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Missed Packet Counter overflow detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Queue 4 Receive Debug"]
pub mod MTL_RXQ4_DEBUG {
    #[doc = "MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    pub mod RWCSTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Rx Queue Write Controller Active Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Rx Queue Write Controller Active Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:"]
    pub mod RRCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Reading packet data"]
            pub const READ_DATA: u32 = 0x01;
            #[doc = "Reading packet status (or timestamp)"]
            pub const READ_STS: u32 = 0x02;
            #[doc = "Flushing the packet data and status"]
            pub const FLUSH: u32 = 0x03;
        }
    }
    #[doc = "MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue:"]
    pub mod RXQSTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Queue empty"]
            pub const EMPTY: u32 = 0;
            #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
            pub const BLW_THR: u32 = 0x01;
            #[doc = "Rx Queue fill-level above flow-control activate threshold"]
            pub const ABV_THR: u32 = 0x02;
            #[doc = "Rx Queue full"]
            pub const FULL: u32 = 0x03;
        }
    }
    #[doc = "Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
    pub mod PRXQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 4 Receive Control"]
pub mod MTL_RXQ4_CONTROL {
    #[doc = "Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    pub mod RXQ_WEGT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Queue Packet Arbitration When this bit is set, the DWC_ether_qos drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    pub mod RXQ_FRM_ARBIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue Packet Arbitration is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Queue Packet Arbitration is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DMA Bus Mode"]
pub mod DMA_MODE {
    #[doc = "Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC."]
    pub mod SWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software Reset is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Software Reset is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Descriptor Posted Write When this bit is set to 0, the descriptor writes are always non-posted."]
    pub mod DSPW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Descriptor Posted Write is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Descriptor Posted Write is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Interrupt Mode This field defines the interrupt mode of DWC_ether_qos."]
    pub mod INTM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "See above description"]
            pub const MODE0: u32 = 0;
            #[doc = "See above description"]
            pub const MODE1: u32 = 0x01;
            #[doc = "See above description"]
            pub const MODE2: u32 = 0x02;
        }
    }
}
#[doc = "DMA System Bus Mode"]
pub mod DMA_SYSBUS_MODE {
    #[doc = "Fixed Burst Length When this bit is set to 1, the EQOS-AXI master initiates burst transfers of specified lengths as given below."]
    pub mod FB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed Burst Length is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Fixed Burst Length is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "AXI Burst Length 4 When this bit is set to 1 or the FB bit is set to 0, the EQOS-AXI master can select a burst length of 4 on the AXI interface."]
    pub mod BLEN4 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "AXI Burst Length 4"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "AXI Burst Length 8 When this bit is set to 1 or the FB bit is set to 0, the EQOS-AXI master can select a burst length of 8 on the AXI interface."]
    pub mod BLEN8 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "AXI Burst Length 8"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "AXI Burst Length 16 When this bit is set to 1 or the FB bit is set to 0, the EQOS-AXI master can select a burst length of 16 on the AXI interface."]
    pub mod BLEN16 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "AXI Burst Length 16"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Automatic AXI LPI enable When set to 1, enables the AXI master to enter into LPI state when there is no activity in the DWC_ether_qos for number of system clock cycles programmed in the LPIEI field of DMA_AXI_LPI_ENTRY_INTERVAL register."]
    pub mod AALE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic AXI LPI is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic AXI LPI is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Address-Aligned Beats When this bit is set to 1, the EQOS-AXI or EQOS-AHB master performs address-aligned burst transfers on Read and Write channels."]
    pub mod AAL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address-Aligned Beats is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address-Aligned Beats is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "1 KB Boundary Crossing Enable for the EQOS-AXI Master When set, the burst transfers performed by the EQOS-AXI master do not cross 1 KB boundary."]
    pub mod ONEKBBE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 KB Boundary Crossing for the EQOS-AXI Master Beats is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "1 KB Boundary Crossing for the EQOS-AXI Master Beats is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface."]
    pub mod RD_OSR_LMT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface."]
    pub mod WR_OSR_LMT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unlock on Magic Packet or Remote Wake-Up Packet When set to 1, this bit enables the AXI master to come out of the LPI mode only when the magic packet or remote wake-up packet is received."]
    pub mod LPI_XIT_PKT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlock on Magic Packet or Remote Wake-Up Packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unlock on Magic Packet or Remote Wake-Up Packet is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Enable Low Power Interface (LPI) When set to 1, this bit enables the LPI mode supported by the EQOS-AXI configuration and accepts the LPI request from the AXI System Clock controller."]
    pub mod EN_LPI {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low Power Interface (LPI) is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Low Power Interface (LPI) is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DMA Interrupt Status"]
pub mod DMA_INTERRUPT_STATUS {
    #[doc = "DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
    pub mod DC0IS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel 0 Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Channel 0 Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
    pub mod DC1IS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel 1 Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Channel 1 Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel 2 Interrupt Status This bit indicates an interrupt event in DMA Channel 2."]
    pub mod DC2IS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel 2 Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Channel 2 Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel 3 Interrupt Status This bit indicates an interrupt event in DMA Channel 3."]
    pub mod DC3IS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel 3 Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Channel 3 Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel 4 Interrupt Status This bit indicates an interrupt event in DMA Channel 4."]
    pub mod DC4IS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA Channel 4 Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Channel 4 Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MTL Interrupt Status This bit indicates an interrupt event in the MTL."]
    pub mod MTLIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MTL Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "MAC Interrupt Status This bit indicates an interrupt event in the MAC."]
    pub mod MACIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC Interrupt Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "DMA Debug Status 0"]
pub mod DMA_DEBUG_STATUS0 {
    #[doc = "AXI Master Write Channel When high, this bit indicates that the write channel of the AXI master is active, and it is transferring data."]
    pub mod AXWHSTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI Master Write Channel or AHB Master Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "AXI Master Write Channel or AHB Master Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "AXI Master Read Channel Status When high, this bit indicates that the read channel of the AXI master is active, and it is transferring the data."]
    pub mod AXRHSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI Master Read Channel Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "AXI Master Read Channel Status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "DMA Channel 0 Receive Process State This field indicates the Rx DMA FSM state for Channel 0."]
    pub mod RPS0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Receive Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Rx Transfer Descriptor)"]
            pub const RUN_FRTD: u32 = 0x01;
            #[doc = "Running (Waiting for Rx packet)"]
            pub const RUN_WRP: u32 = 0x03;
            #[doc = "Suspended (Rx Descriptor Unavailable)"]
            pub const SUSPND: u32 = 0x04;
            #[doc = "Running (Closing the Rx Descriptor)"]
            pub const RUN_CRD: u32 = 0x05;
            #[doc = "Timestamp write state"]
            pub const TSTMP: u32 = 0x06;
            #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
            pub const RUN_TRP: u32 = 0x07;
        }
    }
    #[doc = "DMA Channel 0 Transmit Process State This field indicates the Tx DMA FSM state for Channel 0."]
    pub mod TPS0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Tx Transfer Descriptor)"]
            pub const RUN_FTTD: u32 = 0x01;
            #[doc = "Running (Waiting for status)"]
            pub const RUN_WS: u32 = 0x02;
            #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
            pub const RUN_RDS: u32 = 0x03;
            #[doc = "Timestamp write state"]
            pub const TSTMP_WS: u32 = 0x04;
            #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
            pub const SUSPND: u32 = 0x06;
            #[doc = "Running (Closing Tx Descriptor)"]
            pub const RUN_CTD: u32 = 0x07;
        }
    }
    #[doc = "DMA Channel 1 Receive Process State This field indicates the Rx DMA FSM state for Channel 1."]
    pub mod RPS1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Receive Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Rx Transfer Descriptor)"]
            pub const RUN_FRTD: u32 = 0x01;
            #[doc = "Running (Waiting for Rx packet)"]
            pub const RUN_WRP: u32 = 0x03;
            #[doc = "Suspended (Rx Descriptor Unavailable)"]
            pub const SUSPND: u32 = 0x04;
            #[doc = "Running (Closing the Rx Descriptor)"]
            pub const RUN_CRD: u32 = 0x05;
            #[doc = "Timestamp write state"]
            pub const TSTMP: u32 = 0x06;
            #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
            pub const RUN_TRP: u32 = 0x07;
        }
    }
    #[doc = "DMA Channel 1 Transmit Process State This field indicates the Tx DMA FSM state for Channel 1."]
    pub mod TPS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Tx Transfer Descriptor)"]
            pub const RUN_FTTD: u32 = 0x01;
            #[doc = "Running (Waiting for status)"]
            pub const RUN_WS: u32 = 0x02;
            #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
            pub const RUN_RDS: u32 = 0x03;
            #[doc = "Timestamp write state"]
            pub const TSTMP_WS: u32 = 0x04;
            #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
            pub const SUSPND: u32 = 0x06;
            #[doc = "Running (Closing Tx Descriptor)"]
            pub const RUN_CTD: u32 = 0x07;
        }
    }
    #[doc = "DMA Channel 2 Receive Process State This field indicates the Rx DMA FSM state for Channel 2."]
    pub mod RPS2 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Receive Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Rx Transfer Descriptor)"]
            pub const RUN_FRTD: u32 = 0x01;
            #[doc = "Running (Waiting for Rx packet)"]
            pub const RUN_WRP: u32 = 0x03;
            #[doc = "Suspended (Rx Descriptor Unavailable)"]
            pub const SUSPND: u32 = 0x04;
            #[doc = "Running (Closing the Rx Descriptor)"]
            pub const RUN_CRD: u32 = 0x05;
            #[doc = "Timestamp write state"]
            pub const TSTMP: u32 = 0x06;
            #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
            pub const RUN_TRP: u32 = 0x07;
        }
    }
    #[doc = "DMA Channel 2 Transmit Process State This field indicates the Tx DMA FSM state for Channel 2."]
    pub mod TPS2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Tx Transfer Descriptor)"]
            pub const RUN_FTTD: u32 = 0x01;
            #[doc = "Running (Waiting for status)"]
            pub const RUN_WS: u32 = 0x02;
            #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
            pub const RUN_RDS: u32 = 0x03;
            #[doc = "Timestamp write state"]
            pub const TSTMP_WS: u32 = 0x04;
            #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
            pub const SUSPND: u32 = 0x06;
            #[doc = "Running (Closing Tx Descriptor)"]
            pub const RUN_CTD: u32 = 0x07;
        }
    }
}
#[doc = "DMA Debug Status 1"]
pub mod DMA_DEBUG_STATUS1 {
    #[doc = "DMA Channel 3 Receive Process State This field indicates the Rx DMA FSM state for Channel 3."]
    pub mod RPS3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Receive Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Rx Transfer Descriptor)"]
            pub const RUN_FRTD: u32 = 0x01;
            #[doc = "Running (Waiting for Rx packet)"]
            pub const RUN_WRP: u32 = 0x03;
            #[doc = "Suspended (Rx Descriptor Unavailable)"]
            pub const SUSPND: u32 = 0x04;
            #[doc = "Running (Closing the Rx Descriptor)"]
            pub const RUN_CRD: u32 = 0x05;
            #[doc = "Timestamp write state"]
            pub const TSTMP: u32 = 0x06;
            #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
            pub const RUN_TRP: u32 = 0x07;
        }
    }
    #[doc = "DMA Channel 3 Transmit Process State This field indicates the Tx DMA FSM state for Channel 3."]
    pub mod TPS3 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Tx Transfer Descriptor)"]
            pub const RUN_FTTD: u32 = 0x01;
            #[doc = "Running (Waiting for status)"]
            pub const RUN_WS: u32 = 0x02;
            #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
            pub const RUN_RDS: u32 = 0x03;
            #[doc = "Timestamp write state"]
            pub const TSTMP_WS: u32 = 0x04;
            #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
            pub const SUSPND: u32 = 0x06;
            #[doc = "Running (Closing Tx Descriptor)"]
            pub const RUN_CTD: u32 = 0x07;
        }
    }
    #[doc = "DMA Channel 4 Receive Process State This field indicates the Rx DMA FSM state for Channel 4."]
    pub mod RPS4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Receive Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Rx Transfer Descriptor)"]
            pub const RUN_FRTD: u32 = 0x01;
            #[doc = "Running (Waiting for Rx packet)"]
            pub const RUN_WRP: u32 = 0x03;
            #[doc = "Suspended (Rx Descriptor Unavailable)"]
            pub const SUSPND: u32 = 0x04;
            #[doc = "Running (Closing the Rx Descriptor)"]
            pub const RUN_CRD: u32 = 0x05;
            #[doc = "Timestamp write state"]
            pub const TSTMP: u32 = 0x06;
            #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
            pub const RUN_TRP: u32 = 0x07;
        }
    }
    #[doc = "DMA Channel 4 Transmit Process State This field indicates the Tx DMA FSM state for Channel 4."]
    pub mod TPS4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Tx Transfer Descriptor)"]
            pub const RUN_FTTD: u32 = 0x01;
            #[doc = "Running (Waiting for status)"]
            pub const RUN_WS: u32 = 0x02;
            #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
            pub const RUN_RDS: u32 = 0x03;
            #[doc = "Timestamp write state"]
            pub const TSTMP_WS: u32 = 0x04;
            #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
            pub const SUSPND: u32 = 0x06;
            #[doc = "Running (Closing Tx Descriptor)"]
            pub const RUN_CTD: u32 = 0x07;
        }
    }
}
#[doc = "AXI LPI Entry Interval Control"]
pub mod DMA_AXI_LPI_ENTRY_INTERVAL {
    #[doc = "LPI Entry Interval Contains the number of system clock cycles, multiplied by 64, to wait for an activity in the DWC_ether_qos to enter into the AXI low power state 0 indicates 64 clock cycles"]
    pub mod LPIEI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TBS Control"]
pub mod DMA_TBS_CTRL {
    #[doc = "Fetch Time Offset Valid When set indicates the FTOS field is valid."]
    pub mod FTOV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fetch Time Offset is invalid"]
            pub const INVALID: u32 = 0;
            #[doc = "Fetch Time Offset is valid"]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Fetch GSN Offset The number GSN slots that must be deducted from the Launch GSN to compute the Fetch GSN."]
    pub mod FGOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fetch Time Offset The value in units of 256 nanoseconds, that has to be deducted from the Launch time to compute the Fetch Time."]
    pub mod FTOS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 0 Control"]
pub mod DMA_CH0_CONTROL {
    #[doc = "8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\] in DMA_CH0_TX_CONTROL and Bits\\[21:16\\] in DMA_CH0_RX_CONTROL is multiplied by eight times."]
    pub mod PBLX8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8xPBL mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "8xPBL mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Descriptor Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors."]
    pub mod DSL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 0 Transmit Control"]
pub mod DMA_CH0_TX_CONTROL {
    #[doc = "Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    pub mod ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Transmission Command"]
            pub const STOP: u32 = 0;
            #[doc = "Start Transmission Command"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    pub mod OSF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Operate on Second Packet disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Operate on Second Packet enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Ignore PBL Requirement When this bit is set, the DMA does not check for PBL number of locations in the MTL before initiating a transfer."]
    pub mod IPBL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Ignore PBL Requirement is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Ignore PBL Requirement is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod TXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Descriptor Enable When this bit is set, the corresponding channel uses Enhanced Descriptors that are 32 Bytes for both Normal and Context Descriptors."]
    pub mod EDSE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Descriptor is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Descriptor is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DMA Channel 0 Receive Control"]
pub mod DMA_CH0_RX_CONTROL {
    #[doc = "Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets."]
    pub mod SR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Receive"]
            pub const STOP: u32 = 0;
            #[doc = "Start Receive"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer size Low RBSZ\\[13:0\\] is split into two fields RBSZ_13_y and RBSZ_x_0."]
    pub mod RBSZ_X_0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Buffer size High RBSZ\\[13:0\\] is split into two fields higher RBSZ_13_y and lower RBSZ_x_0."]
    pub mod RBSZ_13_Y {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod RXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Packet Flush."]
    pub mod RPF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Packet Flush is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Rx Packet Flush is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 0 Tx Descriptor List Address register"]
pub mod DMA_CH0_TXDESC_LIST_ADDRESS {
    #[doc = "Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list."]
    pub mod TDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Rx Descriptor List Address register"]
pub mod DMA_CH0_RXDESC_LIST_ADDRESS {
    #[doc = "Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list."]
    pub mod RDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Tx Descriptor Tail Pointer"]
pub mod DMA_CH0_TXDESC_TAIL_POINTER {
    #[doc = "Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring."]
    pub mod TDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Rx Descriptor Tail Pointer"]
pub mod DMA_CH0_RXDESC_TAIL_POINTER {
    #[doc = "Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring."]
    pub mod RDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Tx Descriptor Ring Length"]
pub mod DMA_CH0_TXDESC_RING_LENGTH {
    #[doc = "Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring."]
    pub mod TDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Rx Descriptor Ring Length"]
pub mod DMA_CH0_RXDESC_RING_LENGTH {
    #[doc = "Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring."]
    pub mod RDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Interrupt Enable"]
pub mod DMA_CH0_INTERRUPT_ENABLE {
    #[doc = "Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled."]
    pub mod TIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled."]
    pub mod TXSE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled."]
    pub mod TBUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled."]
    pub mod RIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled."]
    pub mod RBUE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled."]
    pub mod RSE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled."]
    pub mod RWTE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Watchdog Timeout is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled."]
    pub mod ETIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled."]
    pub mod ERIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled."]
    pub mod FBEE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Fatal Bus Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error Enable When this bit is set along with the AIE bit, the Descriptor error interrupt is enabled."]
    pub mod CDEE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Context Descriptor Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled."]
    pub mod AIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled."]
    pub mod NIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Normal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 0 Receive Interrupt Watchdog Timer"]
pub mod DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER {
    #[doc = "Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set."]
    pub mod RWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units This fields indicates the number of system clock cycles corresponding to one unit in RWT field."]
    pub mod RWTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Slot Function Control and Status"]
pub mod DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS {
    #[doc = "Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    pub mod ESC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slot Comparison is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slot Comparison is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is - equal to the reference slot number given in the RSN field or - ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    pub mod ASC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Advance Slot Check is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Advance Slot Check is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Interval Value This field controls the period of the slot interval in which the TxDMA fetches the scheduled packets."]
    pub mod SIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
    pub mod RSN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Current Application Transmit Descriptor"]
pub mod DMA_CH0_CURRENT_APP_TXDESC {
    #[doc = "Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Current Application Receive Descriptor"]
pub mod DMA_CH0_CURRENT_APP_RXDESC {
    #[doc = "Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Current Application Transmit Buffer Address"]
pub mod DMA_CH0_CURRENT_APP_TXBUFFER {
    #[doc = "Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Current Application Receive Buffer Address"]
pub mod DMA_CH0_CURRENT_APP_RXBUFFER {
    #[doc = "Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 0 Status"]
pub mod DMA_CH0_STATUS {
    #[doc = "Transmit Interrupt This bit indicates that the packet transmission is complete."]
    pub mod TI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Process Stopped This bit is set when the transmission is stopped."]
    pub mod TPS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it."]
    pub mod TBU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt This bit indicates that the packet reception is complete."]
    pub mod RI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it."]
    pub mod RBU {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    pub mod RPS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    pub mod RWT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Watchdog Timeout status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt This bit when set indicates that the TxDMA has completed the transfer of packet data to the MTL TXFIFO memory."]
    pub mod ETI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt This bit when set indicates that the RxDMA has completed the transfer of packet data to the memory."]
    pub mod ERI {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    pub mod FBE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Fatal Bus Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow (intermediate descriptor) or all ones descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
    pub mod CDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Context Descriptor Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the INTERRUPT_ENABLE register: - Bit 1: Transmit Process Stopped - Bit 7: Receive Buffer Unavailable - Bit 8: Receive Process Stopped - Bit 10: Early Transmit Interrupt - Bit 12: Fatal Bus Error - Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    pub mod AIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA_CH0_INTERRUPT_ENABLE register: - Bit 0: Transmit Interrupt - Bit 2: Transmit Buffer Unavailable - Bit 6: Receive Interrupt - Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in INTERRUPT_ENABLE register) affect the Normal Interrupt Summary bit."]
    pub mod NIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Normal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Tx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod TEB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod REB {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 0 Missed Frame Counter"]
pub mod DMA_CH0_MISS_FRAME_CNT {
    #[doc = "Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programming RPF field in DMA_CH0_RX_CONTROL register."]
    pub mod MFC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further."]
    pub mod MFCO {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Miss Frame Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Miss Frame Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 0 RXP Frames Accepted Counter"]
pub mod DMA_CH0_RXP_ACCEPT_CNT {
    #[doc = "Rx Parser Accept Counter This 31-bit counter is implemented whenever a Rx Parser Accept a packet due to AF =1."]
    pub mod RXPAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Parser Accept Counter Overflow Bit When set, this bit indicates that the RXPAC Counter field crossed the maximum limit."]
    pub mod RXPACOF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Parser Accept Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Parser Accept Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 0 Receive ERI Counter"]
pub mod DMA_CH0_RX_ERI_CNT {
    #[doc = "ERI Counter When ERIC bit of RX_CONTROL register is set, this counter increments for burst transfer completed by the Rx DMA from the start of packet transfer."]
    pub mod ECNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 1 Control"]
pub mod DMA_CH1_CONTROL {
    #[doc = "8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\] in TX_CONTROL and Bits\\[21:16\\] in DMA_CH1_RX_CONTROL is multiplied by eight times."]
    pub mod PBLX8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8xPBL mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "8xPBL mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Descriptor Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors."]
    pub mod DSL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 1 Transmit Control"]
pub mod DMA_CH1_TX_CONTROL {
    #[doc = "Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    pub mod ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Transmission Command"]
            pub const STOP: u32 = 0;
            #[doc = "Start Transmission Command"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    pub mod OSF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Operate on Second Packet disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Operate on Second Packet enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Ignore PBL Requirement When this bit is set, the DMA does not check for PBL number of locations in the MTL before initiating a transfer."]
    pub mod IPBL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Ignore PBL Requirement is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Ignore PBL Requirement is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod TXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Descriptor Enable When this bit is set, the corresponding channel uses Enhanced Descriptors that are 32 Bytes for both Normal and Context Descriptors."]
    pub mod EDSE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Descriptor is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Descriptor is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DMA Channel 1 Receive Control"]
pub mod DMA_CH1_RX_CONTROL {
    #[doc = "Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets."]
    pub mod SR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Receive"]
            pub const STOP: u32 = 0;
            #[doc = "Start Receive"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer size Low RBSZ\\[13:0\\] is split into two fields RBSZ_13_y and RBSZ_x_0."]
    pub mod RBSZ_X_0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Buffer size High RBSZ\\[13:0\\] is split into two fields higher RBSZ_13_y and lower RBSZ_x_0."]
    pub mod RBSZ_13_Y {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod RXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Packet Flush."]
    pub mod RPF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Packet Flush is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Rx Packet Flush is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 1 Tx Descriptor List Address"]
pub mod DMA_CH1_TXDESC_LIST_ADDRESS {
    #[doc = "Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list."]
    pub mod TDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Rx Descriptor List Address"]
pub mod DMA_CH1_RXDESC_LIST_ADDRESS {
    #[doc = "Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list."]
    pub mod RDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Tx Descriptor Tail Pointer"]
pub mod DMA_CH1_TXDESC_TAIL_POINTER {
    #[doc = "Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring."]
    pub mod TDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Rx Descriptor Tail Pointer"]
pub mod DMA_CH1_RXDESC_TAIL_POINTER {
    #[doc = "Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring."]
    pub mod RDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Tx Descriptor Ring Length"]
pub mod DMA_CH1_TXDESC_RING_LENGTH {
    #[doc = "Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring."]
    pub mod TDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Rx Descriptor Ring Length"]
pub mod DMA_CH1_RXDESC_RING_LENGTH {
    #[doc = "Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring."]
    pub mod RDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Interrupt Enable"]
pub mod DMA_CH1_INTERRUPT_ENABLE {
    #[doc = "Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled."]
    pub mod TIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled."]
    pub mod TXSE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled."]
    pub mod TBUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled."]
    pub mod RIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled."]
    pub mod RBUE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled."]
    pub mod RSE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled."]
    pub mod RWTE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Watchdog Timeout is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled."]
    pub mod ETIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled."]
    pub mod ERIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled."]
    pub mod FBEE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Fatal Bus Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error Enable When this bit is set along with the AIE bit, the Descriptor error interrupt is enabled."]
    pub mod CDEE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Context Descriptor Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled."]
    pub mod AIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled."]
    pub mod NIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Normal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 1 Receive Interrupt Watchdog Timer"]
pub mod DMA_CH1_RX_INTERRUPT_WATCHDOG_TIMER {
    #[doc = "Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set."]
    pub mod RWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units This fields indicates the number of system clock cycles corresponding to one unit in RWT field."]
    pub mod RWTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Slot Function Control and Status"]
pub mod DMA_CH1_SLOT_FUNCTION_CONTROL_STATUS {
    #[doc = "Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    pub mod ESC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slot Comparison is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slot Comparison is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is - equal to the reference slot number given in the RSN field or - ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    pub mod ASC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Advance Slot Check is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Advance Slot Check is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Interval Value This field controls the period of the slot interval in which the TxDMA fetches the scheduled packets."]
    pub mod SIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
    pub mod RSN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Current Application Transmit Descriptor"]
pub mod DMA_CH1_CURRENT_APP_TXDESC {
    #[doc = "Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Current Application Receive Descriptor"]
pub mod DMA_CH1_CURRENT_APP_RXDESC {
    #[doc = "Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Current Application Transmit Buffer Address"]
pub mod DMA_CH1_CURRENT_APP_TXBUFFER {
    #[doc = "Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Current Application Receive Buffer Address"]
pub mod DMA_CH1_CURRENT_APP_RXBUFFER {
    #[doc = "Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 1 Status"]
pub mod DMA_CH1_STATUS {
    #[doc = "Transmit Interrupt This bit indicates that the packet transmission is complete."]
    pub mod TI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Process Stopped This bit is set when the transmission is stopped."]
    pub mod TPS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it."]
    pub mod TBU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt This bit indicates that the packet reception is complete."]
    pub mod RI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it."]
    pub mod RBU {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    pub mod RPS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    pub mod RWT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Watchdog Timeout status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt This bit when set indicates that the TxDMA has completed the transfer of packet data to the MTL TXFIFO memory."]
    pub mod ETI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt This bit when set indicates that the RxDMA has completed the transfer of packet data to the memory."]
    pub mod ERI {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    pub mod FBE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Fatal Bus Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow ( intermediate descriptor) or all one's descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
    pub mod CDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Context Descriptor Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the INTERRUPT_ENABLE register: - Bit 1: Transmit Process Stopped - Bit 7: Receive Buffer Unavailable - Bit 8: Receive Process Stopped - Bit 10: Early Transmit Interrupt - Bit 12: Fatal Bus Error - Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    pub mod AIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the INTERRUPT_ENABLE register: - Bit 0: Transmit Interrupt - Bit 2: Transmit Buffer Unavailable - Bit 6: Receive Interrupt - Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in INTERRUPT_ENABLE register) affect the Normal Interrupt Summary bit."]
    pub mod NIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Normal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Tx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod TEB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod REB {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 1 Missed Frame Counter"]
pub mod DMA_CH1_MISS_FRAME_CNT {
    #[doc = "Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programming RPF field in RX_CONTROL register."]
    pub mod MFC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further."]
    pub mod MFCO {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Miss Frame Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Miss Frame Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 1 RXP Frames Accepted Counter"]
pub mod DMA_CH1_RXP_ACCEPT_CNT {
    #[doc = "Rx Parser Accept Counter This 31-bit counter is implemented whenever a Rx Parser Accept a packet due to AF =1."]
    pub mod RXPAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Parser Accept Counter Overflow Bit When set, this bit indicates that the RXPAC Counter field crossed the maximum limit."]
    pub mod RXPACOF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Parser Accept Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Parser Accept Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 1 Receive ERI Counter"]
pub mod DMA_CH1_RX_ERI_CNT {
    #[doc = "ERI Counter When ERIC bit of RX_CONTROL register is set, this counter increments for burst transfer completed by the Rx DMA from the start of packet transfer."]
    pub mod ECNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 2 Control"]
pub mod DMA_CH2_CONTROL {
    #[doc = "8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\] in DMA_CH2_TX_CONTROL and Bits\\[21:16\\] in DMA_CH2_RX_CONTROL is multiplied by eight times."]
    pub mod PBLX8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8xPBL mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "8xPBL mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Descriptor Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors."]
    pub mod DSL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 2 Transmit Control"]
pub mod DMA_CH2_TX_CONTROL {
    #[doc = "Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    pub mod ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Transmission Command"]
            pub const STOP: u32 = 0;
            #[doc = "Start Transmission Command"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    pub mod OSF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Operate on Second Packet disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Operate on Second Packet enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Ignore PBL Requirement When this bit is set, the DMA does not check for PBL number of locations in the MTL before initiating a transfer."]
    pub mod IPBL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Ignore PBL Requirement is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Ignore PBL Requirement is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod TXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Descriptor Enable When this bit is set, the corresponding channel uses Enhanced Descriptors that are 32 Bytes for both Normal and Context Descriptors."]
    pub mod EDSE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Descriptor is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Descriptor is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DMA Channel 2 Receive Control"]
pub mod DMA_CH2_RX_CONTROL {
    #[doc = "Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets."]
    pub mod SR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Receive"]
            pub const STOP: u32 = 0;
            #[doc = "Start Receive"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer size Low RBSZ\\[13:0\\] is split into two fields RBSZ_13_y and RBSZ_x_0."]
    pub mod RBSZ_X_0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Buffer size High RBSZ\\[13:0\\] is split into two fields higher RBSZ_13_y and lower RBSZ_x_0."]
    pub mod RBSZ_13_Y {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod RXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Packet Flush."]
    pub mod RPF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Packet Flush is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Rx Packet Flush is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 2 Tx Descriptor List Address"]
pub mod DMA_CH2_TXDESC_LIST_ADDRESS {
    #[doc = "Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list."]
    pub mod TDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Rx Descriptor List Address"]
pub mod DMA_CH2_RXDESC_LIST_ADDRESS {
    #[doc = "Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list."]
    pub mod RDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Tx Descriptor Tail Pointer"]
pub mod DMA_CH2_TXDESC_TAIL_POINTER {
    #[doc = "Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring."]
    pub mod TDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Rx Descriptor Tail Pointer"]
pub mod DMA_CH2_RXDESC_TAIL_POINTER {
    #[doc = "Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring."]
    pub mod RDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Tx Descriptor Ring Length"]
pub mod DMA_CH2_TXDESC_RING_LENGTH {
    #[doc = "Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring."]
    pub mod TDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Rx Descriptor Ring Length"]
pub mod DMA_CH2_RXDESC_RING_LENGTH {
    #[doc = "Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring."]
    pub mod RDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Interrupt Enable"]
pub mod DMA_CH2_INTERRUPT_ENABLE {
    #[doc = "Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled."]
    pub mod TIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled."]
    pub mod TXSE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled."]
    pub mod TBUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled."]
    pub mod RIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled."]
    pub mod RBUE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled."]
    pub mod RSE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled."]
    pub mod RWTE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Watchdog Timeout is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled."]
    pub mod ETIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled."]
    pub mod ERIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled."]
    pub mod FBEE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Fatal Bus Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error Enable When this bit is set along with the AIE bit, the Descriptor error interrupt is enabled."]
    pub mod CDEE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Context Descriptor Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled."]
    pub mod AIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled."]
    pub mod NIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Normal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 2 Receive Interrupt Watchdog Timer"]
pub mod DMA_CH2_RX_INTERRUPT_WATCHDOG_TIMER {
    #[doc = "Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set."]
    pub mod RWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units This fields indicates the number of system clock cycles corresponding to one unit in RWT field."]
    pub mod RWTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Slot Function Control and Status"]
pub mod DMA_CH2_SLOT_FUNCTION_CONTROL_STATUS {
    #[doc = "Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    pub mod ESC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slot Comparison is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slot Comparison is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is - equal to the reference slot number given in the RSN field or - ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    pub mod ASC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Advance Slot Check is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Advance Slot Check is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Interval Value This field controls the period of the slot interval in which the TxDMA fetches the scheduled packets."]
    pub mod SIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
    pub mod RSN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Current Application Transmit Descriptor"]
pub mod DMA_CH2_CURRENT_APP_TXDESC {
    #[doc = "Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Current Application Receive Descriptor"]
pub mod DMA_CH2_CURRENT_APP_RXDESC {
    #[doc = "Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Current Application Transmit Buffer Address"]
pub mod DMA_CH2_CURRENT_APP_TXBUFFER {
    #[doc = "Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Current Application Receive Buffer Address"]
pub mod DMA_CH2_CURRENT_APP_RXBUFFER {
    #[doc = "Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 2 Status"]
pub mod DMA_CH2_STATUS {
    #[doc = "Transmit Interrupt This bit indicates that the packet transmission is complete."]
    pub mod TI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Process Stopped This bit is set when the transmission is stopped."]
    pub mod TPS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it."]
    pub mod TBU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt This bit indicates that the packet reception is complete."]
    pub mod RI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it."]
    pub mod RBU {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    pub mod RPS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    pub mod RWT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Watchdog Timeout status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt This bit when set indicates that the TxDMA has completed the transfer of packet data to the MTL TXFIFO memory."]
    pub mod ETI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt This bit when set indicates that the RxDMA has completed the transfer of packet data to the memory."]
    pub mod ERI {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    pub mod FBE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Fatal Bus Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow ( intermediate descriptor) or all one's descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
    pub mod CDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Context Descriptor Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA_CH2_INTERRUPT_ENABLE register: - Bit 1: Transmit Process Stopped - Bit 7: Receive Buffer Unavailable - Bit 8: Receive Process Stopped - Bit 10: Early Transmit Interrupt - Bit 12: Fatal Bus Error - Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    pub mod AIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the INTERRUPT_ENABLE register: - Bit 0: Transmit Interrupt - Bit 2: Transmit Buffer Unavailable - Bit 6: Receive Interrupt - Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in DMA_CH2_INTERRUPT_ENABLE register) affect the Normal Interrupt Summary bit."]
    pub mod NIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Normal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Tx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod TEB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod REB {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 2 Missed Frame Counter"]
pub mod DMA_CH2_MISS_FRAME_CNT {
    #[doc = "Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programming RPF field in DMA_CH2_RX_CONTROL register."]
    pub mod MFC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further."]
    pub mod MFCO {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Miss Frame Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Miss Frame Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 2 RXP Frames Accepted Counter"]
pub mod DMA_CH2_RXP_ACCEPT_CNT {
    #[doc = "Rx Parser Accept Counter This 31-bit counter is implemented whenever a Rx Parser Accept a packet due to AF =1."]
    pub mod RXPAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Parser Accept Counter Overflow Bit When set, this bit indicates that the RXPAC Counter field crossed the maximum limit."]
    pub mod RXPACOF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Parser Accept Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Parser Accept Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 2 Receive ERI Counter"]
pub mod DMA_CH2_RX_ERI_CNT {
    #[doc = "ERI Counter When ERIC bit of DMA_CH2_RX_CONTROL register is set, this counter increments for burst transfer completed by the Rx DMA from the start of packet transfer."]
    pub mod ECNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 3 Control"]
pub mod DMA_CH3_CONTROL {
    #[doc = "8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\] in DMA_CH3_TX_CONTROL and Bits\\[21:16\\] in DMA_CH3_RX_CONTROL is multiplied by eight times."]
    pub mod PBLX8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8xPBL mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "8xPBL mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Descriptor Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors."]
    pub mod DSL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 3 Transmit Control"]
pub mod DMA_CH3_TX_CONTROL {
    #[doc = "Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    pub mod ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Transmission Command"]
            pub const STOP: u32 = 0;
            #[doc = "Start Transmission Command"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    pub mod OSF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Operate on Second Packet disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Operate on Second Packet enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Ignore PBL Requirement When this bit is set, the DMA does not check for PBL number of locations in the MTL before initiating a transfer."]
    pub mod IPBL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Ignore PBL Requirement is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Ignore PBL Requirement is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod TXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Descriptor Enable When this bit is set, the corresponding channel uses Enhanced Descriptors that are 32 Bytes for both Normal and Context Descriptors."]
    pub mod EDSE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Descriptor is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Descriptor is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DMA Channel 3 Receive Control"]
pub mod DMA_CH3_RX_CONTROL {
    #[doc = "Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets."]
    pub mod SR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Receive"]
            pub const STOP: u32 = 0;
            #[doc = "Start Receive"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer size Low RBSZ\\[13:0\\] is split into two fields RBSZ_13_y and RBSZ_x_0."]
    pub mod RBSZ_X_0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Buffer size High RBSZ\\[13:0\\] is split into two fields higher RBSZ_13_y and lower RBSZ_x_0."]
    pub mod RBSZ_13_Y {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod RXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Packet Flush."]
    pub mod RPF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Packet Flush is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Rx Packet Flush is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 3 Tx Descriptor List Address"]
pub mod DMA_CH3_TXDESC_LIST_ADDRESS {
    #[doc = "Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list."]
    pub mod TDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Rx Descriptor List Address"]
pub mod DMA_CH3_RXDESC_LIST_ADDRESS {
    #[doc = "Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list."]
    pub mod RDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Tx Descriptor Tail Pointer"]
pub mod DMA_CH3_TXDESC_TAIL_POINTER {
    #[doc = "Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring."]
    pub mod TDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Rx Descriptor Tail Pointer"]
pub mod DMA_CH3_RXDESC_TAIL_POINTER {
    #[doc = "Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring."]
    pub mod RDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Tx Descriptor Ring Length"]
pub mod DMA_CH3_TXDESC_RING_LENGTH {
    #[doc = "Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring."]
    pub mod TDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Rx Descriptor Ring Length"]
pub mod DMA_CH3_RXDESC_RING_LENGTH {
    #[doc = "Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring."]
    pub mod RDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Interrupt Enable"]
pub mod DMA_CH3_INTERRUPT_ENABLE {
    #[doc = "Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled."]
    pub mod TIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled."]
    pub mod TXSE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled."]
    pub mod TBUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled."]
    pub mod RIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled."]
    pub mod RBUE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled."]
    pub mod RSE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled."]
    pub mod RWTE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Watchdog Timeout is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled."]
    pub mod ETIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled."]
    pub mod ERIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled."]
    pub mod FBEE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Fatal Bus Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error Enable When this bit is set along with the AIE bit, the Descriptor error interrupt is enabled."]
    pub mod CDEE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Context Descriptor Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled."]
    pub mod AIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled."]
    pub mod NIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Normal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 3 Receive Interrupt Watchdog Time"]
pub mod DMA_CH3_RX_INTERRUPT_WATCHDOG_TIMER {
    #[doc = "Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set."]
    pub mod RWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units This fields indicates the number of system clock cycles corresponding to one unit in RWT field."]
    pub mod RWTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Slot Function Control and Status"]
pub mod DMA_CH3_SLOT_FUNCTION_CONTROL_STATUS {
    #[doc = "Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    pub mod ESC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slot Comparison is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slot Comparison is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is - equal to the reference slot number given in the RSN field or - ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    pub mod ASC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Advance Slot Check is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Advance Slot Check is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Interval Value This field controls the period of the slot interval in which the TxDMA fetches the scheduled packets."]
    pub mod SIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
    pub mod RSN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Current Application Transmit Descriptor"]
pub mod DMA_CH3_CURRENT_APP_TXDESC {
    #[doc = "Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Current Application Receive Descriptor"]
pub mod DMA_CH3_CURRENT_APP_RXDESC {
    #[doc = "Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Current Application Transmit Buffer Address"]
pub mod DMA_CH3_CURRENT_APP_TXBUFFER {
    #[doc = "Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Current Application Receive Buffer Address"]
pub mod DMA_CH3_CURRENT_APP_RXBUFFER {
    #[doc = "Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 3 Status"]
pub mod DMA_CH3_STATUS {
    #[doc = "Transmit Interrupt This bit indicates that the packet transmission is complete."]
    pub mod TI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Process Stopped This bit is set when the transmission is stopped."]
    pub mod TPS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it."]
    pub mod TBU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt This bit indicates that the packet reception is complete."]
    pub mod RI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it."]
    pub mod RBU {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    pub mod RPS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    pub mod RWT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Watchdog Timeout status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt This bit when set indicates that the TxDMA has completed the transfer of packet data to the MTL TXFIFO memory."]
    pub mod ETI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt This bit when set indicates that the RxDMA has completed the transfer of packet data to the memory."]
    pub mod ERI {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    pub mod FBE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Fatal Bus Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow ( intermediate descriptor) or all one's descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
    pub mod CDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Context Descriptor Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA_CH3_INTERRUPT_ENABLE register: - Bit 1: Transmit Process Stopped - Bit 7: Receive Buffer Unavailable - Bit 8: Receive Process Stopped - Bit 10: Early Transmit Interrupt - Bit 12: Fatal Bus Error - Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    pub mod AIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA_CH3_INTERRUPT_ENABLE register: - Bit 0: Transmit Interrupt - Bit 2: Transmit Buffer Unavailable - Bit 6: Receive Interrupt - Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in DMA_CH3_INTERRUPT_ENABLE register) affect the Normal Interrupt Summary bit."]
    pub mod NIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Normal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Tx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod TEB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod REB {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 3 Missed Frame Counter"]
pub mod DMA_CH3_MISS_FRAME_CNT {
    #[doc = "Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programming RPF field in RX_CONTROL register."]
    pub mod MFC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further."]
    pub mod MFCO {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Miss Frame Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Miss Frame Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 3 RXP Frames Accepted Counter"]
pub mod DMA_CH3_RXP_ACCEPT_CNT {
    #[doc = "Rx Parser Accept Counter This 31-bit counter is implemented whenever a Rx Parser Accept a packet due to AF =1."]
    pub mod RXPAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Parser Accept Counter Overflow Bit When set, this bit indicates that the RXPAC Counter field crossed the maximum limit."]
    pub mod RXPACOF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Parser Accept Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Parser Accept Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 3 Receive ERI Counter"]
pub mod DMA_CH3_RX_ERI_CNT {
    #[doc = "ERI Counter When ERIC bit of DMA_CH3_RX_CONTROL register is set, this counter increments for burst transfer completed by the Rx DMA from the start of packet transfer."]
    pub mod ECNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 4 Control"]
pub mod DMA_CH4_CONTROL {
    #[doc = "8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\] in DMA_CH4_TX_CONTROL and Bits\\[21:16\\] in DMA_CH4_RX_CONTROL is multiplied by eight times."]
    pub mod PBLX8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8xPBL mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "8xPBL mode is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Descriptor Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors."]
    pub mod DSL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 4 Transmit Control"]
pub mod DMA_CH4_TX_CONTROL {
    #[doc = "Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    pub mod ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Transmission Command"]
            pub const STOP: u32 = 0;
            #[doc = "Start Transmission Command"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    pub mod OSF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Operate on Second Packet disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Operate on Second Packet enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Ignore PBL Requirement When this bit is set, the DMA does not check for PBL number of locations in the MTL before initiating a transfer."]
    pub mod IPBL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Ignore PBL Requirement is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Ignore PBL Requirement is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod TXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Descriptor Enable When this bit is set, the corresponding channel uses Enhanced Descriptors that are 32 Bytes for both Normal and Context Descriptors."]
    pub mod EDSE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Descriptor is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Descriptor is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "DMA Channel 4 Receive Control"]
pub mod DMA_CH4_RX_CONTROL {
    #[doc = "Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets."]
    pub mod SR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop Receive"]
            pub const STOP: u32 = 0;
            #[doc = "Start Receive"]
            pub const START: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer size Low RBSZ\\[13:0\\] is split into two fields RBSZ_13_y and RBSZ_x_0."]
    pub mod RBSZ_X_0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Buffer size High RBSZ\\[13:0\\] is split into two fields higher RBSZ_13_y and lower RBSZ_x_0."]
    pub mod RBSZ_13_Y {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer."]
    pub mod RXPBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Packet Flush."]
    pub mod RPF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Packet Flush is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Rx Packet Flush is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 4 Tx Descriptor List Address"]
pub mod DMA_CH4_TXDESC_LIST_ADDRESS {
    #[doc = "Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list."]
    pub mod TDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Rx Descriptor List Address"]
pub mod DMA_CH4_RXDESC_LIST_ADDRESS {
    #[doc = "Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list."]
    pub mod RDESLA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Tx Descriptor Tail Pointer"]
pub mod DMA_CH4_TXDESC_TAIL_POINTER {
    #[doc = "Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring."]
    pub mod TDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Rx Descriptor Tail Pointer"]
pub mod DMA_CH4_RXDESC_TAIL_POINTER {
    #[doc = "Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring."]
    pub mod RDTP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Tx Descriptor Ring Length"]
pub mod DMA_CH4_TXDESC_RING_LENGTH {
    #[doc = "Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring."]
    pub mod TDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Rx Descriptor Ring Length"]
pub mod DMA_CH4_RXDESC_RING_LENGTH {
    #[doc = "Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring."]
    pub mod RDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Interrupt Enable"]
pub mod DMA_CH4_INTERRUPT_ENABLE {
    #[doc = "Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled."]
    pub mod TIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled."]
    pub mod TXSE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled."]
    pub mod TBUE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled."]
    pub mod RIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled."]
    pub mod RBUE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Buffer Unavailable is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled."]
    pub mod RSE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Stopped is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Stopped is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled."]
    pub mod RWTE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Watchdog Timeout is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled."]
    pub mod ETIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Transmit Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled."]
    pub mod ERIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Early Receive Interrupt is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled."]
    pub mod FBEE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Fatal Bus Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error Enable When this bit is set along with the AIE bit, the Descriptor error interrupt is enabled."]
    pub mod CDEE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Context Descriptor Error is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled."]
    pub mod AIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled."]
    pub mod NIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Normal Interrupt Summary is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 4 Receive Interrupt Watchdog Timer"]
pub mod DMA_CH4_RX_INTERRUPT_WATCHDOG_TIMER {
    #[doc = "Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set."]
    pub mod RWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units This fields indicates the number of system clock cycles corresponding to one unit in RWT field."]
    pub mod RWTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Slot Function Control and Status"]
pub mod DMA_CH4_SLOT_FUNCTION_CONTROL_STATUS {
    #[doc = "Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    pub mod ESC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slot Comparison is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slot Comparison is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is - equal to the reference slot number given in the RSN field or - ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    pub mod ASC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Advance Slot Check is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Advance Slot Check is enabled"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Slot Interval Value This field controls the period of the slot interval in which the TxDMA fetches the scheduled packets."]
    pub mod SIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
    pub mod RSN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Current Application Transmit Descriptor"]
pub mod DMA_CH4_CURRENT_APP_TXDESC {
    #[doc = "Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Current Application Receive Descriptor"]
pub mod DMA_CH4_CURRENT_APP_RXDESC {
    #[doc = "Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRDESAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Current Application Transmit Buffer Address"]
pub mod DMA_CH4_CURRENT_APP_TXBUFFER {
    #[doc = "Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation."]
    pub mod CURTBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Current Application Receive Buffer Address"]
pub mod DMA_CH4_CURRENT_APP_RXBUFFER {
    #[doc = "Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation."]
    pub mod CURRBUFAPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Channel 4 Status"]
pub mod DMA_CH4_STATUS {
    #[doc = "Transmit Interrupt This bit indicates that the packet transmission is complete."]
    pub mod TI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Process Stopped This bit is set when the transmission is stopped."]
    pub mod TPS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it."]
    pub mod TBU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Interrupt This bit indicates that the packet reception is complete."]
    pub mod RI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it."]
    pub mod RBU {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Buffer Unavailable status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Buffer Unavailable status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    pub mod RPS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Process Stopped status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Process Stopped status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    pub mod RWT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Watchdog Timeout status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Watchdog Timeout status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Transmit Interrupt This bit when set indicates that the TxDMA has completed the transfer of packet data to the MTL TXFIFO memory."]
    pub mod ETI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Transmit Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Transmit Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Early Receive Interrupt This bit when set indicates that the RxDMA has completed the transfer of packet data to the memory."]
    pub mod ERI {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Early Receive Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Early Receive Interrupt status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    pub mod FBE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fatal Bus Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Fatal Bus Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow ( intermediate descriptor) or all one's descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid."]
    pub mod CDE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Context Descriptor Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Context Descriptor Error status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the INTERRUPT_ENABLE register: - Bit 1: Transmit Process Stopped - Bit 7: Receive Buffer Unavailable - Bit 8: Receive Process Stopped - Bit 10: Early Transmit Interrupt - Bit 12: Fatal Bus Error - Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    pub mod AIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abnormal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Abnormal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the INTERRUPT_ENABLE register: - Bit 0: Transmit Interrupt - Bit 2: Transmit Buffer Unavailable - Bit 6: Receive Interrupt - Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in INTERRUPT_ENABLE register) affect the Normal Interrupt Summary bit."]
    pub mod NIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal Interrupt Summary status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Normal Interrupt Summary status detected"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Tx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod TEB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    pub mod REB {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel 4 Missed Frame Counter"]
pub mod DMA_CH4_MISS_FRAME_CNT {
    #[doc = "Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programming RPF field in RX_CONTROL register."]
    pub mod MFC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further."]
    pub mod MFCO {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Miss Frame Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Miss Frame Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 4 RXP Frames Accepted Counter"]
pub mod DMA_CH4_RXP_ACCEPT_CNT {
    #[doc = "Rx Parser Accept Counter This 31-bit counter is implemented whenever a Rx Parser Accept a packet due to AF =1."]
    pub mod RXPAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Parser Accept Counter Overflow Bit When set, this bit indicates that the RXPAC Counter field crossed the maximum limit."]
    pub mod RXPACOF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Parser Accept Counter overflow not occurred"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Parser Accept Counter overflow occurred"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Channel 4 Receive ERI Counter"]
pub mod DMA_CH4_RX_ERI_CNT {
    #[doc = "ERI Counter When ERIC bit of DMA_CH4_RX_CONTROL register is set, this counter increments for burst transfer completed by the Rx DMA from the start of packet transfer."]
    pub mod ECNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
