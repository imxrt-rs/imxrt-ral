#[doc = "PCI Express ECAM Event Collector config"]
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
    _reserved1: [u8; 0x1d],
    #[doc = "PCI subsystem vendor ID register"]
    pub PCI_CFH_SUBSYS_VID: crate::RORegister<u16>,
    #[doc = "PCI subsystem ID register"]
    pub PCI_CFH_SUBSYS_ID: crate::RORegister<u16>,
    _reserved2: [u8; 0x04],
    #[doc = "PCI capabilities pointer register"]
    pub PCI_CFH_CAP_PTR: crate::RORegister<u8>,
    _reserved3: [u8; 0x07],
    #[doc = "PCI interrupt line register"]
    pub PCI_CFH_INT_LINE: crate::RWRegister<u8>,
    #[doc = "PCI interrupt pin register"]
    pub PCI_CFH_INT_PIN: crate::RORegister<u8>,
    _reserved4: [u8; 0x02],
    #[doc = "PCI PCIe capabilities list register"]
    pub PCI_CFC_PCIE_CAP_LIST: crate::RORegister<u16>,
    #[doc = "PCI PCIe capabilities register"]
    pub PCI_CFC_PCIE_CAP: crate::RORegister<u16>,
    #[doc = "PCI PCIe device capabilities register"]
    pub PCI_CFC_PCIE_DEV_CAP: crate::RORegister<u32>,
    _reserved5: [u8; 0x02],
    #[doc = "PCI PCIe device status register"]
    pub PCI_CFC_PCIE_DEV_STAT: crate::RORegister<u16>,
    _reserved6: [u8; 0x10],
    #[doc = "PCI PCIe root control register"]
    pub PCI_CFC_PCIE_ROOT_CTL: crate::RWRegister<u16>,
    _reserved7: [u8; 0x02],
    #[doc = "PCI PCIe root status register"]
    pub PCI_CFC_PCIE_ROOT_STAT: crate::RWRegister<u32>,
    _reserved8: [u8; 0x1c],
    #[doc = "PCI PCI-PM capabilities list register"]
    pub PCI_CFC_PCIPM_CAP_LIST: crate::RORegister<u16>,
    #[doc = "PCI PCI-PM capabilities register"]
    pub PCI_CFC_PCIPM_CAP: crate::RORegister<u16>,
    #[doc = "PCI PCI-PM control and status register"]
    pub PCI_CFC_PCIPM_CTL_STAT: crate::RWRegister<u16>,
    _reserved9: [u8; 0x01],
    #[doc = "PCI PCI-PM capabilities data register"]
    pub PCI_CFC_PCIPM_DATA: crate::RORegister<u8>,
    _reserved10: [u8; 0x78],
    #[doc = "PCIe AER extended capability header"]
    pub PCIE_CFC_AER_EXT_CAP_HDR: crate::RORegister<u32>,
    _reserved11: [u8; 0x28],
    #[doc = "PCIe AER root error command register"]
    pub PCIE_CFC_AER_ROOT_ERR_CMD: crate::RWRegister<u32>,
    #[doc = "PCIe AER root error status register"]
    pub PCIE_CFC_AER_ROOT_ERR_STAT: crate::RWRegister<u32>,
    #[doc = "PCIe AER error source identification register"]
    pub PCIE_CFC_AER_ERR_SRC_ID: crate::RORegister<u32>,
    #[doc = "PCIe RCEC Endpoint association extended capability header"]
    pub PCIE_CFC_RCEC_EPA_EXT_CAP_HDR: crate::RORegister<u32>,
    #[doc = "PCIe RCEC Endpoint association bitmap registerr"]
    pub PCIE_CFC_RCEC_EPA_BITMAP: crate::RORegister<u32>,
}
#[doc = "PCI device ID and vendor ID register"]
pub mod PCI_CFH_DID_VID {
    #[doc = "Vendor ID This field identifies the manufacturer of the device"]
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
    #[doc = "Interrupt disable This field controls the ability of an Event Collect to assert the INTx wired interrupts"]
    pub mod INTR_DISABLE {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI status register"]
pub mod PCI_CFH_STAT {
    #[doc = "Interrupt Status When set, indicates that an INTx interrupt is pending for the Event Collector"]
    pub mod INTR_STATUS {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
    #[doc = "Multi-function device When set, indicates that the Root Complex contains multiple Event Collectors."]
    pub mod MULT_FUNC_DEV {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI subsystem vendor ID register"]
pub mod PCI_CFH_SUBSYS_VID {
    #[doc = "This read only field identifies the manufacturer of the subsystem"]
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
    #[doc = "This read only field identifies the particular subsystem"]
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
#[doc = "PCI interrupt line register"]
pub mod PCI_CFH_INT_LINE {
    #[doc = "Interrupt Line register communicates interrupt line routing information"]
    pub mod INT_LINE {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI interrupt pin register"]
pub mod PCI_CFH_INT_PIN {
    #[doc = "The Interrupt Pin register is a read-only register that identifies the legacy interrupt Message(s) the function uses"]
    pub mod INT_PIN {
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
    pub mod NEXT_CAP_ID {
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
    #[doc = "Function level reset capability This bit applies to Endpoints only, hardwired to 0b."]
    pub mod FLR_CAP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe device status register"]
pub mod PCI_CFC_PCIE_DEV_STAT {
    #[doc = "Transaction pending When set, this bit indicates that the Event Collector has issued Non-Posted Requests that have not been completed"]
    pub mod TRANS_PEND {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe root control register"]
pub mod PCI_CFC_PCIE_ROOT_CTL {
    #[doc = "PME interrupt enable When Set, this bit enables PME interrupt generation upon receipt of a PME Message as reflected in the PME Status bit"]
    pub mod PME_INT_EN {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCIe root status register"]
pub mod PCI_CFC_PCIE_ROOT_STAT {
    #[doc = "PME requester ID This field indicates the PCI Requester ID of the last PME Requester"]
    pub mod PME_REQ_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PME status This bit indicates that PME was asserted by the PME Requester indicated in the PME Requester ID field"]
    pub mod PME_STATUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PME pending This bit indicates that another PME is pending when the PME Status bit is Set"]
    pub mod PME_PEND {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
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
    pub mod NEXT_CAP_ID {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCI PCI-PM capabilities register"]
pub mod PCI_CFC_PCIPM_CAP {
    #[doc = "Version RCEC complies with the PCI PM specification, rev 1.2."]
    pub mod VERSION {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PME support Event Collector does not support generating PM_PME notifications, hardwired to 00h."]
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
    #[doc = "Power state This field is used to set and report the power state of a function as follows: 00b = D0 01b = D1 (not supported by EC) 10b = D2 (not supported by EC) 11b = D3 If, for any reason, the operating system software attempts to put a function into a power management state that the function does not support, the function should handle this gracefully and remain in whatever state it was in before the request"]
    pub mod PWR_STATE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No soft reset When set (\"1\"), this bit indicates that when RCEC transitions from D3hot to D0active because of modifying Power State bits in the PCI_CFC_PCIPM_CTL_STAT register, no internal reset is issued and Configuration Context is preserved"]
    pub mod NO_SOFT_RST {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
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
#[doc = "PCIe AER root error command register"]
pub mod PCIE_CFC_AER_ROOT_ERR_CMD {
    #[doc = "Correctable error reporting enable When set, this bit enables the generation of an interrupt when a correctable error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    pub mod CORR_ERR_RPT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-fatal error reporting enable When set, this bit enables the generation of an interrupt when a non-fatal error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    pub mod NON_FATAL_ERR_RPT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fatal error reporting enable When set, this bit enables the generation of an interrupt when a fatal error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    pub mod FATAL_ERR_RPT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER root error status register"]
pub mod PCIE_CFC_AER_ROOT_ERR_STAT {
    #[doc = "Correctable error received Set when a correctable error message is received and this bit is not already set"]
    pub mod ERR_CORR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple correctable errors received Set when a correctable error message is received and ERR_CORR received is already set"]
    pub mod MULT_ERR_CORR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fatal/Non-fatal error received Set when either a fatal or a non-fatal error message is received and this bit is not already set"]
    pub mod ERR_FATAL_NON_FATAL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple fatal/non-fatal errors received Set when either a fatal or a non-fatal error is received and ERR_FATAL_NON_FATAL received is already set"]
    pub mod MULT_ERR_FATAL_NON_FATAL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First uncorrectable error fatal Set when the first uncorrectable error message received is for a fatal error"]
    pub mod FIRST_UCORR_FATAL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-fatal error received Set when one or more non-fatal uncorrectable error messages have been received"]
    pub mod ERR_NON_FATAL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fatal error received Set when one or more fatal uncorrectable error messages have been received"]
    pub mod ERR_FATAL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe AER error source identification register"]
pub mod PCIE_CFC_AER_ERR_SRC_ID {
    #[doc = "ERR_CORR Source Identification Loaded with the Requester ID indicated in the received ERR_CORR message when the ERR_CORR received bit is not already set"]
    pub mod ERR_CORR_SRC_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERR_FATAL_NON_FATAL Source Identification Loaded with the Requester ID indicated in the received ERR_FATAL_NON_FATAL message when the ERR_FATAL_NON_FATAL received bit is not already set"]
    pub mod ERR_FATAL_NON_FATAL_SRC_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PCIe RCEC Endpoint association extended capability header"]
pub mod PCIE_CFC_RCEC_EPA_EXT_CAP_HDR {
    #[doc = "The Extended Capability ID for the Event Collector Endpoint Association Capability is 0007h."]
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
#[doc = "PCIe RCEC Endpoint association bitmap registerr"]
pub mod PCIE_CFC_RCEC_EPA_BITMAP {
    #[doc = "Associated Endpoint device bitmap 31-0."]
    pub mod DEV_BITMAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
