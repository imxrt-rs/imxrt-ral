#[doc = "Event Collector Integrated Endpoint Register Block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Function 0 EC config header device ID and vendor ID register"]
    pub F0_EC_CFH_DIDVID: crate::RWRegister<u32>,
    #[doc = "Function 0 EC config header subsystem ID and subsystem vendor ID register"]
    pub F0_EC_CFH_SIDSVID: crate::RWRegister<u32>,
}
#[doc = "Function 0 EC config header device ID and vendor ID register"]
pub mod F0_EC_CFH_DIDVID {
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
#[doc = "Function 0 EC config header subsystem ID and subsystem vendor ID register"]
pub mod F0_EC_CFH_SIDSVID {
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
