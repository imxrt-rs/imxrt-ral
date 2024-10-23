#[doc = "NETC PCI Express ECAM VF config"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "PCI device ID and vendor ID register"]
    pub PCI_CFH_DID_VID: crate::RORegister<u32>,
    #[doc = "PCI command register"]
    pub PCI_CFH_CMD: crate::RWRegister<u16>,
    #[doc = "PCI status register"]
    pub PCI_CFH_STAT: crate::RORegister<u16>,
    #[doc = "PCI revision ID and classcode register"]
    pub PCI_CFH_REVID_CLASSCODE: crate::RORegister<u32>,
    #[doc = "PCI cache line size register"]
    pub PCI_CFH_CL_SIZE: crate::RORegister<u8>,
    #[doc = "PCI latency timer register"]
    pub PCI_CFH_LAT_TIMER: crate::RORegister<u8>,
    #[doc = "PCI header type register"]
    pub PCI_CFH_HDR_TYPE: crate::RORegister<u8>,
    #[doc = "PCI BIST register"]
    pub PCI_CFH_BIST: crate::RORegister<u8>,
    #[doc = "PCI base address register 0"]
    pub PCI_CFH_BAR0: crate::RORegister<u32>,
    #[doc = "PCI base address register 1"]
    pub PCI_CFH_BAR1: crate::RORegister<u32>,
    #[doc = "PCI base address register 2"]
    pub PCI_CFH_BAR2: crate::RORegister<u32>,
    #[doc = "PCI base address register 3"]
    pub PCI_CFH_BAR3: crate::RORegister<u32>,
    #[doc = "PCI base address register 4"]
    pub PCI_CFH_BAR4: crate::RORegister<u32>,
    #[doc = "PCI base address register 5"]
    pub PCI_CFH_BAR5: crate::RORegister<u32>,
    #[doc = "PCI cardbus CIS register"]
    pub PCI_CFH_CARDBUS_CIS: crate::RORegister<u32>,
    #[doc = "PCI subsystem vendor ID register"]
    pub PCI_CFH_SUBSYS_VID: crate::RORegister<u16>,
    #[doc = "PCI subsystem ID register"]
    pub PCI_CFH_SUBSYS_ID: crate::RORegister<u16>,
    #[doc = "PCI expansion ROM base address register"]
    pub PCI_CFH_EXP_ROM_BA: crate::RORegister<u32>,
    #[doc = "PCI capabilities pointer register"]
    pub PCI_CFH_CAP_PTR: crate::RORegister<u8>,
    _reserved0: [u8; 0x0b],
    #[doc = "PCI PCIe capabilities list register"]
    pub PCI_CFC_PCIE_CAP_LIST: crate::RORegister<u16>,
    #[doc = "PCI PCIe capabilities register"]
    pub PCI_CFC_PCIE_CAP: crate::RORegister<u16>,
    #[doc = "PCI PCIe device capabilities register"]
    pub PCI_CFC_PCIE_DEV_CAP: crate::RORegister<u32>,
    #[doc = "PCI PCIe device control register"]
    pub PCI_CFC_PCIE_DEV_CTL: crate::RWRegister<u16>,
    #[doc = "PCI PCIe device status register"]
    pub PCI_CFC_PCIE_DEV_STAT: crate::RORegister<u16>,
    _reserved1: [u8; 0x18],
    #[doc = "PCI PCIe device capabilities 2 register"]
    pub PCI_CFC_PCIE_DEV_CAP2: crate::RORegister<u32>,
    #[doc = "PCI PCIe device control 2 register"]
    pub PCI_CFC_PCIE_DEV_CTL2: crate::RORegister<u16>,
    _reserved2: [u8; 0x16],
    #[doc = "PCI MSI-X capabilities list register"]
    pub PCI_CFC_MSIX_CAP_LIST: crate::RORegister<u16>,
    #[doc = "PCI MSI-X message control register"]
    pub PCI_CFC_MSIX_MSG_CTL: crate::RWRegister<u16>,
    #[doc = "PCI MSI-X table offset/BIR register"]
    pub PCI_CFC_MSIX_TABLE_OFF_BIR: crate::RORegister<u32>,
    #[doc = "PCI MSI-X PBA offset/BIR register"]
    pub PCI_CFC_MSIX_PBA_OFF_BIR: crate::RORegister<u32>,
    _reserved3: [u8; 0x74],
    #[doc = "PCIe AER extended capability header"]
    pub PCIE_CFC_AER_EXT_CAP_HDR: crate::RORegister<u32>,
    #[doc = "PCIe AER uncorrectable error status register"]
    pub PCIE_CFC_AER_UCORR_ERR_STAT: crate::RWRegister<u32>,
    #[doc = "PCIe AER uncorrectable error mask register"]
    pub PCIE_CFC_AER_UCORR_ERR_MASK: crate::RWRegister<u32>,
    #[doc = "PCIe AER uncorrectable error severity register"]
    pub PCIE_CFC_AER_UCORR_ERR_SEV: crate::RWRegister<u32>,
    #[doc = "PCIe AER correctable error status register"]
    pub PCIE_CFC_AER_CORR_ERR_STAT: crate::RWRegister<u32>,
    #[doc = "PCIe AER correctable error mask register"]
    pub PCIE_CFC_AER_CORR_ERR_MASK: crate::RWRegister<u32>,
    #[doc = "PCIe AER capabilities and control register"]
    pub PCIE_CFC_AER_CAP_CTL: crate::RORegister<u32>,
    _reserved4: [u8; 0x14],
    #[doc = "PCIe ACS capability header"]
    pub PCIE_CFC_ACS_CAP_HDR: crate::RORegister<u32>,
    #[doc = "PCIe ACS capability register"]
    pub PCIE_CFC_ACS_CAP: crate::RORegister<u16>,
    #[doc = "PCIe ACS control register"]
    pub PCIE_CFC_ACS_CTL: crate::RORegister<u16>,
    _reserved5: [u8; 0x08],
    #[doc = "PCIe readiness time reporting capability header"]
    pub PCIE_CFC_RTR_CAP_HDR: crate::RORegister<u32>,
    #[doc = "PCIe RTR readiness time reporting 1 register"]
    pub PCIE_CFC_RTR_RTR1: crate::RORegister<u32>,
    #[doc = "PCIe RTR readiness time reporting 2 register"]
    pub PCIE_CFC_RTR_RTR2: crate::RORegister<u32>,
}
#[doc = "PCI device ID and vendor ID register"]
pub mod PCI_CFH_DID_VID {
    #[doc = "Vendor ID Returns 0xFFFF for all VFs."]
    pub mod VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device ID This field in all VFs returns FFFFh when read"]
    pub mod DEVICE_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI command register"]
pub mod PCI_CFH_CMD {
    #[doc = "Bus master enable Controls the ability of a PCI Express Endpoint to issue Memory and I/O Read/Write Requests"]
    pub mod BUS_MASTER_EN {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI status register"]
pub mod PCI_CFH_STAT {
    #[doc = "Capabilities List Indicates the presence of an Extended Capability list item"]
    pub mod CAP_LIST {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI revision ID and classcode register"]
pub mod PCI_CFH_REVID_CLASSCODE {
    #[doc = "Revision ID This register specifies a device specific revision identifier and is a vendor defined extension to the Device ID"]
    pub mod REV_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Class code Returns the same value as the PF."]
    pub mod CLASS_CODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI header type register"]
pub mod PCI_CFH_HDR_TYPE {
    #[doc = "Header type This field identifies the layout of the second part of the predefined header (beginning at byte 10h in Configuration Space)"]
    pub mod HDR_TYPE {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multi-function device For VFs this field is not applicable and is hardwired to 0b."]
    pub mod MULT_FUNC_DEV {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI base address register 0"]
pub mod PCI_CFH_BAR0 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_IO_IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_TYPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod PF_MEM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    pub mod ADDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI base address register 1"]
pub mod PCI_CFH_BAR1 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_IO_IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_TYPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod PF_MEM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    pub mod ADDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI base address register 2"]
pub mod PCI_CFH_BAR2 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_IO_IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_TYPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod PF_MEM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    pub mod ADDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI base address register 3"]
pub mod PCI_CFH_BAR3 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_IO_IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_TYPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod PF_MEM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    pub mod ADDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI base address register 4"]
pub mod PCI_CFH_BAR4 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_IO_IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_TYPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod PF_MEM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    pub mod ADDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI base address register 5"]
pub mod PCI_CFH_BAR5 {
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_IO_IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod MEM_TYPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 0h."]
    pub mod PF_MEM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EA BARs used. This register is hardwired to 000000h."]
    pub mod ADDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI cardbus CIS register"]
pub mod PCI_CFH_CARDBUS_CIS {
    #[doc = "Not applicable. This register is hardwired to 0000_0000h."]
    pub mod CARDBUS_CIS_PTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI subsystem vendor ID register"]
pub mod PCI_CFH_SUBSYS_VID {
    #[doc = "This Read Only field identifies the manufacturer of the subsystem"]
    pub mod SYBSYSTEM_VENDOR_ID {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI subsystem ID register"]
pub mod PCI_CFH_SUBSYS_ID {
    #[doc = "This Read Only field identifies the particular subsystem"]
    pub mod SYBSYSTEM_ID {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI expansion ROM base address register"]
pub mod PCI_CFH_EXP_ROM_BA {
    #[doc = "Not applicable. This register is hardwired to 0000_0000h."]
    pub mod EXP_ROM_BA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI capabilities pointer register"]
pub mod PCI_CFH_CAP_PTR {
    #[doc = "This register is used to point to a linked list of new capabilities implemented by this device"]
    pub mod CAP_PTR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe capabilities list register"]
pub mod PCI_CFC_PCIE_CAP_LIST {
    #[doc = "Indicates the PCI Express Capability structure. Hardwired to 10h."]
    pub mod CAP_ID {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    pub mod NEXT_CAP_PTR {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe capabilities register"]
pub mod PCI_CFC_PCIE_CAP {
    #[doc = "Capability Version Indicates PCI-SIG defined PCI Express Capability structure version number"]
    pub mod CAP_VER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device/Port type Indicates the specific type of this PCI Express Function"]
    pub mod DEV_PORT_TYPE {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt message number This field indicates which MSI/MSI-X vector is used for the interrupt message generated in association with any of the status bits of this capability structure"]
    pub mod INT_MSG_NUM {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe device capabilities register"]
pub mod PCI_CFC_PCIE_DEV_CAP {
    #[doc = "Function level reset capability A value of 1b indicates the function supports the optional Function Level Reset (FLR) mechanism"]
    pub mod FLR_CAP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe device control register"]
pub mod PCI_CFC_PCIE_DEV_CTL {
    #[doc = "Initiate function level reset A write of b1 initiates Function Level Reset (FLR)"]
    pub mod INIT_FLR {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe device status register"]
pub mod PCI_CFC_PCIE_DEV_STAT {
    #[doc = "Transaction pending If set indicates that the Function has outstanding transactions on its external master interface"]
    pub mod TRANS_PEND {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe device capabilities 2 register"]
pub mod PCI_CFC_PCIE_DEV_CAP2 {
    #[doc = "Completion Timeout Ranges Supported Completion Timeout programming not supported, the Function assumes a timeout value in the range 50 us to 50 ms"]
    pub mod CMPL_TO_RNG_SUPP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Completion Timeout Disable Supported When set (b1) indicates support for the Completion Timeout Disable mechanism"]
    pub mod CMPL_TO_DIS_SUPP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe device control 2 register"]
pub mod PCI_CFC_PCIE_DEV_CTL2 {
    #[doc = "Completion Timeout Value Completion Timeout programming not supported - the Function assumes a timeout value in the range 50 us to 50 ms"]
    pub mod CMPL_TO_VALUE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Completion Timeout Enable Not supported. Hardwired to b0."]
    pub mod CMPL_TO_EN {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI MSI-X capabilities list register"]
pub mod PCI_CFC_MSIX_CAP_LIST {
    #[doc = "Indicates the MSI-X Capability structure. Hardwired to 11h."]
    pub mod CAP_ID {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    pub mod NEXT_CAP_PTR {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI MSI-X message control register"]
pub mod PCI_CFC_MSIX_MSG_CTL {
    #[doc = "Table size System software reads this field to determine the MSI-X Table Size N, which is encoded as N-1"]
    pub mod TABLE_SIZE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Function mask If 1, all of the vectors associated with the function are masked, regardless of their per-vector Mask bit states"]
    pub mod FUNC_MASK {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MSI-X enable If set, the function is permitted to use MSI-X to request service"]
    pub mod MSIX_EN {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI MSI-X table offset/BIR register"]
pub mod PCI_CFC_MSIX_TABLE_OFF_BIR {
    #[doc = "Table BIR Indicates which entry in the Enhanced Allocation capability with a matching BEI, is used to map the Function's MSI-X Table into Memory Space"]
    pub mod TABLE_BIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Table Offset Used as an offset from the address contained by one of the function's Base Address registers to point to the base of the MSI-X Table"]
    pub mod TABLE_OFFSET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI MSI-X PBA offset/BIR register"]
pub mod PCI_CFC_MSIX_PBA_OFF_BIR {
    #[doc = "PBA BIR Indicates which entry in the Enhanced Allocation capability with a matching BEI, is used to map the Function's MSI-X PBA into Memory Space"]
    pub mod PBA_BIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PBA Offset Used as an offset from the address contained by one of the function's Base Address registers to point to the base of the MSI-X PBA"]
    pub mod PBA_OFFSET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER extended capability header"]
pub mod PCIE_CFC_AER_EXT_CAP_HDR {
    #[doc = "The Extended Capability ID for the AER Extended Capability is 0001h."]
    pub mod PCIE_EXT_CAP_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    pub mod CAP_VER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    pub mod NEXT_CAP_OFF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER uncorrectable error status register"]
pub mod PCIE_CFC_AER_UCORR_ERR_STAT {
    #[doc = "Uncorrectable internal error Set to indicate some uncorrectable internal error such as a multi-bit error in an internal memory has occurred"]
    pub mod UCORR_INT_ERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER uncorrectable error mask register"]
pub mod PCIE_CFC_AER_UCORR_ERR_MASK {
    #[doc = "Uncorrectable internal error mask 0 Not masked 1 Masked This bit is \"sticky\" and will be preserved across function level resets"]
    pub mod UCORR_INT_ERR_MASK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER uncorrectable error severity register"]
pub mod PCIE_CFC_AER_UCORR_ERR_SEV {
    #[doc = "Uncorrectable internal severity 0 Non-fatal 1 Fatal This bit is \"sticky\" and will be preserved across function level resets"]
    pub mod UCORR_INT_SEV {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER correctable error status register"]
pub mod PCIE_CFC_AER_CORR_ERR_STAT {
    #[doc = "Correctable internal error Set to indicate some correctable internal error has occurred such as a single bit error in an internal memory"]
    pub mod CORR_INT_ERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER correctable error mask register"]
pub mod PCIE_CFC_AER_CORR_ERR_MASK {
    #[doc = "Correctable internal error mask 0 Not masked 1 Masked This bit is \"sticky\" and will be preserved across function level resets"]
    pub mod CORR_INT_MASK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER capabilities and control register"]
pub mod PCIE_CFC_AER_CAP_CTL {
    #[doc = "First error pointer The First Error Pointer is a field that identifies the bit position of the first error reported in the Uncorrectable Error Status register"]
    pub mod FIRST_ERR_PTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe ACS capability header"]
pub mod PCIE_CFC_ACS_CAP_HDR {
    #[doc = "The Extended Capability ID for the ACS Extended Capability is 000Dh."]
    pub mod PCIE_EXT_CAP_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    pub mod CAP_VER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    pub mod NEXT_CAP_OFF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe ACS capability register"]
pub mod PCIE_CFC_ACS_CAP {
    #[doc = "ACS translation blocking (B) Set to indicate that the Function supports Translation Blocking."]
    pub mod ACS_TRANS_BLOCK {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACS P2P request redirect (R) Set to indicate that the Function supports peer-to-peer request redirection"]
    pub mod ACS_P2P_REQ_REDIR {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACS direct translated P2P (T) Set to indicate that the Function supports direct translated peer-to-peer"]
    pub mod ACS_DIR_TRANS_P2P {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe ACS control register"]
pub mod PCIE_CFC_ACS_CTL {
    #[doc = "ACS translation blocking enable (B) When set the Function directs peer-to-peer requests to the SoC interconnect"]
    pub mod ACS_TRANS_BLOCK_EN {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACS P2P request redirect enable (R) When set the Function directs peer-to-peer requests to the SoC interconnect"]
    pub mod ACS_P2P_REQ_REDIR_EN {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACS direct translated P2P enable (T) When set the Function determines how to direct peer-to-peer requests based on ATS Ignored if ACS Translation Blocking Enable is set"]
    pub mod ACS_DIR_TRANS_P2P_EN {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe readiness time reporting capability header"]
pub mod PCIE_CFC_RTR_CAP_HDR {
    #[doc = "The Extended Capability ID for the Readiness Time Reporting Extended Capability is 0022h."]
    pub mod PCIE_EXT_CAP_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    pub mod CAP_VER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    pub mod NEXT_CAP_OFF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe RTR readiness time reporting 1 register"]
pub mod PCIE_CFC_RTR_RTR1 {
    #[doc = "Reset Time"]
    pub mod RESET_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe RTR readiness time reporting 2 register"]
pub mod PCIE_CFC_RTR_RTR2 {
    #[doc = "FLR Time"]
    pub mod FLR_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D3 hot to D0 time"]
    pub mod D3HOT_D0_TIME {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
