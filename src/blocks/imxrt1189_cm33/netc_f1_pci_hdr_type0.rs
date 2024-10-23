#[doc = "NETC PCI Express ECAM PF config"]
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
    pub PCI_CFH_CL_SIZE: crate::RWRegister<u8>,
    _reserved0: [u8; 0x01],
    #[doc = "PCI header type register"]
    pub PCI_CFH_HDR_TYPE: crate::RORegister<u8>,
    _reserved1: [u8; 0x01],
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
    _reserved2: [u8; 0x04],
    #[doc = "PCI subsystem vendor ID register"]
    pub PCI_CFH_SUBSYS_VID: crate::RORegister<u16>,
    #[doc = "PCI subsystem ID register"]
    pub PCI_CFH_SUBSYS_ID: crate::RORegister<u16>,
    _reserved3: [u8; 0x04],
    #[doc = "PCI capabilities pointer register"]
    pub PCI_CFH_CAP_PTR: crate::RORegister<u8>,
    _reserved4: [u8; 0x0b],
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
    _reserved5: [u8; 0x18],
    #[doc = "PCI PCIe device capabilities 2 register"]
    pub PCI_CFC_PCIE_DEV_CAP2: crate::RORegister<u32>,
    #[doc = "PCI PCIe device control 2 register"]
    pub PCI_CFC_PCIE_DEV_CTL2: crate::RORegister<u16>,
    _reserved6: [u8; 0x16],
    #[doc = "PCI MSI-X capabilities list register"]
    pub PCI_CFC_MSIX_CAP_LIST: crate::RORegister<u16>,
    #[doc = "PCI MSI-X message control register"]
    pub PCI_CFC_MSIX_MSG_CTL: crate::RWRegister<u16>,
    #[doc = "PCI MSI-X table offset/BIR register"]
    pub PCI_CFC_MSIX_TABLE_OFF_BIR: crate::RORegister<u32>,
    #[doc = "PCI MSI-X PBA offset/BIR register"]
    pub PCI_CFC_MSIX_PBA_OFF_BIR: crate::RORegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "PCI PCI-PM capabilities list register"]
    pub PCI_CFC_PCIPM_CAP_LIST: crate::RORegister<u16>,
    #[doc = "PCI PCI-PM capabilities register"]
    pub PCI_CFC_PCIPM_CAP: crate::RORegister<u16>,
    #[doc = "PCI PCI-PM control and status register"]
    pub PCI_CFC_PCIPM_CTL_STAT: crate::RWRegister<u16>,
    _reserved8: [u8; 0x01],
    #[doc = "PCI PCI-PM capabilities data register"]
    pub PCI_CFC_PCIPM_DATA: crate::RORegister<u8>,
    _reserved9: [u8; 0x04],
    #[doc = "PCI EA capabilities list register"]
    pub PCI_CFC_EA_CAP_LIST: crate::RORegister<u16>,
    #[doc = "PCI EA capabilities register"]
    pub PCI_CFC_EA_CAP: crate::RORegister<u16>,
    #[doc = "PCI EA per-entry 0 format register"]
    pub PCI_CFC_EA_PE0_FMT: crate::RORegister<u32>,
    #[doc = "PCI EA per-entry 0 base register"]
    pub PCI_CFC_EA_PE0_BASE: crate::RORegister<u32>,
    #[doc = "PCI EA per-entry 0 max offset register"]
    pub PCI_CFC_EA_PE0_MAXOFF: crate::RORegister<u32>,
    #[doc = "PCI EA per-entry 0 extended base register"]
    pub PCI_CFC_EA_PE0_EXT_BASE: crate::RORegister<u32>,
    #[doc = "PCI EA per-entry 1 format register"]
    pub PCI_CFC_EA_PE1_FMT: crate::RORegister<u32>,
    #[doc = "PCI EA per-entry 1 base register"]
    pub PCI_CFC_EA_PE1_BASE: crate::RORegister<u32>,
    #[doc = "PCI EA per-entry 1 max offset register"]
    pub PCI_CFC_EA_PE1_MAXOFF: crate::RORegister<u32>,
    #[doc = "PCI EA per-entry 1 extended base register"]
    pub PCI_CFC_EA_PE1_EXT_BASE: crate::RORegister<u32>,
    _reserved10: [u8; 0x40],
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
    _reserved11: [u8; 0x14],
    #[doc = "PCIe ACS capability header"]
    pub PCIE_CFC_ACS_CAP_HDR: crate::RORegister<u32>,
    #[doc = "PCIe ACS capability register"]
    pub PCIE_CFC_ACS_CAP: crate::RORegister<u16>,
    #[doc = "PCIe ACS control register"]
    pub PCIE_CFC_ACS_CTL: crate::RORegister<u16>,
    _reserved12: [u8; 0x08],
    #[doc = "PCIe readiness time reporting capability header"]
    pub PCIE_CFC_RTR_CAP_HDR: crate::RORegister<u32>,
    #[doc = "PCIe RTR readiness time reporting 1 register"]
    pub PCIE_CFC_RTR_RTR1: crate::RORegister<u32>,
    #[doc = "PCIe RTR readiness time reporting 2 register"]
    pub PCIE_CFC_RTR_RTR2: crate::RORegister<u32>,
}
#[doc = "PCI device ID and vendor ID register"]
pub mod PCI_CFH_DID_VID {
    #[doc = "Vendor ID This field identifies the manufacturer of the device."]
    pub mod VENDOR_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device ID This field identifies the device ID of the device"]
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
    #[doc = "Controls a device's response to memory space accesses. 0 Disabled 1 Enabled"]
    pub mod MEM_ACCESS {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
    #[doc = "Class code The Class Code register is read-only and is used to identify the generic function of the device and, in some cases, a specific register level programming interface"]
    pub mod CLASS_CODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI cache line size register"]
pub mod PCI_CFH_CL_SIZE {
    #[doc = "Cache line size The Cache Line Size register is set by the system firmware or the operating system to system cache line size"]
    pub mod CACHE_LINE_SIZE {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
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
    #[doc = "Multi-function device When set, indicates that the device may contain multiple functions, but not necessarily"]
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
#[doc = "PCI subsystem vendor ID register"]
pub mod PCI_CFH_SUBSYS_VID {
    #[doc = "This Read Only field identifies the manufacturer of the subsystem"]
    pub mod SUBSYSTEM_VENDOR_ID {
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
    pub mod SUBSYSTEM_ID {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
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
    #[doc = "Completion Timeout Disable Supported Not supported. Hardwired to b0."]
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
#[doc = "PCI PCI-PM capabilities list register"]
pub mod PCI_CFC_PCIPM_CAP_LIST {
    #[doc = "Indicates the PCI-PM Capability structure. Hardwired to 01h."]
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
#[doc = "PCI PCI-PM capabilities register"]
pub mod PCI_CFC_PCIPM_CAP {
    #[doc = "Version ENETC complies with the PCI PM specification, rev 1.2."]
    pub mod VERSION {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PME support Indicates the PM states within which the function is capable of sending PME message"]
    pub mod PME_SUPPORT {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCI-PM control and status register"]
pub mod PCI_CFC_PCIPM_CTL_STAT {
    #[doc = "Power state This field is used to set and report the power state of a function as follows: 00b = D0 01b = D1 (not supported by NETC) 10b = D2 (not supported by NETC) 11b = D3 If, for any reason, the operating system software attempts to put a function into a power management state that the function does not support, the function should handle this gracefully and remain in whatever state it was in before the request"]
    pub mod PWR_STATE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No soft reset When set (\"1\"), this bit indicates that when NETC transitions from D3hot to D0active because of modifying Power State bits in the PCI_CFC_PCIPM_CTL_STAT register, no internal reset is issued and Configuration Context is preserved"]
    pub mod NO_SOFT_RST {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA capabilities list register"]
pub mod PCI_CFC_EA_CAP_LIST {
    #[doc = "Indicates the Enhanced Allocation (EA) Capability structure. Hardwired to 14h."]
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
#[doc = "PCI EA capabilities register"]
pub mod PCI_CFC_EA_CAP {
    #[doc = "Number of entries Number of entries following the first DW of the capability."]
    pub mod NUM_ENTRIES {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA per-entry 0 format register"]
pub mod PCI_CFC_EA_PE0_FMT {
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    pub mod ENTRY_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    pub mod BEI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    pub mod PRIM_PROP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    pub mod SEC_PROP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    pub mod WRITABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA per-entry 0 base register"]
pub mod PCI_CFC_EA_PE0_BASE {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    pub mod S {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Base DW address of the start of the resource range"]
    pub mod BASE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA per-entry 0 max offset register"]
pub mod PCI_CFC_EA_PE0_MAXOFF {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    pub mod S {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    pub mod MAX_OFFSET {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA per-entry 0 extended base register"]
pub mod PCI_CFC_EA_PE0_EXT_BASE {
    #[doc = "Base DW high address of the start of the resource range"]
    pub mod BASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA per-entry 1 format register"]
pub mod PCI_CFC_EA_PE1_FMT {
    #[doc = "Entry size Number of DWs following the initial DW in this entry"]
    pub mod ENTRY_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BAR equivalent indicator This field indicates the equivalent PCI BAR for this entry"]
    pub mod BEI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Primary properties Value (h) Resource Definition 00 Memory Space, Non-Prefetchable"]
    pub mod PRIM_PROP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secondary properties Alternative property when system cannot use the primary property."]
    pub mod SEC_PROP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Writable 1b indicates that the Base, MaxOffset and Field Size fields for this entry are RW, 0b indicates those fields are HWInit"]
    pub mod WRITABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable 0 Disabled 1 Enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA per-entry 1 base register"]
pub mod PCI_CFC_EA_PE1_BASE {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    pub mod S {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Base DW address of the start of the resource range"]
    pub mod BASE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA per-entry 1 max offset register"]
pub mod PCI_CFC_EA_PE1_MAXOFF {
    #[doc = "Field size 0 32-bit 1 64-bit"]
    pub mod S {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Max offset The value in the Base field (\\[63:2\\]) plus the value in the MaxOffset field indicates the address of the last included DW of the resource range"]
    pub mod MAX_OFFSET {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI EA per-entry 1 extended base register"]
pub mod PCI_CFC_EA_PE1_EXT_BASE {
    #[doc = "Base DW high address of the start of the resource range"]
    pub mod BASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
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
    #[doc = "ACS violation status Set to indicate an ACS violation such as an access from a disallowed source"]
    pub mod ACS_VIOLATION_STAT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
    #[doc = "ACS P2P request dedirect (R) Set to indicate that the Function supports peer-to-peer request redirection"]
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
    #[doc = "ACS P2P request dedirect enable (R) When set the Function directs peer-to-peer requests to the SoC interconnect"]
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
